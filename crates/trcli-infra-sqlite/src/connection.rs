//! Opening and creating a workspace's database (FR-002, FR-008, FR-009).
//!
//! Every connection uses a write-ahead journal, so that a second `trcli` can read while
//! one writes; enforces foreign keys; and waits up to the configured time when another
//! command holds the database. Changes are made in transactions begun in immediate mode:
//! a second writer waits or is told the workspace is busy — it is never let in half-way.
//!
//! A database on a disk that cannot be written to is opened as it is, without the
//! journal, so that commands that only read keep working.

use std::path::{Path, PathBuf};
use std::time::Duration;

use sea_orm::sqlx::sqlite::SqliteJournalMode;
use sea_orm::{
    ConnectOptions, ConnectionTrait, Database, DatabaseConnection, DbBackend, DbErr,
    SqliteTransactionMode, Statement, TransactionOptions, TransactionTrait,
};
use sea_orm_migration::MigratorTrait;
use trcli_application::ports::unit_of_work::{Storage, StoreError};
use trcli_application::ports::workspace::StorageOpener;

use crate::convert::store_error;
use crate::migrations::{EXPECTED_TABLES, Migrator};
use crate::unit_of_work::SqliteUnit;

/// How a database file is opened.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum FileAccess {
    /// Create the file; it must not exist.
    Create,
    /// Open an existing file for reading and writing.
    ReadWrite,
    /// Open an existing file that cannot be written to, exactly as it is.
    Unwritable,
}

/// Opens one connection pool on a database file.
async fn connect(
    path: &Path,
    busy_timeout: Duration,
    access: FileAccess,
) -> Result<DatabaseConnection, DbErr> {
    let file = path.to_path_buf();
    // The address here is a placeholder: the real file is set below, as a path, so that
    // spaces and other characters in it need no escaping.
    let mut options = ConnectOptions::new("sqlite:trcli");
    options
        .sqlx_logging(false)
        .max_connections(4)
        .map_sqlx_sqlite_opts(move |sqlite| {
            let sqlite = sqlite
                .filename(&file)
                .foreign_keys(true)
                .busy_timeout(busy_timeout);
            match access {
                FileAccess::Create => sqlite
                    .create_if_missing(true)
                    .journal_mode(SqliteJournalMode::Wal),
                FileAccess::ReadWrite => sqlite
                    .create_if_missing(false)
                    .journal_mode(SqliteJournalMode::Wal),
                FileAccess::Unwritable => sqlite
                    .create_if_missing(false)
                    .read_only(true)
                    .immutable(true),
            }
        });
    Database::connect(options).await
}

/// Damage found in the database at `path`.
fn damaged(what: impl Into<String>, path: &Path) -> StoreError {
    StoreError::Damaged {
        what: what.into(),
        location: path.display().to_string(),
    }
}

/// The names of the tables a database holds; an error when it is not a database at all.
async fn table_names(connection: &DatabaseConnection) -> Result<Vec<String>, DbErr> {
    let statement = Statement::from_string(
        DbBackend::Sqlite,
        "SELECT name FROM sqlite_master WHERE type = 'table' ORDER BY name",
    );
    let rows = connection.query_all_raw(statement).await?;
    rows.iter()
        .map(|row| row.try_get_by_index::<String>(0))
        .collect()
}

/// Creates and opens workspaces' databases.
#[derive(Clone, Copy, Debug)]
pub struct SqliteOpener {
    /// How long a connection waits for another command that is writing.
    busy_timeout: Duration,
}

impl SqliteOpener {
    /// An opener whose connections wait up to `busy_timeout_ms` for another writer
    /// (the `storage.busy_timeout_ms` setting).
    pub fn new(busy_timeout_ms: u64) -> Self {
        Self {
            busy_timeout: Duration::from_millis(busy_timeout_ms),
        }
    }

    /// Opens an existing file, falling back to opening it as it is when it cannot be
    /// written to.
    async fn connect_existing(&self, database: &Path) -> Result<DatabaseConnection, StoreError> {
        match connect(database, self.busy_timeout, FileAccess::ReadWrite).await {
            Ok(connection) => Ok(connection),
            Err(first) => {
                match connect(database, self.busy_timeout, FileAccess::Unwritable).await {
                    Ok(connection) => Ok(connection),
                    // The first error says more about the file than the fallback's would.
                    Err(_) => Err(damaged(
                        format!("the database cannot be opened: {first}"),
                        database,
                    )),
                }
            }
        }
    }
}

impl StorageOpener for SqliteOpener {
    type Storage = SqliteStorage;

    async fn create(&self, database: &Path) -> Result<SqliteStorage, StoreError> {
        if database.exists() {
            return Err(StoreError::AlreadyExists(format!(
                "a workspace's database at {}",
                database.display()
            )));
        }
        let connection = connect(database, self.busy_timeout, FileAccess::Create)
            .await
            .map_err(store_error)?;
        Migrator::up(&connection, None).await.map_err(store_error)?;
        #[cfg(feature = "sample-kind")]
        crate::sample::ensure_tables(&connection)
            .await
            .map_err(store_error)?;
        Ok(SqliteStorage {
            connection,
            path: database.to_path_buf(),
        })
    }

    async fn open(&self, database: &Path) -> Result<SqliteStorage, StoreError> {
        if !database.is_file() {
            return Err(damaged("the database file is missing", database));
        }
        let connection = self.connect_existing(database).await?;
        let tables = table_names(&connection).await.map_err(|error| {
            damaged(
                format!("the file is not a valid database: {error}"),
                database,
            )
        })?;
        if let Some(missing) = EXPECTED_TABLES
            .iter()
            .find(|expected| !tables.iter().any(|table| table == *expected))
        {
            return Err(damaged(
                format!("the table `{missing}` is missing"),
                database,
            ));
        }
        // The sample kinds' tables are not part of the workspace format; a build that has
        // the sample kinds adds them to any workspace it opens. Failing to (an unwritable
        // disk) only means the sample commands will not work there.
        #[cfg(feature = "sample-kind")]
        let _ = crate::sample::ensure_tables(&connection).await;
        Ok(SqliteStorage {
            connection,
            path: database.to_path_buf(),
        })
    }
}

/// One workspace's database.
#[derive(Clone, Debug)]
pub struct SqliteStorage {
    /// The connection pool.
    connection: DatabaseConnection,
    /// The database file, for messages.
    path: PathBuf,
}

impl SqliteStorage {
    /// The database file.
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Folds the write-ahead journal back into the database file, so that the file alone
    /// holds everything committed. Done before the file is copied (FR-007).
    pub async fn checkpoint(&self) -> Result<(), StoreError> {
        self.connection
            .execute_unprepared("PRAGMA wal_checkpoint(TRUNCATE)")
            .await
            .map(|_| ())
            .map_err(store_error)
    }
}

impl Storage for SqliteStorage {
    type Unit = SqliteUnit;

    async fn begin(&self) -> Result<SqliteUnit, StoreError> {
        // Immediate mode takes the write lock now: a second writer waits here, up to the
        // busy timeout, instead of failing in the middle of its work (FR-009).
        let options = TransactionOptions {
            sqlite_transaction_mode: Some(SqliteTransactionMode::Immediate),
            ..TransactionOptions::default()
        };
        let transaction = self
            .connection
            .begin_with_options(options)
            .await
            .map_err(store_error)?;
        Ok(SqliteUnit::new(transaction))
    }

    async fn read(&self) -> Result<SqliteUnit, StoreError> {
        let transaction = self.connection.begin().await.map_err(store_error)?;
        Ok(SqliteUnit::new(transaction))
    }

    async fn close(self) -> Result<(), StoreError> {
        // Closing the last connection folds the write-ahead journal back into the
        // database file and removes it: between commands a workspace is that one file.
        self.connection.close().await.map_err(store_error)
    }
}

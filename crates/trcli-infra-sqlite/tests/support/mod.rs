//! Shared by this crate's integration tests: fresh databases in temporary directories.

#![allow(dead_code)] // each test file uses a part of this module

use std::path::PathBuf;
use std::sync::atomic::{AtomicU32, Ordering};

use tempfile::TempDir;
use trcli_application::ports::workspace::StorageOpener;
use trcli_infra_sqlite::connection::{SqliteOpener, SqliteStorage};

/// A temporary directory that makes a fresh database each time it is asked.
pub struct Databases {
    /// The directory; removed when this value is dropped.
    directory: TempDir,
    /// How many databases were made, to name the next one.
    made: AtomicU32,
}

impl Databases {
    /// A new, empty temporary directory.
    pub fn new() -> Self {
        Self {
            directory: tempfile::tempdir().expect("a temporary directory"),
            made: AtomicU32::new(0),
        }
    }

    /// The path the next database will have.
    pub fn next_path(&self) -> PathBuf {
        let number = self.made.fetch_add(1, Ordering::Relaxed);
        self.directory.path().join(format!("workspace-{number}.db"))
    }

    /// The directory itself.
    pub fn path(&self) -> &std::path::Path {
        self.directory.path()
    }

    /// A fresh, empty database with the current schema.
    pub async fn fresh(&self) -> SqliteStorage {
        SqliteOpener::new(200)
            .create(&self.next_path())
            .await
            .expect("a new database")
    }
}

/// A plain connection to a database file, as something other than the adapter would
/// open it. The file is given as a path, so that no character of it needs escaping on
/// any system.
pub async fn plain_connection(path: &std::path::Path) -> sea_orm::DatabaseConnection {
    let file = path.to_path_buf();
    let mut options = sea_orm::ConnectOptions::new("sqlite:trcli-test");
    options
        .sqlx_logging(false)
        .map_sqlx_sqlite_opts(move |sqlite| sqlite.filename(&file));
    sea_orm::Database::connect(options)
        .await
        .expect("a plain connection")
}

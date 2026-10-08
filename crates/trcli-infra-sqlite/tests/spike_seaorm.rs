//! Spike 1 (research.md, section 3): SeaORM 2.0 on bundled SQLite, on a current-thread
//! tokio runtime, with `time` and `uuid` values. Kept as a regression test of the feature
//! flags chosen in the workspace manifest.

use std::time::Duration;

use sea_orm::sqlx::sqlite::SqliteJournalMode;
use sea_orm::{
    ActiveModelTrait, ConnectOptions, ConnectionTrait, Database, EntityTrait, Set,
    SqliteTransactionMode, TransactionOptions, TransactionTrait,
};
use time::OffsetDateTime;
use uuid::Uuid;

/// The table used by the spike.
mod spike {
    use sea_orm::entity::prelude::*;

    /// One row of the spike table.
    #[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
    #[sea_orm(table_name = "spike")]
    pub struct Model {
        /// Identifier.
        #[sea_orm(primary_key, auto_increment = false)]
        pub id: Uuid,
        /// When it was written.
        pub at: TimeDateTimeWithTimeZone,
    }

    /// The spike table has no relations.
    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}

/// A table is created, a row with a uuid and an instant is written in an immediate
/// transaction and read back unchanged; the journal is write-ahead.
#[tokio::test(flavor = "current_thread")]
async fn sqlite_round_trip_on_a_current_thread_runtime() {
    let directory = tempfile::tempdir().expect("temporary directory");
    let url = format!(
        "sqlite://{}?mode=rwc",
        directory.path().join("spike.db").display()
    );
    let mut options = ConnectOptions::new(url);
    options.sqlx_logging(false).map_sqlx_sqlite_opts(|sqlite| {
        sqlite
            .journal_mode(SqliteJournalMode::Wal)
            .foreign_keys(true)
            .busy_timeout(Duration::from_millis(100))
    });
    let database = Database::connect(options).await.expect("connect");

    database
        .execute_unprepared("CREATE TABLE spike (id TEXT PRIMARY KEY, at TEXT NOT NULL)")
        .await
        .expect("create table");

    let id = Uuid::now_v7();
    let at = OffsetDateTime::UNIX_EPOCH;
    let immediate = TransactionOptions {
        sqlite_transaction_mode: Some(SqliteTransactionMode::Immediate),
        ..TransactionOptions::default()
    };
    let transaction = database.begin_with_options(immediate).await.expect("begin");
    spike::ActiveModel {
        id: Set(id),
        at: Set(at),
    }
    .insert(&transaction)
    .await
    .expect("insert");
    transaction.commit().await.expect("commit");

    let row = spike::Entity::find_by_id(id)
        .one(&database)
        .await
        .expect("query")
        .expect("row");
    assert_eq!(row.at, at);
    let mode = database
        .query_one_raw(sea_orm::Statement::from_string(
            sea_orm::DbBackend::Sqlite,
            "PRAGMA journal_mode",
        ))
        .await
        .expect("pragma")
        .expect("one row");
    let mode: String = mode.try_get_by_index(0).expect("text");
    assert_eq!(mode, "wal");
}

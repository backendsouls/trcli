//! The sample feature's storage: one table per kind, whose `id` refers to the record
//! index, and each kind's port implemented on the unit of work.
//!
//! A feature built into the tool adds a migration to the storage crate and its tables
//! become part of the workspace format. This one lives outside and no release has it, so
//! it creates its tables itself, where they are missing, and they are not part of the
//! format.

mod sample_note;
mod specimen;

use sea_orm::{ConnectionTrait, DbErr};
use trcli_application::ports::unit_of_work::StoreError;
use trcli_cli::compose::AppStorage;

/// The statements that create the sample kinds' tables. The foreign key to `record` is
/// what makes "nothing is left pointing at a deleted record" a rule of the database.
const TABLES: &str = "
CREATE TABLE IF NOT EXISTS specimen (
    id    TEXT PRIMARY KEY NOT NULL REFERENCES record (id),
    title TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS sample_note (
    id     TEXT    PRIMARY KEY NOT NULL REFERENCES record (id),
    body   TEXT    NOT NULL,
    locked INTEGER NOT NULL DEFAULT 0
);
";

/// A database error in the application's words.
fn failed(error: DbErr) -> StoreError {
    let text = error.to_string();
    if text.contains("database is locked") {
        StoreError::Busy
    } else {
        StoreError::Failed(text)
    }
}

/// Creates the sample kinds' tables in a workspace that does not have them yet. A
/// workspace that has them is only read, so that reading commands change nothing.
pub async fn ensure_tables(storage: &AppStorage) -> Result<(), StoreError> {
    let connection = storage.connection();
    let existing = sea_orm::Statement::from_string(
        sea_orm::DbBackend::Sqlite,
        "SELECT COUNT(*) FROM sqlite_master WHERE type = 'table' AND name IN ('specimen', 'sample_note')",
    );
    let found: i64 = match connection.query_one_raw(existing).await.map_err(failed)? {
        Some(row) => row.try_get_by_index(0).map_err(failed)?,
        None => 0,
    };
    if found == 2 {
        return Ok(());
    }
    connection
        .execute_unprepared(TABLES)
        .await
        .map(|_| ())
        .map_err(failed)
}

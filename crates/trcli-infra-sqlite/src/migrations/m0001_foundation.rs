//! Migration 1: the workspace's own row and the audit trail.

use sea_orm_migration::async_trait::async_trait;
use sea_orm_migration::sea_orm::ConnectionTrait;
use sea_orm_migration::{DbErr, MigrationName, MigrationTrait, SchemaManager};

/// The statements of this migration.
const UP: &str = "
CREATE TABLE workspace (
    id             TEXT    PRIMARY KEY NOT NULL,
    name           TEXT    NOT NULL,
    description    TEXT    NOT NULL DEFAULT '',
    format_version INTEGER NOT NULL,
    created_at     INTEGER NOT NULL
);

CREATE TABLE audit_entry (
    sequence      INTEGER PRIMARY KEY NOT NULL,
    at            INTEGER NOT NULL,
    actor         TEXT    NOT NULL,
    action        TEXT    NOT NULL,
    kind          TEXT,
    record_id     TEXT,
    handle        TEXT,
    display_name  TEXT,
    changes       TEXT    NOT NULL DEFAULT '[]',
    previous_hash TEXT    NOT NULL,
    hash          TEXT    NOT NULL
);

CREATE INDEX audit_entry_record_id ON audit_entry (record_id);
CREATE INDEX audit_entry_at ON audit_entry (at);
";

/// Creates `workspace` and `audit_entry`.
pub(super) struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m0001_foundation"
    }
}

#[async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute_unprepared(UP)
            .await
            .map(|_| ())
    }
}

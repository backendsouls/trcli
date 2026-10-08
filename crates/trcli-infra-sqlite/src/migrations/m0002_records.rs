//! Migration 2: the record index, tags, notes, and links.
//!
//! Every column that names a record is a foreign key to `record`, which is what makes
//! "nothing is left pointing at a deleted record" a rule of the database rather than a
//! convention (FR-017). A link is unique for a pair and a relation in either direction,
//! which the last index expresses with the smaller and the larger identifier.

use sea_orm_migration::async_trait::async_trait;
use sea_orm_migration::sea_orm::ConnectionTrait;
use sea_orm_migration::{DbErr, MigrationName, MigrationTrait, SchemaManager};

/// The statements of this migration.
const UP: &str = "
CREATE TABLE record (
    id           TEXT    PRIMARY KEY NOT NULL,
    kind         TEXT    NOT NULL,
    handle       TEXT    NOT NULL UNIQUE,
    display_name TEXT    NOT NULL DEFAULT '',
    search_key   TEXT    NOT NULL DEFAULT '',
    created_at   INTEGER NOT NULL,
    updated_at   INTEGER NOT NULL,
    deleted_at   INTEGER
);
CREATE INDEX record_kind_deleted_at ON record (kind, deleted_at);
CREATE INDEX record_search_key ON record (search_key);

CREATE TABLE tag (
    name TEXT PRIMARY KEY NOT NULL
);

CREATE TABLE tagging (
    tag    TEXT NOT NULL REFERENCES tag (name),
    record TEXT NOT NULL REFERENCES record (id),
    PRIMARY KEY (tag, record)
);
CREATE INDEX tagging_record ON tagging (record);
CREATE INDEX tagging_tag ON tagging (tag);

CREATE TABLE note (
    id         INTEGER PRIMARY KEY AUTOINCREMENT,
    record     TEXT    NOT NULL REFERENCES record (id),
    body       TEXT    NOT NULL,
    created_at INTEGER NOT NULL
);
CREATE INDEX note_record ON note (record);

CREATE TABLE link (
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    from_record TEXT    NOT NULL REFERENCES record (id),
    to_record   TEXT    NOT NULL REFERENCES record (id),
    relation    TEXT    NOT NULL,
    created_at  INTEGER NOT NULL,
    CHECK (from_record <> to_record)
);
CREATE INDEX link_from ON link (from_record);
CREATE INDEX link_to ON link (to_record);
CREATE UNIQUE INDEX link_pair ON link (min(from_record, to_record), max(from_record, to_record), relation);
";

/// Creates `record`, `tag`, `tagging`, `note`, and `link`.
pub(super) struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m0002_records"
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

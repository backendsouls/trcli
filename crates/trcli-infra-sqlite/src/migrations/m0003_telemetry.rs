//! Migration 3: local telemetry.

use sea_orm_migration::async_trait::async_trait;
use sea_orm_migration::sea_orm::ConnectionTrait;
use sea_orm_migration::{DbErr, MigrationName, MigrationTrait, SchemaManager};

/// The statements of this migration.
const UP: &str = "
CREATE TABLE telemetry_record (
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    at          INTEGER NOT NULL,
    command     TEXT    NOT NULL,
    duration_ms INTEGER NOT NULL,
    outcome     TEXT    NOT NULL
);
";

/// Creates `telemetry_record`.
pub(super) struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m0003_telemetry"
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

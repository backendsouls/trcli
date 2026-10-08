//! The tables and stores of the sample record kinds (feature `sample-kind`).
//!
//! This is what a feature adds to this crate for a kind of record: a migration for its
//! own table, whose `id` refers to the record index, and the implementation of its port.
//! The sample tables are created by their own set of migrations, with their own
//! bookkeeping table, and are not part of the workspace format: no release has them.

mod sample_note;
mod specimen;

use sea_orm::DbErr;
use sea_orm_migration::async_trait::async_trait;
use sea_orm_migration::sea_orm::ConnectionTrait;
use sea_orm_migration::sea_query::{Alias, DynIden, IntoIden};
use sea_orm_migration::{
    IntoSchemaManagerConnection, MigrationName, MigrationTrait, MigratorTrait, SchemaManager,
};

/// The statements that create the sample kinds' tables.
const UP: &str = "
CREATE TABLE specimen (
    id    TEXT PRIMARY KEY NOT NULL REFERENCES record (id),
    title TEXT NOT NULL
);

CREATE TABLE sample_note (
    id     TEXT    PRIMARY KEY NOT NULL REFERENCES record (id),
    body   TEXT    NOT NULL,
    locked INTEGER NOT NULL DEFAULT 0
);
";

/// Creates the sample kinds' tables.
struct CreateSampleTables;

impl MigrationName for CreateSampleTables {
    fn name(&self) -> &str {
        "sample_m0001_tables"
    }
}

#[async_trait]
impl MigrationTrait for CreateSampleTables {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute_unprepared(UP)
            .await
            .map(|_| ())
    }
}

/// The sample kinds' migrations, kept apart from the foundation's.
struct SampleMigrator;

impl MigratorTrait for SampleMigrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(CreateSampleTables)]
    }

    fn migration_table_name() -> DynIden {
        Alias::new("sample_migrations").into_iden()
    }
}

/// Creates the sample kinds' tables in a database that does not have them yet.
pub async fn ensure_tables<'c>(
    connection: impl IntoSchemaManagerConnection<'c>,
) -> Result<(), DbErr> {
    SampleMigrator::up(connection, None).await
}

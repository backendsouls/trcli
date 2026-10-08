//! The versioned changes of a workspace's format (FR-007, FR-075).
//!
//! Each migration is one step and is never edited after a release. The workspace's format
//! number increases with each *released* change; the migrations added between two
//! releases together make the next format. Format 1 is everything in this directory at
//! the first release.

mod m0001_foundation;
mod m0002_records;
mod m0003_telemetry;

use sea_orm_migration::{MigrationTrait, MigratorTrait};

/// The foundation's migrations, in order.
#[derive(Debug)]
pub struct Migrator;

impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m0001_foundation::Migration),
            Box::new(m0002_records::Migration),
            Box::new(m0003_telemetry::Migration),
        ]
    }
}

/// The tables a workspace of the current format must have. Opening checks for them, so
/// that a database that lacks one is reported as damaged instead of failing later (FR-008).
pub const EXPECTED_TABLES: [&str; 8] = [
    "workspace",
    "audit_entry",
    "record",
    "tag",
    "tagging",
    "note",
    "link",
    "telemetry_record",
];

/// The shared tables: what a workspace would exchange with another copy of itself. The
/// others (`telemetry_record`, the migration bookkeeping) are local.
pub const SHARED_TABLES: [&str; 7] = [
    "workspace",
    "audit_entry",
    "record",
    "tag",
    "tagging",
    "note",
    "link",
];

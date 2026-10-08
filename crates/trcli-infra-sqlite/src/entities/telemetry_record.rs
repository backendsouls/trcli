//! The `telemetry_record` table: local telemetry. Never part of what a workspace would
//! exchange with another copy of itself.

use sea_orm::entity::prelude::*;

/// One use of one command.
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "telemetry_record")]
pub struct Model {
    /// The order uses were recorded in.
    #[sea_orm(primary_key)]
    pub id: i64,
    /// When, in milliseconds since 1970.
    pub at: i64,
    /// The command path without its arguments.
    pub command: String,
    /// How long it took, in milliseconds.
    pub duration_ms: i64,
    /// How it ended.
    pub outcome: String,
}

/// The table refers to no other.
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

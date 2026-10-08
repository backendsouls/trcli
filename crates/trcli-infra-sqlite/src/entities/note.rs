//! The `note` table: dated remarks about records.

use sea_orm::entity::prelude::*;

/// A note.
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "note")]
pub struct Model {
    /// The order notes were added in.
    #[sea_orm(primary_key)]
    pub id: i64,
    /// The record the note is about.
    pub record: String,
    /// What was noted.
    pub body: String,
    /// When, in milliseconds since 1970.
    pub created_at: i64,
}

/// `record` is a foreign key; see the migration.
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

//! The `record` table: the index of every record of every kind.

use sea_orm::entity::prelude::*;

/// What the index knows about one record.
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "record")]
pub struct Model {
    /// The record's identifier.
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    /// The name of its kind.
    pub kind: String,
    /// Its short name; unique including deleted rows.
    pub handle: String,
    /// What messages and the trail call it.
    pub display_name: String,
    /// What it is found and sorted by.
    pub search_key: String,
    /// When it was created, in milliseconds since 1970.
    pub created_at: i64,
    /// When it was last changed.
    pub updated_at: i64,
    /// When it was deleted; the row stays so that its handle is never reused.
    pub deleted_at: Option<i64>,
}

/// Relations are expressed in the migrations as foreign keys from other tables.
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

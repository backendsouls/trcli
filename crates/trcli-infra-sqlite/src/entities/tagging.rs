//! The `tagging` table: which records carry which tags.

use sea_orm::entity::prelude::*;

/// One record carrying one tag.
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "tagging")]
pub struct Model {
    /// The tag carried.
    #[sea_orm(primary_key, auto_increment = false)]
    pub tag: String,
    /// The record carrying it.
    #[sea_orm(primary_key, auto_increment = false)]
    pub record: String,
}

/// Both columns are foreign keys; see the migration.
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

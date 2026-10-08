//! The `tag` table: every tag in use.

use sea_orm::entity::prelude::*;

/// A tag.
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "tag")]
pub struct Model {
    /// The tag's normalized name.
    #[sea_orm(primary_key, auto_increment = false)]
    pub name: String,
}

/// Taggings refer to this table; see the migration.
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

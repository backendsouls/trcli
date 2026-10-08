//! The `workspace` table: the one row every workspace has.

use sea_orm::entity::prelude::*;

/// The workspace's own details.
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "workspace")]
pub struct Model {
    /// Identifies the workspace across copies of itself.
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    /// What the researcher calls it.
    pub name: String,
    /// What it is for; may be empty.
    pub description: String,
    /// The format the workspace is stored in.
    pub format_version: i64,
    /// When it was created, in milliseconds since 1970.
    pub created_at: i64,
}

/// The table refers to no other.
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

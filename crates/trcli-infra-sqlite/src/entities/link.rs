//! The `link` table: free connections between records.

use sea_orm::entity::prelude::*;

/// A link.
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "link")]
pub struct Model {
    /// The order links were made in.
    #[sea_orm(primary_key)]
    pub id: i64,
    /// The end the link was made from.
    pub from_record: String,
    /// The other end.
    pub to_record: String,
    /// How the two relate.
    pub relation: String,
    /// When, in milliseconds since 1970.
    pub created_at: i64,
}

/// Both ends are foreign keys; see the migration.
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

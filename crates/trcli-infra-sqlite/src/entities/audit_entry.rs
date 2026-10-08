//! The `audit_entry` table: the trail, append-only.

use sea_orm::entity::prelude::*;

/// One entry of the trail.
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "audit_entry")]
pub struct Model {
    /// Position in the trail; assigned by the store, never by the database.
    #[sea_orm(primary_key, auto_increment = false)]
    pub sequence: i64,
    /// When it happened, in milliseconds since 1970.
    pub at: i64,
    /// Who acted.
    pub actor: String,
    /// What was done.
    pub action: String,
    /// The kind of the record concerned.
    pub kind: Option<String>,
    /// The record concerned. Not a foreign key: the entry outlives the record.
    pub record_id: Option<String>,
    /// The record's handle at the time.
    pub handle: Option<String>,
    /// What the record was called at the time.
    pub display_name: Option<String>,
    /// The fields that changed, as a JSON array of `{field, before, after}`.
    pub changes: String,
    /// The hash of the entry before, in hexadecimal.
    pub previous_hash: String,
    /// This entry's hash, in hexadecimal.
    pub hash: String,
}

/// The table refers to no other.
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

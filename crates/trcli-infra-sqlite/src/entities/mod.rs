//! The tables, as SeaORM entities. One module per table; names follow the data model in
//! `snake_case`.
//!
//! These types never leave this crate: stores convert them to and from the domain's.

pub(crate) mod audit_entry;
pub(crate) mod link;
pub(crate) mod note;
pub(crate) mod record;
pub(crate) mod tag;
pub(crate) mod tagging;
pub(crate) mod telemetry_record;
pub(crate) mod workspace;

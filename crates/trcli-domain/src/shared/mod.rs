//! The shared kernel: the vocabulary every bounded context uses.
//!
//! Contains the identity of records, the checked text types, tags, notes, links, and the
//! types that describe a problem with an input. Features refer to one another's records
//! only through [`record::RecordRef`] (FR-072).

pub mod link;
pub mod note;
pub mod problem;
pub mod record;
pub mod tag;
pub mod text;

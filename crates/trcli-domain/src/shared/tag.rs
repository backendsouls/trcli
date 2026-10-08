//! Tags: short labels any record of any kind may carry (FR-015).
//!
//! A tag is nothing but its name; which records carry it is a tagging, kept by storage as
//! a unique pair. This module does not know which records exist.

use super::record::RecordId;
use super::text::TagName;

/// A label that records can be found by.
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Tag {
    /// The tag's normalized name, unique in the workspace.
    pub name: TagName,
}

/// The fact that one record carries one tag.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Tagging {
    /// The tag carried.
    pub tag: TagName,
    /// The record carrying it.
    pub record: RecordId,
}

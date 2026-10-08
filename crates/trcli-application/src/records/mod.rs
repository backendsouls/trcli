//! What every kind of record can do, written once (FR-010 to FR-019).
//!
//! These use cases work on any registered kind; a feature does not write them again.
//!
//! - [`handles`]: giving a new record its short name.
//! - [`create`]: adding a record to the index, and renaming one, with their audit entries.
//! - [`resolve`] with [`similar`]: finding the record a researcher typed.
//! - [`tag`], [`note`], [`link`]: tags, notes, and links.
//! - [`delete`]: guarded deletion that leaves nothing dangling.
//! - [`list_options`]: filtering, searching, sorting, and limiting a list.
//! - [`show`]: one record with its fields, tags, notes, and links.

pub mod create;
pub mod delete;
pub mod handles;
pub mod link;
pub mod list_options;
pub mod note;
pub mod resolve;
pub mod show;
pub mod similar;
pub mod tag;

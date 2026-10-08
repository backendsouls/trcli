//! The TRCLI domain: the rules of research records.
//!
//! This crate holds what is true about a workspace and its records whatever stores them,
//! shows them, or invokes them: value objects that cannot hold an invalid value, the
//! identity of records, tags, notes, links, settings definitions, and the audit trail's
//! hashing rule.
//!
//! It deliberately does **not** perform I/O, use `async`, or depend on any framework
//! (FR-070). `tests/layering.rs` fails the build if a dependency that could do so is added.

pub mod governance;
pub mod settings;
pub mod shared;
pub mod workspace;

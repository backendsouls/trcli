//! The use cases about the workspace itself (FR-001 to FR-009).
//!
//! - [`locate`]: finding the workspace from where the researcher stands.
//! - [`open`]: deciding what may be done with a workspace of a given format.
//! - [`init`], [`show`], [`edit`]: creating, viewing, and changing a workspace.
//! - [`upgrade`]: bringing an older workspace forward, keeping a copy first.
//! - [`check`]: verifying that the stored data and the audit trail are intact.

pub mod check;
pub mod edit;
pub mod init;
pub mod locate;
pub mod open;
pub mod show;
pub mod upgrade;

//! The TRCLI storage adapter: the application's storage ports implemented on SQLite
//! through SeaORM.
//!
//! - [`connection`] opens and creates a workspace's database.
//! - [`unit_of_work`] is one transaction, through which every store is reached.
//! - [`migrations`] are the versioned changes of format.
//! - [`digest`] is the SHA-256 that chains the audit trail.
//! - `entities` and `stores` are private: nothing outside this crate sees a table.
//!
//! This is the only crate that knows SQL or SeaORM. It deliberately knows nothing about
//! the command line, rendering, or the operating system's conventions.

// Ports are used on one thread; see the same note in the application crate.
#![allow(async_fn_in_trait)]
// Tests are straight lists of steps and assertions; see the application crate's note.
#![cfg_attr(test, allow(clippy::cognitive_complexity, clippy::too_many_lines))]

pub mod connection;
pub mod digest;
pub mod migrations;
pub mod unit_of_work;

pub(crate) mod convert;
pub(crate) mod entities;
mod stores;

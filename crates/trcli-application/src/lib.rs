//! The TRCLI application layer: what the tool can do, independent of how it is invoked,
//! shown, or stored.
//!
//! - [`outcome`] is the fixed vocabulary of how a command ends and what went wrong.
//! - [`ports`] are the small interfaces through which use cases reach the outside:
//!   storage, the clock, identifiers, settings files, the person at the terminal.
//! - [`validation`] is the one way input is checked before anything changes.
//! - [`kinds`] and [`settings`] hold the registries through which a feature plugs in
//!   without the foundation knowing it (FR-068).
//! - [`workspace`], [`records`], and [`governance`] are the foundation's own use cases.
//!
//! This crate deliberately does **not** name any adapter, parse a command line, or render
//! output. Ports use `async fn` only where they reach storage.
//!
//! In-memory doubles of every port, and the contract suites every adapter must pass, are
//! in the `trcli-testing` crate; the tests of the use cases, which need them, are in the
//! crate's `tests/use_cases/` directory. Only unit tests of pure
//! logic are in this crate, beside the code.

// Ports are implemented and called on one thread (the runtime is single-threaded), so the
// futures of their `async fn`s need no `Send` bound, which is what this lint warns about.
#![allow(async_fn_in_trait)]

pub mod governance;
pub mod kinds;
pub mod outcome;
pub mod ports;
pub mod records;
pub mod settings;
pub mod validation;
pub mod view;
pub mod workspace;

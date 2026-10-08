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
//! - `testing` (feature `test-support`) has in-memory fakes of every port and the
//!   contract suites every adapter must pass.
//! - `sample` (feature `sample-kind`) has two sample record kinds that prove the
//!   feature contract; no release build enables it.
//!
//! This crate deliberately does **not** name any adapter, parse a command line, or render
//! output. Ports use `async fn` only where they reach storage.

// Ports are implemented and called on one thread (the runtime is single-threaded), so the
// futures of their `async fn`s need no `Send` bound, which is what this lint warns about.
#![allow(async_fn_in_trait)]
// Tests are straight lists of steps and assertions; the limits on length and branching
// that keep production functions readable would only force them to be cut arbitrarily.
#![cfg_attr(test, allow(clippy::cognitive_complexity, clippy::too_many_lines))]

pub mod governance;
pub mod kinds;
pub mod outcome;
pub mod ports;
pub mod records;
pub mod settings;
pub mod validation;
pub mod view;
pub mod workspace;

#[cfg(feature = "sample-kind")]
pub mod sample;
#[cfg(feature = "test-support")]
pub mod testing;

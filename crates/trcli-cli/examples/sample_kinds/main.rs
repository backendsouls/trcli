//! `trcli` with a sample feature: two kinds of record, `specimen` and `sample-note`,
//! added from outside the crates.
//!
//! This example is the proof and the illustration of how a feature plugs in (FR-068):
//! everything the two kinds can do beyond their own `add` and `edit` comes from
//! registering a descriptor, and no file of the foundation names them
//! (`crates/trcli-cli/tests/sample_kind_is_external.rs` checks). The acceptance scenarios about records
//! run against this program; the `trcli` binary itself never contains it.
//!
//! ```sh
//! cargo run -p trcli-cli --example sample_kinds -- init --name "Field work"
//! cargo run -p trcli-cli --example sample_kinds -- specimen add --title "Soil sample 14"
//! cargo run -p trcli-cli --example sample_kinds -- specimen list
//! ```
//!
//! The code is laid out as a feature's is, one part per layer: [`sample::application`]
//! (descriptors, ports, use cases), [`sample::storage`] (tables and stores), and
//! [`sample::commands`] (the command line). A feature built into the tool has the same
//! three parts as modules of the three crates; see
//! `docs/contributing/adding-a-record-kind.md`.

// Ports are used on one thread; see the same note in the application crate.
#![allow(async_fn_in_trait)]
// Tests are straight lists of steps and assertions; see the application crate's note.
#![cfg_attr(test, allow(clippy::cognitive_complexity, clippy::too_many_lines))]

mod sample;

use std::process::ExitCode;

/// Runs one command of the tool with the sample kinds added.
fn main() -> ExitCode {
    trcli_cli::run::main_with(&sample::SampleKinds)
}

//! Tests of the application's use cases, run against the in-memory doubles of the
//! `trcli-testing` crate.
//!
//! One module per use case, named after the source file it tests. They use only what the
//! application crate makes public, which is what a use case's callers see. Tests of pure
//! logic that needs no double stay beside the code they test, as Rust's convention has it.

// Tests are straight lists of steps and assertions; the limits on length and branching
// that keep production functions readable would only force them to be cut arbitrarily.
#![allow(clippy::cognitive_complexity, clippy::too_many_lines)]

mod governance_export;
mod governance_query;
mod governance_record;
mod governance_telemetry_summary;
mod governance_verify;
mod records_create;
mod records_delete;
mod records_handles;
mod records_link;
mod records_list_options;
mod records_note;
mod records_resolve;
mod records_tag;
mod settings_commands;
mod settings_layers;
mod workspace_check;
mod workspace_edit;
mod workspace_init;
mod workspace_locate;
mod workspace_show;
mod workspace_upgrade;

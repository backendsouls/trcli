//! The options every command accepts, exactly as in `contracts/cli-conventions.md`.
//!
//! `--project` and `--all-projects`, also listed in that contract, are added by
//! `specs/003-research-projects` and are not part of the foundation.

use std::path::PathBuf;

use clap::{ArgAction, Args};

/// The heading these options are listed under in help.
const HEADING: &str = "Global options";

/// The options of every command.
#[derive(Clone, Debug, Default, Args)]
pub struct GlobalArgs {
    /// Workspace to use, instead of the one found from the current directory
    /// (also: the TRCLI_WORKSPACE variable)
    #[arg(long, global = true, value_name = "DIR", help_heading = HEADING)]
    pub workspace: Option<PathBuf>,

    /// Output form: for people, or one JSON document for programs [default: human]
    #[arg(long, global = true, value_name = "FORMAT", value_parser = ["human", "json"], help_heading = HEADING)]
    pub output: Option<String>,

    /// Coloured output [default: auto]
    #[arg(long, global = true, value_name = "WHEN", value_parser = ["auto", "always", "never"], help_heading = HEADING)]
    pub color: Option<String>,

    /// Answer yes to confirmations
    #[arg(short = 'y', long, global = true, help_heading = HEADING)]
    pub yes: bool,

    /// Never ask a question; fail instead
    #[arg(long, global = true, help_heading = HEADING)]
    pub no_input: bool,

    /// Only results and errors
    #[arg(short = 'q', long, global = true, help_heading = HEADING)]
    pub quiet: bool,

    /// Diagnostic detail on standard error (repeatable)
    #[arg(short = 'v', long, global = true, action = ArgAction::Count, help_heading = HEADING)]
    pub verbose: u8,
}

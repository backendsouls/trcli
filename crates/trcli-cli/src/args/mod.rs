//! The command line, as clap definitions: one module per group of commands.
//!
//! These types say what can be typed, with the help text and an example for each command
//! (FR-056). They check syntax only; every value is checked again, with all problems
//! reported together, by the command constructors of the application layer (FR-023).
//!
//! The grammar is `trcli [GLOBAL OPTIONS] <noun> <verb> [ARGUMENTS] [OPTIONS]`, as fixed in
//! `specs/000-foundation/contracts/cli-conventions.md`.

pub mod audit;
pub mod completions;
pub mod config;
pub mod global;
pub mod link;
pub mod workspace;

use clap::Subcommand;

/// The foundation's own commands. Commands of record kinds are added beside these by
/// the features that own them.
#[derive(Clone, Debug, Subcommand)]
pub enum Commands {
    /// Create a workspace in a directory
    #[command(after_long_help = workspace::INIT_EXAMPLE)]
    Init(workspace::InitArgs),

    /// View, change, upgrade, and check the workspace
    #[command(subcommand, after_long_help = workspace::WORKSPACE_EXAMPLE)]
    Workspace(workspace::WorkspaceCommand),

    /// View and change settings
    #[command(subcommand, after_long_help = config::CONFIG_EXAMPLE)]
    Config(config::ConfigCommand),

    /// Link any two records, remove a link, or list a record's links
    #[command(subcommand, after_long_help = link::LINK_EXAMPLE)]
    Link(link::LinkCommand),

    /// List the tags in use
    #[command(subcommand, after_long_help = link::TAG_EXAMPLE)]
    Tag(link::TagCommand),

    /// Look through, verify, and export the record of every change
    #[command(subcommand, after_long_help = audit::AUDIT_EXAMPLE)]
    Audit(audit::AuditCommand),

    /// View local telemetry about the tool's use, or turn it on or off
    #[command(subcommand, after_long_help = audit::TELEMETRY_EXAMPLE)]
    Telemetry(audit::TelemetryCommand),

    /// Print a completion script for your shell
    #[command(after_long_help = completions::COMPLETIONS_EXAMPLE)]
    Completions(completions::CompletionsArgs),
}

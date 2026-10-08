//! `trcli init` and `trcli workspace …`, as in `contracts/cli-workspace.md`.

use std::path::PathBuf;

use clap::{Args, Subcommand};

/// The example shown by `trcli init --help`.
pub const INIT_EXAMPLE: &str = "\
Example:
  trcli init --name \"Doctorate\" --description \"Thesis on soil microbes\"
  trcli init ~/research/postdoc --name \"Postdoc\"

Guide: docs/usage/workspace.md";

/// The example shown by `trcli workspace --help`.
pub const WORKSPACE_EXAMPLE: &str = "\
Example:
  trcli workspace show
  trcli workspace edit --name \"Doctorate (2026)\"

Guide: docs/usage/workspace.md";

/// The arguments of `trcli init`.
#[derive(Clone, Debug, Args)]
pub struct InitArgs {
    /// Directory to create the workspace in [default: the current directory]
    #[arg(value_name = "DIR")]
    pub directory: Option<PathBuf>,

    /// Name of the workspace (1 to 200 characters)
    #[arg(long, value_name = "NAME")]
    pub name: Option<String>,

    /// What the workspace is for (up to 20,000 characters)
    #[arg(long, value_name = "TEXT")]
    pub description: Option<String>,
}

/// The verbs of `trcli workspace`.
#[derive(Clone, Debug, Subcommand)]
pub enum WorkspaceCommand {
    /// Show the workspace's name, description, location, format, and record counts
    #[command(after_long_help = "Example:\n  trcli workspace show\n  trcli workspace show --output json")]
    Show,

    /// Change the workspace's details; only what you name is changed
    #[command(after_long_help = "Example:\n  trcli workspace edit --name \"Doctorate (2026)\"\n  trcli workspace edit --researcher \"Ana Souza\"")]
    Edit(EditArgs),

    /// Bring an older workspace to the current format, keeping a copy first
    #[command(after_long_help = "Example:\n  trcli workspace upgrade --check\n  trcli workspace upgrade")]
    Upgrade(UpgradeArgs),

    /// Verify that the stored data is consistent and the audit trail is intact
    #[command(after_long_help = "Example:\n  trcli workspace check")]
    Check,
}

/// The options of `trcli workspace edit`.
#[derive(Clone, Debug, Args)]
pub struct EditArgs {
    /// New name of the workspace (1 to 200 characters)
    #[arg(long, value_name = "NAME")]
    pub name: Option<String>,

    /// New description (up to 20,000 characters)
    #[arg(long, value_name = "TEXT")]
    pub description: Option<String>,

    /// Name your actions are recorded under in the audit trail (1 to 200 characters)
    #[arg(long, value_name = "NAME")]
    pub researcher: Option<String>,
}

/// The options of `trcli workspace upgrade`.
#[derive(Clone, Debug, Args)]
pub struct UpgradeArgs {
    /// Only say whether an upgrade is needed; change nothing
    #[arg(long)]
    pub check: bool,
}

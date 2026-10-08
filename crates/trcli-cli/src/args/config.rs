//! `trcli config …`, as in `contracts/cli-workspace.md` and `contracts/configuration.md`.

use clap::{Args, Subcommand};

/// The example shown by `trcli config --help`.
pub const CONFIG_EXAMPLE: &str = "\
Example:
  trcli config list
  trcli config set output.color never
  trcli config set --user output.page_size 20

Guide: docs/usage/config.md";

/// The verbs of `trcli config`.
#[derive(Clone, Debug, Subcommand)]
pub enum ConfigCommand {
    /// List every setting, its value in effect, and where that value comes from
    #[command(after_long_help = "Example:\n  trcli config list")]
    List,

    /// Describe one setting: meaning, allowed values, default, and value in effect
    #[command(after_long_help = "Example:\n  trcli config get output.color")]
    Get(KeyArgs),

    /// Set a setting for this workspace, or for yourself with --user
    #[command(after_long_help = "Example:\n  trcli config set output.color never\n  trcli config set --user output.page_size 20")]
    Set(SetArgs),

    /// Remove your value of a setting, so that the next source applies again
    #[command(after_long_help = "Example:\n  trcli config unset output.color\n  trcli config unset --user output.page_size")]
    Unset(UnsetArgs),

    /// Show where the settings files are
    #[command(after_long_help = "Example:\n  trcli config path")]
    Path,
}

/// The argument of `trcli config get`.
#[derive(Clone, Debug, Args)]
pub struct KeyArgs {
    /// The setting, for example output.color
    #[arg(value_name = "KEY")]
    pub key: String,
}

/// The arguments of `trcli config set`.
#[derive(Clone, Debug, Args)]
pub struct SetArgs {
    /// The setting, for example output.color
    #[arg(value_name = "KEY")]
    pub key: String,

    /// The value to give it
    #[arg(value_name = "VALUE")]
    pub value: String,

    /// Store it in your own settings, valid in every workspace, instead of this workspace's
    #[arg(long)]
    pub user: bool,
}

/// The arguments of `trcli config unset`.
#[derive(Clone, Debug, Args)]
pub struct UnsetArgs {
    /// The setting, for example output.color
    #[arg(value_name = "KEY")]
    pub key: String,

    /// Remove it from your own settings instead of this workspace's
    #[arg(long)]
    pub user: bool,
}

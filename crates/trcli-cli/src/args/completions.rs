//! `trcli completions <shell>` (FR-059).

use clap::Args;
use clap_complete::Shell;

/// The example shown by `trcli completions --help`.
pub const COMPLETIONS_EXAMPLE: &str = "\
Example:
  trcli completions bash > ~/.local/share/bash-completion/completions/trcli
  trcli completions zsh > ~/.zfunc/_trcli

Guide: docs/usage/README.md";

/// The argument of `trcli completions`.
#[derive(Clone, Debug, Args)]
pub struct CompletionsArgs {
    /// The shell to print a completion script for
    #[arg(value_name = "SHELL", value_parser = clap::builder::EnumValueParser::<Shell>::new())]
    pub shell: Shell,
}

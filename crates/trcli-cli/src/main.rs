//! The `trcli` binary. Everything it does is in the `trcli_cli` library; see
//! [`trcli_cli::run`] for the path of one command.

use std::process::ExitCode;

/// Runs one command and ends with the exit code of its outcome (FR-032).
fn main() -> ExitCode {
    trcli_cli::run::main()
}

//! How a feature's kinds of record and commands are added to the tool (FR-068).
//!
//! An [`Extension`] registers the kinds it owns, contributes its nouns to the command
//! line, and runs its own commands. The foundation asks nothing else of a feature, and
//! knows no feature by name: the `trcli` binary is built with [`NoExtension`], and
//! `examples/sample_kinds` builds the same tool with two sample kinds added from outside
//! the crates.

use clap::{ArgMatches, Command};
use trcli_application::kinds::{KindError, KindRegistry};
use trcli_application::outcome::Problem;

use crate::compose::Session;
use crate::output::Reply;

/// What a feature adds to the tool.
pub trait Extension {
    /// Registers the kinds of record the feature owns.
    fn register_kinds(&self, kinds: &mut KindRegistry) -> Result<(), KindError>;

    /// The nouns the feature adds to the command line, each with its verbs.
    fn commands(&self) -> Vec<Command>;

    /// Runs one of the feature's commands: `noun` is one of the nouns it added, and
    /// `matches` is what was typed after it.
    async fn run(
        &self,
        session: &mut Session,
        noun: &str,
        matches: &ArgMatches,
    ) -> Result<Reply, Problem>;
}

/// The foundation alone: no kind of record, no extra command.
#[derive(Clone, Copy, Debug, Default)]
pub struct NoExtension;

impl Extension for NoExtension {
    fn register_kinds(&self, _kinds: &mut KindRegistry) -> Result<(), KindError> {
        Ok(())
    }

    fn commands(&self) -> Vec<Command> {
        Vec::new()
    }

    async fn run(
        &self,
        _session: &mut Session,
        noun: &str,
        _matches: &ArgMatches,
    ) -> Result<Reply, Problem> {
        // clap accepts only the nouns that were added, and none was: this is never reached.
        Err(Problem::internal(format!(
            "no feature of this build owns the command `{noun}`"
        )))
    }
}

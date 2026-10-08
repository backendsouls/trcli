//! The commands of the sample record kinds (feature `sample-kind`; never released).
//!
//! This is everything a feature adds to the command line for a kind of record: its noun,
//! its own `add` and `edit`, and one call that brings in the verbs every kind shares. It
//! is the worked example behind `docs/contributing/adding-a-record-kind.md`.

mod sample_note;
mod specimen;

use clap::{ArgMatches, Command};
use trcli_application::outcome::Problem;

use crate::compose::Session;
use crate::output::Reply;

/// Adds the sample kinds' nouns to the command line.
pub fn extend(root: Command) -> Command {
    root.subcommand(specimen::command())
        .subcommand(sample_note::command())
}

/// Runs a command of one of the sample kinds.
pub async fn run(
    session: &mut Session,
    noun: &str,
    matches: &ArgMatches,
) -> Result<Reply, Problem> {
    let Some((verb, arguments)) = matches.subcommand() else {
        return Err(Problem::internal(format!("`{noun}` needs a verb")));
    };
    match noun {
        specimen::NOUN => specimen::run(session, verb, arguments).await,
        sample_note::NOUN => sample_note::run(session, verb, arguments).await,
        _ => Err(Problem::internal(format!(
            "no feature of this build owns the command `{noun}`"
        ))),
    }
}

/// The text of one string argument, when given.
fn text(matches: &ArgMatches, name: &str) -> Option<String> {
    matches.get_one::<String>(name).cloned()
}

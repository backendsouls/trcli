//! The sample feature's command line: each kind's noun with its own `add` and `edit`,
//! and one call that brings in the verbs every kind shares.

pub mod sample_note;
pub mod specimen;

use clap::ArgMatches;
use trcli_application::outcome::Problem;

use trcli_cli::compose::Session;
use trcli_cli::output::Reply;

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

//! `trcli sample-note …`: the second sample kind. Its own verbs are `add` and `edit`;
//! the rest come from [`crate::shared_verbs`], exactly as for the first kind.

use clap::{Arg, ArgAction, ArgMatches, Command};
use trcli_application::outcome::Problem;
use trcli_application::ports::unit_of_work::Storage;
use trcli_application::sample::sample_note::{
    AddSampleNote, EditSampleNote, SampleNotes, add, descriptor, edit,
};
use trcli_application::workspace::open::Access;

use super::text;
use crate::commands::finish;
use crate::compose::Session;
use crate::output::Reply;
use crate::shared_verbs;

/// The kind's noun on the command line.
pub const NOUN: &str = "sample-note";

/// `trcli sample-note` with its own verbs and the shared ones.
pub fn command() -> Command {
    let descriptor = descriptor();
    let flag = |name: &'static str, help: &'static str| {
        Arg::new(name)
            .long(name)
            .action(ArgAction::SetTrue)
            .help(help)
    };
    let own = [
        Command::new("add")
            .about("Add a sample note")
            .arg(Arg::new("body").long("body").value_name("TEXT").help("The note's text (1 to 20,000 characters)"))
            .arg(flag("locked", "Lock the note: a locked note cannot be deleted"))
            .after_long_help("Example:\n  trcli sample-note add --body \"Collected in the rain\""),
        Command::new("edit")
            .about("Change a sample note; only what you name is changed")
            .arg(Arg::new("ref").value_name("REF").required(true).help("Short name of the note, or a unique beginning of it"))
            .arg(Arg::new("body").long("body").value_name("TEXT").help("New text (1 to 20,000 characters)"))
            .arg(flag("lock", "Lock the note against deletion").conflicts_with("unlock"))
            .arg(flag("unlock", "Unlock the note so that it can be deleted"))
            .after_long_help("Example:\n  trcli sample-note edit smp-7k3f --body \"Collected in heavy rain\"\n  trcli sample-note edit smp-7k3f --unlock"),
    ];
    Command::new(NOUN)
        .about(descriptor.summary)
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommands(own)
        .subcommands(shared_verbs::commands(&descriptor))
        .after_long_help("Example:\n  trcli sample-note add --body \"Collected in the rain\"\n  trcli sample-note list\n\nGuide: docs/usage/records.md")
}

/// Runs one verb of `trcli sample-note`.
pub async fn run(
    session: &mut Session,
    verb: &str,
    matches: &ArgMatches,
) -> Result<Reply, Problem> {
    if let Some(shared) = shared_verbs::run(session, &SampleNotes::new(), verb, matches).await {
        return shared;
    }
    match verb {
        "add" => add_note(session, matches).await,
        "edit" => edit_note(session, matches).await,
        _ => Err(Problem::internal(format!(
            "`sample-note {verb}` is not a command"
        ))),
    }
}

/// `trcli sample-note add --body <text> [--locked]`.
async fn add_note(session: &mut Session, matches: &ArgMatches) -> Result<Reply, Problem> {
    let valid = AddSampleNote::new(text(matches, "body").as_deref(), matches.get_flag("locked"))?;
    let storage = session.storage(Access::Write).await?;
    let mut unit = storage.begin().await?;
    let added = add(&mut unit, &session.stamp(), &session.ids, valid.command).await?;
    finish(session, unit).await?;
    Ok(Reply::new(added).with_warnings(valid.warnings))
}

/// `trcli sample-note edit <ref> [--body <text>] [--lock | --unlock]`.
async fn edit_note(session: &mut Session, matches: &ArgMatches) -> Result<Reply, Problem> {
    let locked = match (matches.get_flag("lock"), matches.get_flag("unlock")) {
        (true, _) => Some(true),
        (_, true) => Some(false),
        _ => None,
    };
    let reference = text(matches, "ref").unwrap_or_default();
    let valid = EditSampleNote::new(&reference, text(matches, "body").as_deref(), locked)?;
    let storage = session.storage(Access::Write).await?;
    let mut unit = storage.begin().await?;
    let edited = edit(&mut unit, &session.stamp(), valid.command).await?;
    finish(session, unit).await?;
    Ok(Reply::new(edited).with_warnings(valid.warnings))
}

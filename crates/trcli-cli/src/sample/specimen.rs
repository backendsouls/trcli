//! `trcli specimen …`: the first sample kind. Its own verbs are `add`, `edit`, and
//! `slow`; the rest come from [`crate::shared_verbs`].

use std::time::{Duration, Instant};

use clap::{Arg, ArgMatches, Command};
use trcli_application::outcome::Problem;
use trcli_application::ports::interaction::Progress;
use trcli_application::ports::unit_of_work::Storage;
use trcli_application::sample::specimen::{
    AddSpecimen, EditSpecimen, Specimens, add, descriptor, edit,
};
use trcli_application::validation::{Checker, integer_in_range};
use trcli_application::view::Done;
use trcli_application::workspace::open::Access;

use super::text;
use crate::commands::finish;
use crate::compose::Session;
use crate::output::Reply;
use crate::shared_verbs;

/// The kind's noun on the command line.
pub const NOUN: &str = "specimen";

/// `trcli specimen` with its own verbs and the shared ones.
pub fn command() -> Command {
    let descriptor = descriptor();
    let own = [
        Command::new("add")
            .about("Add a specimen")
            .arg(Arg::new("title").long("title").value_name("TEXT").help("The specimen's title (1 to 500 characters)"))
            .after_long_help("Example:\n  trcli specimen add --title \"Soil sample 14\""),
        Command::new("edit")
            .about("Change a specimen; only what you name is changed")
            .arg(Arg::new("ref").value_name("REF").required(true).help("Short name of the specimen, or a unique beginning of it"))
            .arg(Arg::new("title").long("title").value_name("TEXT").help("New title (1 to 500 characters)"))
            .after_long_help("Example:\n  trcli specimen edit spc-7k3f --title \"Soil sample 14 (dry)\""),
        Command::new("slow")
            .about("Hold the workspace for a while, showing progress (for trying interruption and concurrency)")
            .arg(Arg::new("seconds").long("seconds").value_name("N").help("How long, in seconds (1 to 600) [default: 5]"))
            .arg(Arg::new("title").long("title").value_name("TEXT").help("Add a specimen with this title at the end, if not interrupted"))
            .after_long_help("Example:\n  trcli specimen slow --seconds 20"),
    ];
    Command::new(NOUN)
        .about(descriptor.summary)
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommands(own)
        .subcommands(shared_verbs::commands(&descriptor))
        .after_long_help("Example:\n  trcli specimen add --title \"Soil sample 14\"\n  trcli specimen list --search soil\n\nGuide: docs/usage/records.md")
}

/// Runs one verb of `trcli specimen`.
pub async fn run(
    session: &mut Session,
    verb: &str,
    matches: &ArgMatches,
) -> Result<Reply, Problem> {
    if let Some(shared) = shared_verbs::run(session, &Specimens::new(), verb, matches).await {
        return shared;
    }
    match verb {
        "add" => add_specimen(session, matches).await,
        "edit" => edit_specimen(session, matches).await,
        "slow" => slow(session, matches).await,
        _ => Err(Problem::internal(format!(
            "`specimen {verb}` is not a command"
        ))),
    }
}

/// `trcli specimen add --title <text>`.
async fn add_specimen(session: &mut Session, matches: &ArgMatches) -> Result<Reply, Problem> {
    let valid = AddSpecimen::new(text(matches, "title").as_deref())?;
    let storage = session.storage(Access::Write).await?;
    let mut unit = storage.begin().await?;
    let added = add(&mut unit, &session.stamp(), &session.ids, valid.command).await?;
    finish(session, unit).await?;
    Ok(Reply::new(added.command)
        .with_warnings(valid.warnings)
        .with_warnings(added.warnings))
}

/// `trcli specimen edit <ref> [--title <text>]`.
async fn edit_specimen(session: &mut Session, matches: &ArgMatches) -> Result<Reply, Problem> {
    let reference = text(matches, "ref").unwrap_or_default();
    let valid = EditSpecimen::new(&reference, text(matches, "title").as_deref())?;
    let storage = session.storage(Access::Write).await?;
    let mut unit = storage.begin().await?;
    let edited = edit(&mut unit, &session.stamp(), valid.command).await?;
    finish(session, unit).await?;
    Ok(Reply::new(edited).with_warnings(valid.warnings))
}

/// `trcli specimen slow [--seconds <n>] [--title <text>]`: holds a unit of work open and
/// reports progress, so that interruption, a busy workspace, and a killed process can be
/// tried. When it is allowed to finish and a title was given, it adds that specimen; when
/// it is stopped, the unit is dropped and nothing is added.
async fn slow(session: &mut Session, matches: &ArgMatches) -> Result<Reply, Problem> {
    let mut checker = Checker::new();
    let seconds = checker.optional("--seconds", text(matches, "seconds").as_deref(), |n| {
        integer_in_range(n, 1, 600)
    });
    let title = text(matches, "title");
    let added = title
        .as_deref()
        .map(|title| AddSpecimen::new(Some(title)))
        .transpose()?;
    let seconds = checker.finish(|| seconds.flatten().unwrap_or(5))?.command;

    let storage = session.storage(Access::Write).await?;
    let mut unit = storage.begin().await?;
    wait(session, Duration::from_secs(seconds.unsigned_abs())).await;
    let message = match added {
        Some(valid) => {
            let row = add(&mut unit, &session.stamp(), &session.ids, valid.command)
                .await?
                .command;
            format!(
                "Held the workspace for {seconds} s, then added specimen {} \"{}\"",
                row.handle, row.name
            )
        }
        None => format!("Held the workspace for {seconds} s; nothing was changed"),
    };
    finish(session, unit).await?;
    Ok(Reply::new(Done::new(message)))
}

/// Waits, in short steps, reporting progress and letting an interruption be noticed
/// between them.
async fn wait(session: &mut Session, length: Duration) {
    let started = Instant::now();
    let total = length.as_millis() as u64 / 100;
    session.progress.start("Holding the workspace");
    while started.elapsed() < length {
        // The foundation has no timer: nothing else in it waits. A short sleep and a
        // yield keep the runtime turning so that Ctrl-C is seen.
        std::thread::sleep(Duration::from_millis(20));
        tokio::task::yield_now().await;
        session
            .progress
            .advance(started.elapsed().as_millis() as u64 / 100, Some(total));
    }
    session.progress.finish();
}

//! The verbs every kind of record has, built from the kind's descriptor (FR-013, FR-068).
//!
//! Given a registered kind, this module provides the clap definitions and the handlers of
//! `list`, `show`, `rm`, `tag`, and `note`. A feature adds its own `add` and `edit` beside
//! them, with its own fields; everything else it gets from here, so the words, the order,
//! and the options are the same for every kind.

use clap::{Arg, ArgAction, ArgMatches, Command};
use trcli_application::kinds::{KindBehaviour, RecordKindDescriptor};
use trcli_application::outcome::Problem;
use trcli_application::ports::unit_of_work::Storage;
use trcli_application::records::delete::delete;
use trcli_application::records::list_options::{ListInput, list, query};
use trcli_application::records::note::{NoteCommand, add_note};
use trcli_application::records::show::show;
use trcli_application::records::tag::{TagCommand, tag};
use trcli_application::workspace::open::Access;

use crate::commands::finish;
use crate::compose::{AppUnit, Session};
use crate::output::Reply;

/// The argument that names a record.
fn reference() -> Arg {
    Arg::new("ref")
        .value_name("REF")
        .required(true)
        .help("Short name of the record, or a unique beginning of it")
}

/// The text of one string argument, when given.
fn text(matches: &ArgMatches, name: &str) -> Option<String> {
    matches.get_one::<String>(name).cloned()
}

/// The texts of a repeatable argument.
fn texts(matches: &ArgMatches, name: &str) -> Vec<String> {
    matches
        .get_many::<String>(name)
        .map(|values| values.cloned().collect())
        .unwrap_or_default()
}

/// `<noun> list`.
fn list_command(descriptor: &RecordKindDescriptor) -> Command {
    let noun = descriptor.name();
    let sort_help = format!(
        "Sort by {}, created, updated, or handle [default: {}]",
        descriptor.name_field, descriptor.name_field
    );
    Command::new("list")
        .about(format!("List {noun} records, filtered, searched, sorted, and limited"))
        .arg(Arg::new("search").long("search").value_name("TEXT").help("Only records whose main text contains these words"))
        .arg(
            Arg::new("tag")
                .long("tag")
                .value_name("TAG")
                .action(ArgAction::Append)
                .help("Only records carrying this tag (repeatable: every tag given must be carried)"),
        )
        .arg(Arg::new("sort").long("sort").value_name("FIELD").help(sort_help))
        .arg(Arg::new("desc").long("desc").action(ArgAction::SetTrue).help("Reverse the order"))
        .arg(Arg::new("limit").long("limit").value_name("N").help("Show at most this many (1 to 1000) [default: the output.page_size setting]"))
        .after_long_help(format!("Example:\n  trcli {noun} list --tag field-work --search rain --sort created --desc --limit 10"))
}

/// The shared verbs of a kind, to be added under the kind's noun.
pub fn commands(descriptor: &RecordKindDescriptor) -> Vec<Command> {
    let noun = descriptor.name();
    let prefix = descriptor.kind.prefix();
    vec![
        list_command(descriptor),
        Command::new("show")
            .about(format!("Show one {noun} record with its tags, notes, and links"))
            .arg(reference())
            .after_long_help(format!("Example:\n  trcli {noun} show {prefix}-7k3f")),
        Command::new("rm")
            .about(format!("Delete a {noun} record, after listing what refers to it and asking"))
            .arg(reference())
            .after_long_help(format!("Example:\n  trcli {noun} rm {prefix}-7k3f\n  trcli {noun} rm {prefix}-7k3f --yes")),
        Command::new("tag")
            .about(format!("Add tags to a {noun} record, or remove them"))
            .arg(reference())
            .arg(Arg::new("tags").value_name("TAG").required(true).num_args(1..).help("Tags: 1 to 50 characters of a-z, 0-9, '-' and '_'"))
            .arg(Arg::new("remove").long("remove").action(ArgAction::SetTrue).help("Remove the tags instead of adding them"))
            .after_long_help(format!("Example:\n  trcli {noun} tag {prefix}-7k3f field-work to-read\n  trcli {noun} tag {prefix}-7k3f to-read --remove")),
        Command::new("note")
            .about(format!("Add a dated note to a {noun} record"))
            .arg(reference())
            .arg(Arg::new("text").value_name("TEXT").required(true).help("The note (1 to 20,000 characters)"))
            .after_long_help(format!("Example:\n  trcli {noun} note {prefix}-7k3f \"Collected in the rain\"")),
    ]
}

/// Runs a shared verb for a kind; `None` when the verb is not one of the shared ones, so
/// that the feature can try its own.
pub async fn run<K>(
    session: &mut Session,
    kind: &K,
    verb: &str,
    matches: &ArgMatches,
) -> Option<Result<Reply, Problem>>
where
    K: KindBehaviour<AppUnit>,
{
    let result = match verb {
        "list" => list_records(session, kind.descriptor(), matches).await,
        "show" => show_record(session, kind, matches).await,
        "rm" => remove_record(session, kind, matches).await,
        "tag" => tag_record(session, kind.descriptor(), matches).await,
        "note" => note_record(session, kind.descriptor(), matches).await,
        _ => return None,
    };
    Some(result)
}

/// `<noun> list [--search] [--tag]... [--sort] [--desc] [--limit]`.
async fn list_records(
    session: &mut Session,
    descriptor: &RecordKindDescriptor,
    matches: &ArgMatches,
) -> Result<Reply, Problem> {
    let input = ListInput {
        search: text(matches, "search"),
        tags: texts(matches, "tag"),
        sort: text(matches, "sort"),
        descending: matches.get_flag("desc"),
        limit: text(matches, "limit"),
    };
    let valid = query(descriptor, &input, session.page_size())?;
    let storage = session.storage(Access::Read).await?;
    let unit = storage.read().await?;
    Ok(Reply::new(list(&unit, descriptor, &valid.command).await?).with_warnings(valid.warnings))
}

/// `<noun> show <ref>`.
async fn show_record<K: KindBehaviour<AppUnit>>(
    session: &mut Session,
    kind: &K,
    matches: &ArgMatches,
) -> Result<Reply, Problem> {
    let storage = session.storage(Access::Read).await?;
    let unit = storage.read().await?;
    Ok(Reply::new(
        show(&unit, kind, &text(matches, "ref").unwrap_or_default()).await?,
    ))
}

/// `<noun> rm <ref>`.
async fn remove_record<K: KindBehaviour<AppUnit>>(
    session: &mut Session,
    kind: &K,
    matches: &ArgMatches,
) -> Result<Reply, Problem> {
    let storage = session.storage(Access::Write).await?;
    let mut unit = storage.begin().await?;
    let reference = text(matches, "ref").unwrap_or_default();
    let deleted = delete(
        &mut unit,
        &session.stamp(),
        kind,
        &mut session.prompter,
        &reference,
    )
    .await?;
    finish(session, unit).await?;
    Ok(Reply::new(deleted))
}

/// `<noun> tag <ref> <tag>... [--remove]`.
async fn tag_record(
    session: &mut Session,
    descriptor: &RecordKindDescriptor,
    matches: &ArgMatches,
) -> Result<Reply, Problem> {
    let reference = text(matches, "ref").unwrap_or_default();
    let valid = TagCommand::new(
        &reference,
        &texts(matches, "tags"),
        matches.get_flag("remove"),
    )?;
    let storage = session.storage(Access::Write).await?;
    let mut unit = storage.begin().await?;
    let tagged = tag(
        &mut unit,
        &session.stamp(),
        &[descriptor.name().to_owned()],
        valid.command,
    )
    .await?;
    finish(session, unit).await?;
    Ok(Reply::new(tagged).with_warnings(valid.warnings))
}

/// `<noun> note <ref> <text>`.
async fn note_record(
    session: &mut Session,
    descriptor: &RecordKindDescriptor,
    matches: &ArgMatches,
) -> Result<Reply, Problem> {
    let reference = text(matches, "ref").unwrap_or_default();
    let valid = NoteCommand::new(&reference, &text(matches, "text").unwrap_or_default())?;
    let storage = session.storage(Access::Write).await?;
    let mut unit = storage.begin().await?;
    let noted = add_note(
        &mut unit,
        &session.stamp(),
        &[descriptor.name().to_owned()],
        valid.command,
    )
    .await?;
    finish(session, unit).await?;
    Ok(Reply::new(noted).with_warnings(valid.warnings))
}

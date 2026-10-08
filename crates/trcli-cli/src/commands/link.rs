//! `trcli link …` and `trcli tag list`.

use trcli_application::outcome::Problem;
use trcli_application::ports::unit_of_work::Storage;
use trcli_application::records::link::{LinkCommand as Link, add_link, list_links, remove_link};
use trcli_application::records::tag::list_tags;
use trcli_application::workspace::open::Access;

use super::finish;
use crate::args::link::{LinkArgs, LinkCommand, TagCommand};
use crate::compose::Session;
use crate::output::Reply;

/// Runs one verb of `trcli link`.
pub async fn run(session: &mut Session, command: &LinkCommand) -> Result<Reply, Problem> {
    match command {
        LinkCommand::Add(arguments) => change(session, arguments, false).await,
        LinkCommand::Rm(arguments) => change(session, arguments, true).await,
        LinkCommand::List(arguments) => {
            let storage = session.storage(Access::Read).await?;
            let unit = storage.read().await?;
            Ok(Reply::new(list_links(&unit, &arguments.reference).await?))
        }
    }
}

/// `trcli link add|rm <ref> <ref> [--relation <text>]`.
async fn change(
    session: &mut Session,
    arguments: &LinkArgs,
    remove: bool,
) -> Result<Reply, Problem> {
    let valid = Link::new(
        &arguments.one,
        &arguments.other,
        arguments.relation.as_deref(),
    )?;
    let storage = session.storage(Access::Write).await?;
    let mut unit = storage.begin().await?;
    let stamp = session.stamp();
    let linked = if remove {
        remove_link(&mut unit, &stamp, valid.command).await?
    } else {
        add_link(&mut unit, &stamp, valid.command).await?
    };
    finish(session, unit).await?;
    Ok(Reply::new(linked).with_warnings(valid.warnings))
}

/// `trcli tag list [--kind <kind>]`.
pub async fn tags(session: &mut Session, command: &TagCommand) -> Result<Reply, Problem> {
    let TagCommand::List(arguments) = command;
    let storage = session.storage(Access::Read).await?;
    let unit = storage.read().await?;
    Ok(Reply::new(
        list_tags(&unit, &session.registries.kinds, arguments.kind.as_deref()).await?,
    ))
}

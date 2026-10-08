//! `trcli config …`.

use trcli_application::outcome::Problem;
use trcli_application::ports::unit_of_work::Storage;
use trcli_application::settings::commands::{
    SetCommand, UnsetCommand, get, list, paths, set_for_user, set_for_workspace, unset_for_user, unset_for_workspace,
};
use trcli_application::workspace::open::Access;
use trcli_domain::settings::Place;

use super::finish;
use crate::args::config::{ConfigCommand, SetArgs, UnsetArgs};
use crate::compose::Session;
use crate::output::Reply;

/// Runs one verb of `trcli config`.
pub async fn run(session: &mut Session, command: &ConfigCommand) -> Result<Reply, Problem> {
    match command {
        ConfigCommand::List => Ok(Reply::new(list(&session.settings))),
        ConfigCommand::Get(arguments) => {
            Ok(Reply::new(get(&session.registries.settings, &session.settings, &arguments.key)?))
        }
        ConfigCommand::Set(arguments) => set(session, arguments).await,
        ConfigCommand::Unset(arguments) => unset(session, arguments).await,
        ConfigCommand::Path => Ok(Reply::new(paths(&session.files))),
    }
}

/// The file `--user` chooses.
fn place(user: bool) -> Place {
    if user { Place::User } else { Place::Workspace }
}

/// `trcli config set <key> <value> [--user]`.
pub async fn set(session: &mut Session, arguments: &SetArgs) -> Result<Reply, Problem> {
    store(session, &arguments.key, &arguments.value, place(arguments.user)).await
}

/// Stores one value in one place. Shared with the commands that are a setting under
/// another name, such as `telemetry on`.
pub async fn store(session: &mut Session, key: &str, value: &str, place: Place) -> Result<Reply, Problem> {
    let valid = SetCommand::new(&session.registries.settings, key, value, place)?;
    let done = match place {
        // The researcher's own settings need no workspace.
        Place::User => set_for_user(&session.files, &valid.command)?,
        Place::Workspace => {
            let storage = session.storage(Access::Write).await?;
            let mut unit = storage.begin().await?;
            let done =
                set_for_workspace(&mut unit, &session.stamp(), &session.files, &session.settings, &valid.command).await?;
            finish(session, unit).await?;
            done
        }
    };
    Ok(Reply::new(done).with_warnings(valid.warnings))
}

/// `trcli config unset <key> [--user]`.
async fn unset(session: &mut Session, arguments: &UnsetArgs) -> Result<Reply, Problem> {
    let valid = UnsetCommand::new(&session.registries.settings, &arguments.key, place(arguments.user))?;
    let done = match valid.command.place {
        Place::User => unset_for_user(&session.files, &valid.command)?,
        Place::Workspace => {
            let storage = session.storage(Access::Write).await?;
            let mut unit = storage.begin().await?;
            let done = unset_for_workspace(&mut unit, &session.stamp(), &session.files, &valid.command).await?;
            finish(session, unit).await?;
            done
        }
    };
    Ok(Reply::new(done).with_warnings(valid.warnings))
}

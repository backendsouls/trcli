//! `trcli init` and `trcli workspace …`.

use trcli_application::outcome::Problem;
use trcli_application::ports::unit_of_work::Storage;
use trcli_application::ports::workspace::WorkspaceStore;
use trcli_application::settings::foundation::RESEARCHER_NAME;
use trcli_application::workspace::check::{CheckServices, check};
use trcli_application::workspace::edit::{EditCommand, EditInput, edit};
use trcli_application::workspace::init::{InitCommand, InitInput, InitServices, init as create};
use trcli_application::workspace::open::Access;
use trcli_application::workspace::show::show;
use trcli_application::workspace::upgrade::{self, upgrade};

use super::finish;
use crate::args::workspace::{EditArgs, InitArgs, UpgradeArgs, WorkspaceCommand};
use crate::compose::{AppDigest, Session};
use crate::output::Reply;

/// `trcli init [<dir>] --name <name> [--description <text>]`.
pub async fn init(session: &mut Session, arguments: &InitArgs) -> Result<Reply, Problem> {
    let directory = arguments.directory.clone().unwrap_or_else(|| ".".into());
    let input = InitInput {
        directory: Session::absolute(&directory),
        name: arguments.name.clone(),
        description: arguments.description.clone(),
    };
    let files = session.workspace_files();
    let valid = InitCommand::new(input, &files, &session.probe())?;
    let (database, head) = (session.new_database_in(&valid.command.root), Session::head_in(&valid.command.root));
    let services = InitServices { opener: &session.opener(), files: &files, ids: &session.ids, head: &head };
    let created = create(&services, &session.stamp(), &database, valid.command).await?;
    Ok(Reply::new(created).with_warnings(valid.warnings))
}

/// Runs one verb of `trcli workspace`.
pub async fn run(session: &mut Session, command: &WorkspaceCommand) -> Result<Reply, Problem> {
    match command {
        WorkspaceCommand::Show => show_workspace(session).await,
        WorkspaceCommand::Edit(arguments) => edit_workspace(session, arguments).await,
        WorkspaceCommand::Upgrade(arguments) => upgrade_workspace(session, arguments).await,
        WorkspaceCommand::Check => check_workspace(session).await,
    }
}

/// `trcli workspace show`.
async fn show_workspace(session: &mut Session) -> Result<Reply, Problem> {
    let storage = session.storage(Access::Read).await?;
    let unit = storage.read().await?;
    Ok(Reply::new(show(&unit, session.located()?, &session.registries.kinds).await?))
}

/// `trcli workspace edit [--name] [--description] [--researcher]`.
async fn edit_workspace(session: &mut Session, arguments: &EditArgs) -> Result<Reply, Problem> {
    let input = EditInput {
        name: arguments.name.clone(),
        description: arguments.description.clone(),
        researcher: arguments.researcher.clone(),
    };
    let valid = EditCommand::new(input)?;
    let storage = session.storage(Access::Write).await?;
    let mut unit = storage.begin().await?;
    let researcher = session.settings.text(RESEARCHER_NAME).map(str::to_owned);
    let done = edit(&mut unit, &session.stamp(), &session.files, researcher.as_deref(), valid.command).await?;
    finish(session, unit).await?;
    Ok(Reply::new(done).with_warnings(valid.warnings))
}

/// `trcli workspace upgrade [--check]`.
async fn upgrade_workspace(session: &mut Session, arguments: &UpgradeArgs) -> Result<Reply, Problem> {
    // Opened without the usual guard: a workspace that needs an upgrade is exactly what
    // this command is for.
    let storage = session.storage_unguarded().await?;
    let workspace = storage.read().await?.workspace().await?;
    if arguments.check {
        return Ok(Reply::new(upgrade::check(&workspace)?));
    }
    // Everything committed must be in the file itself before it is copied.
    storage.checkpoint().await?;
    let backup = session.backup(workspace.format_version.number())?;
    let report = upgrade(&storage, &backup, &session.head()?, &session.stamp()).await?;
    Ok(Reply::new(report))
}

/// `trcli workspace check`.
async fn check_workspace(session: &mut Session) -> Result<Reply, Problem> {
    let storage = session.storage_unguarded().await?;
    let unit = storage.read().await?;
    let head = session.head()?;
    let services = CheckServices { head: &head, digest: &AppDigest::default(), progress: &mut session.progress };
    Ok(Reply::new(check(&unit, services).await?))
}

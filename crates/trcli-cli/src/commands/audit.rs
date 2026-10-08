//! `trcli audit …` and `trcli telemetry …`.

use trcli_application::governance::export::{ExportCommand, collect, record_export};
use trcli_application::governance::query::{AuditInput, check_filters, list};
use trcli_application::governance::telemetry_summary::summarise;
use trcli_application::governance::verify::{TrailVerdict, verify};
use trcli_application::outcome::Problem;
use trcli_application::ports::audit::{AuditFilter, AuditHeadStore};
use trcli_application::ports::interaction::{Confirmation, Prompter};
use trcli_application::ports::unit_of_work::Storage;
use trcli_application::ports::workspace::WorkspaceStore;
use trcli_application::records::resolve::resolve_option;
use trcli_application::settings::foundation::TELEMETRY_ENABLED;
use trcli_application::validation::Checker;
use trcli_application::view::Done;
use trcli_application::workspace::open::Access;
use trcli_domain::settings::Place;

use super::{config, finish};
use crate::args::audit::{AuditCommand, AuditFilters, ExportArgs, TelemetryCommand};
use crate::compose::{AppDigest, AppUnit, Session};
use crate::output::Reply;
use crate::render::export::report;
use crate::render::views::Verified;

/// Runs one verb of `trcli audit`.
pub async fn run(session: &mut Session, command: &AuditCommand) -> Result<Reply, Problem> {
    match command {
        AuditCommand::List(filters) => list_entries(session, filters).await,
        AuditCommand::Verify => verify_trail(session).await,
        AuditCommand::Export(arguments) => export(session, arguments).await,
    }
}

/// Checks the filters, resolving the record named with `--record` first: a record that
/// does not exist is reported with the other invalid values, not before them.
async fn filters(
    session: &Session,
    unit: &AppUnit,
    checker: &mut Checker,
    input: (&AuditInput, Option<&str>),
) -> Result<Option<AuditFilter>, Problem> {
    let (input, record) = input;
    let record = resolve_option(unit, checker, "--record", record).await?;
    let record = record.map(|record| record.id);
    Ok(check_filters(
        checker,
        input,
        record,
        &session.registries.kinds,
        session.page_size(),
    ))
}

/// `trcli audit list [filters]`.
async fn list_entries(session: &mut Session, arguments: &AuditFilters) -> Result<Reply, Problem> {
    let input = AuditInput {
        kind: arguments.kind.clone(),
        actor: arguments.actor.clone(),
        action: arguments.action.clone(),
        from: arguments.from.clone(),
        to: arguments.to.clone(),
        to_name: "--to".to_owned(),
        limit: arguments.limit.clone(),
    };
    let storage = session.storage(Access::Read).await?;
    let unit = storage.read().await?;
    let mut checker = Checker::new();
    let filter = filters(
        session,
        &unit,
        &mut checker,
        (&input, arguments.record.as_deref()),
    )
    .await?;
    let valid = checker.finish(|| filter.expect("checked"))?;
    Ok(Reply::new(list(&unit, &valid.command).await?).with_warnings(valid.warnings))
}

/// `trcli audit verify`.
async fn verify_trail(session: &mut Session) -> Result<Reply, Problem> {
    let storage = session.storage(Access::Read).await?;
    let unit = storage.read().await?;
    let head = session.head()?.read_head()?;
    match verify(&unit, head, &AppDigest::default(), &mut session.progress).await? {
        TrailVerdict::Intact { entries } => Ok(Reply::new(Verified {
            intact: true,
            entries,
        })),
        TrailVerdict::Broken(finding) => Err(finding.into_problem()),
    }
}

/// `trcli audit export [filters] --to <file> [--format <markdown|json|csv>]`.
async fn export(session: &mut Session, arguments: &ExportArgs) -> Result<Reply, Problem> {
    let input = AuditInput {
        kind: arguments.kind.clone(),
        actor: arguments.actor.clone(),
        action: arguments.action.clone(),
        from: arguments.from.clone(),
        to: arguments.until.clone(),
        to_name: "--until".to_owned(),
        limit: None,
    };
    let storage = session.storage(Access::Write).await?;
    let mut unit = storage.begin().await?;
    let mut checker = Checker::new();
    let filter = filters(
        session,
        &unit,
        &mut checker,
        (&input, arguments.record.as_deref()),
    )
    .await?;
    let command = ExportCommand::check(
        &mut checker,
        arguments.to.as_deref(),
        arguments.format.as_deref(),
    );
    let valid = checker.finish(|| (filter.expect("checked"), command.expect("checked")))?;
    let (filter, command) = valid.command;

    confirm_overwrite(session, &command).await?;
    let entries = collect(&unit, &filter).await?;
    let workspace = unit.workspace().await?;
    // The file is written before the export is recorded: an entry never says that a
    // report exists when it does not.
    session.write_file(
        &command.to,
        &report(&entries, command.format, workspace.name.as_str()),
    )?;
    let exported =
        record_export(&mut unit, &session.stamp(), &command, entries.len() as u64).await?;
    finish(session, unit).await?;
    Ok(Reply::new(exported).with_warnings(valid.warnings))
}

/// Asks before writing over a file that exists.
async fn confirm_overwrite(session: &mut Session, command: &ExportCommand) -> Result<(), Problem> {
    if !command.to.exists() {
        return Ok(());
    }
    let what = format!("Writing over {}", command.to.display());
    match session
        .prompter
        .confirm(&format!("{} exists. Write over it?", command.to.display()))
        .await
    {
        Confirmation::Yes => Ok(()),
        Confirmation::No => Err(Problem::declined(&what)),
        Confirmation::CannotAsk => Err(Problem::confirmation_required(
            &what,
            vec![command.to.display().to_string()],
        )),
    }
}

/// Runs one verb of `trcli telemetry`.
pub async fn telemetry(
    session: &mut Session,
    command: &TelemetryCommand,
) -> Result<Reply, Problem> {
    match command {
        TelemetryCommand::Show => {
            let storage = session.storage(Access::Read).await?;
            let unit = storage.read().await?;
            Ok(Reply::new(
                summarise(&unit, session.telemetry_enabled()).await?,
            ))
        }
        // Turning telemetry on or off is setting `telemetry.enabled` for the workspace,
        // audit entry included.
        TelemetryCommand::On => {
            config::store(session, TELEMETRY_ENABLED, "true", Place::Workspace).await
        }
        TelemetryCommand::Off => {
            config::store(session, TELEMETRY_ENABLED, "false", Place::Workspace).await
        }
        TelemetryCommand::Status => {
            session.located()?;
            let state = if session.telemetry_enabled() {
                "on"
            } else {
                "off"
            };
            Ok(Reply::new(Done::new(format!(
                "Telemetry is {state}. It is kept in this workspace only and is never sent anywhere."
            ))))
        }
    }
}

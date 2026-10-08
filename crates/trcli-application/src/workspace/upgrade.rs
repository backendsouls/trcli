//! Bringing an older workspace to the current format (FR-007).
//!
//! Nothing is upgraded until the researcher asks. A copy of the storage is kept first;
//! the changes of format are applied in one unit of work together with the new format
//! number and an `upgrade` audit entry; if anything fails the unit is dropped and the copy
//! is put back, so the workspace is exactly as it was.

use serde::Serialize;
use trcli_domain::governance::audit::{AuditAction, AuditDraft, Change, Stamp};
use trcli_domain::workspace::{FormatVersion, OpeningState, Workspace};

use super::open::{Access, guard};
use crate::governance::record::commit;
use crate::outcome::{Problem, codes};
use crate::ports::audit::{AuditHeadStore, AuditLog};
use crate::ports::unit_of_work::Storage;
use crate::ports::workspace::{BackupCopy, SchemaUpgrade, WorkspaceStore};

/// What `workspace upgrade` reports.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct UpgradeReport {
    /// The format the workspace was stored in.
    pub from_format: u32,
    /// The format this version of the tool writes.
    pub to_format: u32,
    /// Whether an upgrade is (with `--check`) or was needed.
    pub needed: bool,
    /// Whether the workspace was upgraded by this command.
    pub upgraded: bool,
    /// Where the copy of the workspace as it was is kept.
    pub backup: Option<String>,
}

/// Says whether a workspace needs an upgrade, without changing anything (`--check`).
pub fn check(workspace: &Workspace) -> Result<UpgradeReport, Problem> {
    let state = workspace.opening_state(FormatVersion::CURRENT);
    if matches!(state, OpeningState::TooNew { .. }) {
        guard(&state, Access::Write)?;
    }
    Ok(UpgradeReport {
        from_format: workspace.format_version.number(),
        to_format: FormatVersion::CURRENT.number(),
        needed: matches!(state, OpeningState::NeedsUpgrade { .. }),
        upgraded: false,
        backup: None,
    })
}

/// Upgrades the workspace when it needs it; does nothing, successfully, when it does not.
pub async fn upgrade<S>(
    storage: &S,
    backup: &impl BackupCopy,
    head: &impl AuditHeadStore,
    stamp: &Stamp,
) -> Result<UpgradeReport, Problem>
where
    S: Storage,
    S::Unit: WorkspaceStore + SchemaUpgrade + AuditLog,
{
    let workspace = storage.read().await?.workspace().await?;
    let report = check(&workspace)?;
    if !report.needed {
        return Ok(report);
    }
    // The copy is taken before anything is touched (FR-007).
    let copy = backup.copy()?;
    match apply(storage, head, stamp, workspace).await {
        Ok(()) => Ok(UpgradeReport {
            upgraded: true,
            backup: Some(copy.display().to_string()),
            ..report
        }),
        Err(cause) => {
            let restored = backup.restore(&copy);
            let outcome = match restored {
                Ok(()) => "the workspace is as it was before".to_owned(),
                Err(error) => format!(
                    "restoring the copy also failed ({error}); the copy is at {}",
                    copy.display()
                ),
            };
            Err(Problem::new(
                codes::INTERNAL,
                format!("the upgrade failed: {}; {outcome}", cause.message),
            ))
        }
    }
}

/// Applies the changes of format, the new format number, and the audit entry together.
async fn apply<S>(
    storage: &S,
    head: &impl AuditHeadStore,
    stamp: &Stamp,
    mut workspace: Workspace,
) -> Result<(), Problem>
where
    S: Storage,
    S::Unit: WorkspaceStore + SchemaUpgrade + AuditLog,
{
    let mut unit = storage.begin().await?;
    unit.apply_pending_migrations().await?;
    let (before, after) = (
        workspace.format_version.to_string(),
        FormatVersion::CURRENT.to_string(),
    );
    workspace.format_version = FormatVersion::CURRENT;
    unit.save_workspace(&workspace).await?;
    let change = Change::new("format_version", Some(&before), Some(&after));
    let draft = AuditDraft::workspace(AuditAction::UPGRADE)
        .named(workspace.name.as_str())
        .with_changes(vec![change]);
    unit.record(stamp, draft).await?;
    commit(unit, head).await
}

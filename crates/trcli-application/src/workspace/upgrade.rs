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

#[cfg(test)]
mod tests {
    //! Unit tests for upgrading a workspace.

    use trcli_domain::shared::text::{LongText, Name};
    use trcli_domain::workspace::{FormatVersion, Workspace};

    use super::upgrade;
    use crate::outcome::codes;
    use crate::testing::audit::MemoryHead;
    use crate::testing::block_on;
    use crate::testing::environment::{SeededIds, stamp};
    use crate::testing::unit::FakeStorage;
    use crate::testing::workspace::FakeBackup;

    /// Storage holding a workspace stored in the given format.
    fn storage(format: u32) -> FakeStorage {
        let storage = FakeStorage::new();
        let (name, about) = (
            Name::new("Doctorate").expect("valid"),
            LongText::new("").expect("valid"),
        );
        let mut workspace = Workspace::new(SeededIds::at(0), name, about, stamp().at);
        workspace.format_version = FormatVersion::new(format);
        storage.tamper(|state| state.workspace = Some(workspace));
        storage
    }

    #[test]
    fn an_older_workspace_is_copied_then_upgraded_with_an_audit_entry() {
        let storage = storage(0);
        let (backup, head) = (FakeBackup::of(&storage), MemoryHead::new());
        let report = block_on(upgrade(&storage, &backup, &head, &stamp())).expect("upgraded");
        assert!(report.needed && report.upgraded && report.backup.is_some());
        let state = storage.snapshot();
        assert_eq!(
            state.workspace.expect("row").format_version,
            FormatVersion::CURRENT
        );
        assert_eq!(state.migrations_applied, 1);
        assert_eq!(state.audit[0].action.as_str(), "upgrade");
        assert_eq!(
            backup.copies.borrow()[0]
                .workspace
                .clone()
                .expect("row")
                .format_version
                .number(),
            0
        );
        assert_eq!(head.current().expect("head").sequence, 1);
    }

    #[test]
    fn a_current_workspace_needs_nothing_and_is_not_copied() {
        let storage = storage(FormatVersion::CURRENT.number());
        let backup = FakeBackup::of(&storage);
        let report = block_on(upgrade(&storage, &backup, &MemoryHead::new(), &stamp()))
            .expect("nothing to do");
        assert!(!report.needed && !report.upgraded);
        assert!(backup.copies.borrow().is_empty());
    }

    #[test]
    fn a_failed_upgrade_puts_the_copy_back_and_changes_nothing() {
        let storage = storage(0);
        storage.tamper(|state| state.failing_migration = true);
        let backup = FakeBackup::of(&storage);
        let problem =
            block_on(upgrade(&storage, &backup, &MemoryHead::new(), &stamp())).expect_err("failed");
        assert_eq!(problem.code, codes::INTERNAL);
        assert!(problem.message.contains("as it was before"));
        assert_eq!(*backup.restored.borrow(), 1);
        let state = storage.snapshot();
        assert_eq!(state.workspace.expect("row").format_version.number(), 0);
        assert!(state.audit.is_empty());
    }

    #[test]
    fn a_newer_workspace_is_refused() {
        let storage = storage(99);
        let problem = block_on(upgrade(
            &storage,
            &FakeBackup::of(&storage),
            &MemoryHead::new(),
            &stamp(),
        ));
        assert_eq!(problem.expect_err("too new").code, codes::WORKSPACE_TOO_NEW);
    }
}

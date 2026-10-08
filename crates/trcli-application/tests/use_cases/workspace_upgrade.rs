//! Unit tests for upgrading a workspace.

use trcli_domain::shared::text::{LongText, Name};
use trcli_domain::workspace::{FormatVersion, Workspace};

use trcli_application::outcome::codes;
use trcli_application::workspace::upgrade::upgrade;
use trcli_testing::audit::MemoryHead;
use trcli_testing::block_on;
use trcli_testing::environment::{SeededIds, stamp};
use trcli_testing::unit::FakeStorage;
use trcli_testing::workspace::FakeBackup;

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
    let report =
        block_on(upgrade(&storage, &backup, &MemoryHead::new(), &stamp())).expect("nothing to do");
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

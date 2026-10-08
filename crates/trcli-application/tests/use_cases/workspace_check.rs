//! Unit tests for checking a workspace.

use trcli_domain::governance::audit::{AuditAction, AuditDraft, AuditHead};

use trcli_application::outcome::codes;
use trcli_application::ports::audit::AuditLog;
use trcli_application::ports::unit_of_work::{Storage, UnitOfWork};
use trcli_application::workspace::check::{CheckServices, check};
use trcli_testing::audit::{MemoryHead, ToyDigest};
use trcli_testing::block_on;
use trcli_testing::environment::stamp;
use trcli_testing::interaction::RecordingProgress;
use trcli_testing::unit::FakeStorage;

/// Storage with two audit entries, and the head that matches it.
fn storage_with_trail() -> (FakeStorage, AuditHead) {
    let storage = FakeStorage::new();
    let head = block_on(async {
        let mut unit = storage.begin().await.expect("begin");
        unit.record(&stamp(), AuditDraft::workspace(AuditAction::CREATE))
            .await
            .expect("record");
        unit.record(&stamp(), AuditDraft::workspace(AuditAction::UPDATE))
            .await
            .expect("record");
        unit.commit().await.expect("commit").expect("a head")
    });
    (storage, head)
}

/// Runs the check against the storage with the given head.
fn run(
    storage: &FakeStorage,
    head: &MemoryHead,
) -> Result<trcli_application::workspace::check::CheckReport, trcli_application::outcome::Problem> {
    let mut progress = RecordingProgress::default();
    block_on(async {
        let unit = storage.read().await.expect("read");
        check(
            &unit,
            CheckServices {
                head,
                digest: &ToyDigest,
                progress: &mut progress,
            },
        )
        .await
    })
}

#[test]
fn a_sound_workspace_passes_with_the_number_of_entries_verified() {
    let (storage, head) = storage_with_trail();
    let report = run(&storage, &MemoryHead::holding(head)).expect("sound");
    assert_eq!((report.storage_ok, report.audit_entries), (true, 2));
}

#[test]
fn inconsistent_storage_is_reported_as_damage_with_what_was_found() {
    let (storage, head) = storage_with_trail();
    storage.tamper(|state| {
        state.integrity_problems = vec!["row 3 missing from index record_kind".into()]
    });
    let problem = run(&storage, &MemoryHead::holding(head)).expect_err("damaged");
    assert_eq!(problem.code, codes::WORKSPACE_DAMAGED);
}

#[test]
fn a_tampered_trail_fails_the_check() {
    let (storage, head) = storage_with_trail();
    storage.tamper(|state| state.audit[0].display_name = Some("forged".into()));
    let problem = run(&storage, &MemoryHead::holding(head)).expect_err("tampered");
    assert_eq!(problem.code, codes::CHECK_FAILED);
}

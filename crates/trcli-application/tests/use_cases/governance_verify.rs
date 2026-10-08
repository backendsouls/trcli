//! Unit tests for verification (T103).

use trcli_domain::governance::audit::{AuditAction, AuditDraft, AuditHead};

use trcli_application::governance::verify::{Fault, Finding, TrailVerdict, verify};
use trcli_application::ports::audit::AuditLog;
use trcli_application::ports::unit_of_work::{Storage, UnitOfWork};
use trcli_testing::audit::ToyDigest;
use trcli_testing::block_on;
use trcli_testing::environment::stamp;
use trcli_testing::interaction::RecordingProgress;
use trcli_testing::unit::{FakeStorage, State};

/// Storage with a trail of `count` entries, and the head that matches it.
fn trail(count: usize) -> (FakeStorage, AuditHead) {
    let storage = FakeStorage::new();
    let head = block_on(async {
        let mut unit = storage.begin().await.expect("begin");
        for _ in 0..count {
            unit.record(&stamp(), AuditDraft::workspace(AuditAction::SETTING))
                .await
                .expect("record");
        }
        unit.commit().await.expect("commit").expect("a head")
    });
    (storage, head)
}

/// Verifies the storage against a head.
fn verdict(storage: &FakeStorage, head: Option<AuditHead>) -> TrailVerdict {
    let mut progress = RecordingProgress::default();
    block_on(async {
        let unit = storage.read().await.expect("read");
        verify(&unit, head, &ToyDigest, &mut progress)
            .await
            .expect("verified")
    })
}

/// The verdict after tampering with a trail of five entries.
fn after(tamper: impl FnOnce(&mut State)) -> TrailVerdict {
    let (storage, head) = trail(5);
    storage.tamper(tamper);
    verdict(&storage, Some(head))
}

/// A broken verdict.
fn broken(sequence: u64, fault: Fault) -> TrailVerdict {
    TrailVerdict::Broken(Finding { sequence, fault })
}

#[test]
fn an_intact_trail_passes() {
    let (storage, head) = trail(5);
    assert_eq!(
        verdict(&storage, Some(head)),
        TrailVerdict::Intact { entries: 5 }
    );
    assert_eq!(
        verdict(&FakeStorage::new(), None),
        TrailVerdict::Intact { entries: 0 }
    );
}

#[test]
fn a_wrong_hash_is_reported_as_altered_with_its_sequence() {
    assert_eq!(
        after(|state| state.audit[2].display_name = Some("forged".into())),
        broken(3, Fault::Altered)
    );
}

#[test]
fn a_wrong_previous_hash_is_reported_as_altered() {
    assert_eq!(
        after(|state| state.audit[3].previous_hash = [7; 32]),
        broken(4, Fault::Altered)
    );
}

#[test]
fn a_gap_in_sequences_is_reported_as_missing() {
    assert_eq!(
        after(|state| drop(state.audit.remove(1))),
        broken(2, Fault::Missing)
    );
}

#[test]
fn a_removed_last_entry_is_reported_as_shortened() {
    assert_eq!(
        after(|state| drop(state.audit.pop())),
        broken(5, Fault::Shortened)
    );
}

#[test]
fn a_head_that_does_not_match_its_entry_is_reported() {
    let (storage, mut head) = trail(5);
    head.hash = [1; 32];
    assert_eq!(verdict(&storage, Some(head)), broken(5, Fault::Altered));
}

#[test]
fn a_trail_longer_than_its_head_is_accepted_when_the_head_still_matches() {
    // A command stopped between committing and writing the head leaves this state.
    let (storage, head_of_five) = trail(5);
    block_on(async {
        let mut unit = storage.begin().await.expect("begin");
        unit.record(&stamp(), AuditDraft::workspace(AuditAction::SETTING))
            .await
            .expect("record");
        unit.commit().await.expect("commit");
    });
    assert_eq!(
        verdict(&storage, Some(head_of_five)),
        TrailVerdict::Intact { entries: 6 }
    );
}

#[test]
fn entries_without_a_head_cannot_be_vouched_for() {
    let (storage, _) = trail(2);
    assert_eq!(verdict(&storage, None), broken(2, Fault::HeadMissing));
}

#[test]
fn a_finding_becomes_a_check_failed_problem_naming_where() {
    let problem = Finding {
        sequence: 3,
        fault: Fault::Altered,
    }
    .into_problem();
    assert_eq!(problem.outcome().exit_code(), 6);
    assert!(format!("{:?}", problem.details).contains("entry 3 was altered"));
}

#[test]
fn progress_is_reported_and_finished() {
    let (storage, head) = trail(3);
    let mut progress = RecordingProgress::default();
    block_on(async {
        let unit = storage.read().await.expect("read");
        verify(&unit, Some(head), &ToyDigest, &mut progress)
            .await
            .expect("verified");
    });
    assert!(progress.finished && progress.advances >= 1);
}

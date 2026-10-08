//! Unit tests for recording changes (T105).

use trcli_domain::governance::audit::{AuditAction, AuditDraft};

use trcli_application::governance::record::{ChangeSet, commit};
use trcli_application::outcome::Details;
use trcli_application::ports::audit::AuditLog;
use trcli_application::ports::unit_of_work::Storage;
use trcli_application::validation::Checker;
use trcli_testing::audit::MemoryHead;
use trcli_testing::block_on;
use trcli_testing::environment::stamp;
use trcli_testing::unit::FakeStorage;

#[test]
fn a_secret_field_is_absent_from_an_entrys_changes() {
    let mut changes = ChangeSet::new();
    changes
        .field("title", Some("Old"), Some("New"))
        .secret_field("passphrase");
    let recorded = changes.into_changes();
    assert_eq!(recorded.len(), 1);
    assert!(recorded.iter().all(|change| change.field != "passphrase"));
}

#[test]
fn a_secret_value_is_absent_from_problem_messages_and_details() {
    let mut checker = Checker::new();
    let rejection =
        trcli_domain::shared::problem::Rejection::new("is too short", "at least 12 characters");
    checker.reject_secret("--passphrase", rejection);
    let problem = checker.finish(|| ()).expect_err("invalid");
    assert!(!problem.message.contains("hunter2"));
    let Details::Fields(fields) = problem.details else {
        panic!("fields expected")
    };
    // The structured form is built from these fields: with no value here, none can
    // reach the JSON envelope either.
    assert_eq!(fields[0].value, None);
}

#[test]
fn an_unchanged_field_is_not_recorded() {
    let mut changes = ChangeSet::new();
    changes.field("title", Some("Same"), Some("Same"));
    assert!(changes.is_empty());
}

#[test]
fn committing_stores_the_new_end_of_the_trail() {
    let (storage, head) = (FakeStorage::new(), MemoryHead::new());
    block_on(async {
        let mut unit = storage.begin().await.expect("begin");
        unit.record(&stamp(), AuditDraft::workspace(AuditAction::CREATE))
            .await
            .expect("record");
        commit(unit, &head).await.expect("commit");
    });
    let stored = head.current().expect("a head");
    assert_eq!(
        (stored.sequence, stored.hash),
        (1, storage.snapshot().audit[0].hash)
    );
}

#[test]
fn committing_without_entries_leaves_the_head_alone() {
    let (storage, head) = (FakeStorage::new(), MemoryHead::new());
    block_on(async {
        commit(storage.begin().await.expect("begin"), &head)
            .await
            .expect("commit")
    });
    assert_eq!(head.current(), None);
}

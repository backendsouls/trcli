//! Unit tests for the trail's filters and views.

use trcli_domain::governance::audit::{AuditAction, AuditDraft, Change};

use trcli_application::governance::query::{AuditInput, filter, list};
use trcli_application::kinds::{KindRegistry, RecordKindDescriptor};
use trcli_application::outcome::Details;
use trcli_application::ports::audit::AuditLog;
use trcli_application::ports::unit_of_work::Storage;
use trcli_testing::block_on;
use trcli_testing::environment::{SeededIds, stamp};
use trcli_testing::unit::FakeStorage;

/// A registry with the kind `alpha`.
fn kinds() -> KindRegistry {
    let mut registry = KindRegistry::new();
    registry
        .register(RecordKindDescriptor::new("alpha", "alp", "Alpha.", "title"))
        .expect("registered");
    registry
}

/// The names of the fields a set of filters is rejected for.
fn rejected(input: AuditInput) -> Vec<String> {
    let problem = filter(&input, None, &kinds(), 50).expect_err("invalid");
    let Details::Fields(fields) = problem.details else {
        panic!("fields expected")
    };
    fields.into_iter().map(|field| field.field).collect()
}

#[test]
fn valid_filters_are_accepted_with_the_default_limit() {
    let input = AuditInput {
        kind: Some("alpha".into()),
        action: Some("create".into()),
        ..AuditInput::default()
    };
    let checked = filter(&input, Some(SeededIds::at(1)), &kinds(), 50)
        .expect("valid")
        .command;
    assert_eq!(
        (checked.limit, checked.kind.as_deref()),
        (50, Some("alpha"))
    );
    assert_eq!(checked.record, Some(SeededIds::at(1)));
}

#[test]
fn every_invalid_filter_is_reported_at_once() {
    let input = AuditInput {
        kind: Some("gamma".into()),
        action: Some("explode".into()),
        from: Some("yesterday".into()),
        limit: Some("0".into()),
        ..AuditInput::default()
    };
    assert_eq!(rejected(input), ["--kind", "--action", "--from", "--limit"]);
}

#[test]
fn an_end_before_its_start_is_reported_with_the_rule() {
    let input = AuditInput {
        from: Some("2026-10-08".into()),
        to: Some("2026-10-01".into()),
        ..AuditInput::default()
    };
    assert_eq!(rejected(input), ["--to"]);
}

#[test]
fn an_entry_says_what_it_is_about() {
    let storage = FakeStorage::new();
    let listed = block_on(async {
        let mut unit = storage.begin().await.expect("begin");
        let draft = AuditDraft::record(
            AuditAction::UPDATE,
            "alpha",
            SeededIds::at(0),
            "alp-7k3f",
            "A title",
        )
        .with_changes(vec![Change::new(
            "status",
            Some("to_read"),
            Some("reading"),
        )]);
        unit.record(&stamp(), draft).await.expect("record");
        list(
            &unit,
            &filter(&AuditInput::default(), None, &kinds(), 50)
                .expect("valid")
                .command,
        )
        .await
    })
    .expect("listed");
    assert_eq!(listed.total, 1);
    assert_eq!(
        listed.items[0].what("->"),
        "alp-7k3f \"A title\" status: to_read -> reading"
    );
    assert_eq!(listed.items[0].hash.len(), 64);
}

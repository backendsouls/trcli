//! Unit tests for tagging (T066).

use trcli_application::kinds::{KindRegistry, RecordKindDescriptor};
use trcli_application::outcome::{Details, codes};
use trcli_application::ports::records::RecordIndex;
use trcli_application::ports::unit_of_work::Storage;
use trcli_application::records::tag::{TagCommand, list_tags, tag};
use trcli_testing::block_on;
use trcli_testing::contract_records::indexed;
use trcli_testing::environment::stamp;
use trcli_testing::unit::{FakeStorage, FakeUnit};

/// A unit holding one `alpha` record; returns its handle too.
fn unit() -> (FakeUnit, String) {
    block_on(async {
        let mut unit = FakeStorage::new().begin().await.expect("begin");
        let record = indexed(0, "alpha", "alp", "First");
        unit.insert_record(&record).await.expect("insert");
        (unit, record.handle.to_string())
    })
}

/// The checked command tagging `handle` with `tags`.
fn command(handle: &str, tags: &[&str], remove: bool) -> TagCommand {
    let tags: Vec<String> = tags.iter().map(|tag| (*tag).to_owned()).collect();
    TagCommand::new(handle, &tags, remove)
        .expect("valid")
        .command
}

#[test]
fn tagging_twice_leaves_one_tagging_and_one_audit_entry() {
    let (mut unit, handle) = unit();
    block_on(async {
        let first = tag(
            &mut unit,
            &stamp(),
            &[],
            command(&handle, &["Field-Work"], false),
        )
        .await
        .expect("tagged");
        assert_eq!(first.added, ["field-work"]);
        let again = tag(
            &mut unit,
            &stamp(),
            &[],
            command(&handle, &["field-work"], false),
        )
        .await
        .expect("tagged");
        assert!(again.added.is_empty());
        assert_eq!(again.tags, ["field-work"]);
    });
    assert_eq!(unit.state().taggings.len(), 1);
    assert_eq!(unit.state().audit.len(), 1);
    assert_eq!(unit.state().audit[0].action.as_str(), "tag");
}

#[test]
fn each_tag_changed_records_one_entry_and_removal_is_recorded_as_untag() {
    let (mut unit, handle) = unit();
    block_on(async {
        tag(
            &mut unit,
            &stamp(),
            &[],
            command(&handle, &["a", "b"], false),
        )
        .await
        .expect("tagged");
        let removed = tag(
            &mut unit,
            &stamp(),
            &[],
            command(&handle, &["a", "zzz"], true),
        )
        .await
        .expect("untagged");
        assert_eq!(
            (removed.removed, removed.tags),
            (vec!["a".to_owned()], vec!["b".to_owned()])
        );
    });
    let actions: Vec<&str> = unit
        .state()
        .audit
        .iter()
        .map(|entry| entry.action.as_str())
        .collect();
    assert_eq!(actions, ["tag", "tag", "untag"]);
}

#[test]
fn every_invalid_tag_name_is_reported_with_the_rule() {
    let tags = vec!["two words".to_owned(), "ok".to_owned(), "ação".to_owned()];
    let problem = TagCommand::new("alp-1", &tags, false).expect_err("invalid");
    let Details::Fields(fields) = problem.details else {
        panic!("fields expected")
    };
    assert_eq!(fields.len(), 2);
    assert!(fields[0].expected.contains("a-z"));
    assert!(TagCommand::new("alp-1", &[], false).is_err());
}

#[test]
fn tags_are_listed_with_counts_and_an_unknown_kind_is_invalid() {
    let (mut unit, handle) = unit();
    let mut kinds = KindRegistry::new();
    kinds
        .register(RecordKindDescriptor::new("alpha", "alp", "Alpha.", "title"))
        .expect("registered");
    block_on(async {
        tag(
            &mut unit,
            &stamp(),
            &[],
            command(&handle, &["b", "a"], false),
        )
        .await
        .expect("tagged");
        let listed = list_tags(&unit, &kinds, Some("alpha"))
            .await
            .expect("listed");
        let names: Vec<&str> = listed.items.iter().map(|item| item.tag.as_str()).collect();
        assert_eq!((names, listed.total), (vec!["a", "b"], 2));
        let problem = list_tags(&unit, &kinds, Some("gamma"))
            .await
            .expect_err("unknown kind");
        assert_eq!(problem.code, codes::VALIDATION_FAILED);
    });
}

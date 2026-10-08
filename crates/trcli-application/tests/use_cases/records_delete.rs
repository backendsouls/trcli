//! Unit tests for the generic deletion use case (T064).

use trcli_domain::shared::link::Link;
use trcli_domain::shared::note::Note;
use trcli_domain::shared::record::RecordId;
use trcli_domain::shared::text::{Relation, TagName};

use trcli_application::kinds::{DeletionBlock, KindBehaviour, KindField, RecordKindDescriptor};
use trcli_application::outcome::{Details, Problem, codes};
use trcli_application::ports::interaction::Confirmation;
use trcli_application::ports::records::{LinkStore, NoteStore, RecordIndex, TagStore};
use trcli_application::ports::unit_of_work::{Storage, StoreError};
use trcli_application::records::delete::delete;
use trcli_testing::block_on;
use trcli_testing::contract_records::indexed;
use trcli_testing::environment::stamp;
use trcli_testing::interaction::ScriptedPrompter;
use trcli_testing::unit::{FakeStorage, FakeUnit};

/// A kind whose records keep one row each, and which can forbid deletion.
struct Alpha {
    /// The kind's descriptor.
    descriptor: RecordKindDescriptor,
    /// Whether deletion is forbidden.
    blocked: bool,
}

impl Alpha {
    /// The kind, allowing or forbidding deletion.
    fn new(blocked: bool) -> Self {
        Self {
            descriptor: RecordKindDescriptor::new("alpha", "alp", "Alpha.", "title"),
            blocked,
        }
    }
}

impl KindBehaviour<FakeUnit> for Alpha {
    fn descriptor(&self) -> &RecordKindDescriptor {
        &self.descriptor
    }

    async fn deletion_block(
        &self,
        _unit: &FakeUnit,
        _id: RecordId,
    ) -> Result<Option<DeletionBlock>, StoreError> {
        Ok(self.blocked.then(|| DeletionBlock {
            reason: "it has results that would lose their history".to_owned(),
            alternative: "archive it instead with `trcli alpha archive`".to_owned(),
        }))
    }

    async fn dependents(&self, _unit: &FakeUnit, _id: RecordId) -> Result<Vec<String>, StoreError> {
        Ok(vec!["1 result".to_owned()])
    }

    async fn remove_rows(&self, unit: &mut FakeUnit, id: RecordId) -> Result<(), StoreError> {
        unit.remove_kind_row("alpha", id);
        Ok(())
    }

    async fn fields(&self, _unit: &FakeUnit, _id: RecordId) -> Result<Vec<KindField>, StoreError> {
        Ok(Vec::new())
    }
}

/// A unit holding an `alpha` record with its own row, a tag, a note, and a link to a
/// second record. Returns the first record's handle too.
fn unit() -> (FakeUnit, String) {
    block_on(async {
        let mut unit = FakeStorage::new().begin().await.expect("begin");
        let (first, second) = (
            indexed(0, "alpha", "alp", "First"),
            indexed(1, "beta", "bet", "Second"),
        );
        unit.insert_record(&first).await.expect("insert");
        unit.insert_record(&second).await.expect("insert");
        unit.put_kind_row(
            "alpha",
            first.id,
            [("title".to_owned(), "First".to_owned())].into(),
        );
        unit.attach_tag(first.id, &TagName::new("field-work").expect("valid"))
            .await
            .expect("tag");
        unit.add_note(&Note::new(
            first.id,
            Note::body("a remark").expect("valid"),
            stamp().at,
        ))
        .await
        .expect("note");
        let link =
            Link::new(second.id, first.id, Relation::related(), stamp().at).expect("two records");
        unit.add_link(&link).await.expect("link");
        (unit, first.handle.to_string())
    })
}

/// Deletes the first record with the given kind behaviour and answer.
fn run(
    unit: &mut FakeUnit,
    handle: &str,
    blocked: bool,
    answer: Confirmation,
) -> (
    Result<trcli_application::records::delete::Deleted, Problem>,
    ScriptedPrompter,
) {
    let mut prompter = ScriptedPrompter::answering(answer);
    let result = block_on(delete(
        unit,
        &stamp(),
        &Alpha::new(blocked),
        &mut prompter,
        handle,
    ));
    (result, prompter)
}

#[test]
fn a_blocking_policy_refuses_with_the_alternative_and_asks_nothing() {
    let (mut unit, handle) = unit();
    let (result, prompter) = run(&mut unit, &handle, true, Confirmation::Yes);
    let problem = result.expect_err("blocked");
    assert_eq!(problem.code, codes::BLOCKED_BY_DEPENDENTS);
    assert!(problem.message.contains("would lose their history"));
    assert!(
        problem
            .next_step
            .expect("an alternative")
            .contains("archive")
    );
    assert!(prompter.questions.is_empty());
    assert_eq!(unit.state().links.len(), 1);
}

#[test]
fn dependents_are_listed_before_asking() {
    let (mut unit, handle) = unit();
    let (_, prompter) = run(&mut unit, &handle, false, Confirmation::No);
    let question = &prompter.questions[0];
    for expected in [
        "will also remove:",
        "link to bet-",
        "1 tag: field-work",
        "1 note",
        "1 result",
        "Delete?",
    ] {
        assert!(
            question.contains(expected),
            "{expected} missing from: {question}"
        );
    }
}

#[test]
fn without_confirmation_nothing_changes() {
    for (answer, code) in [
        (Confirmation::No, codes::DECLINED),
        (Confirmation::CannotAsk, codes::CONFIRMATION_REQUIRED),
    ] {
        let (mut unit, handle) = unit();
        let (result, _) = run(&mut unit, &handle, false, answer);
        let problem = result.expect_err("not confirmed");
        assert_eq!(problem.code, code);
        assert!(!problem.changed);
        let state = unit.state();
        assert_eq!(
            (state.links.len(), state.taggings.len(), state.notes.len()),
            (1, 1, 1)
        );
        assert!(state.audit.is_empty() && state.records[0].deleted_at.is_none());
    }
}

#[test]
fn nobody_to_ask_lists_what_would_be_affected() {
    let (mut unit, handle) = unit();
    let (result, _) = run(&mut unit, &handle, false, Confirmation::CannotAsk);
    let Details::Items(affected) = result.expect_err("cannot ask").details else {
        panic!("items expected")
    };
    assert_eq!(affected.len(), 4);
}

#[test]
fn confirmed_deletion_removes_everything_and_writes_one_audit_entry() {
    let (mut unit, handle) = unit();
    let (result, _) = run(&mut unit, &handle, false, Confirmation::Yes);
    let deleted = result.expect("deleted");
    assert_eq!(
        (deleted.handle.as_str(), deleted.name.as_str()),
        (handle.as_str(), "First")
    );
    let first = unit.state().records[0].id;
    assert!(
        unit.kind_row("alpha", first).is_none(),
        "the kind's own row is removed"
    );
    let state = unit.state();
    assert!(state.links.is_empty() && state.taggings.is_empty() && state.notes.is_empty());
    assert!(
        state.records[0].deleted_at.is_some(),
        "the index row stays, marked deleted"
    );
    assert_eq!(state.audit.len(), 1);
    assert_eq!(
        (
            state.audit[0].action.as_str(),
            state.audit[0].display_name.as_deref()
        ),
        ("delete", Some("First"))
    );
}

#[test]
fn a_record_of_another_kind_is_not_found_by_this_kinds_command() {
    let (mut unit, _) = unit();
    let other = unit.state().records[1].handle.to_string();
    let (result, _) = run(&mut unit, &other, false, Confirmation::Yes);
    assert_eq!(result.expect_err("other kind").code, codes::NOT_FOUND);
}

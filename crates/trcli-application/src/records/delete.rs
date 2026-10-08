//! Deleting a record of any kind, leaving nothing pointing at it (FR-012, FR-017).
//!
//! ```text
//!  ask the kind: is deletion blocked?  ── yes ──▶ refuse; say what stands in the way and the alternative
//!                 │ no
//!  list dependents: links, tags, notes, and what the kind reports as referring to it
//!                 │
//!  confirm (Prompter) ── no / cannot ask ──▶ nothing changes
//!                 │ yes
//!  in one unit of work: the kind removes its own rows ─▶ links, taggings, notes removed
//!                       ─▶ index row marked deleted ─▶ audit entry
//! ```

use serde::Serialize;
use trcli_domain::governance::audit::{AuditAction, AuditDraft, Stamp};

use super::link::links_of;
use super::resolve::resolve;
use crate::kinds::KindBehaviour;
use crate::outcome::{Problem, codes};
use crate::ports::audit::AuditLog;
use crate::ports::interaction::{Confirmation, Prompter};
use crate::ports::records::{
    IndexedRecord, LinkStore, NoteStore, RecordIndex, RecordResolver, TagStore,
};

/// Everything deletion needs from a unit of work.
pub trait DeletionStores:
    RecordResolver + RecordIndex + TagStore + NoteStore + LinkStore + AuditLog
{
}

impl<U: RecordResolver + RecordIndex + TagStore + NoteStore + LinkStore + AuditLog> DeletionStores
    for U
{
}

/// What deleting a record reports.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct Deleted {
    /// The short name the record had; it is never given to another record.
    pub handle: String,
    /// What the record was called.
    pub name: String,
    /// The record's kind.
    pub kind: String,
    /// What was removed with it.
    pub removed: Vec<String>,
}

/// Deletes the record that was typed, after listing what refers to it and asking.
pub async fn delete<U, K, P>(
    unit: &mut U,
    stamp: &Stamp,
    kind: &K,
    prompter: &mut P,
    reference: &str,
) -> Result<Deleted, Problem>
where
    U: DeletionStores,
    K: KindBehaviour<U>,
    P: Prompter,
{
    let kinds = [kind.descriptor().name().to_owned()];
    let record = resolve(unit, reference, &kinds).await?;
    if let Some(block) = kind.deletion_block(unit, record.id).await? {
        let message = format!(
            "{} \"{}\" cannot be deleted: {}",
            record.handle, record.display_name, block.reason
        );
        return Err(
            Problem::new(codes::BLOCKED_BY_DEPENDENTS, message).with_next_step(block.alternative)
        );
    }
    let dependents = dependents(unit, kind, &record).await?;
    confirm(prompter, &record, &dependents).await?;
    remove(unit, stamp, kind, &record).await?;
    Ok(Deleted {
        handle: record.handle.to_string(),
        name: record.display_name,
        kind: record.kind,
        removed: dependents,
    })
}

/// Removes the record and everything attached to it, and records the deletion: all in
/// the caller's unit of work, so that it happens whole or not at all.
async fn remove<U, K>(
    unit: &mut U,
    stamp: &Stamp,
    kind: &K,
    record: &IndexedRecord,
) -> Result<(), Problem>
where
    U: DeletionStores,
    K: KindBehaviour<U>,
{
    kind.remove_rows(unit, record.id).await?;
    unit.remove_links_of(record.id).await?;
    unit.remove_taggings_of(record.id).await?;
    unit.remove_notes_of(record.id).await?;
    unit.mark_deleted(record.id, stamp.at).await?;
    // The entry keeps the name the record had, since the record will no longer say (FR-046).
    let draft = AuditDraft::record(
        AuditAction::DELETE,
        &record.kind,
        record.id,
        record.handle.as_str(),
        &record.display_name,
    );
    unit.record(stamp, draft).await?;
    Ok(())
}

/// What refers to the record and will go with it: each link, the tags, the notes, and
/// whatever the kind adds.
async fn dependents<U, K>(
    unit: &U,
    kind: &K,
    record: &IndexedRecord,
) -> Result<Vec<String>, Problem>
where
    U: DeletionStores,
    K: KindBehaviour<U>,
{
    let mut dependents = Vec::new();
    for link in links_of(unit, record).await? {
        dependents.push(format!(
            "link to {} \"{}\" ({})",
            link.other.handle, link.other.name, link.relation
        ));
    }
    let tags = unit.tags_of(record.id).await?;
    if !tags.is_empty() {
        let names: Vec<&str> = tags.iter().map(|tag| tag.as_str()).collect();
        dependents.push(format!(
            "{}: {}",
            counted(tags.len(), "tag"),
            names.join(", ")
        ));
    }
    let notes = unit.notes_of(record.id).await?.len();
    if notes > 0 {
        dependents.push(counted(notes, "note"));
    }
    dependents.extend(kind.dependents(unit, record.id).await?);
    Ok(dependents)
}

/// "1 note", "3 notes".
fn counted(count: usize, noun: &str) -> String {
    if count == 1 {
        format!("1 {noun}")
    } else {
        format!("{count} {noun}s")
    }
}

/// Asks for confirmation; anything but yes changes nothing.
async fn confirm<P: Prompter>(
    prompter: &mut P,
    record: &IndexedRecord,
    dependents: &[String],
) -> Result<(), Problem> {
    let what = format!(
        "Deleting {} {} \"{}\"",
        record.kind, record.handle, record.display_name
    );
    let mut question = what.clone();
    if !dependents.is_empty() {
        question.push_str(" will also remove:");
        for dependent in dependents {
            question.push_str("\n  ");
            question.push_str(dependent);
        }
    }
    question.push_str("\nDelete?");
    match prompter.confirm(&question).await {
        Confirmation::Yes => Ok(()),
        Confirmation::No => Err(Problem::declined(&what)),
        Confirmation::CannotAsk => Err(Problem::confirmation_required(&what, dependents.to_vec())),
    }
}

#[cfg(test)]
mod tests {
    //! Unit tests for the generic deletion use case (T064).

    use trcli_domain::shared::link::Link;
    use trcli_domain::shared::note::Note;
    use trcli_domain::shared::record::RecordId;
    use trcli_domain::shared::text::{Relation, TagName};

    use super::delete;
    use crate::kinds::{DeletionBlock, KindBehaviour, KindField, RecordKindDescriptor};
    use crate::outcome::{Details, Problem, codes};
    use crate::ports::interaction::Confirmation;
    use crate::ports::records::{LinkStore, NoteStore, RecordIndex, TagStore};
    use crate::ports::unit_of_work::{Storage, StoreError};
    use crate::testing::block_on;
    use crate::testing::contract_records::indexed;
    use crate::testing::environment::stamp;
    use crate::testing::interaction::ScriptedPrompter;
    use crate::testing::unit::{FakeStorage, FakeUnit};

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

        async fn dependents(
            &self,
            _unit: &FakeUnit,
            _id: RecordId,
        ) -> Result<Vec<String>, StoreError> {
            Ok(vec!["1 result".to_owned()])
        }

        async fn remove_rows(&self, unit: &mut FakeUnit, id: RecordId) -> Result<(), StoreError> {
            unit.remove_kind_row("alpha", id);
            Ok(())
        }

        async fn fields(
            &self,
            _unit: &FakeUnit,
            _id: RecordId,
        ) -> Result<Vec<KindField>, StoreError> {
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
            let link = Link::new(second.id, first.id, Relation::related(), stamp().at)
                .expect("two records");
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
    ) -> (Result<super::Deleted, Problem>, ScriptedPrompter) {
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
}

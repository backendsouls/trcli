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

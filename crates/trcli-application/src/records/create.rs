//! Adding a record to the index and renaming one, each with its audit entry (FR-010,
//! FR-018, FR-046).
//!
//! A feature's `add` use case stores its own row and calls [`register`]; its `edit` use
//! case calls [`update`], with the new name when the name changed. That is all a feature writes to give its records a short name, a place in
//! searches, and a history.

use trcli_domain::governance::audit::{AuditAction, AuditDraft, Change, Stamp};
use trcli_domain::shared::record::RecordId;
use trcli_domain::shared::text::SearchKey;

use super::handles::assign;
use crate::kinds::RecordKindDescriptor;
use crate::outcome::Problem;
use crate::ports::audit::AuditLog;
use crate::ports::records::{IndexedRecord, RecordIndex};

/// Adds a new record of a kind to the index under a fresh short name and records its
/// creation. `changes` lists the fields it was created with.
pub async fn register<U>(
    unit: &mut U,
    stamp: &Stamp,
    descriptor: &RecordKindDescriptor,
    id: RecordId,
    name_and_changes: (&str, Vec<Change>),
) -> Result<IndexedRecord, Problem>
where
    U: RecordIndex + AuditLog,
{
    let (display_name, changes) = name_and_changes;
    let record = IndexedRecord {
        id,
        kind: descriptor.name().to_owned(),
        handle: assign(unit, &descriptor.kind, id).await?,
        display_name: display_name.to_owned(),
        search_key: SearchKey::from_text(display_name),
        created_at: stamp.at,
        updated_at: stamp.at,
        deleted_at: None,
    };
    unit.insert_record(&record).await?;
    let draft = AuditDraft::record(
        AuditAction::CREATE,
        &record.kind,
        id,
        record.handle.as_str(),
        display_name,
    )
    .with_changes(changes);
    unit.record(stamp, draft).await?;
    Ok(record)
}

/// Records that a record's fields changed, updating what it is called and found by when
/// `new_name` is given. Returns the record as it now is.
pub async fn update<U>(
    unit: &mut U,
    stamp: &Stamp,
    record: IndexedRecord,
    new_name: Option<&str>,
    changes: Vec<Change>,
) -> Result<IndexedRecord, Problem>
where
    U: RecordIndex + AuditLog,
{
    let mut record = record;
    match new_name {
        Some(name) => {
            record.display_name = name.to_owned();
            record.search_key = SearchKey::from_text(name);
            unit.rename_record(record.id, name, &record.search_key, stamp.at)
                .await?;
        }
        None => unit.touch_record(record.id, stamp.at).await?,
    }
    record.updated_at = stamp.at;
    let draft = AuditDraft::record(
        AuditAction::UPDATE,
        &record.kind,
        record.id,
        record.handle.as_str(),
        &record.display_name,
    )
    .with_changes(changes);
    unit.record(stamp, draft).await?;
    Ok(record)
}

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

#[cfg(test)]
mod tests {
    //! Unit tests for registering and updating records.

    use trcli_domain::governance::audit::Change;

    use super::{register, update};
    use crate::kinds::RecordKindDescriptor;
    use crate::ports::records::RecordIndex;
    use crate::ports::unit_of_work::Storage;
    use crate::testing::block_on;
    use crate::testing::environment::{SeededIds, stamp};
    use crate::testing::unit::FakeStorage;

    /// The kind used by these tests.
    fn descriptor() -> RecordKindDescriptor {
        RecordKindDescriptor::new("alpha", "alp", "Alpha.", "title")
    }

    #[test]
    fn a_registered_record_has_a_handle_a_search_key_and_a_create_entry() {
        block_on(async {
            let mut unit = FakeStorage::new().begin().await.expect("begin");
            let changes = vec![Change::set("title", "Ação")];
            let record = register(
                &mut unit,
                &stamp(),
                &descriptor(),
                SeededIds::at(0),
                ("Ação", changes),
            )
            .await;
            let record = record.expect("registered");
            assert!(record.handle.as_str().starts_with("alp-"));
            assert_eq!(record.search_key.as_str(), "acao");
            let entry = &unit.state().audit[0];
            assert_eq!(
                (entry.action.as_str(), entry.handle.as_deref()),
                ("create", Some(record.handle.as_str()))
            );
            assert_eq!(entry.display_name.as_deref(), Some("Ação"));
        });
    }

    #[test]
    fn renaming_changes_what_the_record_is_found_by_and_records_an_update() {
        block_on(async {
            let mut unit = FakeStorage::new().begin().await.expect("begin");
            let record = register(
                &mut unit,
                &stamp(),
                &descriptor(),
                SeededIds::at(0),
                ("Old", Vec::new()),
            )
            .await;
            let changes = vec![Change::new("title", Some("Old"), Some("New name"))];
            let renamed = update(
                &mut unit,
                &stamp(),
                record.expect("registered"),
                Some("New name"),
                changes,
            )
            .await;
            let stored = unit
                .record(renamed.expect("renamed").id)
                .await
                .expect("read")
                .expect("there");
            assert_eq!(
                (stored.display_name.as_str(), stored.search_key.as_str()),
                ("New name", "new name")
            );
            assert_eq!(unit.state().audit[1].action.as_str(), "update");
        });
    }
}

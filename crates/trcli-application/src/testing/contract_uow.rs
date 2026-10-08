//! Contract of the unit of work and the audit log (T018): what every storage adapter must
//! guarantee about changes and their audit entries (FR-047, FR-051).
//!
//! Run it against a storage adapter by calling [`run`] with a function that makes fresh,
//! empty storage each time it is called.

use trcli_domain::governance::audit::{AuditAction, AuditDraft, GENESIS_HASH};
use trcli_domain::shared::text::{LongText, Name};
use trcli_domain::workspace::Workspace;

use super::environment::{SeededIds, stamp};
use crate::ports::audit::{AuditFilter, AuditLog, AuditQuery};
use crate::ports::unit_of_work::{Storage, UnitOfWork};
use crate::ports::workspace::WorkspaceStore;

/// A workspace row to store as "the change".
fn workspace(name: &str) -> Workspace {
    Workspace::new(
        SeededIds::at(0),
        Name::new(name).expect("a valid name"),
        LongText::new("").expect("valid text"),
        stamp().at,
    )
}

/// Runs every case of the contract, each on fresh storage.
pub async fn run<S>(fresh: impl AsyncFn() -> S)
where
    S: Storage,
    S::Unit: AuditLog + AuditQuery + WorkspaceStore,
{
    a_change_and_its_entry_are_both_present_after_commit(&fresh().await).await;
    neither_is_present_when_the_unit_is_dropped(&fresh().await).await;
    sequences_start_at_one_and_increase_by_one(&fresh().await).await;
    each_entry_follows_the_hash_before_it(&fresh().await).await;
    commit_reports_the_new_end_of_the_trail(&fresh().await).await;
}

/// After commit, both the change and its audit entry can be read.
async fn a_change_and_its_entry_are_both_present_after_commit<S>(storage: &S)
where
    S: Storage,
    S::Unit: AuditLog + AuditQuery + WorkspaceStore,
{
    let mut unit = storage.begin().await.expect("begin");
    unit.save_workspace(&workspace("Doctorate"))
        .await
        .expect("save");
    unit.record(
        &stamp(),
        AuditDraft::workspace(AuditAction::CREATE).named("Doctorate"),
    )
    .await
    .expect("record");
    unit.commit().await.expect("commit");

    let reader = storage.read().await.expect("read");
    assert_eq!(
        reader.workspace().await.expect("workspace").name.as_str(),
        "Doctorate"
    );
    let page = reader
        .entries(&AuditFilter::everything(10))
        .await
        .expect("entries");
    assert_eq!(page.total, 1);
    assert_eq!(page.entries[0].display_name.as_deref(), Some("Doctorate"));
    assert_eq!(page.entries[0].actor.as_str(), "ana");
}

/// A unit dropped without commit leaves neither the change nor the entry.
async fn neither_is_present_when_the_unit_is_dropped<S>(storage: &S)
where
    S: Storage,
    S::Unit: AuditLog + AuditQuery + WorkspaceStore,
{
    {
        let mut unit = storage.begin().await.expect("begin");
        unit.save_workspace(&workspace("Abandoned"))
            .await
            .expect("save");
        unit.record(&stamp(), AuditDraft::workspace(AuditAction::CREATE))
            .await
            .expect("record");
        // Dropped here: the command failed or was interrupted.
    }
    let reader = storage.read().await.expect("read");
    assert!(
        reader.workspace().await.is_err(),
        "the change must be absent"
    );
    assert_eq!(
        reader.entry_count().await.expect("count"),
        0,
        "the entry must be absent"
    );
}

/// Entries are numbered 1, 2, 3, … across units of work, with no gap.
async fn sequences_start_at_one_and_increase_by_one<S>(storage: &S)
where
    S: Storage,
    S::Unit: AuditLog + AuditQuery + WorkspaceStore,
{
    for _ in 0..2 {
        let mut unit = storage.begin().await.expect("begin");
        unit.record(&stamp(), AuditDraft::workspace(AuditAction::SETTING))
            .await
            .expect("record");
        unit.record(&stamp(), AuditDraft::workspace(AuditAction::SETTING))
            .await
            .expect("record");
        unit.commit().await.expect("commit");
    }
    let reader = storage.read().await.expect("read");
    let sequences: Vec<u64> = reader
        .entries_after(0, 100)
        .await
        .expect("entries")
        .iter()
        .map(|entry| entry.sequence)
        .collect();
    assert_eq!(sequences, [1, 2, 3, 4]);
}

/// The first entry follows zeros; every other follows the hash of the one before.
async fn each_entry_follows_the_hash_before_it<S>(storage: &S)
where
    S: Storage,
    S::Unit: AuditLog + AuditQuery + WorkspaceStore,
{
    let mut unit = storage.begin().await.expect("begin");
    for _ in 0..3 {
        unit.record(&stamp(), AuditDraft::workspace(AuditAction::SETTING))
            .await
            .expect("record");
    }
    unit.commit().await.expect("commit");

    let reader = storage.read().await.expect("read");
    let entries = reader.entries_after(0, 100).await.expect("entries");
    assert_eq!(entries[0].previous_hash, GENESIS_HASH);
    assert_eq!(entries[1].previous_hash, entries[0].hash);
    assert_eq!(entries[2].previous_hash, entries[1].hash);
    assert_ne!(entries[1].hash, entries[2].hash);
}

/// Commit hands back the last entry recorded, and nothing when none was.
async fn commit_reports_the_new_end_of_the_trail<S>(storage: &S)
where
    S: Storage,
    S::Unit: AuditLog + AuditQuery + WorkspaceStore,
{
    let mut unit = storage.begin().await.expect("begin");
    unit.record(&stamp(), AuditDraft::workspace(AuditAction::SETTING))
        .await
        .expect("record");
    let last = unit
        .record(&stamp(), AuditDraft::workspace(AuditAction::SETTING))
        .await
        .expect("record");
    let head = unit.commit().await.expect("commit").expect("a head");
    assert_eq!((head.sequence, head.hash), (last.sequence, last.hash));

    let unchanged = storage.begin().await.expect("begin");
    assert_eq!(unchanged.commit().await.expect("commit"), None);
}

#[cfg(test)]
mod tests {
    //! The fake passes the contract it is used in place of.

    use crate::testing::block_on;
    use crate::testing::unit::FakeStorage;

    #[test]
    fn the_in_memory_storage_passes_the_unit_of_work_contract() {
        block_on(super::run(async || FakeStorage::new()));
    }
}

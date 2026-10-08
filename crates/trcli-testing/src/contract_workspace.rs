//! Contract of opening and creating a workspace's storage (T047; FR-002, FR-008).
//!
//! Run it against an opener by calling [`run`] with a directory nothing else uses.

use std::path::Path;

use trcli_domain::shared::text::{LongText, Name};
use trcli_domain::workspace::Workspace;

use super::environment::{SeededIds, stamp};
use trcli_application::ports::unit_of_work::{Storage, StoreError, UnitOfWork};
use trcli_application::ports::workspace::{StorageOpener, WorkspaceStore};

/// A workspace row with a name and a description in two scripts.
fn workspace() -> Workspace {
    Workspace::new(
        SeededIds::at(3),
        Name::new("Doutorado em Ação").expect("a valid name"),
        LongText::new("東京 field work").expect("valid text"),
        stamp().at,
    )
}

/// Runs every case of the contract in `directory`.
pub async fn run<O>(opener: &O, directory: &Path)
where
    O: StorageOpener,
    <O::Storage as Storage>::Unit: WorkspaceStore,
{
    let database = directory.join("first.db");
    the_workspace_row_round_trips(opener, &database).await;
    creating_where_one_exists_is_refused_and_leaves_it_untouched(opener, &database).await;
    missing_storage_is_reported_as_damage(opener, &directory.join("absent.db")).await;
}

/// What is saved is what is read back, after reopening.
async fn the_workspace_row_round_trips<O>(opener: &O, database: &Path)
where
    O: StorageOpener,
    <O::Storage as Storage>::Unit: WorkspaceStore,
{
    let storage = opener.create(database).await.expect("create");
    let mut unit = storage.begin().await.expect("begin");
    unit.save_workspace(&workspace()).await.expect("save");
    unit.commit().await.expect("commit");
    drop(storage);

    let reopened = opener.open(database).await.expect("open");
    let stored = reopened
        .read()
        .await
        .expect("read")
        .workspace()
        .await
        .expect("workspace");
    assert_eq!(stored, workspace());
}

/// A second creation in the same place fails and changes nothing.
async fn creating_where_one_exists_is_refused_and_leaves_it_untouched<O>(
    opener: &O,
    database: &Path,
) where
    O: StorageOpener,
    <O::Storage as Storage>::Unit: WorkspaceStore,
{
    let refused = opener.create(database).await;
    assert!(
        matches!(refused, Err(StoreError::AlreadyExists(_))),
        "creating twice must be refused"
    );
    let stored = opener
        .open(database)
        .await
        .expect("open")
        .read()
        .await
        .expect("read")
        .workspace()
        .await;
    assert_eq!(stored.expect("still there"), workspace());
}

/// Opening storage that is not there says what is wrong and where.
async fn missing_storage_is_reported_as_damage<O>(opener: &O, database: &Path)
where
    O: StorageOpener,
    <O::Storage as Storage>::Unit: WorkspaceStore,
{
    match opener.open(database).await {
        Err(StoreError::Damaged { what, location }) => {
            assert!(!what.is_empty());
            assert!(location.contains("absent.db"), "{location}");
        }
        Err(other) => panic!("expected damage, got {other:?}"),
        Ok(_) => panic!("expected damage, got storage"),
    }
}

#[cfg(test)]
mod tests {
    //! The fake passes the contract it is used in place of.

    use std::path::Path;

    use crate::block_on;
    use crate::workspace::FakeOpener;

    #[test]
    fn the_in_memory_opener_passes_the_workspace_contract() {
        block_on(super::run(&FakeOpener::new(), Path::new("/memory")));
    }
}

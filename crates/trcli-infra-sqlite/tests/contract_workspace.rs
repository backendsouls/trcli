//! The SQLite adapter passes the workspace contract the in-memory fake passes (T054), and
//! reports each kind of damage with what and where (FR-008).

mod support;

use trcli_application::ports::unit_of_work::{Storage, StoreError};
use trcli_application::ports::workspace::{IntegrityCheck, StorageOpener};
use trcli_infra_sqlite::connection::SqliteOpener;

#[tokio::test(flavor = "current_thread")]
async fn sqlite_passes_the_workspace_contract() {
    let databases = support::Databases::new();
    trcli_testing::contract_workspace::run(&SqliteOpener::new(200), databases.path()).await;
}

/// The damage reported when opening `path`.
async fn damage(path: &std::path::Path) -> (String, String) {
    match SqliteOpener::new(200).open(path).await {
        Err(StoreError::Damaged { what, location }) => (what, location),
        Err(other) => panic!("expected damage, got {other:?}"),
        Ok(_) => panic!("expected damage, got a database"),
    }
}

#[tokio::test(flavor = "current_thread")]
async fn a_file_that_is_not_a_database_is_reported_as_damage() {
    let databases = support::Databases::new();
    let path = databases.next_path();
    std::fs::write(&path, b"these few bytes are not a database").expect("write");
    let (what, location) = damage(&path).await;
    assert!(
        what.contains("not a valid database") || what.contains("cannot be opened"),
        "{what}"
    );
    assert_eq!(location, path.display().to_string());
    assert_eq!(
        std::fs::read(&path).expect("read"),
        b"these few bytes are not a database",
        "nothing is overwritten"
    );
}

#[tokio::test(flavor = "current_thread")]
async fn a_database_that_lacks_an_expected_table_is_reported_as_damage() {
    let databases = support::Databases::new();
    let path = databases.next_path();
    // An empty file is a valid, empty SQLite database: it has no table at all.
    std::fs::write(&path, b"").expect("write");
    let (what, _) = damage(&path).await;
    assert_eq!(what, "the table `workspace` is missing");
}

#[tokio::test(flavor = "current_thread")]
async fn a_sound_database_passes_its_own_check() {
    let databases = support::Databases::new();
    let unit = databases.fresh().await.read().await.expect("read");
    assert_eq!(unit.integrity_problems().await, Ok(Vec::new()));
}

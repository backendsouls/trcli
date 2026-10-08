//! How the database is opened (T022): foreign keys enforced, a write-ahead journal, and a
//! second writer that waits and is then told the workspace is busy (FR-009).
//!
//! This also keeps what spike 1 established: SeaORM 2.0 on bundled SQLite runs on a
//! current-thread tokio runtime with the feature flags chosen in the workspace manifest.

mod support;

use std::time::{Duration, Instant};

use sea_orm::{ConnectionTrait, Database, DbBackend, Statement};
use trcli_application::ports::records::TagStore;
use trcli_application::ports::unit_of_work::{Storage, StoreError, UnitOfWork};
use trcli_application::ports::workspace::{StorageOpener, WorkspaceStore};
use trcli_application::testing::environment::{SeededIds, stamp};
use trcli_domain::shared::text::{LongText, Name, TagName};
use trcli_domain::workspace::Workspace;
use trcli_infra_sqlite::connection::SqliteOpener;

#[tokio::test(flavor = "current_thread")]
async fn foreign_keys_are_enforced() {
    let databases = support::Databases::new();
    let mut unit = databases.fresh().await.begin().await.expect("begin");
    let tag = TagName::new("ghost").expect("valid");
    let refused = unit.attach_tag(SeededIds::at(9), &tag).await;
    assert!(
        matches!(refused, Err(StoreError::Constraint(_))),
        "{refused:?}"
    );
}

#[tokio::test(flavor = "current_thread")]
async fn the_journal_is_write_ahead() {
    let databases = support::Databases::new();
    let storage = databases.fresh().await;
    // Asked through a separate, plain connection: the mode is a property of the file.
    let url = format!("sqlite://{}?mode=ro", storage.path().display());
    let plain = Database::connect(url).await.expect("connect");
    let row = plain
        .query_one_raw(Statement::from_string(
            DbBackend::Sqlite,
            "PRAGMA journal_mode",
        ))
        .await
        .expect("pragma")
        .expect("one row");
    assert_eq!(row.try_get_by_index::<String>(0).expect("text"), "wal");
}

#[tokio::test(flavor = "current_thread")]
async fn a_second_writer_waits_for_the_timeout_and_is_then_told_the_workspace_is_busy() {
    let databases = support::Databases::new();
    let path = databases.next_path();
    let first = SqliteOpener::new(5_000)
        .create(&path)
        .await
        .expect("create");
    let second = SqliteOpener::new(150).open(&path).await.expect("open");

    let holding = first.begin().await.expect("the first writer begins");
    let started = Instant::now();
    let refused = second.begin().await;
    assert!(matches!(refused, Err(StoreError::Busy)), "expected busy");
    assert!(
        started.elapsed() >= Duration::from_millis(100),
        "the second writer must wait first"
    );

    // Once the first writer is done, the second gets in.
    holding.commit().await.expect("commit");
    second
        .begin()
        .await
        .expect("the second writer begins")
        .commit()
        .await
        .expect("commit");
}

#[tokio::test(flavor = "current_thread")]
async fn a_reader_is_not_blocked_by_a_writer() {
    let databases = support::Databases::new();
    let path = databases.next_path();
    let first = SqliteOpener::new(5_000)
        .create(&path)
        .await
        .expect("create");
    let second = SqliteOpener::new(150).open(&path).await.expect("open");
    let mut writer = first.begin().await.expect("begin");
    let name = Name::new("Doctorate").expect("valid");
    let workspace = Workspace::new(
        SeededIds::at(0),
        name,
        LongText::new("").expect("valid"),
        stamp().at,
    );
    writer.save_workspace(&workspace).await.expect("save");

    let reader = second.read().await.expect("a reader is let in");
    assert!(
        reader.workspace().await.is_err(),
        "uncommitted changes are not visible"
    );
}

#[cfg(unix)]
#[tokio::test(flavor = "current_thread")]
async fn a_database_that_cannot_be_written_to_can_still_be_read() {
    use std::os::unix::fs::PermissionsExt;

    let databases = support::Databases::new();
    let directory = databases.path().join("locked");
    std::fs::create_dir(&directory).expect("directory");
    let path = directory.join("trcli.db");
    let storage = SqliteOpener::new(200).create(&path).await.expect("create");
    let mut unit = storage.begin().await.expect("begin");
    let name = Name::new("Doctorate").expect("valid");
    let workspace = Workspace::new(
        SeededIds::at(0),
        name,
        LongText::new("").expect("valid"),
        stamp().at,
    );
    unit.save_workspace(&workspace).await.expect("save");
    unit.commit().await.expect("commit");
    storage.close().await.expect("close");

    let read_only = |mode| std::fs::Permissions::from_mode(mode);
    std::fs::set_permissions(&path, read_only(0o444)).expect("file");
    std::fs::set_permissions(&directory, read_only(0o555)).expect("directory");
    // A process that may write anywhere (root, in some containers) cannot test this.
    if std::fs::write(directory.join("probe"), b"").is_ok() {
        std::fs::set_permissions(&directory, read_only(0o755)).expect("directory");
        return;
    }

    let storage = SqliteOpener::new(200)
        .open(&path)
        .await
        .expect("open without writing");
    let reader = storage.read().await.expect("read");
    assert_eq!(
        reader.workspace().await.expect("workspace").name.as_str(),
        "Doctorate"
    );
    drop(reader);
    let refused = match storage.begin().await {
        Ok(mut unit) => unit.save_workspace(&workspace).await,
        Err(error) => Err(error),
    };
    assert!(
        matches!(refused, Err(StoreError::ReadOnly(_))),
        "{refused:?}"
    );

    std::fs::set_permissions(&directory, read_only(0o755)).expect("directory");
}

//! Bringing a workspace's format forward (T049; FR-007, FR-075).
//!
//! - A failing change of format leaves the database exactly as the copy taken before it.
//! - A workspace of a newer format is refused.
//! - Every fixture under `tests/fixtures/formats/` — one workspace per released format —
//!   upgrades without loss. Before the first release there is none, and that loop passes
//!   without doing anything.

mod support;

use std::path::{Path, PathBuf};

use sea_orm::{ConnectionTrait, DbErr, TransactionTrait};
use sea_orm_migration::async_trait::async_trait;
use sea_orm_migration::{MigrationName, MigrationTrait, MigratorTrait, SchemaManager};
use trcli_application::governance::verify::{TrailVerdict, verify};
use trcli_application::ports::audit::AuditQuery;
use trcli_application::ports::records::RecordIndex;
use trcli_application::ports::unit_of_work::{Storage, UnitOfWork};
use trcli_application::ports::workspace::{StorageOpener, WorkspaceStore};
use trcli_application::workspace::open::{Access, guard};
use trcli_application::workspace::upgrade::upgrade;
use trcli_domain::governance::audit::AuditHead;
use trcli_domain::shared::text::{LongText, Name};
use trcli_domain::workspace::{FormatVersion, Workspace};
use trcli_infra_sqlite::connection::{SqliteOpener, SqliteStorage};
use trcli_infra_sqlite::digest::Sha256Digest;
use trcli_testing::audit::MemoryHead;
use trcli_testing::environment::{SeededIds, stamp};
use trcli_testing::interaction::RecordingProgress;

/// A change of format that creates a table and then fails.
struct FailsHalfWay;

impl MigrationName for FailsHalfWay {
    fn name(&self) -> &str {
        "m9999_fails_half_way"
    }
}

#[async_trait]
impl MigrationTrait for FailsHalfWay {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute_unprepared("CREATE TABLE half_made (id INTEGER PRIMARY KEY)")
            .await?;
        Err(DbErr::Migration(
            "this migration fails on purpose".to_owned(),
        ))
    }
}

/// A set of migrations whose only member fails.
struct FailingMigrator;

impl MigratorTrait for FailingMigrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(FailsHalfWay)]
    }
}

/// Stores a workspace row in the given format.
async fn store_workspace(storage: &SqliteStorage, format: u32) {
    let name = Name::new("Doctorate").expect("valid");
    let mut workspace = Workspace::new(
        SeededIds::at(0),
        name,
        LongText::new("").expect("valid"),
        stamp().at,
    );
    workspace.format_version = FormatVersion::new(format);
    let mut unit = storage.begin().await.expect("begin");
    unit.save_workspace(&workspace).await.expect("save");
    unit.commit().await.expect("commit");
}

#[tokio::test(flavor = "current_thread")]
async fn a_failing_migration_leaves_the_database_identical_to_the_copy_taken_before_it() {
    let databases = support::Databases::new();
    let path = databases.next_path();
    let storage = SqliteOpener::new(200).create(&path).await.expect("create");
    store_workspace(&storage, 1).await;
    storage.close().await.expect("close");
    let copy = std::fs::read(&path).expect("the copy taken before");

    let connection = support::plain_connection(&path).await;
    let transaction = connection.begin().await.expect("begin");
    let failed = FailingMigrator::up(&transaction, None).await;
    assert!(failed.is_err(), "the migration must fail");
    drop(transaction);
    connection.close().await.expect("close");

    assert_eq!(
        std::fs::read(&path).expect("the database after"),
        copy,
        "nothing of the failed change remains"
    );
}

#[tokio::test(flavor = "current_thread")]
async fn a_workspace_of_a_newer_format_is_refused() {
    let databases = support::Databases::new();
    let storage = databases.fresh().await;
    store_workspace(&storage, FormatVersion::CURRENT.number() + 1).await;
    let workspace = storage
        .read()
        .await
        .expect("read")
        .workspace()
        .await
        .expect("workspace");
    let state = workspace.opening_state(FormatVersion::CURRENT);
    assert_eq!(
        guard(&state, Access::Write).expect_err("too new").code.name,
        "workspace_too_new"
    );
    assert_eq!(
        guard(&state, Access::Read).expect_err("too new").code.name,
        "workspace_too_new"
    );
}

#[tokio::test(flavor = "current_thread")]
async fn an_older_workspace_is_upgraded_and_its_trail_records_it() {
    let databases = support::Databases::new();
    let storage = databases.fresh().await;
    store_workspace(&storage, 0).await;
    let head = MemoryHead::new();
    let report = upgrade(&storage, &NoCopy, &head, &stamp())
        .await
        .expect("upgraded");
    assert!(report.upgraded);
    let reader = storage.read().await.expect("read");
    assert_eq!(
        reader.workspace().await.expect("workspace").format_version,
        FormatVersion::CURRENT
    );
    assert_eq!(reader.entry_count().await.expect("count"), 1);
}

/// A backup that copies nothing: the upgrade's own tests with fakes cover the copy.
struct NoCopy;

impl trcli_application::ports::workspace::BackupCopy for NoCopy {
    fn copy(&self) -> Result<PathBuf, trcli_application::ports::unit_of_work::StoreError> {
        Ok(PathBuf::from("no-copy"))
    }

    fn restore(
        &self,
        _copy: &Path,
    ) -> Result<(), trcli_application::ports::unit_of_work::StoreError> {
        Ok(())
    }
}

/// The directories under `tests/fixtures/formats/`, one per released format.
fn fixtures() -> Vec<PathBuf> {
    let formats = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../tests/fixtures/formats");
    let Ok(entries) = std::fs::read_dir(formats) else {
        return Vec::new();
    };
    entries
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| path.is_dir())
        .collect()
}

#[tokio::test(flavor = "current_thread")]
async fn every_released_format_upgrades_without_loss() {
    for fixture in fixtures() {
        let scratch = tempfile::tempdir().expect("a temporary directory");
        let database = scratch.path().join("trcli.db");
        std::fs::copy(fixture.join(".trcli/trcli.db"), &database).expect("copy the fixture");
        let head_line =
            std::fs::read_to_string(fixture.join(".trcli/audit.head")).expect("the fixture's head");
        let head = MemoryHead::holding(AuditHead::parse(&head_line).expect("a head"));

        let storage = SqliteOpener::new(200).open(&database).await.expect("open");
        let before = storage
            .read()
            .await
            .expect("read")
            .count_by_kind()
            .await
            .expect("counts");
        upgrade(&storage, &NoCopy, &head, &stamp())
            .await
            .expect("upgraded");

        let reader = storage.read().await.expect("read");
        assert_eq!(
            reader.count_by_kind().await.expect("counts"),
            before,
            "{}",
            fixture.display()
        );
        let verdict = verify(
            &reader,
            head.current(),
            &Sha256Digest,
            &mut RecordingProgress::default(),
        )
        .await;
        assert!(
            matches!(verdict, Ok(TrailVerdict::Intact { .. })),
            "{}",
            fixture.display()
        );
    }
}

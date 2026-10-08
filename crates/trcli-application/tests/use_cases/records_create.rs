//! Unit tests for registering and updating records.

use trcli_domain::governance::audit::Change;

use trcli_application::kinds::RecordKindDescriptor;
use trcli_application::ports::records::RecordIndex;
use trcli_application::ports::unit_of_work::Storage;
use trcli_application::records::create::{register, update};
use trcli_testing::block_on;
use trcli_testing::environment::{SeededIds, stamp};
use trcli_testing::unit::FakeStorage;

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

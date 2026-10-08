//! Unit tests for handle assignment (T061).

use trcli_domain::shared::record::{Handle, RecordKind};

use trcli_application::ports::records::RecordIndex;
use trcli_application::ports::unit_of_work::Storage;
use trcli_application::records::handles::assign;
use trcli_testing::block_on;
use trcli_testing::contract_records::indexed;
use trcli_testing::environment::{SeededIds, stamp};
use trcli_testing::unit::FakeStorage;

/// The kind used by these tests.
fn kind() -> RecordKind {
    RecordKind::new("alpha", "alp").expect("a valid kind")
}

#[test]
fn a_handle_is_the_prefix_and_four_characters_of_the_identifier() {
    let handle = block_on(async {
        let unit = FakeStorage::new().begin().await.expect("begin");
        assign(&unit, &kind(), SeededIds::at(0))
            .await
            .expect("assigned")
    });
    assert_eq!(handle, Handle::derive(&kind(), SeededIds::at(0), 4));
    assert_eq!(handle.as_str().len(), "alp-".len() + 4);
}

#[test]
fn on_collision_the_code_grows_by_one_character_at_a_time() {
    block_on(async {
        let mut unit = FakeStorage::new().begin().await.expect("begin");
        let mut taken = indexed(5, "alpha", "alp", "Taken");
        taken.handle = Handle::derive(&kind(), SeededIds::at(0), 4);
        unit.insert_record(&taken).await.expect("insert");
        let five = assign(&unit, &kind(), SeededIds::at(0))
            .await
            .expect("assigned");
        assert_eq!(five, Handle::derive(&kind(), SeededIds::at(0), 5));

        let mut also_taken = indexed(6, "alpha", "alp", "Also taken");
        also_taken.handle = five;
        unit.insert_record(&also_taken).await.expect("insert");
        let six = assign(&unit, &kind(), SeededIds::at(0))
            .await
            .expect("assigned");
        assert_eq!(six.as_str().len(), "alp-".len() + 6);
    });
}

#[test]
fn a_deleted_records_handle_is_never_assigned_again() {
    block_on(async {
        let mut unit = FakeStorage::new().begin().await.expect("begin");
        let mut deleted = indexed(5, "alpha", "alp", "Deleted");
        deleted.handle = Handle::derive(&kind(), SeededIds::at(0), 4);
        unit.insert_record(&deleted).await.expect("insert");
        unit.mark_deleted(deleted.id, stamp().at)
            .await
            .expect("delete");
        let handle = assign(&unit, &kind(), SeededIds::at(0))
            .await
            .expect("assigned");
        assert_ne!(handle, deleted.handle);
    });
}

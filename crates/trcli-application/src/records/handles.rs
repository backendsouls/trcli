//! Giving a new record its short name (FR-010).
//!
//! A short name is the kind's prefix, a hyphen, and a code taken from the random part of
//! the record's identifier: four characters, or more when a record — existing or deleted —
//! already has that name. A deleted record keeps its row in the index, so its name is
//! never given out again.

use trcli_domain::shared::record::{Handle, RecordId, RecordKind};

use crate::ports::records::RecordIndex;
use crate::ports::unit_of_work::StoreError;

/// The shortest free short name for a new record of `kind` with identifier `id`.
pub async fn assign<U: RecordIndex>(
    unit: &U,
    kind: &RecordKind,
    id: RecordId,
) -> Result<Handle, StoreError> {
    for length in Handle::MINIMUM_CODE_LENGTH..=Handle::MAXIMUM_CODE_LENGTH {
        let handle = Handle::derive(kind, id, length);
        if !unit.handle_is_taken(&handle).await? {
            return Ok(handle);
        }
    }
    // Twelve characters are the whole random part of the identifier: only two records
    // with the same identifier could get here.
    Err(StoreError::Constraint(format!(
        "no free short name for record {id}"
    )))
}

#[cfg(test)]
mod tests {
    //! Unit tests for handle assignment (T061).

    use trcli_domain::shared::record::{Handle, RecordKind};

    use super::assign;
    use crate::ports::records::RecordIndex;
    use crate::ports::unit_of_work::Storage;
    use crate::testing::block_on;
    use crate::testing::contract_records::indexed;
    use crate::testing::environment::{SeededIds, stamp};
    use crate::testing::unit::FakeStorage;

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
}

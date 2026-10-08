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

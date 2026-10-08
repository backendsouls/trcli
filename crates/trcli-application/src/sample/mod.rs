//! Two sample kinds of record, `specimen` and `sample-note` (feature `sample-kind`).
//!
//! They exist to prove the feature contract: everything they can do beyond their own
//! `add` and `edit` comes from registering a descriptor, and no foundation file names
//! them (`tests/sample_kind_is_external.rs` checks). The acceptance scenarios of user
//! stories 2 and 8 run against them. No release build enables this feature.
//!
//! They are also the reference for `docs/contributing/adding-a-record-kind.md`.

pub mod sample_note;
pub mod specimen;

use crate::kinds::{KindError, KindRegistry};

/// Registers both sample kinds: the only thing the composition root does for them.
pub fn register(registry: &mut KindRegistry) -> Result<(), KindError> {
    registry.register(specimen::descriptor())?;
    registry.register(sample_note::descriptor())
}

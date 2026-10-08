//! New identifiers: version 7 UUIDs, which never collide across copies of a workspace
//! and sort by when they were made.
//!
//! In a build with the `test-clock` feature, identifiers are derived from the
//! `TRCLI_TEST_ID_SEED` variable and a counter, so that behaviour scenarios see the same
//! short names on every run. A release build has no such variable.

use std::cell::Cell;

use trcli_application::ports::environment::IdGenerator;
use trcli_domain::shared::record::RecordId;
use uuid::Uuid;

/// The generator of identifiers.
#[derive(Debug, Default)]
pub struct UuidGenerator {
    /// How many identifiers this command has made; used only by seeded generation.
    made: Cell<u64>,
}

impl UuidGenerator {
    /// A generator.
    pub fn new() -> Self {
        Self::default()
    }

    /// The seed fixed by the environment, when this build allows it and it is set.
    #[cfg(feature = "test-clock")]
    fn seed() -> Option<u64> {
        std::env::var("TRCLI_TEST_ID_SEED").ok()?.parse().ok()
    }

    /// A release build never takes a seed from the environment.
    #[cfg(not(feature = "test-clock"))]
    fn seed() -> Option<u64> {
        None
    }

    /// The identifier at a position of the sequence a seed gives. The multiplier is odd,
    /// so every position gives different random bits, and the short names derived from
    /// them differ from their first character on.
    fn seeded(seed: u64, position: u64) -> RecordId {
        let index = seed
            .wrapping_mul(1_000)
            .wrapping_add(position)
            .wrapping_add(1);
        let random = index.wrapping_mul(0x9E37_79B9_7F4A_7C15) & 0x0FFF_FFFF_FFFF_FFFF;
        // The fixed bits make this a well-formed version 7 UUID.
        RecordId::from_u128(
            (0x0192_0000_0000_7000_u128 << 64) | (0x8000_0000_0000_0000_u128 | u128::from(random)),
        )
    }
}

impl IdGenerator for UuidGenerator {
    fn next_id(&self) -> RecordId {
        let position = self.made.get();
        self.made.set(position + 1);
        match Self::seed() {
            Some(seed) => Self::seeded(seed, position),
            None => RecordId::from_uuid(Uuid::now_v7()),
        }
    }
}

#[cfg(test)]
mod tests {
    //! Unit tests for identifiers.

    use std::collections::BTreeSet;

    use trcli_application::ports::environment::IdGenerator;

    use super::UuidGenerator;

    #[test]
    fn identifiers_differ() {
        let generator = UuidGenerator::new();
        let made: BTreeSet<String> = (0..200).map(|_| generator.next_id().to_string()).collect();
        assert_eq!(made.len(), 200);
    }

    #[test]
    fn seeded_identifiers_are_the_same_on_every_run_and_differ_between_seeds() {
        assert_eq!(UuidGenerator::seeded(1, 0), UuidGenerator::seeded(1, 0));
        assert_ne!(UuidGenerator::seeded(1, 0), UuidGenerator::seeded(1, 1));
        assert_ne!(UuidGenerator::seeded(1, 0), UuidGenerator::seeded(2, 0));
        assert_eq!(UuidGenerator::seeded(1, 0).as_uuid().get_version_num(), 7);
    }
}

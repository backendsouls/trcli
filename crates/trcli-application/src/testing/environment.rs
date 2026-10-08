//! Fakes of the clock, the identifier generator, and the actor.

use std::cell::Cell;

use time::OffsetDateTime;
use trcli_domain::governance::audit::Stamp;
use trcli_domain::shared::record::RecordId;
use trcli_domain::shared::text::ActorName;

use crate::ports::environment::{ActorProvider, Clock, IdGenerator};

/// A clock that always tells the same time.
#[derive(Clone, Copy, Debug)]
pub struct FixedClock(pub OffsetDateTime);

impl FixedClock {
    /// A clock stopped at 2026-10-08 14:00 UTC.
    pub fn default_moment() -> Self {
        Self(OffsetDateTime::from_unix_timestamp(1_791_468_000).expect("a valid moment"))
    }
}

impl Clock for FixedClock {
    fn now(&self) -> OffsetDateTime {
        self.0
    }
}

/// Identifiers that are the same on every run: a counter spread over the random bits, so
/// that the handles derived from them differ from the first character on.
#[derive(Debug, Default)]
pub struct SeededIds {
    /// How many identifiers were given out.
    next: Cell<u64>,
}

impl SeededIds {
    /// A generator starting from the beginning of its sequence.
    pub fn new() -> Self {
        Self::default()
    }

    /// The identifier at a position of the sequence, for tests that need one directly.
    pub fn at(position: u64) -> RecordId {
        // An odd multiplier walks through every 60-bit value before repeating.
        let random =
            position.wrapping_add(1).wrapping_mul(0x9E37_79B9_7F4A_7C15) & 0x0FFF_FFFF_FFFF_FFFF;
        RecordId::from_u128(
            (0x0192_0000_0000_7000_u128 << 64) | (0x8000_0000_0000_0000_u128 | u128::from(random)),
        )
    }
}

impl IdGenerator for SeededIds {
    fn next_id(&self) -> RecordId {
        let position = self.next.get();
        self.next.set(position + 1);
        Self::at(position)
    }
}

/// An actor with a fixed name.
#[derive(Clone, Debug)]
pub struct FixedActor(pub ActorName);

impl FixedActor {
    /// The actor "ana".
    pub fn ana() -> Self {
        Self(ActorName::new("ana").expect("a valid name"))
    }
}

impl ActorProvider for FixedActor {
    fn actor(&self) -> ActorName {
        self.0.clone()
    }
}

/// The stamp tests use: the fixed clock's moment and the actor "ana".
pub fn stamp() -> Stamp {
    Stamp::new(
        FixedClock::default_moment().now(),
        FixedActor::ana().actor(),
    )
}

//! Ports for what a use case takes from its surroundings: the time, new identifiers, and
//! who is acting. Behind ports so that tests fix all three (FR-071).

use time::OffsetDateTime;
use trcli_domain::shared::record::RecordId;
use trcli_domain::shared::text::ActorName;

/// Tells the time.
pub trait Clock {
    /// The current moment.
    fn now(&self) -> OffsetDateTime;
}

/// Makes identifiers for new records.
pub trait IdGenerator {
    /// A new identifier, different from every one given before.
    fn next_id(&self) -> RecordId;
}

/// Knows under which name the researcher's actions are recorded (FR-050).
pub trait ActorProvider {
    /// The name the researcher set for themselves, else the name they are known by on
    /// their machine, else the placeholder `unknown`.
    fn actor(&self) -> ActorName;
}

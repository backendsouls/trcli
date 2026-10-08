//! Small types shared by view models: what a use case hands back to be shown.
//!
//! A view model is a plain value that can be serialized (the structured form) and rendered
//! (the form for people). Both forms come from the same value, so their content cannot
//! drift apart (SC-005). This module holds the pieces several view models share; rendering
//! itself belongs to the presentation layer.

use serde::{Serialize, Serializer};
use time::OffsetDateTime;
use time::format_description::well_known::Rfc3339;

/// A moment in time inside a view model.
///
/// It serializes as RFC 3339 in UTC, as the output contract requires; the presentation
/// layer shows it in the researcher's local time (FR-038).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Instant(pub OffsetDateTime);

impl Instant {
    /// The moment as RFC 3339 text in UTC, for example `2026-10-08T14:02:11Z`.
    pub fn to_rfc3339(&self) -> String {
        let utc = self.0.to_offset(time::UtcOffset::UTC);
        // Formatting a valid moment in this well-known form cannot fail.
        utc.format(&Rfc3339).unwrap_or_default()
    }
}

impl Serialize for Instant {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.to_rfc3339())
    }
}

impl From<OffsetDateTime> for Instant {
    fn from(moment: OffsetDateTime) -> Self {
        Self(moment)
    }
}

/// A message with nothing else to show: "Linked a ⟷ b", "Telemetry is off".
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct Done {
    /// What was done, as one sentence.
    pub message: String,
}

impl Done {
    /// A view that says what was done.
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    //! Unit tests for the shared view pieces.

    use time::OffsetDateTime;

    use super::Instant;

    #[test]
    fn an_instant_is_written_as_rfc_3339_in_utc() {
        let moment = OffsetDateTime::UNIX_EPOCH
            .to_offset(time::UtcOffset::from_hms(2, 0, 0).expect("offset"));
        assert_eq!(Instant(moment).to_rfc3339(), "1970-01-01T00:00:00Z");
    }
}

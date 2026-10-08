//! Accountability: the audit trail and local telemetry (FR-046 to FR-054).
//!
//! [`audit`] defines what an entry of the trail is and the rule that chains entries
//! together so that tampering can be detected. [`telemetry`] defines the little that is
//! recorded about the tool's own use.

pub mod audit;
pub mod telemetry;

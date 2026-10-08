//! The use cases of accountability (FR-046 to FR-054).
//!
//! - [`record`]: committing a change with the end of the trail, and describing changes
//!   without ever including a secret.
//! - [`query`]: looking through the trail.
//! - [`verify`]: checking that the trail was not altered or shortened outside the tool.
//! - [`export`]: handing a range of the trail to someone else.
//! - [`telemetry_summary`]: recording and summarising the tool's own use, locally.

pub mod export;
pub mod query;
pub mod record;
pub mod telemetry_summary;
pub mod verify;

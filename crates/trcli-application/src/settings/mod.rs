//! Settings: one registry, several sources, one rule of precedence (FR-039 to FR-045).
//!
//! - [`registry`] holds the definitions features register.
//! - [`foundation`] registers the foundation's own.
//! - [`layers`] combines the sources into the values in effect, each with where it came
//!   from.
//! - [`commands`] are the use cases behind `trcli config`.

pub mod commands;
pub mod foundation;
pub mod layers;
pub mod registry;

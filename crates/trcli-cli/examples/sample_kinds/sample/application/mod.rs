//! The sample feature's application layer: for each kind, its descriptor, the port for
//! its own rows, the behaviour only it knows, and its `add` and `edit` use cases.
//!
//! Everything else the kinds can do is the foundation's: short names, resolving what was
//! typed, tags, notes, links, guarded deletion, listing, showing.

pub mod sample_note;
pub mod specimen;

use trcli_application::kinds::{KindError, KindRegistry};

/// Registers both sample kinds: the only thing the composition root does for them.
pub fn register(registry: &mut KindRegistry) -> Result<(), KindError> {
    registry.register(specimen::descriptor())?;
    registry.register(sample_note::descriptor())
}

//! Recording changes (FR-047, FR-054).
//!
//! [`commit`] is how every changing command ends: it commits the unit of work and then
//! updates the head file with the new end of the trail. [`ChangeSet`] is how a use case
//! lists the fields it changed; a field marked secret never reaches the trail.

use trcli_domain::governance::audit::Change;

use crate::outcome::Problem;
use crate::ports::audit::AuditHeadStore;
use crate::ports::unit_of_work::UnitOfWork;

/// Commits a unit of work and, when it recorded audit entries, stores the new end of the
/// trail so that a later removal of entries can be noticed (FR-048).
pub async fn commit<U: UnitOfWork>(unit: U, head: &impl AuditHeadStore) -> Result<(), Problem> {
    if let Some(new_head) = unit.commit().await? {
        // The change is already permanent here. If the head cannot be written, the trail
        // is longer than the head says, which verification accepts and repairs; failing
        // the command now would wrongly tell the researcher that nothing was changed.
        let _ = head.write_head(&new_head);
    }
    Ok(())
}

/// The fields one change touched, for its audit entry.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ChangeSet {
    /// The changes that will be recorded.
    changes: Vec<Change>,
}

impl ChangeSet {
    /// No change yet.
    pub fn new() -> Self {
        Self::default()
    }

    /// Notes a field that went from one value to another. A field whose value did not
    /// change is not noted.
    pub fn field(&mut self, name: &str, before: Option<&str>, after: Option<&str>) -> &mut Self {
        if before != after {
            self.changes.push(Change::new(name, before, after));
        }
        self
    }

    /// Notes nothing: a secret field's values are never written to the trail, and neither
    /// is the fact that it changed (FR-054). The method exists so that a use case states,
    /// in code, that it considered the field.
    pub fn secret_field(&mut self, _name: &str) -> &mut Self {
        self
    }

    /// Whether nothing changed.
    pub fn is_empty(&self) -> bool {
        self.changes.is_empty()
    }

    /// The changes to record.
    pub fn into_changes(self) -> Vec<Change> {
        self.changes
    }
}

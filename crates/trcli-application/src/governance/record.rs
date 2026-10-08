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

#[cfg(test)]
mod tests {
    //! Unit tests for recording changes (T105).

    use trcli_domain::governance::audit::{AuditAction, AuditDraft};

    use super::{ChangeSet, commit};
    use crate::outcome::Details;
    use crate::ports::audit::AuditLog;
    use crate::ports::unit_of_work::Storage;
    use crate::testing::audit::MemoryHead;
    use crate::testing::block_on;
    use crate::testing::environment::stamp;
    use crate::testing::unit::FakeStorage;
    use crate::validation::Checker;

    #[test]
    fn a_secret_field_is_absent_from_an_entrys_changes() {
        let mut changes = ChangeSet::new();
        changes
            .field("title", Some("Old"), Some("New"))
            .secret_field("passphrase");
        let recorded = changes.into_changes();
        assert_eq!(recorded.len(), 1);
        assert!(recorded.iter().all(|change| change.field != "passphrase"));
    }

    #[test]
    fn a_secret_value_is_absent_from_problem_messages_and_details() {
        let mut checker = Checker::new();
        let rejection =
            trcli_domain::shared::problem::Rejection::new("is too short", "at least 12 characters");
        checker.reject_secret("--passphrase", rejection);
        let problem = checker.finish(|| ()).expect_err("invalid");
        assert!(!problem.message.contains("hunter2"));
        let Details::Fields(fields) = problem.details else {
            panic!("fields expected")
        };
        // The structured form is built from these fields: with no value here, none can
        // reach the JSON envelope either.
        assert_eq!(fields[0].value, None);
    }

    #[test]
    fn an_unchanged_field_is_not_recorded() {
        let mut changes = ChangeSet::new();
        changes.field("title", Some("Same"), Some("Same"));
        assert!(changes.is_empty());
    }

    #[test]
    fn committing_stores_the_new_end_of_the_trail() {
        let (storage, head) = (FakeStorage::new(), MemoryHead::new());
        block_on(async {
            let mut unit = storage.begin().await.expect("begin");
            unit.record(&stamp(), AuditDraft::workspace(AuditAction::CREATE))
                .await
                .expect("record");
            commit(unit, &head).await.expect("commit");
        });
        let stored = head.current().expect("a head");
        assert_eq!(
            (stored.sequence, stored.hash),
            (1, storage.snapshot().audit[0].hash)
        );
    }

    #[test]
    fn committing_without_entries_leaves_the_head_alone() {
        let (storage, head) = (FakeStorage::new(), MemoryHead::new());
        block_on(async {
            commit(storage.begin().await.expect("begin"), &head)
                .await
                .expect("commit")
        });
        assert_eq!(head.current(), None);
    }
}

//! Verifying a workspace (FR-008): the stored data can be opened and is consistent, and
//! the audit trail is intact.

use serde::Serialize;

use crate::governance::verify::{TrailVerdict, verify};
use crate::outcome::{Problem, codes};
use crate::ports::audit::{AuditDigest, AuditHeadStore, AuditQuery};
use crate::ports::interaction::Progress;
use crate::ports::workspace::IntegrityCheck;

/// What `workspace check` reports when all is well.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct CheckReport {
    /// Whether the storage's own check passed. Always `true` in a report: a failure is a
    /// problem instead.
    pub storage_ok: bool,
    /// How many audit entries were verified.
    pub audit_entries: u64,
}

/// The outside things verification needs.
#[derive(Debug)]
pub struct CheckServices<'a, H, D, P> {
    /// The end of the trail as last seen.
    pub head: &'a H,
    /// The hash function of the trail.
    pub digest: &'a D,
    /// Where progress is shown.
    pub progress: &'a mut P,
}

/// Checks the stored data, then the audit trail. The first thing found wrong is reported
/// with what and where.
pub async fn check<U, H, D, P>(
    unit: &U,
    services: CheckServices<'_, H, D, P>,
) -> Result<CheckReport, Problem>
where
    U: IntegrityCheck + AuditQuery,
    H: AuditHeadStore,
    D: AuditDigest,
    P: Progress,
{
    let problems = unit.integrity_problems().await?;
    if !problems.is_empty() {
        return Err(Problem::new(
            codes::WORKSPACE_DAMAGED,
            "the workspace's stored data is not consistent",
        )
        .with_items(problems)
        .with_next_step(
            "restore a copy from `.trcli/backups/` if there is one; nothing was written",
        ));
    }
    let head = services.head.read_head()?;
    match verify(unit, head, services.digest, services.progress).await? {
        TrailVerdict::Intact { entries } => Ok(CheckReport {
            storage_ok: true,
            audit_entries: entries,
        }),
        TrailVerdict::Broken(finding) => Err(finding.into_problem()),
    }
}

#[cfg(test)]
mod tests {
    //! Unit tests for checking a workspace.

    use trcli_domain::governance::audit::{AuditAction, AuditDraft, AuditHead};

    use super::{CheckServices, check};
    use crate::outcome::codes;
    use crate::ports::audit::AuditLog;
    use crate::ports::unit_of_work::{Storage, UnitOfWork};
    use crate::testing::audit::{MemoryHead, ToyDigest};
    use crate::testing::block_on;
    use crate::testing::environment::stamp;
    use crate::testing::interaction::RecordingProgress;
    use crate::testing::unit::FakeStorage;

    /// Storage with two audit entries, and the head that matches it.
    fn storage_with_trail() -> (FakeStorage, AuditHead) {
        let storage = FakeStorage::new();
        let head = block_on(async {
            let mut unit = storage.begin().await.expect("begin");
            unit.record(&stamp(), AuditDraft::workspace(AuditAction::CREATE))
                .await
                .expect("record");
            unit.record(&stamp(), AuditDraft::workspace(AuditAction::UPDATE))
                .await
                .expect("record");
            unit.commit().await.expect("commit").expect("a head")
        });
        (storage, head)
    }

    /// Runs the check against the storage with the given head.
    fn run(
        storage: &FakeStorage,
        head: &MemoryHead,
    ) -> Result<super::CheckReport, crate::outcome::Problem> {
        let mut progress = RecordingProgress::default();
        block_on(async {
            let unit = storage.read().await.expect("read");
            check(
                &unit,
                CheckServices {
                    head,
                    digest: &ToyDigest,
                    progress: &mut progress,
                },
            )
            .await
        })
    }

    #[test]
    fn a_sound_workspace_passes_with_the_number_of_entries_verified() {
        let (storage, head) = storage_with_trail();
        let report = run(&storage, &MemoryHead::holding(head)).expect("sound");
        assert_eq!((report.storage_ok, report.audit_entries), (true, 2));
    }

    #[test]
    fn inconsistent_storage_is_reported_as_damage_with_what_was_found() {
        let (storage, head) = storage_with_trail();
        storage.tamper(|state| {
            state.integrity_problems = vec!["row 3 missing from index record_kind".into()]
        });
        let problem = run(&storage, &MemoryHead::holding(head)).expect_err("damaged");
        assert_eq!(problem.code, codes::WORKSPACE_DAMAGED);
    }

    #[test]
    fn a_tampered_trail_fails_the_check() {
        let (storage, head) = storage_with_trail();
        storage.tamper(|state| state.audit[0].display_name = Some("forged".into()));
        let problem = run(&storage, &MemoryHead::holding(head)).expect_err("tampered");
        assert_eq!(problem.code, codes::CHECK_FAILED);
    }
}

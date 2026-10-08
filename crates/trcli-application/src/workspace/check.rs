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

//! Ports of the audit trail and of local telemetry (FR-046 to FR-054).
//!
//! Note what is absent: there is no operation, here or in any adapter, that changes or
//! removes an audit entry (FR-048).

use time::Date;
use trcli_domain::governance::audit::{
    AuditAction, AuditDraft, AuditEntry, AuditHead, Digest, Stamp,
};
use trcli_domain::governance::telemetry::TelemetryRecord;
use trcli_domain::shared::record::RecordId;

use super::unit_of_work::StoreError;

/// Appends entries to the trail, inside the unit of work that makes the change.
pub trait AuditLog {
    /// Records what was done. The entry gets the next sequence and is chained to the one
    /// before it; it is committed with the change or not at all (FR-047).
    async fn record(&mut self, stamp: &Stamp, draft: AuditDraft) -> Result<AuditEntry, StoreError>;
}

/// Which entries to show.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AuditFilter {
    /// Only entries about this record.
    pub record: Option<RecordId>,
    /// Only entries about records of this kind.
    pub kind: Option<String>,
    /// Only entries made by this actor.
    pub actor: Option<String>,
    /// Only entries of this action.
    pub action: Option<AuditAction>,
    /// Only entries made on or after this day (UTC).
    pub from: Option<Date>,
    /// Only entries made on or before this day (UTC).
    pub to: Option<Date>,
    /// At most this many entries.
    pub limit: u32,
}

impl AuditFilter {
    /// A filter that matches every entry, up to `limit`.
    pub fn everything(limit: u32) -> Self {
        Self {
            record: None,
            kind: None,
            actor: None,
            action: None,
            from: None,
            to: None,
            limit,
        }
    }

    /// Whether an entry passes every condition of the filter.
    pub fn matches(&self, entry: &AuditEntry) -> bool {
        let day = entry.at.to_offset(time::UtcOffset::UTC).date();
        self.record
            .is_none_or(|record| entry.record_id == Some(record))
            && self
                .kind
                .as_ref()
                .is_none_or(|kind| entry.kind.as_ref() == Some(kind))
            && self
                .actor
                .as_ref()
                .is_none_or(|actor| entry.actor.as_str() == actor)
            && self
                .action
                .as_ref()
                .is_none_or(|action| &entry.action == action)
            && self.from.is_none_or(|from| day >= from)
            && self.to.is_none_or(|to| day <= to)
    }
}

/// A page of entries and how many matched in all.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AuditPage {
    /// The entries, newest first.
    pub entries: Vec<AuditEntry>,
    /// How many entries match the filter, shown or not.
    pub total: u64,
}

/// Reads the trail.
pub trait AuditQuery {
    /// Entries matching the filter, newest first, up to the filter's limit.
    async fn entries(&self, filter: &AuditFilter) -> Result<AuditPage, StoreError>;

    /// Up to `limit` entries with a sequence greater than `after`, oldest first. This is
    /// how verification walks a long trail without holding it all in memory.
    async fn entries_after(&self, after: u64, limit: u32) -> Result<Vec<AuditEntry>, StoreError>;

    /// How many entries the trail has.
    async fn entry_count(&self) -> Result<u64, StoreError>;
}

/// The end of the trail as last seen, kept outside the database (FR-048).
pub trait AuditHeadStore {
    /// The stored head; `None` when there is none yet.
    fn read_head(&self) -> Result<Option<AuditHead>, StoreError>;

    /// Replaces the stored head; never leaves a half-written file (FR-066).
    fn write_head(&self, head: &AuditHead) -> Result<(), StoreError>;
}

/// The hash function that chains the trail.
pub trait AuditDigest {
    /// The digest of some bytes.
    fn digest(&self, bytes: &[u8]) -> Digest;
}

/// Local telemetry about the tool's own use (FR-052).
pub trait TelemetryLog {
    /// Adds one record. Callers do this only while `telemetry.enabled` is true.
    async fn append_telemetry(&mut self, record: &TelemetryRecord) -> Result<(), StoreError>;

    /// Every record kept in this workspace.
    async fn telemetry(&self) -> Result<Vec<TelemetryRecord>, StoreError>;
}

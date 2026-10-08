//! The in-memory storage: a fake of the unit of work and of every store reached through
//! it.
//!
//! A [`FakeUnit`] works on its own copy of the [`State`] and writes it back only when it
//! is committed, so dropping a unit undoes everything, exactly as a transaction does.

use std::cell::RefCell;
use std::collections::BTreeMap;
use std::rc::Rc;

use time::OffsetDateTime;
use trcli_domain::governance::audit::{AuditDraft, AuditEntry, AuditHead, GENESIS_HASH, Stamp};
use trcli_domain::governance::telemetry::TelemetryRecord;
use trcli_domain::shared::link::Link;
use trcli_domain::shared::note::Note;
use trcli_domain::shared::record::{Handle, RecordId};
use trcli_domain::shared::tag::Tagging;
use trcli_domain::shared::text::{SearchKey, TagName};
use trcli_domain::workspace::Workspace;

use super::audit::ToyDigest;
use trcli_application::ports::audit::{AuditFilter, AuditLog, AuditPage, AuditQuery, TelemetryLog};
use trcli_application::ports::records::{
    IndexedRecord, LinkStore, ListPage, ListQuery, NoteStore, RecordIndex, RecordResolver,
    SortField, TagStore,
};
use trcli_application::ports::unit_of_work::{Storage, StoreError, UnitOfWork};
use trcli_application::ports::workspace::{IntegrityCheck, SchemaUpgrade, WorkspaceStore};

/// Everything a workspace's storage holds.
#[derive(Clone, Debug, Default)]
pub struct State {
    /// The workspace's own row.
    pub workspace: Option<Workspace>,
    /// The record index.
    pub records: Vec<IndexedRecord>,
    /// Which records carry which tags.
    pub taggings: Vec<Tagging>,
    /// The notes.
    pub notes: Vec<Note>,
    /// The links.
    pub links: Vec<Link>,
    /// The audit trail, oldest first.
    pub audit: Vec<AuditEntry>,
    /// Local telemetry.
    pub telemetry: Vec<TelemetryRecord>,
    /// The rows features keep for their own kinds: kind name and record, then field name
    /// and value. Generic, so that the fake names no feature.
    pub kind_rows: BTreeMap<(String, RecordId), BTreeMap<String, String>>,
    /// How many times pending migrations were applied.
    pub migrations_applied: u32,
    /// What the integrity check reports.
    pub integrity_problems: Vec<String>,
    /// Whether another command holds the storage.
    pub busy: bool,
    /// Whether applying migrations fails.
    pub failing_migration: bool,
}

/// One workspace's storage, in memory.
#[derive(Clone, Debug, Default)]
pub struct FakeStorage {
    /// The committed state, shared with every unit begun from this storage.
    state: Rc<RefCell<State>>,
}

impl FakeStorage {
    /// Empty storage.
    pub fn new() -> Self {
        Self::default()
    }

    /// A copy of the committed state, to assert on.
    pub fn snapshot(&self) -> State {
        self.state.borrow().clone()
    }

    /// Changes the committed state directly, as something outside the tool would.
    pub fn tamper(&self, change: impl FnOnce(&mut State)) {
        change(&mut self.state.borrow_mut());
    }

    /// A unit working on a copy of the committed state.
    fn unit(&self) -> Result<FakeUnit, StoreError> {
        if self.state.borrow().busy {
            return Err(StoreError::Busy);
        }
        Ok(FakeUnit {
            shared: Rc::clone(&self.state),
            working: self.state.borrow().clone(),
            recorded: None,
        })
    }
}

impl Storage for FakeStorage {
    type Unit = FakeUnit;

    async fn begin(&self) -> Result<FakeUnit, StoreError> {
        self.unit()
    }

    async fn read(&self) -> Result<FakeUnit, StoreError> {
        self.unit()
    }

    async fn close(self) -> Result<(), StoreError> {
        Ok(())
    }
}

/// A unit of work on the in-memory storage.
#[derive(Debug)]
pub struct FakeUnit {
    /// The committed state this unit will write back to.
    shared: Rc<RefCell<State>>,
    /// This unit's own copy, where its changes are made.
    working: State,
    /// The end of the trail after the entries recorded through this unit.
    recorded: Option<AuditHead>,
}

impl FakeUnit {
    /// The unit's working state, to assert on or to prepare a test.
    pub fn state(&mut self) -> &mut State {
        &mut self.working
    }

    /// The row a feature keeps for one of its records.
    pub fn kind_row(&self, kind: &str, id: RecordId) -> Option<&BTreeMap<String, String>> {
        self.working.kind_rows.get(&(kind.to_owned(), id))
    }

    /// Stores the row a feature keeps for one of its records.
    pub fn put_kind_row(&mut self, kind: &str, id: RecordId, row: BTreeMap<String, String>) {
        self.working.kind_rows.insert((kind.to_owned(), id), row);
    }

    /// Removes the row a feature keeps for one of its records.
    pub fn remove_kind_row(&mut self, kind: &str, id: RecordId) {
        self.working.kind_rows.remove(&(kind.to_owned(), id));
    }

    /// Refuses to point at a record the index does not hold, as a foreign key would.
    fn require_record(&self, id: RecordId) -> Result<(), StoreError> {
        let exists = self.working.records.iter().any(|record| record.id == id);
        if exists {
            Ok(())
        } else {
            Err(StoreError::Constraint(format!(
                "record {id} does not exist"
            )))
        }
    }

    /// The index row of a record, to change it.
    fn record_mut(&mut self, id: RecordId) -> Result<&mut IndexedRecord, StoreError> {
        self.working
            .records
            .iter_mut()
            .find(|record| record.id == id)
            .ok_or_else(|| StoreError::Constraint(format!("record {id} does not exist")))
    }

    /// The records that are not deleted, of one of `kinds` (any kind when empty).
    fn live<'a>(&'a self, kinds: &'a [String]) -> impl Iterator<Item = &'a IndexedRecord> {
        self.working.records.iter().filter(move |record| {
            record.deleted_at.is_none() && (kinds.is_empty() || kinds.contains(&record.kind))
        })
    }
}

impl UnitOfWork for FakeUnit {
    async fn commit(self) -> Result<Option<AuditHead>, StoreError> {
        *self.shared.borrow_mut() = self.working;
        Ok(self.recorded)
    }
}

impl AuditLog for FakeUnit {
    async fn record(&mut self, stamp: &Stamp, draft: AuditDraft) -> Result<AuditEntry, StoreError> {
        let (sequence, previous) = match self.working.audit.last() {
            Some(last) => (last.sequence + 1, last.hash),
            None => (1, GENESIS_HASH),
        };
        let entry = AuditEntry::seal(sequence, stamp, draft, previous, ToyDigest::of);
        self.recorded = Some(AuditHead::of(&entry));
        self.working.audit.push(entry.clone());
        Ok(entry)
    }
}

impl AuditQuery for FakeUnit {
    async fn entries(&self, filter: &AuditFilter) -> Result<AuditPage, StoreError> {
        let matching: Vec<&AuditEntry> = self
            .working
            .audit
            .iter()
            .rev()
            .filter(|entry| filter.matches(entry))
            .collect();
        let total = matching.len() as u64;
        let entries = matching
            .into_iter()
            .take(filter.limit as usize)
            .cloned()
            .collect();
        Ok(AuditPage { entries, total })
    }

    async fn entries_after(&self, after: u64, limit: u32) -> Result<Vec<AuditEntry>, StoreError> {
        let entries = self
            .working
            .audit
            .iter()
            .filter(|entry| entry.sequence > after);
        Ok(entries.take(limit as usize).cloned().collect())
    }

    async fn entry_count(&self) -> Result<u64, StoreError> {
        Ok(self.working.audit.len() as u64)
    }
}

impl TelemetryLog for FakeUnit {
    async fn append_telemetry(&mut self, record: &TelemetryRecord) -> Result<(), StoreError> {
        self.working.telemetry.push(record.clone());
        Ok(())
    }

    async fn telemetry(&self) -> Result<Vec<TelemetryRecord>, StoreError> {
        Ok(self.working.telemetry.clone())
    }
}

impl WorkspaceStore for FakeUnit {
    async fn workspace(&self) -> Result<Workspace, StoreError> {
        self.working
            .workspace
            .clone()
            .ok_or_else(|| StoreError::Damaged {
                what: "the workspace's own details are missing".to_owned(),
                location: "table workspace".to_owned(),
            })
    }

    async fn save_workspace(&mut self, workspace: &Workspace) -> Result<(), StoreError> {
        self.working.workspace = Some(workspace.clone());
        Ok(())
    }
}

impl SchemaUpgrade for FakeUnit {
    async fn apply_pending_migrations(&mut self) -> Result<(), StoreError> {
        if self.working.failing_migration {
            return Err(StoreError::Failed("a migration failed".to_owned()));
        }
        self.working.migrations_applied += 1;
        Ok(())
    }
}

impl IntegrityCheck for FakeUnit {
    async fn integrity_problems(&self) -> Result<Vec<String>, StoreError> {
        Ok(self.working.integrity_problems.clone())
    }
}

impl RecordIndex for FakeUnit {
    async fn insert_record(&mut self, record: &IndexedRecord) -> Result<(), StoreError> {
        if self
            .working
            .records
            .iter()
            .any(|existing| existing.handle == record.handle)
        {
            return Err(StoreError::Constraint(format!(
                "handle {} is taken",
                record.handle
            )));
        }
        self.working.records.push(record.clone());
        Ok(())
    }

    async fn rename_record(
        &mut self,
        id: RecordId,
        display_name: &str,
        search_key: &SearchKey,
        at: OffsetDateTime,
    ) -> Result<(), StoreError> {
        let record = self.record_mut(id)?;
        record.display_name = display_name.to_owned();
        record.search_key = search_key.clone();
        record.updated_at = at;
        Ok(())
    }

    async fn touch_record(&mut self, id: RecordId, at: OffsetDateTime) -> Result<(), StoreError> {
        self.record_mut(id)?.updated_at = at;
        Ok(())
    }

    async fn mark_deleted(&mut self, id: RecordId, at: OffsetDateTime) -> Result<(), StoreError> {
        self.record_mut(id)?.deleted_at = Some(at);
        Ok(())
    }

    async fn handle_is_taken(&self, handle: &Handle) -> Result<bool, StoreError> {
        Ok(self
            .working
            .records
            .iter()
            .any(|record| &record.handle == handle))
    }

    async fn record(&self, id: RecordId) -> Result<Option<IndexedRecord>, StoreError> {
        Ok(self
            .working
            .records
            .iter()
            .find(|record| record.id == id)
            .cloned())
    }

    async fn list_records(&self, query: &ListQuery) -> Result<ListPage, StoreError> {
        let kinds = [query.kind.clone()];
        let carries = |record: &IndexedRecord, tag: &TagName| {
            self.working
                .taggings
                .iter()
                .any(|tagging| tagging.record == record.id && &tagging.tag == tag)
        };
        let mut matching: Vec<IndexedRecord> = self
            .live(&kinds)
            .filter(|record| query.tags.iter().all(|tag| carries(record, tag)))
            .filter(|record| {
                query
                    .search
                    .as_ref()
                    .is_none_or(|key| record.search_key.as_str().contains(key.as_str()))
            })
            .cloned()
            .collect();
        matching.sort_by(|one, other| match query.sort {
            SortField::Name => one
                .search_key
                .cmp(&other.search_key)
                .then(one.handle.cmp(&other.handle)),
            SortField::Created => one
                .created_at
                .cmp(&other.created_at)
                .then(one.handle.cmp(&other.handle)),
            SortField::Updated => one
                .updated_at
                .cmp(&other.updated_at)
                .then(one.handle.cmp(&other.handle)),
            SortField::Handle => one.handle.cmp(&other.handle),
        });
        if query.descending {
            matching.reverse();
        }
        let total = matching.len() as u64;
        matching.truncate(query.limit as usize);
        Ok(ListPage {
            items: matching,
            total,
        })
    }

    async fn count_by_kind(&self) -> Result<Vec<(String, u64)>, StoreError> {
        let mut counts: BTreeMap<String, u64> = BTreeMap::new();
        for record in self.live(&[]) {
            *counts.entry(record.kind.clone()).or_default() += 1;
        }
        Ok(counts.into_iter().collect())
    }
}

impl RecordResolver for FakeUnit {
    async fn records_starting_with(
        &self,
        beginning: &str,
        kinds: &[String],
    ) -> Result<Vec<IndexedRecord>, StoreError> {
        let mut found: Vec<IndexedRecord> = self
            .live(kinds)
            .filter(|record| record.handle.as_str().starts_with(beginning))
            .cloned()
            .collect();
        found.sort_by(|one, other| one.handle.cmp(&other.handle));
        Ok(found)
    }

    async fn handles(&self, kinds: &[String]) -> Result<Vec<Handle>, StoreError> {
        Ok(self
            .live(kinds)
            .map(|record| record.handle.clone())
            .collect())
    }
}

impl TagStore for FakeUnit {
    async fn attach_tag(&mut self, record: RecordId, tag: &TagName) -> Result<bool, StoreError> {
        self.require_record(record)?;
        let tagging = Tagging {
            tag: tag.clone(),
            record,
        };
        if self.working.taggings.contains(&tagging) {
            return Ok(false);
        }
        self.working.taggings.push(tagging);
        Ok(true)
    }

    async fn detach_tag(&mut self, record: RecordId, tag: &TagName) -> Result<bool, StoreError> {
        let before = self.working.taggings.len();
        self.working
            .taggings
            .retain(|tagging| !(tagging.record == record && &tagging.tag == tag));
        Ok(self.working.taggings.len() < before)
    }

    async fn tags_of(&self, record: RecordId) -> Result<Vec<TagName>, StoreError> {
        let mut tags: Vec<TagName> = self
            .working
            .taggings
            .iter()
            .filter(|tagging| tagging.record == record)
            .map(|t| t.tag.clone())
            .collect();
        tags.sort();
        Ok(tags)
    }

    async fn tag_counts(&self, kind: Option<&str>) -> Result<Vec<(TagName, u64)>, StoreError> {
        let kinds: Vec<String> = kind.map(str::to_owned).into_iter().collect();
        let live: Vec<RecordId> = self.live(&kinds).map(|record| record.id).collect();
        let mut counts: BTreeMap<TagName, u64> = BTreeMap::new();
        for tagging in self
            .working
            .taggings
            .iter()
            .filter(|tagging| live.contains(&tagging.record))
        {
            *counts.entry(tagging.tag.clone()).or_default() += 1;
        }
        Ok(counts.into_iter().collect())
    }

    async fn remove_taggings_of(&mut self, record: RecordId) -> Result<u64, StoreError> {
        let before = self.working.taggings.len();
        self.working
            .taggings
            .retain(|tagging| tagging.record != record);
        Ok((before - self.working.taggings.len()) as u64)
    }
}

impl NoteStore for FakeUnit {
    async fn add_note(&mut self, note: &Note) -> Result<(), StoreError> {
        self.require_record(note.record)?;
        self.working.notes.push(note.clone());
        Ok(())
    }

    async fn notes_of(&self, record: RecordId) -> Result<Vec<Note>, StoreError> {
        Ok(self
            .working
            .notes
            .iter()
            .filter(|note| note.record == record)
            .cloned()
            .collect())
    }

    async fn remove_notes_of(&mut self, record: RecordId) -> Result<u64, StoreError> {
        let before = self.working.notes.len();
        self.working.notes.retain(|note| note.record != record);
        Ok((before - self.working.notes.len()) as u64)
    }
}

impl LinkStore for FakeUnit {
    async fn add_link(&mut self, link: &Link) -> Result<(), StoreError> {
        self.require_record(link.from)?;
        self.require_record(link.to)?;
        if self
            .working
            .links
            .iter()
            .any(|existing| existing.is_same_as(link))
        {
            return Err(StoreError::Constraint("the link already exists".to_owned()));
        }
        self.working.links.push(link.clone());
        Ok(())
    }

    async fn remove_link(&mut self, link: &Link) -> Result<bool, StoreError> {
        let before = self.working.links.len();
        self.working
            .links
            .retain(|existing| !existing.is_same_as(link));
        Ok(self.working.links.len() < before)
    }

    async fn links_of(&self, record: RecordId) -> Result<Vec<Link>, StoreError> {
        Ok(self
            .working
            .links
            .iter()
            .filter(|link| link.other_end(record).is_some())
            .cloned()
            .collect())
    }

    async fn remove_links_of(&mut self, record: RecordId) -> Result<u64, StoreError> {
        let before = self.working.links.len();
        self.working
            .links
            .retain(|link| link.other_end(record).is_none());
        Ok((before - self.working.links.len()) as u64)
    }
}

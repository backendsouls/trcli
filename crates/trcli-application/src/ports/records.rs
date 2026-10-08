//! Ports for what all records have in common: the index of records, tags, notes, and links
//! (FR-010 to FR-019).
//!
//! The record index holds one row for every record of every kind. It is what makes short
//! names unique across kinds, lets a tag, note, or link point at any record, and makes
//! "nothing is left pointing at a deleted record" a rule of storage (FR-017).

use time::OffsetDateTime;
use trcli_domain::shared::link::Link;
use trcli_domain::shared::note::Note;
use trcli_domain::shared::record::{Handle, RecordId};
use trcli_domain::shared::text::{SearchKey, TagName};

use super::unit_of_work::StoreError;

/// What the index knows about one record of any kind.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct IndexedRecord {
    /// The record's identifier.
    pub id: RecordId,
    /// The name of its kind.
    pub kind: String,
    /// Its short name.
    pub handle: Handle,
    /// What messages and the audit trail call it.
    pub display_name: String,
    /// What it is found and sorted by.
    pub search_key: SearchKey,
    /// When it was created.
    pub created_at: OffsetDateTime,
    /// When it was last changed.
    pub updated_at: OffsetDateTime,
    /// When it was deleted; the row stays so that its handle is never given out again.
    pub deleted_at: Option<OffsetDateTime>,
}

/// What a list is ordered by.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SortField {
    /// By name, ignoring case and accents.
    Name,
    /// By when the record was created.
    Created,
    /// By when the record was last changed.
    Updated,
    /// By short name.
    Handle,
}

/// Which records of one kind to list, and how.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ListQuery {
    /// The kind of record.
    pub kind: String,
    /// Only records carrying every one of these tags.
    pub tags: Vec<TagName>,
    /// Only records whose search key contains this.
    pub search: Option<SearchKey>,
    /// The order.
    pub sort: SortField,
    /// Reverse the order.
    pub descending: bool,
    /// At most this many.
    pub limit: u32,
}

/// One page of a list, with how many records matched in all (FR-037).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ListPage {
    /// The records shown.
    pub items: Vec<IndexedRecord>,
    /// How many records match, shown or not.
    pub total: u64,
}

/// The index of every record of every kind.
pub trait RecordIndex {
    /// Adds a record to the index.
    async fn insert_record(&mut self, record: &IndexedRecord) -> Result<(), StoreError>;

    /// Changes what a record is called and found by, and when it was last changed.
    async fn rename_record(
        &mut self,
        id: RecordId,
        display_name: &str,
        search_key: &SearchKey,
        at: OffsetDateTime,
    ) -> Result<(), StoreError>;

    /// Notes that a record was changed at `at`.
    async fn touch_record(&mut self, id: RecordId, at: OffsetDateTime) -> Result<(), StoreError>;

    /// Marks a record deleted. The row stays; its handle is never reused (FR-010).
    async fn mark_deleted(&mut self, id: RecordId, at: OffsetDateTime) -> Result<(), StoreError>;

    /// Whether a handle was ever given out, to a record that still exists or not.
    async fn handle_is_taken(&self, handle: &Handle) -> Result<bool, StoreError>;

    /// The record with this identifier, deleted or not.
    async fn record(&self, id: RecordId) -> Result<Option<IndexedRecord>, StoreError>;

    /// One page of the records of a kind that are not deleted.
    async fn list_records(&self, query: &ListQuery) -> Result<ListPage, StoreError>;

    /// How many records of each kind exist, deleted ones excluded (FR-006).
    async fn count_by_kind(&self) -> Result<Vec<(String, u64)>, StoreError>;
}

/// Finds records by what a researcher typed (FR-011).
pub trait RecordResolver {
    /// Records that are not deleted, of one of `kinds` (any kind when empty), whose handle
    /// starts with `beginning`.
    async fn records_starting_with(
        &self,
        beginning: &str,
        kinds: &[String],
    ) -> Result<Vec<IndexedRecord>, StoreError>;

    /// The handles of every record that is not deleted, of one of `kinds` (any when
    /// empty); used to suggest close matches.
    async fn handles(&self, kinds: &[String]) -> Result<Vec<Handle>, StoreError>;
}

/// Which records carry which tags (FR-015).
pub trait TagStore {
    /// Gives a record a tag; `false` when it already carried it.
    async fn attach_tag(&mut self, record: RecordId, tag: &TagName) -> Result<bool, StoreError>;

    /// Takes a tag from a record; `false` when it did not carry it.
    async fn detach_tag(&mut self, record: RecordId, tag: &TagName) -> Result<bool, StoreError>;

    /// The tags a record carries, in alphabetical order.
    async fn tags_of(&self, record: RecordId) -> Result<Vec<TagName>, StoreError>;

    /// Every tag with the number of records carrying it, of one kind when given.
    async fn tag_counts(&self, kind: Option<&str>) -> Result<Vec<(TagName, u64)>, StoreError>;

    /// Removes every tagging of a record; returns how many there were.
    async fn remove_taggings_of(&mut self, record: RecordId) -> Result<u64, StoreError>;
}

/// The notes of records (FR-015).
pub trait NoteStore {
    /// Adds a note.
    async fn add_note(&mut self, note: &Note) -> Result<(), StoreError>;

    /// The notes of a record, oldest first.
    async fn notes_of(&self, record: RecordId) -> Result<Vec<Note>, StoreError>;

    /// Removes every note of a record; returns how many there were.
    async fn remove_notes_of(&mut self, record: RecordId) -> Result<u64, StoreError>;
}

/// The links between records (FR-016).
pub trait LinkStore {
    /// Adds a link.
    async fn add_link(&mut self, link: &Link) -> Result<(), StoreError>;

    /// Removes the link joining the same two records with the same relation, whichever
    /// end it was made from; `false` when there was none.
    async fn remove_link(&mut self, link: &Link) -> Result<bool, StoreError>;

    /// The links a record has, from either end, oldest first.
    async fn links_of(&self, record: RecordId) -> Result<Vec<Link>, StoreError>;

    /// Removes every link of a record; returns how many there were.
    async fn remove_links_of(&mut self, record: RecordId) -> Result<u64, StoreError>;
}

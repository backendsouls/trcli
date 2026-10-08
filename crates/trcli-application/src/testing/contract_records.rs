//! Contract of the record index and of tags, notes, and links (T063; FR-010 to FR-017).
//!
//! Run it against a storage adapter by calling [`run`] with a function that makes fresh,
//! empty storage each time it is called.

use time::Duration;
use trcli_domain::shared::link::Link;
use trcli_domain::shared::note::Note;
use trcli_domain::shared::record::{Handle, RecordKind};
use trcli_domain::shared::text::{Relation, SearchKey, TagName};

use super::environment::{SeededIds, stamp};
use crate::ports::records::{
    IndexedRecord, LinkStore, ListQuery, NoteStore, RecordIndex, RecordResolver, SortField,
    TagStore,
};
use crate::ports::unit_of_work::{Storage, UnitOfWork};

/// Everything the contract needs from a unit of work.
pub trait RecordStores: RecordIndex + RecordResolver + TagStore + NoteStore + LinkStore {}

impl<U: RecordIndex + RecordResolver + TagStore + NoteStore + LinkStore> RecordStores for U {}

/// An index row for the record at `position` of the seeded sequence.
pub fn indexed(position: u64, kind: &str, prefix: &str, name: &str) -> IndexedRecord {
    let id = SeededIds::at(position);
    let kind_value = RecordKind::new(kind, prefix).expect("a valid kind");
    let at = stamp().at + Duration::seconds(position as i64);
    IndexedRecord {
        id,
        kind: kind.to_owned(),
        handle: Handle::derive(&kind_value, id, 4),
        display_name: name.to_owned(),
        search_key: SearchKey::from_text(name),
        created_at: at,
        updated_at: at,
        deleted_at: None,
    }
}

/// A tag name.
fn tag(name: &str) -> TagName {
    TagName::new(name).expect("a valid tag")
}

/// A query listing every record of a kind by name.
fn query(kind: &str) -> ListQuery {
    ListQuery {
        kind: kind.to_owned(),
        tags: Vec::new(),
        search: None,
        sort: SortField::Name,
        descending: false,
        limit: 50,
    }
}

/// Storage holding three records: two specimens ("Ação", "Banana") and one sample note.
async fn seeded<S>(storage: &S) -> [IndexedRecord; 3]
where
    S: Storage,
    S::Unit: RecordStores,
{
    let records = [
        indexed(0, "alpha", "alp", "Ação"),
        indexed(1, "alpha", "alp", "Banana split"),
        indexed(2, "beta", "bet", "A note"),
    ];
    let mut unit = storage.begin().await.expect("begin");
    for record in &records {
        unit.insert_record(record).await.expect("insert");
    }
    unit.commit().await.expect("commit");
    records
}

/// Runs every case of the contract, each on fresh storage.
pub async fn run<S>(fresh: impl AsyncFn() -> S)
where
    S: Storage,
    S::Unit: RecordStores,
{
    records_round_trip_and_are_found_by_the_start_of_their_handle(&fresh().await).await;
    lists_filter_search_sort_and_limit(&fresh().await).await;
    counts_per_kind_exclude_deleted_records(&fresh().await).await;
    a_deleted_records_handle_stays_taken(&fresh().await).await;
    nothing_can_point_at_a_record_that_does_not_exist(&fresh().await).await;
    tags_are_attached_once_and_counted(&fresh().await).await;
    links_are_seen_from_both_ends_and_exist_once(&fresh().await).await;
    after_removal_nothing_refers_to_the_record(&fresh().await).await;
}

/// A record is read back as stored, and found by any beginning of its handle.
async fn records_round_trip_and_are_found_by_the_start_of_their_handle<S>(storage: &S)
where
    S: Storage,
    S::Unit: RecordStores,
{
    let [first, _, third] = seeded(storage).await;
    let unit = storage.read().await.expect("read");
    assert_eq!(
        unit.record(first.id).await.expect("record"),
        Some(first.clone())
    );

    let exact = unit
        .records_starting_with(first.handle.as_str(), &[])
        .await
        .expect("resolve");
    assert_eq!(exact, vec![first.clone()]);
    let by_kind = unit
        .records_starting_with("alp", &[])
        .await
        .expect("resolve");
    assert_eq!(by_kind.len(), 2);
    let other_kind_only = unit
        .records_starting_with("", &["beta".to_owned()])
        .await
        .expect("resolve");
    assert_eq!(other_kind_only, vec![third]);
    assert_eq!(
        unit.handles(&["alpha".to_owned()])
            .await
            .expect("handles")
            .len(),
        2
    );
}

/// Listing filters by tag, searches the search key, sorts, and limits with a total.
async fn lists_filter_search_sort_and_limit<S>(storage: &S)
where
    S: Storage,
    S::Unit: RecordStores,
{
    let [first, second, _] = seeded(storage).await;
    let mut unit = storage.begin().await.expect("begin");
    unit.attach_tag(second.id, &tag("fruit"))
        .await
        .expect("tag");

    let by_name = unit.list_records(&query("alpha")).await.expect("list");
    let names: Vec<&str> = by_name
        .items
        .iter()
        .map(|record| record.display_name.as_str())
        .collect();
    assert_eq!((names, by_name.total), (vec!["Ação", "Banana split"], 2));

    let newest_first = ListQuery {
        sort: SortField::Created,
        descending: true,
        ..query("alpha")
    };
    assert_eq!(
        unit.list_records(&newest_first).await.expect("list").items[0].id,
        second.id
    );

    let searched = ListQuery {
        search: Some(SearchKey::from_text("acao")),
        ..query("alpha")
    };
    assert_eq!(
        unit.list_records(&searched).await.expect("list").items,
        vec![first]
    );

    let tagged = ListQuery {
        tags: vec![tag("fruit")],
        ..query("alpha")
    };
    assert_eq!(
        unit.list_records(&tagged).await.expect("list").items,
        vec![second]
    );

    let limited = unit
        .list_records(&ListQuery {
            limit: 1,
            ..query("alpha")
        })
        .await
        .expect("list");
    assert_eq!((limited.items.len(), limited.total), (1, 2));
}

/// Deleted records are not counted and not listed.
async fn counts_per_kind_exclude_deleted_records<S>(storage: &S)
where
    S: Storage,
    S::Unit: RecordStores,
{
    let [first, _, _] = seeded(storage).await;
    let mut unit = storage.begin().await.expect("begin");
    unit.mark_deleted(first.id, stamp().at)
        .await
        .expect("delete");
    let counts = unit.count_by_kind().await.expect("counts");
    assert_eq!(
        counts,
        vec![("alpha".to_owned(), 1), ("beta".to_owned(), 1)]
    );
    assert_eq!(
        unit.list_records(&query("alpha"))
            .await
            .expect("list")
            .total,
        1
    );
    assert!(
        unit.records_starting_with(first.handle.as_str(), &[])
            .await
            .expect("resolve")
            .is_empty()
    );
}

/// A handle is never given out again, even after its record is deleted (FR-010).
async fn a_deleted_records_handle_stays_taken<S>(storage: &S)
where
    S: Storage,
    S::Unit: RecordStores,
{
    let [first, _, _] = seeded(storage).await;
    let mut unit = storage.begin().await.expect("begin");
    unit.mark_deleted(first.id, stamp().at)
        .await
        .expect("delete");
    assert!(unit.handle_is_taken(&first.handle).await.expect("taken"));
    let mut reused = indexed(9, "alpha", "alp", "Reuser");
    reused.handle = first.handle.clone();
    assert!(
        unit.insert_record(&reused).await.is_err(),
        "a handle must be unique including deleted rows"
    );
}

/// A tag, note, or link cannot be created for a record the index does not hold.
async fn nothing_can_point_at_a_record_that_does_not_exist<S>(storage: &S)
where
    S: Storage,
    S::Unit: RecordStores,
{
    let [first, _, _] = seeded(storage).await;
    let missing = SeededIds::at(77);
    let body = Note::body("a remark").expect("valid");
    let link = Link::new(first.id, missing, Relation::related(), stamp().at).expect("two records");

    let mut unit = storage.begin().await.expect("begin");
    assert!(unit.attach_tag(missing, &tag("ghost")).await.is_err());
    let mut unit = storage.begin_again(unit).await;
    assert!(
        unit.add_note(&Note::new(missing, body, stamp().at))
            .await
            .is_err()
    );
    let mut unit = storage.begin_again(unit).await;
    assert!(unit.add_link(&link).await.is_err());
}

/// Tagging twice leaves one tagging; counts follow.
async fn tags_are_attached_once_and_counted<S>(storage: &S)
where
    S: Storage,
    S::Unit: RecordStores,
{
    let [first, second, third] = seeded(storage).await;
    let mut unit = storage.begin().await.expect("begin");
    assert!(
        unit.attach_tag(first.id, &tag("field-work"))
            .await
            .expect("tag")
    );
    assert!(
        !unit
            .attach_tag(first.id, &tag("field-work"))
            .await
            .expect("tag")
    );
    unit.attach_tag(first.id, &tag("archive"))
        .await
        .expect("tag");
    unit.attach_tag(second.id, &tag("field-work"))
        .await
        .expect("tag");
    unit.attach_tag(third.id, &tag("field-work"))
        .await
        .expect("tag");

    assert_eq!(
        unit.tags_of(first.id).await.expect("tags"),
        vec![tag("archive"), tag("field-work")]
    );
    assert_eq!(
        unit.tag_counts(None).await.expect("counts"),
        vec![(tag("archive"), 1), (tag("field-work"), 3)]
    );
    assert_eq!(
        unit.tag_counts(Some("beta")).await.expect("counts"),
        vec![(tag("field-work"), 1)]
    );

    assert!(
        unit.detach_tag(first.id, &tag("archive"))
            .await
            .expect("untag")
    );
    assert!(
        !unit
            .detach_tag(first.id, &tag("archive"))
            .await
            .expect("untag")
    );
}

/// A link is listed from both records; the same link cannot be made twice, either way.
async fn links_are_seen_from_both_ends_and_exist_once<S>(storage: &S)
where
    S: Storage,
    S::Unit: RecordStores,
{
    let [first, second, _] = seeded(storage).await;
    let link =
        Link::new(first.id, second.id, Relation::related(), stamp().at).expect("two records");
    let reverse =
        Link::new(second.id, first.id, Relation::related(), stamp().at).expect("two records");

    let mut unit = storage.begin().await.expect("begin");
    unit.add_link(&link).await.expect("link");
    assert_eq!(
        unit.links_of(first.id).await.expect("links"),
        vec![link.clone()]
    );
    assert_eq!(
        unit.links_of(second.id).await.expect("links"),
        vec![link.clone()]
    );
    unit.commit().await.expect("commit");

    let mut unit = storage.begin().await.expect("begin");
    assert!(
        unit.add_link(&reverse).await.is_err(),
        "the reverse of a link is the same link"
    );
    let mut unit = storage.begin_again(unit).await;
    assert!(
        unit.remove_link(&reverse).await.expect("unlink"),
        "a link is removed from either end"
    );
    assert!(unit.links_of(first.id).await.expect("links").is_empty());
}

/// After a record's taggings, notes, and links are removed, nothing refers to it (FR-017).
async fn after_removal_nothing_refers_to_the_record<S>(storage: &S)
where
    S: Storage,
    S::Unit: RecordStores,
{
    let [first, second, _] = seeded(storage).await;
    let body = Note::body("a remark").expect("valid");
    let mut unit = storage.begin().await.expect("begin");
    unit.attach_tag(first.id, &tag("field-work"))
        .await
        .expect("tag");
    unit.add_note(&Note::new(first.id, body.clone(), stamp().at))
        .await
        .expect("note");
    unit.add_note(&Note::new(first.id, body, stamp().at))
        .await
        .expect("note");
    let link =
        Link::new(second.id, first.id, Relation::related(), stamp().at).expect("two records");
    unit.add_link(&link).await.expect("link");
    assert_eq!(unit.notes_of(first.id).await.expect("notes").len(), 2);

    assert_eq!(unit.remove_links_of(first.id).await.expect("links"), 1);
    assert_eq!(
        unit.remove_taggings_of(first.id).await.expect("taggings"),
        1
    );
    assert_eq!(unit.remove_notes_of(first.id).await.expect("notes"), 2);
    unit.mark_deleted(first.id, stamp().at)
        .await
        .expect("delete");

    assert!(unit.links_of(second.id).await.expect("links").is_empty());
    assert!(unit.tags_of(first.id).await.expect("tags").is_empty());
    assert!(unit.notes_of(first.id).await.expect("notes").is_empty());
    assert!(unit.tag_counts(None).await.expect("counts").is_empty());
}

/// Starting over after a statement failed: some stores end a unit when a rule is broken.
trait BeginAgain: Storage {
    /// Drops the unit and begins a new one.
    async fn begin_again(&self, unit: Self::Unit) -> Self::Unit;
}

impl<S: Storage> BeginAgain for S {
    async fn begin_again(&self, unit: Self::Unit) -> Self::Unit {
        drop(unit);
        self.begin().await.expect("begin")
    }
}

#[cfg(test)]
mod tests {
    //! The fake passes the contract it is used in place of.

    use crate::testing::block_on;
    use crate::testing::unit::FakeStorage;

    #[test]
    fn the_in_memory_storage_passes_the_records_contract() {
        block_on(super::run(async || FakeStorage::new()));
    }
}

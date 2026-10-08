//! Showing one record of any kind: the fields every record has, the kind's own fields,
//! and its tags, notes, and links (FR-013, FR-018).

use serde::ser::{Serialize, SerializeMap, Serializer};

use super::link::{LinkView, links_of};
use super::resolve::resolve;
use crate::kinds::{KindBehaviour, KindField};
use crate::outcome::Problem;
use crate::ports::records::{
    IndexedRecord, LinkStore, NoteStore, RecordIndex, RecordResolver, TagStore,
};
use crate::view::Instant;

/// A note, in a view.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize)]
pub struct NoteView {
    /// When the note was added.
    pub created_at: Instant,
    /// What was noted.
    pub body: String,
}

/// One record with everything attached to it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RecordDetail {
    /// The record's short name.
    pub handle: String,
    /// The record's identifier.
    pub id: String,
    /// The record's kind.
    pub kind: String,
    /// What the record is called.
    pub name: String,
    /// When it was created.
    pub created_at: Instant,
    /// When it was last changed.
    pub updated_at: Instant,
    /// The kind's own fields, in the order the kind shows them.
    pub fields: Vec<KindField>,
    /// The tags it carries.
    pub tags: Vec<String>,
    /// Its notes, oldest first.
    pub notes: Vec<NoteView>,
    /// Its links, seen from this record.
    pub links: Vec<LinkView>,
}

impl Serialize for RecordDetail {
    /// The kind's own fields sit beside the common ones, as `snake_case` keys, so that a
    /// record of any kind has the same shape plus what is particular to it.
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(9 + self.fields.len()))?;
        map.serialize_entry("handle", &self.handle)?;
        map.serialize_entry("id", &self.id)?;
        map.serialize_entry("kind", &self.kind)?;
        map.serialize_entry("name", &self.name)?;
        for field in &self.fields {
            map.serialize_entry(field.name, &field.value)?;
        }
        map.serialize_entry("created_at", &self.created_at)?;
        map.serialize_entry("updated_at", &self.updated_at)?;
        map.serialize_entry("tags", &self.tags)?;
        map.serialize_entry("notes", &self.notes)?;
        map.serialize_entry("links", &self.links)?;
        map.end()
    }
}

/// Everything showing a record needs from a unit of work.
pub trait ShowStores: RecordResolver + RecordIndex + TagStore + NoteStore + LinkStore {}

impl<U: RecordResolver + RecordIndex + TagStore + NoteStore + LinkStore> ShowStores for U {}

/// Gathers everything about a record that is already resolved.
pub async fn detail<U, K>(
    unit: &U,
    kind: &K,
    record: &IndexedRecord,
) -> Result<RecordDetail, Problem>
where
    U: ShowStores,
    K: KindBehaviour<U>,
{
    let notes = unit.notes_of(record.id).await?;
    Ok(RecordDetail {
        handle: record.handle.to_string(),
        id: record.id.to_string(),
        kind: record.kind.clone(),
        name: record.display_name.clone(),
        created_at: record.created_at.into(),
        updated_at: record.updated_at.into(),
        fields: kind.fields(unit, record.id).await?,
        tags: unit
            .tags_of(record.id)
            .await?
            .iter()
            .map(ToString::to_string)
            .collect(),
        notes: notes
            .iter()
            .map(|note| NoteView {
                created_at: note.created_at.into(),
                body: note.body.to_string(),
            })
            .collect(),
        links: links_of(unit, record).await?,
    })
}

/// Shows the record of this kind that was typed.
pub async fn show<U, K>(unit: &U, kind: &K, reference: &str) -> Result<RecordDetail, Problem>
where
    U: ShowStores,
    K: KindBehaviour<U>,
{
    let kinds = [kind.descriptor().name().to_owned()];
    let record = resolve(unit, reference, &kinds).await?;
    detail(unit, kind, &record).await
}

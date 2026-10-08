//! Listing the records of a kind: filter by tag, search, sort, and limit — the same way
//! for every kind (FR-013, FR-037).

use serde::Serialize;
use trcli_domain::shared::text::{SearchKey, TagName};

use crate::kinds::RecordKindDescriptor;
use crate::outcome::Problem;
use crate::ports::records::{IndexedRecord, ListQuery, RecordIndex, SortField, TagStore};
use crate::validation::{Checker, Valid, integer_in_range, one_of};
use crate::view::Instant;

/// The options of `<noun> list`, unchecked.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ListInput {
    /// `--search`.
    pub search: Option<String>,
    /// `--tag`, repeatable.
    pub tags: Vec<String>,
    /// `--sort`.
    pub sort: Option<String>,
    /// `--desc`.
    pub descending: bool,
    /// `--limit`.
    pub limit: Option<String>,
}

/// The fields every kind can be sorted by, besides its own name for its main text.
const SORT_FIELDS: [&str; 4] = ["name", "created", "updated", "handle"];

/// Checks the options together and builds the query. Without `--sort` a list is ordered
/// by name; without `--limit` it shows `default_limit` rows.
pub fn query(
    descriptor: &RecordKindDescriptor,
    input: &ListInput,
    default_limit: u32,
) -> Result<Valid<ListQuery>, Problem> {
    let mut checker = Checker::new();
    let tags: Vec<Option<TagName>> = input
        .tags
        .iter()
        .map(|tag| checker.check("--tag", tag, TagName::new))
        .collect();
    let mut sort_fields = vec![descriptor.name_field];
    sort_fields.extend(
        SORT_FIELDS
            .iter()
            .filter(|field| **field != descriptor.name_field),
    );
    let sort = checker.optional("--sort", input.sort.as_deref(), |sort| {
        one_of(sort, &sort_fields)
    });
    let limit = checker.optional("--limit", input.limit.as_deref(), |limit| {
        integer_in_range(limit, 1, 1000)
    });
    checker.finish(|| ListQuery {
        kind: descriptor.name().to_owned(),
        tags: tags.into_iter().flatten().collect(),
        search: input
            .search
            .as_deref()
            .map(SearchKey::from_text)
            .filter(|key| !key.is_empty()),
        sort: sort
            .flatten()
            .map_or(SortField::Name, |name| sort_field(&name)),
        descending: input.descending,
        limit: limit.flatten().map_or(default_limit, |limit| limit as u32),
    })
}

/// The sort field with this name; a kind's own name for its main text means "name".
fn sort_field(name: &str) -> SortField {
    match name {
        "created" => SortField::Created,
        "updated" => SortField::Updated,
        "handle" => SortField::Handle,
        _ => SortField::Name,
    }
}

/// One record in a list.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct RecordRow {
    /// The record's short name.
    pub handle: String,
    /// The record's identifier.
    pub id: String,
    /// The record's kind.
    pub kind: String,
    /// What the record is called.
    pub name: String,
    /// The tags it carries.
    pub tags: Vec<String>,
    /// When it was created.
    pub created_at: Instant,
    /// When it was last changed.
    pub updated_at: Instant,
}

/// A list of records of one kind.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct RecordList {
    /// The kind listed.
    pub kind: String,
    /// What the kind calls its records' main text; the heading of that column.
    pub name_field: String,
    /// The records shown.
    pub items: Vec<RecordRow>,
    /// How many records match, shown or not (FR-037).
    pub total: u64,
}

/// The row of one record.
pub async fn row<U: TagStore>(unit: &U, record: &IndexedRecord) -> Result<RecordRow, Problem> {
    Ok(RecordRow {
        handle: record.handle.to_string(),
        id: record.id.to_string(),
        kind: record.kind.clone(),
        name: record.display_name.clone(),
        tags: unit
            .tags_of(record.id)
            .await?
            .iter()
            .map(ToString::to_string)
            .collect(),
        created_at: record.created_at.into(),
        updated_at: record.updated_at.into(),
    })
}

/// Lists the records that match the query.
pub async fn list<U>(
    unit: &U,
    descriptor: &RecordKindDescriptor,
    query: &ListQuery,
) -> Result<RecordList, Problem>
where
    U: RecordIndex + TagStore,
{
    let page = unit.list_records(query).await?;
    let mut items = Vec::with_capacity(page.items.len());
    for record in &page.items {
        items.push(row(unit, record).await?);
    }
    Ok(RecordList {
        kind: descriptor.name().to_owned(),
        name_field: descriptor.name_field.to_owned(),
        items,
        total: page.total,
    })
}

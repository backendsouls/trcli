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

#[cfg(test)]
mod tests {
    //! Unit tests for listing options (T066).

    use trcli_domain::shared::text::TagName;

    use super::{ListInput, list, query};
    use crate::kinds::RecordKindDescriptor;
    use crate::outcome::Details;
    use crate::ports::records::{RecordIndex, SortField, TagStore};
    use crate::ports::unit_of_work::Storage;
    use crate::testing::block_on;
    use crate::testing::contract_records::indexed;
    use crate::testing::unit::{FakeStorage, FakeUnit};

    /// The kind listed in these tests: its main text is called `title`.
    fn descriptor() -> RecordKindDescriptor {
        RecordKindDescriptor::new("alpha", "alp", "Alpha.", "title")
    }

    /// A unit with three `alpha` records, the second tagged `fruit`.
    fn unit() -> FakeUnit {
        block_on(async {
            let mut unit = FakeStorage::new().begin().await.expect("begin");
            for (position, name) in ["Cherry", "Ação", "banana"].iter().enumerate() {
                unit.insert_record(&indexed(position as u64, "alpha", "alp", name))
                    .await
                    .expect("insert");
            }
            let second = unit.state().records[1].id;
            unit.attach_tag(second, &TagName::new("fruit").expect("valid"))
                .await
                .expect("tag");
            unit
        })
    }

    /// The names a listing with these options shows, and its total.
    fn names(input: ListInput) -> (Vec<String>, u64) {
        let query = query(&descriptor(), &input, 50).expect("valid").command;
        let listed = block_on(list(&unit(), &descriptor(), &query)).expect("listed");
        (
            listed.items.into_iter().map(|row| row.name).collect(),
            listed.total,
        )
    }

    #[test]
    fn a_list_is_ordered_by_name_ignoring_case_and_accents_by_default() {
        assert_eq!(names(ListInput::default()).0, ["Ação", "banana", "Cherry"]);
    }

    #[test]
    fn a_list_filters_by_tag_and_searches_without_regard_to_accents() {
        assert_eq!(
            names(ListInput {
                tags: vec!["Fruit".into()],
                ..ListInput::default()
            })
            .0,
            ["Ação"]
        );
        assert_eq!(
            names(ListInput {
                search: Some("ACAO".into()),
                ..ListInput::default()
            })
            .0,
            ["Ação"]
        );
    }

    #[test]
    fn a_list_sorts_by_the_kinds_own_name_for_its_main_text_and_reverses() {
        let by_title = ListInput {
            sort: Some("title".into()),
            descending: true,
            ..ListInput::default()
        };
        assert_eq!(names(by_title).0, ["Cherry", "banana", "Ação"]);
        let checked = query(
            &descriptor(),
            &ListInput {
                sort: Some("created".into()),
                ..ListInput::default()
            },
            50,
        );
        assert_eq!(checked.expect("valid").command.sort, SortField::Created);
    }

    #[test]
    fn a_list_is_limited_with_a_count_of_all_that_match() {
        let (shown, total) = names(ListInput {
            limit: Some("2".into()),
            ..ListInput::default()
        });
        assert_eq!((shown.len(), total), (2, 3));
        assert_eq!(
            query(&descriptor(), &ListInput::default(), 7)
                .expect("valid")
                .command
                .limit,
            7
        );
    }

    #[test]
    fn invalid_options_are_reported_together_with_what_is_allowed() {
        let input = ListInput {
            tags: vec!["two words".into()],
            sort: Some("colour".into()),
            limit: Some("0".into()),
            ..ListInput::default()
        };
        let problem = query(&descriptor(), &input, 50).expect_err("invalid");
        let Details::Fields(fields) = problem.details else {
            panic!("fields expected")
        };
        let named: Vec<&str> = fields.iter().map(|field| field.field.as_str()).collect();
        assert_eq!(named, ["--tag", "--sort", "--limit"]);
        assert_eq!(
            fields[1].choices,
            ["title", "name", "created", "updated", "handle"]
        );
    }
}

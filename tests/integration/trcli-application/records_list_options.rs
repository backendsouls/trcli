//! Unit tests for listing options (T066).

use trcli_domain::shared::text::TagName;

use trcli_application::kinds::RecordKindDescriptor;
use trcli_application::outcome::Details;
use trcli_application::ports::records::{RecordIndex, SortField, TagStore};
use trcli_application::ports::unit_of_work::Storage;
use trcli_application::records::list_options::{ListInput, list, query};
use trcli_testing::block_on;
use trcli_testing::contract_records::indexed;
use trcli_testing::unit::{FakeStorage, FakeUnit};

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

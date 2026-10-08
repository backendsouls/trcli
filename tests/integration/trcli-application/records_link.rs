//! Unit tests for links (T066).

use trcli_application::outcome::codes;
use trcli_application::ports::records::RecordIndex;
use trcli_application::ports::unit_of_work::Storage;
use trcli_application::records::link::{LinkCommand, add_link, list_links, remove_link};
use trcli_testing::block_on;
use trcli_testing::contract_records::indexed;
use trcli_testing::environment::stamp;
use trcli_testing::unit::{FakeStorage, FakeUnit};

/// A unit holding an `alpha` and a `beta` record; returns their handles too.
fn unit() -> (FakeUnit, String, String) {
    block_on(async {
        let mut unit = FakeStorage::new().begin().await.expect("begin");
        let (first, second) = (
            indexed(0, "alpha", "alp", "First"),
            indexed(1, "beta", "bet", "Second"),
        );
        unit.insert_record(&first).await.expect("insert");
        unit.insert_record(&second).await.expect("insert");
        (unit, first.handle.to_string(), second.handle.to_string())
    })
}

/// The checked command linking two handles.
fn command(one: &str, other: &str, relation: Option<&str>) -> LinkCommand {
    LinkCommand::new(one, other, relation)
        .expect("valid")
        .command
}

#[test]
fn a_link_between_two_kinds_is_visible_from_both_ends() {
    let (mut unit, first, second) = unit();
    block_on(async {
        let linked = add_link(
            &mut unit,
            &stamp(),
            command(&first, &second, Some("same site")),
        )
        .await
        .expect("linked");
        assert_eq!(
            (linked.relation.as_str(), linked.to.kind.as_str()),
            ("same site", "beta")
        );
        let from_first = list_links(&unit, &first).await.expect("listed");
        let from_second = list_links(&unit, &second).await.expect("listed");
        assert_eq!(from_first.items[0].other.handle, second);
        assert_eq!(from_second.items[0].other.handle, first);
    });
    assert_eq!(unit.state().audit[0].action.as_str(), "link");
}

#[test]
fn a_record_cannot_be_linked_to_itself() {
    let (mut unit, first, _) = unit();
    let problem =
        block_on(add_link(&mut unit, &stamp(), command(&first, &first, None))).expect_err("same");
    assert_eq!(problem.code, codes::VALIDATION_FAILED);
    assert!(unit.state().links.is_empty() && unit.state().audit.is_empty());
}

#[test]
fn the_same_link_cannot_be_made_twice_from_either_end() {
    let (mut unit, first, second) = unit();
    block_on(async {
        add_link(&mut unit, &stamp(), command(&first, &second, None))
            .await
            .expect("linked");
        let again = add_link(&mut unit, &stamp(), command(&second, &first, None))
            .await
            .expect_err("duplicate");
        assert_eq!(again.code, codes::VALIDATION_FAILED);
        add_link(&mut unit, &stamp(), command(&first, &second, Some("cites")))
            .await
            .expect("another relation");
    });
    assert_eq!(unit.state().links.len(), 2);
}

#[test]
fn removing_a_link_records_it_and_a_missing_link_is_not_found() {
    let (mut unit, first, second) = unit();
    block_on(async {
        add_link(&mut unit, &stamp(), command(&first, &second, None))
            .await
            .expect("linked");
        let removed = remove_link(&mut unit, &stamp(), command(&second, &first, None))
            .await
            .expect("removed");
        assert!(removed.removed);
        let missing = remove_link(&mut unit, &stamp(), command(&first, &second, None))
            .await
            .expect_err("gone");
        assert_eq!(missing.code, codes::NOT_FOUND);
    });
    let actions: Vec<&str> = unit
        .state()
        .audit
        .iter()
        .map(|entry| entry.action.as_str())
        .collect();
    assert_eq!(actions, ["link", "unlink"]);
}

#[test]
fn a_relation_is_at_most_fifty_characters() {
    assert!(LinkCommand::new("a", "b", Some(&"r".repeat(51))).is_err());
    assert_eq!(command("a", "b", None).relation.as_str(), "related");
}

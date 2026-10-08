//! Unit tests for viewing a workspace (T048, and the counts added by T075).

use std::path::PathBuf;

use trcli_domain::shared::text::{LongText, Name};
use trcli_domain::workspace::Workspace;

use trcli_application::kinds::{KindRegistry, RecordKindDescriptor};
use trcli_application::ports::records::RecordIndex;
use trcli_application::ports::unit_of_work::Storage;
use trcli_application::ports::workspace::WorkspaceStore;
use trcli_application::workspace::locate::{FoundBy, Located};
use trcli_application::workspace::show::{KindCount, show};
use trcli_testing::block_on;
use trcli_testing::contract_records::indexed;
use trcli_testing::environment::{SeededIds, stamp};
use trcli_testing::unit::{FakeStorage, FakeUnit};

/// A unit holding a workspace named "Doctorate".
fn unit() -> FakeUnit {
    block_on(async {
        let mut unit = FakeStorage::new().begin().await.expect("begin");
        let (name, about) = (
            Name::new("Doctorate").expect("valid"),
            LongText::new("Thesis work").expect("valid"),
        );
        unit.save_workspace(&Workspace::new(SeededIds::at(0), name, about, stamp().at))
            .await
            .expect("save");
        unit
    })
}

/// Where the workspace was found.
fn located(found_by: FoundBy) -> Located {
    Located {
        root: PathBuf::from("/research"),
        found_by,
    }
}

/// A registry with the kinds `alpha` and `beta`.
fn kinds() -> KindRegistry {
    let mut registry = KindRegistry::new();
    registry
        .register(RecordKindDescriptor::new("alpha", "alp", "Alpha.", "title"))
        .expect("registered");
    registry
        .register(RecordKindDescriptor::new("beta", "bet", "Beta.", "title"))
        .expect("registered");
    registry
}

#[test]
fn show_returns_name_description_location_and_format_version() {
    let view = block_on(show(
        &unit(),
        &located(FoundBy::Ancestor),
        &KindRegistry::new(),
    ))
    .expect("shown");
    assert_eq!(
        (view.name.as_str(), view.description.as_str()),
        ("Doctorate", "Thesis work")
    );
    assert_eq!(
        (view.location.as_str(), view.format_version),
        ("/research", 1)
    );
    assert!(!view.is_default);
    assert!(view.records.is_empty());
}

#[test]
fn show_says_when_the_default_workspace_was_used() {
    let view = block_on(show(
        &unit(),
        &located(FoundBy::Default),
        &KindRegistry::new(),
    ))
    .expect("shown");
    assert!(view.is_default);
}

#[test]
fn show_counts_records_per_kind_including_kinds_with_none() {
    let mut unit = unit();
    block_on(async {
        unit.insert_record(&indexed(0, "alpha", "alp", "One"))
            .await
            .expect("insert");
        unit.insert_record(&indexed(1, "alpha", "alp", "Two"))
            .await
            .expect("insert");
        unit.insert_record(&indexed(2, "gamma", "gam", "Unknown kind"))
            .await
            .expect("insert");
    });
    let view = block_on(show(&unit, &located(FoundBy::Ancestor), &kinds())).expect("shown");
    let count = |kind: &str, count| KindCount {
        kind: kind.to_owned(),
        count,
    };
    assert_eq!(
        view.records,
        vec![count("alpha", 2), count("beta", 0), count("gamma", 1)]
    );
}

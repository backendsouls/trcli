//! Unit tests for changing a workspace's details (T048).

use trcli_domain::settings::{Place, SettingValue};
use trcli_domain::shared::text::{LongText, Name};
use trcli_domain::workspace::Workspace;

use trcli_application::ports::unit_of_work::Storage;
use trcli_application::ports::workspace::WorkspaceStore;
use trcli_application::settings::foundation::RESEARCHER_NAME;
use trcli_application::workspace::edit::{EditCommand, EditInput, edit};
use trcli_testing::block_on;
use trcli_testing::environment::{SeededIds, stamp};
use trcli_testing::settings::MemoryFiles;
use trcli_testing::unit::{FakeStorage, FakeUnit};

/// A unit holding a workspace named "Doctorate" described as "Thesis".
fn unit() -> FakeUnit {
    block_on(async {
        let mut unit = FakeStorage::new().begin().await.expect("begin");
        let (name, about) = (
            Name::new("Doctorate").expect("valid"),
            LongText::new("Thesis").expect("valid"),
        );
        unit.save_workspace(&Workspace::new(SeededIds::at(0), name, about, stamp().at))
            .await
            .expect("save");
        unit
    })
}

/// The checked command for the given options.
fn command(name: Option<&str>, description: Option<&str>, researcher: Option<&str>) -> EditCommand {
    let owned = |value: Option<&str>| value.map(str::to_owned);
    let input = EditInput {
        name: owned(name),
        description: owned(description),
        researcher: owned(researcher),
    };
    EditCommand::new(input).expect("valid").command
}

#[test]
fn edit_changes_only_what_was_named_and_records_one_entry() {
    let (mut unit, files) = (unit(), MemoryFiles::new());
    let done = block_on(edit(
        &mut unit,
        &stamp(),
        &files,
        None,
        command(Some("Renamed"), None, None),
    ))
    .expect("ok");
    assert_eq!(done.message, "Updated workspace \"Renamed\": name");
    let state = unit.state();
    let workspace = state.workspace.clone().expect("row");
    assert_eq!(
        (workspace.name.as_str(), workspace.description.as_str()),
        ("Renamed", "Thesis")
    );
    assert_eq!(state.audit.len(), 1);
    assert_eq!(
        state.audit[0].changes[0].before.as_deref(),
        Some("Doctorate")
    );
}

#[test]
fn the_researcher_name_goes_to_the_workspace_settings() {
    let (mut unit, files) = (unit(), MemoryFiles::new());
    block_on(edit(
        &mut unit,
        &stamp(),
        &files,
        Some("ana"),
        command(None, None, Some("Ana Souza")),
    ))
    .expect("ok");
    assert_eq!(
        files.value(Place::Workspace, RESEARCHER_NAME),
        Some(SettingValue::Text("Ana Souza".into()))
    );
    assert_eq!(unit.state().audit[0].changes[0].field, RESEARCHER_NAME);
}

#[test]
fn giving_the_same_details_changes_nothing_and_records_nothing() {
    let (mut unit, files) = (unit(), MemoryFiles::new());
    let done = block_on(edit(
        &mut unit,
        &stamp(),
        &files,
        None,
        command(Some("Doctorate"), None, None),
    ))
    .expect("ok");
    assert!(done.message.starts_with("Nothing to change"));
    assert!(unit.state().audit.is_empty());
}

#[test]
fn giving_nothing_or_an_invalid_value_is_rejected() {
    assert!(EditCommand::new(EditInput::default()).is_err());
    let invalid = EditInput {
        name: Some(String::new()),
        ..EditInput::default()
    };
    assert!(EditCommand::new(invalid).is_err());
}

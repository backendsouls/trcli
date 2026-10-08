//! Unit tests for the settings use cases (T097).

use trcli_domain::settings::{Place, SettingValue};

use trcli_application::outcome::{Details, codes};
use trcli_application::ports::unit_of_work::Storage;
use trcli_application::settings::commands::{
    SetCommand, UnsetCommand, ValueView, get, list, paths, set_for_user, set_for_workspace,
    unset_for_user,
};
use trcli_application::settings::foundation::{
    self, DEFAULT_WORKSPACE, OUTPUT_COLOR, OUTPUT_PAGE_SIZE,
};
use trcli_application::settings::layers::{Settings, resolve};
use trcli_application::settings::registry::SettingsRegistry;
use trcli_testing::block_on;
use trcli_testing::environment::stamp;
use trcli_testing::settings::MemoryFiles;
use trcli_testing::unit::FakeStorage;

/// The foundation's registry.
fn registry() -> SettingsRegistry {
    let mut registry = SettingsRegistry::new();
    foundation::register(&mut registry);
    registry
}

/// The settings in effect given what the files hold.
fn settings(files: &MemoryFiles) -> Settings {
    let (user, workspace) = (files.source(Place::User), files.source(Place::Workspace));
    resolve(&registry(), &[&user, &workspace])
        .expect("valid")
        .settings
}

#[test]
fn set_validates_against_the_definition() {
    let invalid =
        SetCommand::new(&registry(), OUTPUT_PAGE_SIZE, "0", Place::Workspace).expect_err("invalid");
    assert_eq!(invalid.code, codes::VALIDATION_FAILED);
    let Details::Fields(fields) = invalid.details else {
        panic!("fields expected")
    };
    assert_eq!(fields[0].expected, "a whole number from 1 to 1000");
    let unknown = SetCommand::new(&registry(), "no.such.setting", "1", Place::Workspace)
        .expect_err("unknown");
    assert_eq!(unknown.code, codes::VALIDATION_FAILED);
}

#[test]
fn set_refuses_a_place_the_setting_does_not_have() {
    let problem =
        SetCommand::new(&registry(), DEFAULT_WORKSPACE, "/w", Place::Workspace).expect_err("scope");
    let Details::Fields(fields) = problem.details else {
        panic!("fields expected")
    };
    assert!(fields[0].problem.contains("can only be set for a user"));
}

#[test]
fn set_writes_to_one_file_only() {
    let files = MemoryFiles::new();
    let command = SetCommand::new(&registry(), OUTPUT_PAGE_SIZE, "20", Place::User)
        .expect("valid")
        .command;
    set_for_user(&files, &command).expect("stored");
    assert_eq!(
        files.value(Place::User, OUTPUT_PAGE_SIZE),
        Some(SettingValue::Integer(20))
    );
    assert_eq!(files.value(Place::Workspace, OUTPUT_PAGE_SIZE), None);
}

#[test]
fn a_workspace_setting_is_recorded_in_the_trail_with_before_and_after() {
    let (files, storage) = (MemoryFiles::new(), FakeStorage::new());
    let command = SetCommand::new(&registry(), OUTPUT_COLOR, "never", Place::Workspace)
        .expect("valid")
        .command;
    block_on(async {
        let mut unit = storage.begin().await.expect("begin");
        set_for_workspace(&mut unit, &stamp(), &files, &settings(&files), &command)
            .await
            .expect("stored");
        trcli_application::ports::unit_of_work::UnitOfWork::commit(unit)
            .await
            .expect("commit");
    });
    let entry = &storage.snapshot().audit[0];
    assert_eq!(entry.action.as_str(), "setting");
    assert_eq!(
        (
            entry.changes[0].before.as_deref(),
            entry.changes[0].after.as_deref()
        ),
        (Some("auto"), Some("never"))
    );
    assert_eq!(
        files.value(Place::Workspace, OUTPUT_COLOR),
        Some(SettingValue::Text("never".into()))
    );
}

#[test]
fn a_failed_write_leaves_no_entry() {
    let (files, storage) = (MemoryFiles::read_only(), FakeStorage::new());
    let command = SetCommand::new(&registry(), OUTPUT_COLOR, "never", Place::Workspace)
        .expect("valid")
        .command;
    block_on(async {
        let mut unit = storage.begin().await.expect("begin");
        let result =
            set_for_workspace(&mut unit, &stamp(), &files, &Settings::default(), &command).await;
        assert_eq!(result.expect_err("read-only").code, codes::OPERATION_FAILED);
    });
    assert!(storage.snapshot().audit.is_empty());
}

#[test]
fn unset_removes_the_value_from_that_file_and_the_next_source_applies() {
    let files = MemoryFiles::new();
    let set = SetCommand::new(&registry(), OUTPUT_COLOR, "never", Place::User)
        .expect("valid")
        .command;
    set_for_user(&files, &set).expect("stored");
    assert_eq!(settings(&files).text(OUTPUT_COLOR), Some("never"));
    let unset = UnsetCommand::new(&registry(), OUTPUT_COLOR, Place::User)
        .expect("valid")
        .command;
    assert!(
        unset_for_user(&files, &unset)
            .expect("removed")
            .message
            .starts_with("Removed")
    );
    assert_eq!(settings(&files).text(OUTPUT_COLOR), Some("auto"));
    assert!(
        unset_for_user(&files, &unset)
            .expect("nothing")
            .message
            .contains("was not set")
    );
}

#[test]
fn get_returns_meaning_allowed_values_default_and_the_value_with_its_source() {
    let files = MemoryFiles::new();
    let set = SetCommand::new(&registry(), OUTPUT_COLOR, "never", Place::User)
        .expect("valid")
        .command;
    set_for_user(&files, &set).expect("stored");
    let detail = get(&registry(), &settings(&files), OUTPUT_COLOR).expect("a setting");
    assert_eq!(detail.summary, "Coloured output.");
    assert_eq!(detail.allowed, "one of: auto, always, never");
    assert_eq!(detail.default, Some(ValueView::Text("auto".into())));
    assert_eq!(detail.value, Some(ValueView::Text("never".into())));
    assert!(detail.source.starts_with("user file"));
    assert!(get(&registry(), &settings(&files), "output.colour").is_err());
}

#[test]
fn list_shows_every_setting_with_its_source() {
    let listing = list(&settings(&MemoryFiles::new()));
    assert_eq!(listing.total, 16);
    assert!(listing.items.iter().all(|row| row.source == "default"));
}

#[test]
fn paths_show_no_workspace_file_outside_a_workspace() {
    let found = paths(&MemoryFiles::without_workspace());
    assert!(found.user.is_some());
    assert_eq!(found.workspace, None);
}

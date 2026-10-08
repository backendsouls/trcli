//! Unit tests for settings layering (T020).

use std::path::PathBuf;

use trcli_domain::settings::SettingValue;

use trcli_application::outcome::{Details, codes};
use trcli_application::ports::settings::{Origin, SourceError};
use trcli_application::settings::foundation::{
    self, DEFAULT_WORKSPACE, OUTPUT_COLOR, OUTPUT_PAGE_SIZE, TELEMETRY_ENABLED,
};
use trcli_application::settings::layers::resolve;
use trcli_application::settings::registry::SettingsRegistry;
use trcli_testing::settings::FixedSource;

/// The foundation's registry.
fn registry() -> SettingsRegistry {
    let mut registry = SettingsRegistry::new();
    foundation::register(&mut registry);
    registry
}

/// The user file as an origin.
fn user() -> Origin {
    Origin::UserFile(PathBuf::from("/home/ana/.config/trcli/config.toml"))
}

/// The workspace file as an origin.
fn workspace() -> Origin {
    Origin::WorkspaceFile(PathBuf::from("/work/.trcli/config.toml"))
}

#[test]
fn with_no_source_every_setting_has_its_default() {
    let resolved = resolve(&registry(), &[]).expect("valid");
    let color = resolved.settings.get(OUTPUT_COLOR).expect("registered");
    assert_eq!(color.value, Some(SettingValue::Text("auto".into())));
    assert_eq!(color.origin, Origin::Default);
    assert_eq!(
        resolved
            .settings
            .get(DEFAULT_WORKSPACE)
            .expect("registered")
            .value,
        None
    );
}

#[test]
fn precedence_is_default_user_workspace_session_command() {
    let user = FixedSource::new(user())
        .with(OUTPUT_COLOR, "never")
        .with(OUTPUT_PAGE_SIZE, "10");
    let workspace = FixedSource::new(workspace()).with(OUTPUT_PAGE_SIZE, "20");
    let session = FixedSource::new(Origin::Environment).with(OUTPUT_PAGE_SIZE, "30");
    let command = FixedSource::new(Origin::CommandLine).with(OUTPUT_PAGE_SIZE, "40");
    let layers: [&dyn trcli_application::ports::settings::SettingsSource; 4] =
        [&user, &workspace, &session, &command];
    for (count, expected) in [(1, 10), (2, 20), (3, 30), (4, 40)] {
        let resolved = resolve(&registry(), &layers[..count]).expect("valid");
        assert_eq!(resolved.settings.integer(OUTPUT_PAGE_SIZE), Some(expected));
    }
}

#[test]
fn each_value_reports_where_it_came_from() {
    let user = FixedSource::new(user()).with(OUTPUT_COLOR, "never");
    let session = FixedSource::new(Origin::Environment).with(OUTPUT_PAGE_SIZE, "30");
    let resolved = resolve(&registry(), &[&user, &session]).expect("valid");
    assert_eq!(
        resolved
            .settings
            .get(OUTPUT_COLOR)
            .expect("registered")
            .origin,
        self::user()
    );
    assert_eq!(
        resolved
            .settings
            .get(OUTPUT_PAGE_SIZE)
            .expect("registered")
            .origin,
        Origin::Environment
    );
    assert_eq!(resolved.settings.text(OUTPUT_COLOR), Some("never"));
}

#[test]
fn an_unknown_key_is_an_error_naming_the_source_and_the_key() {
    let workspace = FixedSource::new(workspace()).with("output.colour", "never");
    let problem = resolve(&registry(), &[&workspace]).expect_err("invalid");
    assert_eq!(problem.code, codes::SETTINGS_INVALID);
    let Details::Fields(faults) = problem.details else {
        panic!("fields expected")
    };
    assert!(faults[0].field.contains("output.colour"));
    assert!(faults[0].field.contains("/work/.trcli/config.toml"));
}

#[test]
fn an_invalid_value_is_an_error_that_says_what_is_allowed() {
    let user = FixedSource::new(user())
        .with(OUTPUT_PAGE_SIZE, "0")
        .with(OUTPUT_COLOR, "sometimes");
    let problem = resolve(&registry(), &[&user]).expect_err("invalid");
    assert_eq!(problem.message, "2 settings are invalid");
    let Details::Fields(faults) = problem.details else {
        panic!("fields expected")
    };
    assert_eq!(faults[0].expected, "a whole number from 1 to 1000");
    assert_eq!(faults[1].choices, ["auto", "always", "never"]);
}

#[test]
fn a_key_in_a_scope_it_does_not_have_is_ignored_with_a_warning() {
    let user = FixedSource::new(user()).with(TELEMETRY_ENABLED, "false");
    let workspace = FixedSource::new(workspace()).with(DEFAULT_WORKSPACE, "/elsewhere");
    let resolved = resolve(&registry(), &[&user, &workspace]).expect("valid");
    assert_eq!(resolved.settings.boolean(TELEMETRY_ENABLED), Some(true));
    assert_eq!(
        resolved
            .settings
            .get(DEFAULT_WORKSPACE)
            .expect("registered")
            .value,
        None
    );
    assert_eq!(resolved.warnings.len(), 2);
    assert_eq!(resolved.warnings[0].code, "setting_out_of_scope");
}

#[test]
fn an_unreadable_source_stops_everything_and_names_the_file() {
    let broken = FixedSource::failing(
        user(),
        SourceError::Unreadable {
            file: "/home/ana/.config/trcli/config.toml".into(),
            reason: "denied".into(),
        },
    );
    let problem = resolve(&registry(), &[&broken]).expect_err("unreadable");
    assert_eq!(problem.code, codes::SETTINGS_INVALID);
    assert!(
        problem
            .message
            .contains("/home/ana/.config/trcli/config.toml")
    );
}

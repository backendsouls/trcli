//! Combining the sources of settings into the values in effect (FR-040 to FR-044).
//!
//! Precedence, later wins: default ‹ user file ‹ workspace file ‹ `TRCLI_*` variables ‹
//! command-line option. The caller passes the sources in that order. Each value in effect
//! remembers where it came from, so `config list` can say (FR-041).
//!
//! The tool does not run on a partly valid configuration: an unknown key or an invalid
//! value in any source is one problem that names the source and the key (FR-043). A key
//! found in a place its scope does not allow is ignored with a warning (FR-044).

use trcli_domain::settings::{SettingDefinition, SettingKey, SettingValue};
use trcli_domain::shared::problem::{FieldProblem, Rejection, Warning};

use super::registry::SettingsRegistry;
use crate::outcome::{Details, Problem, codes};
use crate::ports::settings::{Origin, RawSetting, SettingsSource};

/// One setting as it is in effect.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EffectiveSetting {
    /// The setting.
    pub key: SettingKey,
    /// Its value; `None` when no source gives one and it has no default.
    pub value: Option<SettingValue>,
    /// Where the value comes from.
    pub origin: Origin,
}

/// Every setting in effect for one command.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Settings {
    /// One entry per registered setting, in registration order.
    values: Vec<EffectiveSetting>,
}

impl Settings {
    /// The setting in effect under this key.
    pub fn get(&self, key: &str) -> Option<&EffectiveSetting> {
        self.values
            .iter()
            .find(|setting| setting.key.as_str() == key)
    }

    /// Every setting in effect.
    pub fn all(&self) -> &[EffectiveSetting] {
        &self.values
    }

    /// The text of a text setting, when it has a value.
    pub fn text(&self, key: &str) -> Option<&str> {
        self.get(key)?.value.as_ref()?.as_text()
    }

    /// The number of a whole-number setting, when it has a value.
    pub fn integer(&self, key: &str) -> Option<i64> {
        self.get(key)?.value.as_ref()?.as_integer()
    }

    /// The answer of a yes/no setting, when it has a value.
    pub fn boolean(&self, key: &str) -> Option<bool> {
        self.get(key)?.value.as_ref()?.as_boolean()
    }
}

/// What resolving found: the settings in effect and what was ignored on the way.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Resolved {
    /// The settings in effect.
    pub settings: Settings,
    /// Keys ignored because they were found where their scope does not allow them.
    pub warnings: Vec<Warning>,
}

/// Combines the sources, given in order of increasing precedence, into the settings in
/// effect. Any invalid source stops everything with one problem listing every fault.
pub fn resolve(
    registry: &SettingsRegistry,
    sources: &[&dyn SettingsSource],
) -> Result<Resolved, Problem> {
    let mut values: Vec<EffectiveSetting> = registry.all().iter().map(default_of).collect();
    let mut faults = Vec::new();
    let mut warnings = Vec::new();
    for source in sources {
        let origin = source.origin();
        let raw = source
            .load()
            .map_err(|error| Problem::new(codes::SETTINGS_INVALID, error.to_string()))?;
        for setting in raw {
            match accept(registry, &origin, &setting) {
                Accepted::Value(key, value) => apply(&mut values, &key, value, &origin),
                Accepted::Ignored(warning) => warnings.push(warning),
                Accepted::Fault(fault) => faults.push(fault),
            }
        }
    }
    if faults.is_empty() {
        Ok(Resolved {
            settings: Settings { values },
            warnings,
        })
    } else {
        Err(invalid(faults))
    }
}

/// A setting at its default.
fn default_of(definition: &SettingDefinition) -> EffectiveSetting {
    EffectiveSetting {
        key: definition.key.clone(),
        value: definition.default.clone(),
        origin: Origin::Default,
    }
}

/// What became of one raw value.
enum Accepted {
    /// It is valid: use it.
    Value(SettingKey, SettingValue),
    /// It is in a place its scope does not allow: ignore it and say so.
    Ignored(Warning),
    /// It is unknown or invalid: the configuration cannot be used.
    Fault(FieldProblem),
}

/// Checks one raw value from one source against the registry.
fn accept(registry: &SettingsRegistry, origin: &Origin, raw: &RawSetting) -> Accepted {
    let field = format!("{} in {}", raw.key, origin.describe());
    let Some(definition) = registry.find(&raw.key) else {
        let rejection = Rejection::new(
            "is not a setting",
            "a setting listed by `trcli config list`",
        );
        return Accepted::Fault(FieldProblem::new(field, raw.value.to_string(), rejection));
    };
    if let Some(place) = origin.place()
        && !definition.scope.allows(place)
    {
        let message = format!(
            "setting `{}` in {} is ignored: it can only be set for a {}",
            raw.key,
            origin.describe(),
            definition.scope.as_str()
        );
        return Accepted::Ignored(
            Warning::new("setting_out_of_scope", message).about(raw.key.clone()),
        );
    }
    match definition.kind.accept(&raw.value) {
        Ok(value) => Accepted::Value(definition.key.clone(), value),
        Err(rejection) => {
            Accepted::Fault(FieldProblem::new(field, raw.value.to_string(), rejection))
        }
    }
}

/// Puts a valid value in effect, replacing what an earlier source gave.
fn apply(values: &mut [EffectiveSetting], key: &SettingKey, value: SettingValue, origin: &Origin) {
    if let Some(setting) = values.iter_mut().find(|setting| &setting.key == key) {
        setting.value = Some(value);
        setting.origin = origin.clone();
    }
}

/// The one problem that reports every fault of the configuration.
fn invalid(faults: Vec<FieldProblem>) -> Problem {
    let message = match faults.len() {
        1 => "1 setting is invalid".to_owned(),
        count => format!("{count} settings are invalid"),
    };
    Problem {
        details: Details::Fields(faults),
        ..Problem::new(codes::SETTINGS_INVALID, message)
    }
    .with_next_step("correct or remove the setting; `trcli config path` shows the files in use")
}

#[cfg(test)]
mod tests {
    //! Unit tests for settings layering (T020).

    use std::path::PathBuf;

    use trcli_domain::settings::SettingValue;

    use super::resolve;
    use crate::outcome::{Details, codes};
    use crate::ports::settings::{Origin, SourceError};
    use crate::settings::foundation::{
        self, DEFAULT_WORKSPACE, OUTPUT_COLOR, OUTPUT_PAGE_SIZE, TELEMETRY_ENABLED,
    };
    use crate::settings::registry::SettingsRegistry;
    use crate::testing::settings::FixedSource;

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
        let layers: [&dyn crate::ports::settings::SettingsSource; 4] =
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
}

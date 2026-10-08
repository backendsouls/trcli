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

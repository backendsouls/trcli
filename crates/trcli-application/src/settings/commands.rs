//! The use cases behind `trcli config`: list, get, set, unset, path (FR-039, FR-041).
//!
//! `set` and `unset` check the key and the value against the registry before anything is
//! written, write to one file only, and — when that file is the workspace's — record a
//! `setting` entry in the audit trail in the same unit of work.

use serde::Serialize;
use trcli_domain::governance::audit::{AuditAction, AuditDraft, Change, Stamp};
use trcli_domain::settings::{Place, SettingDefinition, SettingKey, SettingValue};
use trcli_domain::shared::problem::Rejection;

use super::layers::Settings;
use super::registry::SettingsRegistry;
use crate::outcome::{Problem, codes};
use crate::ports::audit::AuditLog;
use crate::ports::settings::{SettingsFiles, SourceError};
use crate::validation::{Checker, Valid};
use crate::view::Done;

/// A setting's value in a view: typed, so the structured form keeps numbers as numbers.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(untagged)]
pub enum ValueView {
    /// Text.
    Text(String),
    /// A whole number.
    Integer(i64),
    /// Yes or no.
    Boolean(bool),
}

impl From<&SettingValue> for ValueView {
    fn from(value: &SettingValue) -> Self {
        match value {
            SettingValue::Text(text) => Self::Text(text.clone()),
            SettingValue::Integer(number) => Self::Integer(*number),
            SettingValue::Boolean(answer) => Self::Boolean(*answer),
        }
    }
}

impl std::fmt::Display for ValueView {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Text(text) => formatter.write_str(text),
            Self::Integer(number) => write!(formatter, "{number}"),
            Self::Boolean(answer) => write!(formatter, "{answer}"),
        }
    }
}

/// One setting in effect, as `config list` shows it.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct SettingRow {
    /// The setting's name.
    pub key: String,
    /// Its value in effect; `null` when unset.
    pub value: Option<ValueView>,
    /// Where the value comes from: `default`, `user`, `workspace`, `environment`, `command_line`.
    pub source: String,
    /// The same, spelled out with the file's path.
    pub source_detail: String,
}

/// Every setting in effect.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct SettingsList {
    /// One row per setting.
    pub items: Vec<SettingRow>,
    /// How many settings there are.
    pub total: u64,
}

/// Lists every setting with its value in effect and where that value comes from (FR-041).
pub fn list(settings: &Settings) -> SettingsList {
    let items: Vec<SettingRow> = settings
        .all()
        .iter()
        .map(|setting| SettingRow {
            key: setting.key.to_string(),
            value: setting.value.as_ref().map(ValueView::from),
            source: setting.origin.name().to_owned(),
            source_detail: setting.origin.describe(),
        })
        .collect();
    SettingsList {
        total: items.len() as u64,
        items,
    }
}

/// Everything about one setting, as `config get` shows it.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct SettingDetail {
    /// The setting's name.
    pub key: String,
    /// What it is for.
    pub summary: String,
    /// What values it allows.
    pub allowed: String,
    /// Its default; `null` when it has none.
    pub default: Option<ValueView>,
    /// Where it may be set: `user`, `workspace`, or `both`.
    pub scope: String,
    /// Its value in effect.
    pub value: Option<ValueView>,
    /// Where that value comes from.
    pub source: String,
}

/// The problem for a key that is not a registered setting, listing close names.
fn unknown_key(registry: &SettingsRegistry, key: &str) -> Problem {
    let section = key.split('.').next().unwrap_or(key);
    let close: Vec<&str> = registry
        .all()
        .iter()
        .map(|definition| definition.key.as_str())
        .filter(|name| name.starts_with(section))
        .collect();
    let rejection = Rejection::new(
        "is not a setting",
        "a setting listed by `trcli config list`",
    );
    let rejection = if close.is_empty() {
        rejection
    } else {
        rejection.with_choices(close)
    };
    let mut checker = Checker::new();
    checker.reject("<key>", key, rejection);
    checker.finish(|| ()).expect_err("a problem was recorded")
}

/// The definition of a key, or the problem that says it is not a setting.
fn definition<'a>(
    registry: &'a SettingsRegistry,
    key: &str,
) -> Result<&'a SettingDefinition, Problem> {
    registry.find(key).ok_or_else(|| unknown_key(registry, key))
}

/// Describes one setting: its meaning, allowed values, default, and value in effect.
pub fn get(
    registry: &SettingsRegistry,
    settings: &Settings,
    key: &str,
) -> Result<SettingDetail, Problem> {
    let definition = definition(registry, key)?;
    let effective = settings.get(key);
    Ok(SettingDetail {
        key: key.to_owned(),
        summary: definition.summary.to_owned(),
        allowed: definition.kind.describe(),
        default: definition.default.as_ref().map(ValueView::from),
        scope: definition.scope.as_str().to_owned(),
        value: effective
            .and_then(|setting| setting.value.as_ref())
            .map(ValueView::from),
        source: effective.map_or_else(|| "default".to_owned(), |setting| setting.origin.describe()),
    })
}

/// The name of a place as the researcher knows it.
fn place_name(place: Place) -> &'static str {
    match place {
        Place::User => "your user settings",
        Place::Workspace => "this workspace",
    }
}

/// Checks that a setting may be stored in a place; the rejection says where it may.
fn check_scope(definition: &SettingDefinition, place: Place) -> Result<(), Rejection> {
    if definition.scope.allows(place) {
        return Ok(());
    }
    let hint = match place {
        Place::User => "set it without --user",
        Place::Workspace => "set it with --user",
    };
    Err(Rejection::new(
        format!("can only be set for a {}", definition.scope.as_str()),
        format!(
            "a setting that can be stored in {}; {hint}",
            place_name(place)
        ),
    ))
}

/// A checked request to store one value in one file.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SetCommand {
    /// The setting.
    pub key: SettingKey,
    /// The value, in the form its kind requires.
    pub value: SettingValue,
    /// The file to write to.
    pub place: Place,
}

impl SetCommand {
    /// Checks the key, the value, and the place together (FR-043, FR-044).
    pub fn new(
        registry: &SettingsRegistry,
        key: &str,
        value: &str,
        place: Place,
    ) -> Result<Valid<Self>, Problem> {
        let definition = definition(registry, key)?;
        let mut checker = Checker::new();
        let checked = checker.check("<value>", value, |value| definition.kind.parse(value));
        if let Err(rejection) = check_scope(definition, place) {
            checker.reject("<key>", key, rejection);
        }
        checker.finish(|| Self {
            key: definition.key.clone(),
            value: checked.expect("checked"),
            place,
        })
    }
}

/// A checked request to remove one value from one file.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UnsetCommand {
    /// The setting.
    pub key: SettingKey,
    /// The file to remove it from.
    pub place: Place,
}

impl UnsetCommand {
    /// Checks that the key is a setting.
    pub fn new(
        registry: &SettingsRegistry,
        key: &str,
        place: Place,
    ) -> Result<Valid<Self>, Problem> {
        let definition = definition(registry, key)?;
        Checker::new().finish(|| Self {
            key: definition.key.clone(),
            place,
        })
    }
}

/// Turns a failure to write a settings file into a problem.
fn unwritable(error: SourceError) -> Problem {
    Problem::new(codes::OPERATION_FAILED, error.to_string())
}

/// Stores a value in the researcher's own file. Nothing is recorded in a workspace's
/// audit trail: the change is not about any workspace.
pub fn set_for_user(files: &impl SettingsFiles, command: &SetCommand) -> Result<Done, Problem> {
    files
        .store(Place::User, &command.key, &command.value)
        .map_err(unwritable)?;
    Ok(Done::new(format!(
        "Set {} = {} in {}",
        command.key,
        command.value,
        place_name(Place::User)
    )))
}

/// Stores a value in the workspace's file and records it in the audit trail.
pub async fn set_for_workspace<U: AuditLog>(
    unit: &mut U,
    stamp: &Stamp,
    files: &impl SettingsFiles,
    settings: &Settings,
    command: &SetCommand,
) -> Result<Done, Problem> {
    let before = settings
        .get(command.key.as_str())
        .and_then(|setting| setting.value.as_ref())
        .map(ToString::to_string);
    let after = command.value.to_string();
    let change = Change::new(command.key.as_str(), before.as_deref(), Some(&after));
    let draft = AuditDraft::workspace(AuditAction::SETTING)
        .named(command.key.as_str())
        .with_changes(vec![change]);
    unit.record(stamp, draft).await?;
    // The file is written after the entry is recorded and before the caller commits, so a
    // failure to write leaves no entry behind.
    files
        .store(Place::Workspace, &command.key, &command.value)
        .map_err(unwritable)?;
    Ok(Done::new(format!(
        "Set {} = {} in {}",
        command.key,
        command.value,
        place_name(Place::Workspace)
    )))
}

/// Removes a value from the researcher's own file; the next source applies again.
pub fn unset_for_user(files: &impl SettingsFiles, command: &UnsetCommand) -> Result<Done, Problem> {
    let removed = files
        .remove(Place::User, &command.key)
        .map_err(unwritable)?;
    Ok(Done::new(unset_message(&command.key, Place::User, removed)))
}

/// Removes a value from the workspace's file and, when there was one, records it.
pub async fn unset_for_workspace<U: AuditLog>(
    unit: &mut U,
    stamp: &Stamp,
    files: &impl SettingsFiles,
    command: &UnsetCommand,
) -> Result<Done, Problem> {
    let removed = files
        .remove(Place::Workspace, &command.key)
        .map_err(unwritable)?;
    if removed {
        let change = Change::new(command.key.as_str(), Some("(set)"), None);
        let draft = AuditDraft::workspace(AuditAction::SETTING)
            .named(command.key.as_str())
            .with_changes(vec![change]);
        unit.record(stamp, draft).await?;
    }
    Ok(Done::new(unset_message(
        &command.key,
        Place::Workspace,
        removed,
    )))
}

/// What `unset` says it did.
fn unset_message(key: &SettingKey, place: Place, removed: bool) -> String {
    if removed {
        format!("Removed {key} from {}", place_name(place))
    } else {
        format!(
            "{key} was not set in {}; nothing to remove",
            place_name(place)
        )
    }
}

/// The settings files in use.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct SettingsPaths {
    /// The researcher's own file.
    pub user: Option<String>,
    /// The workspace's file; `null` outside a workspace.
    pub workspace: Option<String>,
}

/// Says where the settings files are.
pub fn paths(files: &impl SettingsFiles) -> SettingsPaths {
    let path = |place| files.path(place).map(|path| path.display().to_string());
    SettingsPaths {
        user: path(Place::User),
        workspace: path(Place::Workspace),
    }
}

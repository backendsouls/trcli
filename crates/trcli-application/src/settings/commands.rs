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

#[cfg(test)]
mod tests {
    //! Unit tests for the settings use cases (T097).

    use trcli_domain::settings::{Place, SettingValue};

    use super::{
        SetCommand, UnsetCommand, ValueView, get, list, paths, set_for_user, set_for_workspace,
        unset_for_user,
    };
    use crate::outcome::{Details, codes};
    use crate::ports::unit_of_work::Storage;
    use crate::settings::foundation::{self, DEFAULT_WORKSPACE, OUTPUT_COLOR, OUTPUT_PAGE_SIZE};
    use crate::settings::layers::{Settings, resolve};
    use crate::settings::registry::SettingsRegistry;
    use crate::testing::block_on;
    use crate::testing::environment::stamp;
    use crate::testing::settings::MemoryFiles;
    use crate::testing::unit::FakeStorage;

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
        let invalid = SetCommand::new(&registry(), OUTPUT_PAGE_SIZE, "0", Place::Workspace)
            .expect_err("invalid");
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
        let problem = SetCommand::new(&registry(), DEFAULT_WORKSPACE, "/w", Place::Workspace)
            .expect_err("scope");
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
            crate::ports::unit_of_work::UnitOfWork::commit(unit)
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
                set_for_workspace(&mut unit, &stamp(), &files, &Settings::default(), &command)
                    .await;
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
}

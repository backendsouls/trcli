//! Settings on disk and in the session (FR-039, FR-040, FR-043).
//!
//! Settings files are TOML. A key such as `output.color` is the entry `color` of the
//! table `output`; a file may write it either way. A missing or empty file holds no
//! value; a file that exists and cannot be read, or is not TOML, is an error that names
//! the file. Writing one key keeps every other key, and the file is replaced atomically
//! (comments in the file are not kept).
//!
//! The session's source reads `TRCLI_<SECTION>_<KEY>` variables for the keys it is given.

use std::fs;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};

use toml::{Table, Value};
use trcli_application::ports::settings::{
    Origin, RawSetting, SettingsFiles, SettingsSource, SourceError,
};
use trcli_domain::settings::{Place, SettingKey, SettingValue};

use crate::atomic::write_atomically;

/// Reads a settings file as a TOML table; a missing file is an empty table.
fn read_table(path: &Path) -> Result<Table, SourceError> {
    let file = || path.display().to_string();
    let text = match fs::read_to_string(path) {
        Ok(text) => text,
        Err(error) if error.kind() == ErrorKind::NotFound => return Ok(Table::new()),
        Err(error) => {
            return Err(SourceError::Unreadable {
                file: file(),
                reason: error.to_string(),
            });
        }
    };
    text.parse::<Table>()
        .map_err(|error| SourceError::Malformed {
            file: file(),
            reason: error.message().to_owned(),
        })
}

/// Turns nested tables into dotted keys: `[output] color = "never"` is `output.color`.
fn flatten(
    prefix: &str,
    table: &Table,
    file: &Path,
    settings: &mut Vec<RawSetting>,
) -> Result<(), SourceError> {
    for (name, value) in table {
        let key = if prefix.is_empty() {
            name.clone()
        } else {
            format!("{prefix}.{name}")
        };
        let value = match value {
            Value::Table(inner) => {
                flatten(&key, inner, file, settings)?;
                continue;
            }
            Value::String(text) => SettingValue::Text(text.clone()),
            Value::Integer(number) => SettingValue::Integer(*number),
            Value::Boolean(answer) => SettingValue::Boolean(*answer),
            other => {
                let reason = format!("`{key}` is {}, which no setting takes", other.type_str());
                return Err(SourceError::Malformed {
                    file: file.display().to_string(),
                    reason,
                });
            }
        };
        settings.push(RawSetting { key, value });
    }
    Ok(())
}

/// A settings file as a source of settings.
#[derive(Clone, Debug)]
pub struct TomlFileSource {
    /// Which of the two files this is, with its path.
    origin: Origin,
    /// The file.
    path: PathBuf,
}

impl TomlFileSource {
    /// The researcher's own file.
    pub fn user(path: PathBuf) -> Self {
        Self {
            origin: Origin::UserFile(path.clone()),
            path,
        }
    }

    /// A workspace's file.
    pub fn workspace(path: PathBuf) -> Self {
        Self {
            origin: Origin::WorkspaceFile(path.clone()),
            path,
        }
    }
}

impl SettingsSource for TomlFileSource {
    fn origin(&self) -> Origin {
        self.origin.clone()
    }

    fn load(&self) -> Result<Vec<RawSetting>, SourceError> {
        let mut settings = Vec::new();
        flatten("", &read_table(&self.path)?, &self.path, &mut settings)?;
        Ok(settings)
    }
}

/// The session's variables as a source of settings.
#[derive(Clone, Debug)]
pub struct EnvironmentSource {
    /// The values found, by key.
    found: Vec<RawSetting>,
}

impl EnvironmentSource {
    /// Reads the variables that override the given keys from the process's environment.
    pub fn new(keys: &[SettingKey]) -> Self {
        Self::from_environment(keys, |name| std::env::var(name).ok())
    }

    /// As [`EnvironmentSource::new`], given a way to read environment variables.
    pub fn from_environment(
        keys: &[SettingKey],
        variable: impl Fn(&str) -> Option<String>,
    ) -> Self {
        let found = keys
            .iter()
            .filter_map(|key| {
                let value = variable(&key.variable_name())?;
                Some(RawSetting {
                    key: key.to_string(),
                    value: SettingValue::Text(value),
                })
            })
            .collect();
        Self { found }
    }
}

impl SettingsSource for EnvironmentSource {
    fn origin(&self) -> Origin {
        Origin::Environment
    }

    fn load(&self) -> Result<Vec<RawSetting>, SourceError> {
        Ok(self.found.clone())
    }
}

/// Options of one command as a source of settings: the last word (FR-040).
#[derive(Clone, Debug, Default)]
pub struct CommandLineSource {
    /// The values given as options.
    given: Vec<RawSetting>,
}

impl CommandLineSource {
    /// No option yet.
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds the value of an option that overrides a setting, when the option was given.
    #[must_use]
    pub fn with(mut self, key: &str, value: Option<&str>) -> Self {
        if let Some(value) = value {
            self.given.push(RawSetting {
                key: key.to_owned(),
                value: SettingValue::Text(value.to_owned()),
            });
        }
        self
    }
}

impl SettingsSource for CommandLineSource {
    fn origin(&self) -> Origin {
        Origin::CommandLine
    }

    fn load(&self) -> Result<Vec<RawSetting>, SourceError> {
        Ok(self.given.clone())
    }
}

/// The two settings files a researcher can write to.
#[derive(Clone, Debug)]
pub struct FileSettings {
    /// The researcher's own file, when the system says where it is.
    user: Option<PathBuf>,
    /// The workspace's file, inside a workspace.
    workspace: Option<PathBuf>,
}

impl FileSettings {
    /// The files at these paths.
    pub fn new(user: Option<PathBuf>, workspace: Option<PathBuf>) -> Self {
        Self { user, workspace }
    }

    /// The file of a place, or the error that says there is none.
    fn file(&self, place: Place) -> Result<&Path, SourceError> {
        self.path_of(place).ok_or_else(|| SourceError::Unwritable {
            file: "the settings file".to_owned(),
            reason: "its location is not known on this system".to_owned(),
        })
    }

    /// The file of a place.
    fn path_of(&self, place: Place) -> Option<&Path> {
        match place {
            Place::User => self.user.as_deref(),
            Place::Workspace => self.workspace.as_deref(),
        }
    }

    /// Replaces a file with a table, atomically, creating its directory when needed.
    fn write_table(path: &Path, table: &Table) -> Result<(), SourceError> {
        let unwritable = |reason: String| SourceError::Unwritable {
            file: path.display().to_string(),
            reason,
        };
        if let Some(directory) = path.parent() {
            fs::create_dir_all(directory).map_err(|error| unwritable(error.to_string()))?;
        }
        let text = toml::to_string(table).map_err(|error| unwritable(error.to_string()))?;
        write_atomically(path, text.as_bytes()).map_err(|error| unwritable(error.to_string()))
    }
}

/// The table that holds a dotted key's last segment, created on the way when `create`.
fn parent_table<'a>(
    table: &'a mut Table,
    segments: &[&str],
    create: bool,
) -> Option<&'a mut Table> {
    let mut current = table;
    for segment in segments {
        if create && !current.get(*segment).is_some_and(Value::is_table) {
            current.insert((*segment).to_owned(), Value::Table(Table::new()));
        }
        current = current.get_mut(*segment)?.as_table_mut()?;
    }
    Some(current)
}

/// A setting's value as TOML: numbers stay numbers, yes/no stays yes/no.
fn to_toml(value: &SettingValue) -> Value {
    match value {
        SettingValue::Text(text) => Value::String(text.clone()),
        SettingValue::Integer(number) => Value::Integer(*number),
        SettingValue::Boolean(answer) => Value::Boolean(*answer),
    }
}

impl SettingsFiles for FileSettings {
    fn path(&self, place: Place) -> Option<PathBuf> {
        self.path_of(place).map(Path::to_path_buf)
    }

    fn store(
        &self,
        place: Place,
        key: &SettingKey,
        value: &SettingValue,
    ) -> Result<(), SourceError> {
        let path = self.file(place)?;
        let mut table = read_table(path)?;
        let segments: Vec<&str> = key.as_str().split('.').collect();
        let (last, parents) = segments
            .split_last()
            .expect("a key has at least one segment");
        let parent = parent_table(&mut table, parents, true).expect("created on the way");
        parent.insert((*last).to_owned(), to_toml(value));
        Self::write_table(path, &table)
    }

    fn remove(&self, place: Place, key: &SettingKey) -> Result<bool, SourceError> {
        let path = self.file(place)?;
        let mut table = read_table(path)?;
        let segments: Vec<&str> = key.as_str().split('.').collect();
        let (last, parents) = segments
            .split_last()
            .expect("a key has at least one segment");
        let removed = parent_table(&mut table, parents, false)
            .and_then(|parent| parent.remove(*last))
            .is_some();
        if removed {
            Self::write_table(path, &table)?;
        }
        Ok(removed)
    }
}

#[cfg(test)]
mod tests {
    //! Unit tests for the settings files and the session's variables (T021).

    use std::fs;

    use trcli_application::ports::settings::{
        Origin, RawSetting, SettingsFiles, SettingsSource, SourceError,
    };
    use trcli_domain::settings::{Place, SettingKey, SettingValue};

    use super::{CommandLineSource, EnvironmentSource, FileSettings, TomlFileSource};

    /// A setting key.
    fn key(name: &str) -> SettingKey {
        SettingKey::new(name).expect("a valid key")
    }

    /// A raw setting.
    fn raw(key: &str, value: SettingValue) -> RawSetting {
        RawSetting {
            key: key.to_owned(),
            value,
        }
    }

    #[test]
    fn a_missing_or_empty_file_yields_no_values() {
        let directory = tempfile::tempdir().expect("a temporary directory");
        let path = directory.path().join("config.toml");
        assert_eq!(TomlFileSource::user(path.clone()).load(), Ok(Vec::new()));
        fs::write(&path, "# only a comment\n").expect("written");
        assert_eq!(TomlFileSource::user(path).load(), Ok(Vec::new()));
    }

    #[test]
    fn tables_and_dotted_keys_both_give_dotted_settings_with_their_types() {
        let directory = tempfile::tempdir().expect("a temporary directory");
        let path = directory.path().join("config.toml");
        fs::write(
            &path,
            "default_workspace = \"/w\"\noutput.page_size = 20\n[telemetry]\nenabled = false\n",
        )
        .expect("written");
        let source = TomlFileSource::workspace(path.clone());
        assert_eq!(source.origin(), Origin::WorkspaceFile(path));
        let mut loaded = source.load().expect("loaded");
        loaded.sort_by(|one, other| one.key.cmp(&other.key));
        assert_eq!(
            loaded,
            vec![
                raw("default_workspace", SettingValue::Text("/w".into())),
                raw("output.page_size", SettingValue::Integer(20)),
                raw("telemetry.enabled", SettingValue::Boolean(false)),
            ]
        );
    }

    #[test]
    fn a_file_that_is_not_toml_or_holds_what_no_setting_takes_names_the_file() {
        let directory = tempfile::tempdir().expect("a temporary directory");
        let path = directory.path().join("config.toml");
        for content in ["this is = not [toml", "output.color = [1, 2]\n"] {
            fs::write(&path, content).expect("written");
            match TomlFileSource::user(path.clone()).load() {
                Err(SourceError::Malformed { file, .. }) => {
                    assert_eq!(file, path.display().to_string())
                }
                other => panic!("expected a malformed file, got {other:?}"),
            }
        }
    }

    #[test]
    fn an_unreadable_file_is_an_error_naming_the_file() {
        let directory = tempfile::tempdir().expect("a temporary directory");
        // A directory where the file should be cannot be read as a file on any system.
        let path = directory.path().join("config.toml");
        fs::create_dir(&path).expect("directory");
        match TomlFileSource::user(path.clone()).load() {
            Err(SourceError::Unreadable { file, .. }) => {
                assert_eq!(file, path.display().to_string())
            }
            other => panic!("expected an unreadable file, got {other:?}"),
        }
    }

    #[test]
    fn writing_one_key_preserves_every_other_key() {
        let directory = tempfile::tempdir().expect("a temporary directory");
        let path = directory.path().join("nested").join("config.toml");
        let files = FileSettings::new(Some(path.clone()), None);
        files
            .store(
                Place::User,
                &key("output.color"),
                &SettingValue::Text("never".into()),
            )
            .expect("stored");
        files
            .store(
                Place::User,
                &key("output.page_size"),
                &SettingValue::Integer(20),
            )
            .expect("stored");
        files
            .store(
                Place::User,
                &key("default_workspace"),
                &SettingValue::Text("/w".into()),
            )
            .expect("stored");
        files
            .store(
                Place::User,
                &key("output.color"),
                &SettingValue::Text("always".into()),
            )
            .expect("stored");
        let mut loaded = TomlFileSource::user(path.clone()).load().expect("loaded");
        loaded.sort_by(|one, other| one.key.cmp(&other.key));
        assert_eq!(
            loaded,
            vec![
                raw("default_workspace", SettingValue::Text("/w".into())),
                raw("output.color", SettingValue::Text("always".into())),
                raw("output.page_size", SettingValue::Integer(20)),
            ]
        );
        let beside: Vec<_> = fs::read_dir(path.parent().expect("a directory"))
            .expect("list")
            .collect();
        assert_eq!(beside.len(), 1, "no half-written or temporary file is left");
    }

    #[test]
    fn removing_a_key_says_whether_it_was_there_and_keeps_the_rest() {
        let directory = tempfile::tempdir().expect("a temporary directory");
        let path = directory.path().join("config.toml");
        let files = FileSettings::new(None, Some(path.clone()));
        files
            .store(
                Place::Workspace,
                &key("output.color"),
                &SettingValue::Text("never".into()),
            )
            .expect("stored");
        files
            .store(
                Place::Workspace,
                &key("output.page_size"),
                &SettingValue::Integer(20),
            )
            .expect("stored");
        assert_eq!(
            files.remove(Place::Workspace, &key("output.color")),
            Ok(true)
        );
        assert_eq!(
            files.remove(Place::Workspace, &key("output.color")),
            Ok(false)
        );
        assert_eq!(
            TomlFileSource::workspace(path).load(),
            Ok(vec![raw("output.page_size", SettingValue::Integer(20))])
        );
    }

    #[test]
    fn a_place_without_a_file_cannot_be_written_to() {
        let files = FileSettings::new(None, None);
        assert_eq!(files.path(Place::Workspace), None);
        let refused = files.store(
            Place::Workspace,
            &key("output.color"),
            &SettingValue::Text("never".into()),
        );
        assert!(matches!(refused, Err(SourceError::Unwritable { .. })));
    }

    #[test]
    fn session_variables_override_the_keys_they_are_named_after() {
        let keys = [
            key("output.color"),
            key("storage.busy_timeout_ms"),
            key("output.format"),
        ];
        let variable = |name: &str| match name {
            "TRCLI_OUTPUT_COLOR" => Some("never".to_owned()),
            "TRCLI_STORAGE_BUSY_TIMEOUT_MS" => Some("100".to_owned()),
            _ => None,
        };
        let source = EnvironmentSource::from_environment(&keys, variable);
        assert_eq!(source.origin(), Origin::Environment);
        assert_eq!(
            source.load(),
            Ok(vec![
                raw("output.color", SettingValue::Text("never".into())),
                raw("storage.busy_timeout_ms", SettingValue::Text("100".into())),
            ])
        );
    }

    #[test]
    fn options_of_the_command_are_a_source_of_their_own() {
        let source = CommandLineSource::new()
            .with("output.color", Some("always"))
            .with("output.format", None);
        assert_eq!(source.origin(), Origin::CommandLine);
        assert_eq!(
            source.load(),
            Ok(vec![raw(
                "output.color",
                SettingValue::Text("always".into())
            )])
        );
    }
}

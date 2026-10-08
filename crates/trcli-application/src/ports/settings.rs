//! Ports for where settings come from and where they are stored (FR-039, FR-040).

use std::path::PathBuf;

use trcli_domain::settings::{Place, SettingKey, SettingValue};

/// Where a value in effect comes from; shown by `config list` (FR-041).
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Origin {
    /// The setting's built-in default.
    Default,
    /// The researcher's own settings file.
    UserFile(PathBuf),
    /// The workspace's settings file.
    WorkspaceFile(PathBuf),
    /// A `TRCLI_*` variable of the session.
    Environment,
    /// An option of this command.
    CommandLine,
}

impl Origin {
    /// The place this origin stores settings in, when it is a file.
    pub fn place(&self) -> Option<Place> {
        match self {
            Self::UserFile(_) => Some(Place::User),
            Self::WorkspaceFile(_) => Some(Place::Workspace),
            Self::Default | Self::Environment | Self::CommandLine => None,
        }
    }

    /// The origin as shown to the researcher.
    pub fn describe(&self) -> String {
        match self {
            Self::Default => "default".to_owned(),
            Self::UserFile(path) => format!("user file {}", path.display()),
            Self::WorkspaceFile(path) => format!("workspace file {}", path.display()),
            Self::Environment => "environment".to_owned(),
            Self::CommandLine => "command line".to_owned(),
        }
    }

    /// A one-word name for the structured form.
    pub fn name(&self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::UserFile(_) => "user",
            Self::WorkspaceFile(_) => "workspace",
            Self::Environment => "environment",
            Self::CommandLine => "command_line",
        }
    }
}

/// One value as a source gives it, before it is checked against its definition.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RawSetting {
    /// The key as written in the source.
    pub key: String,
    /// The value as written.
    pub value: SettingValue,
}

/// Why a source of settings could not be used.
#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]
pub enum SourceError {
    /// The file exists and cannot be read.
    #[error("the settings file {file} cannot be read: {reason}")]
    Unreadable {
        /// The file.
        file: String,
        /// Why.
        reason: String,
    },
    /// The file is not valid TOML, or holds something that cannot be a setting.
    #[error("the settings file {file} is not valid: {reason}")]
    Malformed {
        /// The file.
        file: String,
        /// Why.
        reason: String,
    },
    /// The file could not be written.
    #[error("the settings file {file} cannot be written: {reason}")]
    Unwritable {
        /// The file.
        file: String,
        /// Why.
        reason: String,
    },
}

/// One place settings are read from.
pub trait SettingsSource {
    /// Which place this is.
    fn origin(&self) -> Origin;

    /// The values this source holds. A missing or empty file holds none; an unreadable one
    /// is an error that names the file.
    fn load(&self) -> Result<Vec<RawSetting>, SourceError>;
}

/// The two settings files a researcher can write to.
pub trait SettingsFiles {
    /// Where the file of a place is; `None` when there is no such file (no workspace).
    fn path(&self, place: Place) -> Option<PathBuf>;

    /// Stores one value, keeping every other value in the file.
    fn store(
        &self,
        place: Place,
        key: &SettingKey,
        value: &SettingValue,
    ) -> Result<(), SourceError>;

    /// Removes one value; says whether there was one to remove.
    fn remove(&self, place: Place, key: &SettingKey) -> Result<bool, SourceError>;
}

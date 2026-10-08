//! Fakes of the sources and files of settings.

use std::cell::RefCell;
use std::collections::BTreeMap;
use std::path::PathBuf;

use trcli_domain::settings::{Place, SettingKey, SettingValue};

use trcli_application::ports::settings::{
    Origin, RawSetting, SettingsFiles, SettingsSource, SourceError,
};

/// A source that holds what it was given.
#[derive(Clone, Debug)]
pub struct FixedSource {
    /// Which place this source stands for.
    origin: Origin,
    /// What loading it gives.
    content: Result<Vec<RawSetting>, SourceError>,
}

impl FixedSource {
    /// An empty source standing for `origin`.
    pub fn new(origin: Origin) -> Self {
        Self {
            origin,
            content: Ok(Vec::new()),
        }
    }

    /// A source that cannot be loaded.
    pub fn failing(origin: Origin, error: SourceError) -> Self {
        Self {
            origin,
            content: Err(error),
        }
    }

    /// Adds a value, given as text.
    #[must_use]
    pub fn with(mut self, key: &str, value: &str) -> Self {
        if let Ok(content) = &mut self.content {
            content.push(RawSetting {
                key: key.to_owned(),
                value: SettingValue::Text(value.to_owned()),
            });
        }
        self
    }
}

impl SettingsSource for FixedSource {
    fn origin(&self) -> Origin {
        self.origin.clone()
    }

    fn load(&self) -> Result<Vec<RawSetting>, SourceError> {
        self.content.clone()
    }
}

/// The two settings files, kept in memory.
#[derive(Debug, Default)]
pub struct MemoryFiles {
    /// The content of each file, by place.
    files: RefCell<BTreeMap<&'static str, BTreeMap<String, SettingValue>>>,
    /// Whether there is no workspace, and so no workspace file.
    without_workspace: bool,
    /// Whether writing fails, as on a read-only disk.
    read_only: bool,
}

impl MemoryFiles {
    /// Two empty files.
    pub fn new() -> Self {
        Self::default()
    }

    /// Only the user's file exists: there is no workspace.
    pub fn without_workspace() -> Self {
        Self {
            without_workspace: true,
            ..Self::default()
        }
    }

    /// Files that cannot be written.
    pub fn read_only() -> Self {
        Self {
            read_only: true,
            ..Self::default()
        }
    }

    /// The name a place's file is kept under.
    fn name(place: Place) -> &'static str {
        match place {
            Place::User => "user",
            Place::Workspace => "workspace",
        }
    }

    /// The value a file holds for a key.
    pub fn value(&self, place: Place, key: &str) -> Option<SettingValue> {
        self.files
            .borrow()
            .get(Self::name(place))?
            .get(key)
            .cloned()
    }

    /// A file as a source of settings, for resolving.
    pub fn source(&self, place: Place) -> FixedSource {
        let origin = match place {
            Place::User => Origin::UserFile(PathBuf::from("/user/config.toml")),
            Place::Workspace => {
                Origin::WorkspaceFile(PathBuf::from("/workspace/.trcli/config.toml"))
            }
        };
        let content = self
            .files
            .borrow()
            .get(Self::name(place))
            .cloned()
            .unwrap_or_default();
        let raw = content
            .into_iter()
            .map(|(key, value)| RawSetting { key, value })
            .collect();
        FixedSource {
            origin,
            content: Ok(raw),
        }
    }

    /// The error a read-only file gives.
    fn unwritable(place: Place) -> SourceError {
        SourceError::Unwritable {
            file: Self::name(place).to_owned(),
            reason: "read-only".to_owned(),
        }
    }
}

impl SettingsFiles for MemoryFiles {
    fn path(&self, place: Place) -> Option<PathBuf> {
        match place {
            Place::User => Some(PathBuf::from("/user/config.toml")),
            Place::Workspace if self.without_workspace => None,
            Place::Workspace => Some(PathBuf::from("/workspace/.trcli/config.toml")),
        }
    }

    fn store(
        &self,
        place: Place,
        key: &SettingKey,
        value: &SettingValue,
    ) -> Result<(), SourceError> {
        if self.read_only {
            return Err(Self::unwritable(place));
        }
        let mut files = self.files.borrow_mut();
        files
            .entry(Self::name(place))
            .or_default()
            .insert(key.to_string(), value.clone());
        Ok(())
    }

    fn remove(&self, place: Place, key: &SettingKey) -> Result<bool, SourceError> {
        if self.read_only {
            return Err(Self::unwritable(place));
        }
        let mut files = self.files.borrow_mut();
        Ok(files
            .entry(Self::name(place))
            .or_default()
            .remove(key.as_str())
            .is_some())
    }
}

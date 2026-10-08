//! Fakes of the workspace's surroundings: where workspaces are, their files, opening
//! their storage, and the copy kept before an upgrade.

use std::cell::RefCell;
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use super::unit::{FakeStorage, State};
use crate::ports::unit_of_work::StoreError;
use crate::ports::workspace::{BackupCopy, StorageOpener, WorkspaceFiles, WorkspaceProbe};

/// A file system that holds workspaces in the directories it was told about.
#[derive(Clone, Debug, Default)]
pub struct FakeProbe {
    /// The directories that hold a workspace.
    roots: Vec<PathBuf>,
}

impl FakeProbe {
    /// A file system with workspaces in these directories.
    pub fn with_workspaces(roots: &[&str]) -> Self {
        Self {
            roots: roots.iter().map(PathBuf::from).collect(),
        }
    }
}

impl WorkspaceProbe for FakeProbe {
    fn holds_workspace(&self, directory: &Path) -> bool {
        self.roots.iter().any(|root| root == directory)
    }
}

/// Workspace files kept as a list of what was prepared.
#[derive(Debug, Default)]
pub struct FakeFiles {
    /// The roots prepared and not discarded.
    pub prepared: RefCell<Vec<PathBuf>>,
    /// Directories that cannot be written to.
    unwritable: Vec<PathBuf>,
}

impl FakeFiles {
    /// Every directory can be written to.
    pub fn new() -> Self {
        Self::default()
    }

    /// `directory` cannot be written to.
    pub fn with_unwritable(directory: &str) -> Self {
        Self {
            unwritable: vec![PathBuf::from(directory)],
            ..Self::default()
        }
    }
}

impl WorkspaceFiles for FakeFiles {
    fn is_writable(&self, directory: &Path) -> bool {
        !self
            .unwritable
            .iter()
            .any(|unwritable| unwritable == directory)
    }

    fn prepare(&self, root: &Path) -> Result<(), StoreError> {
        self.prepared.borrow_mut().push(root.to_path_buf());
        Ok(())
    }

    fn discard(&self, root: &Path) {
        self.prepared
            .borrow_mut()
            .retain(|prepared| prepared != root);
    }
}

/// Storage kept in memory, by the path of its database.
#[derive(Debug, Default)]
pub struct FakeOpener {
    /// The storage that exists, by path.
    databases: RefCell<BTreeMap<PathBuf, FakeStorage>>,
    /// Whether creating storage fails.
    failing: bool,
}

impl FakeOpener {
    /// No storage exists yet.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creating storage fails.
    pub fn failing() -> Self {
        Self {
            failing: true,
            ..Self::default()
        }
    }

    /// The storage at a path, if it exists.
    pub fn storage(&self, database: &Path) -> Option<FakeStorage> {
        self.databases.borrow().get(database).cloned()
    }
}

impl StorageOpener for FakeOpener {
    type Storage = FakeStorage;

    async fn create(&self, database: &Path) -> Result<FakeStorage, StoreError> {
        if self.failing {
            return Err(StoreError::Failed("the disk is full".to_owned()));
        }
        if self.databases.borrow().contains_key(database) {
            return Err(StoreError::AlreadyExists(format!(
                "a workspace in {}",
                database.display()
            )));
        }
        let storage = FakeStorage::new();
        self.databases
            .borrow_mut()
            .insert(database.to_path_buf(), storage.clone());
        Ok(storage)
    }

    async fn open(&self, database: &Path) -> Result<FakeStorage, StoreError> {
        self.storage(database).ok_or_else(|| StoreError::Damaged {
            what: "the database file is missing".to_owned(),
            location: database.display().to_string(),
        })
    }
}

/// A copy of the storage's state, kept in memory.
#[derive(Debug)]
pub struct FakeBackup {
    /// The storage being copied.
    storage: FakeStorage,
    /// The copies made, in order.
    pub copies: RefCell<Vec<State>>,
    /// How many times a copy was put back.
    pub restored: RefCell<u32>,
}

impl FakeBackup {
    /// A backup of this storage.
    pub fn of(storage: &FakeStorage) -> Self {
        Self {
            storage: storage.clone(),
            copies: RefCell::new(Vec::new()),
            restored: RefCell::new(0),
        }
    }
}

impl BackupCopy for FakeBackup {
    fn copy(&self) -> Result<PathBuf, StoreError> {
        self.copies.borrow_mut().push(self.storage.snapshot());
        Ok(PathBuf::from(format!(
            "/workspace/.trcli/backups/copy-{}.db",
            self.copies.borrow().len()
        )))
    }

    fn restore(&self, _copy: &Path) -> Result<(), StoreError> {
        let copy = self
            .copies
            .borrow()
            .last()
            .cloned()
            .ok_or_else(|| StoreError::Failed("no copy".to_owned()))?;
        self.storage.tamper(|state| *state = copy);
        *self.restored.borrow_mut() += 1;
        Ok(())
    }
}

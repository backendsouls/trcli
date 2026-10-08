//! Ports for the workspace itself: finding it, creating it, opening it, upgrading it
//! (FR-001 to FR-008).

use std::path::{Path, PathBuf};

use trcli_domain::workspace::Workspace;

use super::unit_of_work::{Storage, StoreError};

/// Tells whether a directory holds a workspace. The search order built on this is in
/// [`crate::workspace::locate`].
pub trait WorkspaceProbe {
    /// Whether `directory` itself holds a workspace (a `.trcli/` directory).
    fn holds_workspace(&self, directory: &Path) -> bool;
}

/// The files of a workspace other than its database.
pub trait WorkspaceFiles {
    /// Whether a workspace could be created in `directory`: it exists and can be written.
    fn is_writable(&self, directory: &Path) -> bool;

    /// Creates the workspace's directory with its settings file and backups directory.
    fn prepare(&self, root: &Path) -> Result<(), StoreError>;

    /// Removes what [`WorkspaceFiles::prepare`] created, when creating the workspace
    /// failed afterwards, so that nothing half-made is left behind.
    fn discard(&self, root: &Path);
}

/// Creates and opens the storage of a workspace.
pub trait StorageOpener {
    /// The storage this opener gives out.
    type Storage: Storage;

    /// Creates empty storage at `database`. Refuses with [`StoreError::AlreadyExists`]
    /// when something is there, leaving it untouched (FR-002).
    async fn create(&self, database: &Path) -> Result<Self::Storage, StoreError>;

    /// Opens existing storage. Reports [`StoreError::Damaged`], with what and where, when
    /// it is missing, cannot be opened, or lacks what a workspace must have (FR-008).
    async fn open(&self, database: &Path) -> Result<Self::Storage, StoreError>;
}

/// Reads and saves the workspace's own details.
pub trait WorkspaceStore {
    /// The workspace's details.
    async fn workspace(&self) -> Result<Workspace, StoreError>;

    /// Saves the workspace's details, creating the row the first time.
    async fn save_workspace(&mut self, workspace: &Workspace) -> Result<(), StoreError>;
}

/// Brings stored data to the current format, inside a unit of work.
pub trait SchemaUpgrade {
    /// Applies every change of format not yet applied.
    async fn apply_pending_migrations(&mut self) -> Result<(), StoreError>;
}

/// Checks that stored data is consistent.
pub trait IntegrityCheck {
    /// What the storage's own check finds wrong; empty when all is well.
    async fn integrity_problems(&self) -> Result<Vec<String>, StoreError>;
}

/// Keeps a copy of the workspace's storage as it was before an upgrade (FR-007).
pub trait BackupCopy {
    /// Copies the storage aside, never leaving a half-written copy, and says where.
    fn copy(&self) -> Result<PathBuf, StoreError>;

    /// Puts a copy back in place of the storage.
    fn restore(&self, copy: &Path) -> Result<(), StoreError>;
}

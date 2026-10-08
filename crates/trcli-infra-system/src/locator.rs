//! The workspace on disk: whether a directory holds one, and its files other than the
//! database (FR-001 to FR-005).
//!
//! A workspace is a directory containing `.trcli/`. Nothing inside refers to where the
//! workspace is, so it keeps working when moved or renamed (FR-005).

use std::fs;
use std::path::{Path, PathBuf};

use trcli_application::ports::unit_of_work::StoreError;
use trcli_application::ports::workspace::{WorkspaceFiles, WorkspaceProbe};

use crate::atomic::write_atomically;

/// The name of the directory that makes a directory a workspace.
pub const WORKSPACE_DIRECTORY: &str = ".trcli";

/// What a new workspace's settings file starts with.
const SETTINGS_HEADER: &str = "# Settings of this workspace. Change them with `trcli config set <key> <value>`;\n\
# list them with `trcli config list`.\n";

/// The `.trcli` directory of a workspace.
pub fn workspace_directory(root: &Path) -> PathBuf {
    root.join(WORKSPACE_DIRECTORY)
}

/// The workspace's settings file.
pub fn workspace_settings_file(root: &Path) -> PathBuf {
    workspace_directory(root).join("config.toml")
}

/// The file that keeps the end of the audit trail.
pub fn audit_head_file(root: &Path) -> PathBuf {
    workspace_directory(root).join("audit.head")
}

/// The directory that keeps copies of the database taken before upgrades.
pub fn backups_directory(root: &Path) -> PathBuf {
    workspace_directory(root).join("backups")
}

/// The database of a workspace, given the `storage.path` setting: a relative path is
/// relative to the workspace's root.
pub fn database_file(root: &Path, storage_path: &str) -> PathBuf {
    let path = Path::new(storage_path);
    if path.is_absolute() {
        path.to_path_buf()
    } else {
        root.join(path)
    }
}

/// The file system, asked whether a directory holds a workspace.
#[derive(Clone, Copy, Debug, Default)]
pub struct FileSystemProbe;

impl WorkspaceProbe for FileSystemProbe {
    fn holds_workspace(&self, directory: &Path) -> bool {
        workspace_directory(directory).is_dir()
    }
}

/// The workspace's files on disk.
#[derive(Clone, Copy, Debug, Default)]
pub struct FileSystemWorkspaceFiles;

impl WorkspaceFiles for FileSystemWorkspaceFiles {
    fn is_writable(&self, directory: &Path) -> bool {
        if !directory.is_dir() {
            return false;
        }
        // Permission bits do not tell the whole story on every system; trying is the only
        // check that is right everywhere.
        let probe = directory.join(format!(".trcli-write-check-{}", std::process::id()));
        let writable = fs::write(&probe, b"").is_ok();
        let _ = fs::remove_file(&probe);
        writable
    }

    fn prepare(&self, root: &Path) -> Result<(), StoreError> {
        let directory = workspace_directory(root);
        let failed =
            |error: std::io::Error| StoreError::Failed(format!("{}: {error}", directory.display()));
        // `create_dir` fails when the directory exists: a workspace is created only where
        // there is none (FR-002).
        fs::create_dir(&directory).map_err(failed)?;
        fs::create_dir(backups_directory(root)).map_err(failed)?;
        write_atomically(&workspace_settings_file(root), SETTINGS_HEADER.as_bytes()).map_err(failed)
    }

    fn discard(&self, root: &Path) {
        let _ = fs::remove_dir_all(workspace_directory(root));
    }
}

#[cfg(test)]
mod tests {
    //! Unit tests for the workspace on disk.

    use std::path::{Path, PathBuf};

    use trcli_application::ports::workspace::{WorkspaceFiles, WorkspaceProbe};
    use trcli_application::workspace::locate::{LocateRequest, locate};

    use super::{
        FileSystemProbe, FileSystemWorkspaceFiles, backups_directory, database_file,
        workspace_settings_file,
    };

    #[test]
    fn preparing_creates_the_directory_the_settings_file_and_the_backups_directory() {
        let directory = tempfile::tempdir().expect("a temporary directory");
        let (files, probe) = (FileSystemWorkspaceFiles, FileSystemProbe);
        assert!(!probe.holds_workspace(directory.path()));
        files.prepare(directory.path()).expect("prepared");
        assert!(probe.holds_workspace(directory.path()));
        assert!(workspace_settings_file(directory.path()).is_file());
        assert!(backups_directory(directory.path()).is_dir());
        assert!(
            files.prepare(directory.path()).is_err(),
            "a second workspace is not created over the first"
        );
        files.discard(directory.path());
        assert!(!probe.holds_workspace(directory.path()));
    }

    #[test]
    fn a_workspace_is_found_by_walking_up_from_a_directory_beneath_it() {
        let directory = tempfile::tempdir().expect("a temporary directory");
        FileSystemWorkspaceFiles
            .prepare(directory.path())
            .expect("prepared");
        let deep = directory.path().join("a").join("b").join("c");
        std::fs::create_dir_all(&deep).expect("directories");
        let request = LocateRequest {
            current_directory: deep,
            ..LocateRequest::default()
        };
        assert_eq!(
            locate(&request, &FileSystemProbe).expect("found").root,
            directory.path()
        );
    }

    #[test]
    fn a_directory_that_does_not_exist_is_not_writable() {
        let directory = tempfile::tempdir().expect("a temporary directory");
        assert!(FileSystemWorkspaceFiles.is_writable(directory.path()));
        assert!(!FileSystemWorkspaceFiles.is_writable(&directory.path().join("missing")));
        assert_eq!(
            std::fs::read_dir(directory.path()).expect("list").count(),
            0,
            "the check leaves nothing behind"
        );
    }

    #[test]
    fn a_relative_storage_path_is_relative_to_the_workspace() {
        assert_eq!(
            database_file(Path::new("/w"), ".trcli/trcli.db"),
            PathBuf::from("/w/.trcli/trcli.db")
        );
        let absolute = if cfg!(windows) {
            "C:\\data\\trcli.db"
        } else {
            "/data/trcli.db"
        };
        assert_eq!(
            database_file(Path::new("/w"), absolute),
            PathBuf::from(absolute)
        );
    }
}

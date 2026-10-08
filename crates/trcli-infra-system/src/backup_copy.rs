//! The copy of the database kept before an upgrade (FR-007).
//!
//! The copy goes to `.trcli/backups/`, under a name that says when it was taken and which
//! format it holds. It is written under a temporary name and renamed, so a copy that
//! exists is always a whole copy (FR-066).

use std::fs;
use std::path::{Path, PathBuf};

use trcli_application::ports::unit_of_work::StoreError;
use trcli_application::ports::workspace::BackupCopy;

use crate::atomic::copy_atomically;

/// Copies one workspace's database to its backups directory.
#[derive(Clone, Debug)]
pub struct FileBackup {
    /// The database to copy.
    database: PathBuf,
    /// The directory copies are kept in.
    backups: PathBuf,
    /// What tells this copy from others: a timestamp and the format it holds.
    label: String,
}

impl FileBackup {
    /// A backup of `database` into `backups`. `label` becomes part of the copy's name.
    pub fn new(database: PathBuf, backups: PathBuf, label: &str) -> Self {
        // Only what is safe in a file name on every system is kept.
        let label = label
            .chars()
            .filter(|character| character.is_ascii_alphanumeric() || *character == '-')
            .collect();
        Self {
            database,
            backups,
            label,
        }
    }

    /// A name for the copy that no existing copy has.
    fn free_name(&self) -> PathBuf {
        let name = |suffix: String| {
            self.backups
                .join(format!("trcli-{}{suffix}.db", self.label))
        };
        let mut candidate = name(String::new());
        let mut number = 1;
        while candidate.exists() {
            number += 1;
            candidate = name(format!("-{number}"));
        }
        candidate
    }
}

/// An error that names the file concerned.
fn failed(path: &Path, error: std::io::Error) -> StoreError {
    StoreError::ReadOnly(format!("{}: {error}", path.display()))
}

impl BackupCopy for FileBackup {
    fn copy(&self) -> Result<PathBuf, StoreError> {
        fs::create_dir_all(&self.backups).map_err(|error| failed(&self.backups, error))?;
        let copy = self.free_name();
        copy_atomically(&self.database, &copy).map_err(|error| failed(&copy, error))?;
        Ok(copy)
    }

    fn restore(&self, copy: &Path) -> Result<(), StoreError> {
        let kept = fs::read(copy).map_err(|error| failed(copy, error))?;
        // A failed upgrade is undone by its transaction, which leaves the file untouched;
        // only when the file really differs is it replaced, since replacing a database
        // that is open is something to avoid when it is not needed.
        if fs::read(&self.database).is_ok_and(|current| current == kept) {
            return Ok(());
        }
        copy_atomically(copy, &self.database).map_err(|error| failed(&self.database, error))
    }
}

#[cfg(test)]
mod tests {
    //! Unit tests for the copy kept before an upgrade.

    use std::fs;

    use trcli_application::ports::workspace::BackupCopy;

    use super::FileBackup;

    #[test]
    fn a_copy_is_kept_under_a_name_of_its_own_and_can_be_put_back() {
        let directory = tempfile::tempdir().expect("a temporary directory");
        let database = directory.path().join("trcli.db");
        fs::write(&database, b"format one").expect("written");
        let backup = FileBackup::new(
            database.clone(),
            directory.path().join("backups"),
            "2026-10-08T14:00:00Z-format1",
        );

        let first = backup.copy().expect("copied");
        let second = backup.copy().expect("copied");
        assert_ne!(first, second);
        assert!(
            first
                .file_name()
                .expect("a name")
                .to_string_lossy()
                .starts_with("trcli-2026-10-08T140000Z-format1")
        );
        assert_eq!(fs::read(&first).expect("read"), b"format one");

        fs::write(&database, b"half upgraded").expect("written");
        backup.restore(&first).expect("restored");
        assert_eq!(fs::read(&database).expect("read"), b"format one");
    }

    #[test]
    fn copying_a_database_that_is_not_there_fails_and_leaves_no_copy() {
        let directory = tempfile::tempdir().expect("a temporary directory");
        let backups = directory.path().join("backups");
        let backup = FileBackup::new(directory.path().join("missing.db"), backups.clone(), "x");
        assert!(backup.copy().is_err());
        assert_eq!(fs::read_dir(backups).expect("list").count(), 0);
    }
}

//! The end of the audit trail, kept in `.trcli/audit.head` beside the database (FR-048).
//!
//! The file holds the sequence and the hash of the last entry. Because it is outside the
//! database, removing the last entries from the database does not remove the memory of
//! them: verification then reports that the trail was shortened.

use std::fs;
use std::io::ErrorKind;
use std::path::PathBuf;

use trcli_application::ports::audit::AuditHeadStore;
use trcli_application::ports::unit_of_work::StoreError;
use trcli_domain::governance::audit::AuditHead;

use crate::atomic::write_atomically;

/// The head file of one workspace.
#[derive(Clone, Debug)]
pub struct FileHead {
    /// Where the file is.
    path: PathBuf,
}

impl FileHead {
    /// The head kept at `path`.
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }
}

impl AuditHeadStore for FileHead {
    fn read_head(&self) -> Result<Option<AuditHead>, StoreError> {
        let location = || self.path.display().to_string();
        match fs::read_to_string(&self.path) {
            Ok(line) => AuditHead::parse(&line)
                .map(Some)
                .ok_or_else(|| StoreError::Damaged {
                    what: "the audit head file cannot be read as a sequence and a hash".to_owned(),
                    location: location(),
                }),
            Err(error) if error.kind() == ErrorKind::NotFound => Ok(None),
            Err(error) => Err(StoreError::Failed(format!("{}: {error}", location()))),
        }
    }

    fn write_head(&self, head: &AuditHead) -> Result<(), StoreError> {
        write_atomically(&self.path, head.to_line().as_bytes())
            .map_err(|error| StoreError::ReadOnly(format!("{}: {error}", self.path.display())))
    }
}

#[cfg(test)]
mod tests {
    //! Unit tests for the head file.

    use trcli_application::ports::audit::AuditHeadStore;
    use trcli_application::ports::unit_of_work::StoreError;
    use trcli_domain::governance::audit::AuditHead;

    use super::FileHead;

    #[test]
    fn the_head_round_trips_and_is_absent_before_the_first_entry() {
        let directory = tempfile::tempdir().expect("a temporary directory");
        let head = FileHead::new(directory.path().join("audit.head"));
        assert_eq!(head.read_head(), Ok(None));
        let value = AuditHead {
            sequence: 12,
            hash: [7; 32],
        };
        head.write_head(&value).expect("written");
        assert_eq!(head.read_head(), Ok(Some(value)));
    }

    #[test]
    fn a_head_file_that_was_edited_by_hand_is_reported() {
        let directory = tempfile::tempdir().expect("a temporary directory");
        let path = directory.path().join("audit.head");
        std::fs::write(&path, "not a head").expect("written");
        assert!(matches!(
            FileHead::new(path).read_head(),
            Err(StoreError::Damaged { .. })
        ));
    }
}

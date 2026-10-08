//! Storage and the unit of work: how a change is made whole or not at all (FR-009, FR-047).
//!
//! A use case changes data only through a [`UnitOfWork`]. Everything done through one unit
//! — the change and its audit entry — becomes visible together when it is committed, and
//! none of it when the unit is dropped instead.

use trcli_domain::governance::audit::AuditHead;

use crate::outcome::{Problem, codes};

/// Why storage could not do what was asked.
#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]
pub enum StoreError {
    /// Another command is changing the workspace and did not finish within the wait.
    #[error("the workspace is busy: another trcli command is changing it")]
    Busy,
    /// The stored data cannot be opened or fails a check.
    #[error("{what} ({location})")]
    Damaged {
        /// What is wrong.
        what: String,
        /// Where: the file or table concerned.
        location: String,
    },
    /// Something already exists where it was to be created.
    #[error("{0} already exists")]
    AlreadyExists(String),
    /// A rule the store enforces would be broken, for example pointing at a missing record.
    #[error("{0}")]
    Constraint(String),
    /// The storage cannot be written to (a read-only or full disk).
    #[error("the workspace cannot be written to: {0}")]
    ReadOnly(String),
    /// Anything else.
    #[error("storage failed: {0}")]
    Failed(String),
}

impl From<StoreError> for Problem {
    fn from(error: StoreError) -> Self {
        match &error {
            StoreError::Busy => Problem::new(codes::WORKSPACE_BUSY, error.to_string())
                .with_next_step("wait for the other command to finish and run this one again"),
            StoreError::Damaged { what, location } => {
                Problem::new(codes::WORKSPACE_DAMAGED, format!("the workspace is damaged: {what}"))
                    .with_items(vec![format!("where: {location}")])
                    .with_next_step("restore a copy from `.trcli/backups/` if there is one; nothing was written")
            }
            StoreError::AlreadyExists(_) => Problem::new(codes::WORKSPACE_EXISTS, error.to_string()),
            StoreError::ReadOnly(_) => Problem::new(codes::OPERATION_FAILED, error.to_string())
                .with_next_step("check that the disk is not full or read-only; commands that only read still work"),
            StoreError::Constraint(_) | StoreError::Failed(_) => Problem::internal(error.to_string()),
        }
    }
}

/// Where units of work come from: one workspace's storage.
pub trait Storage {
    /// The unit of work this storage gives out.
    type Unit: UnitOfWork;

    /// Begins a unit of work that will change something. A second writer waits, up to the
    /// configured time, and then gets [`StoreError::Busy`].
    async fn begin(&self) -> Result<Self::Unit, StoreError>;

    /// Begins a unit of work that only reads; committing it changes nothing.
    async fn read(&self) -> Result<Self::Unit, StoreError>;
}

/// A set of changes that happen together or not at all.
///
/// Dropping a unit without committing it undoes everything done through it; that is how an
/// interrupted or failed command leaves the workspace as it was.
pub trait UnitOfWork {
    /// Makes everything done through this unit permanent, and returns the new end of the
    /// audit trail when entries were recorded, so that the head file can be updated.
    async fn commit(self) -> Result<Option<AuditHead>, StoreError>;
}

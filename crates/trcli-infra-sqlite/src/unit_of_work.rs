//! One transaction: the unit of work every store is reached through (FR-009, FR-047).
//!
//! The stores are implemented on [`SqliteUnit`] in the private `stores` module, one file
//! per port. Dropping a unit without committing rolls the transaction back, which is how
//! a failed or interrupted command leaves the workspace as it was.

use sea_orm::DatabaseTransaction;
use trcli_application::ports::unit_of_work::{StoreError, UnitOfWork};
use trcli_domain::governance::audit::AuditHead;

use crate::convert::store_error;

/// A unit of work on a workspace's database.
#[derive(Debug)]
pub struct SqliteUnit {
    /// The open transaction.
    pub(crate) transaction: DatabaseTransaction,
    /// The end of the audit trail after the entries recorded through this unit.
    pub(crate) recorded: Option<AuditHead>,
}

impl SqliteUnit {
    /// Wraps a transaction that has just begun.
    pub(crate) fn new(transaction: DatabaseTransaction) -> Self {
        Self {
            transaction,
            recorded: None,
        }
    }
}

impl SqliteUnit {
    /// The transaction itself, for a feature kept outside this crate to implement its own
    /// store on this unit of work. Stores built into this crate use the field directly;
    /// the one caller is the example that shows a feature added from outside.
    pub fn transaction(&self) -> &DatabaseTransaction {
        &self.transaction
    }
}

impl UnitOfWork for SqliteUnit {
    async fn commit(self) -> Result<Option<AuditHead>, StoreError> {
        // A busy database surfaces here too: `store_error` reports it as such, so the
        // researcher is told the command can be tried again.
        self.transaction.commit().await.map_err(store_error)?;
        Ok(self.recorded)
    }
}

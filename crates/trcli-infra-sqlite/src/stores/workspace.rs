//! The workspace's own row, bringing the schema forward, and the database's own
//! consistency check (FR-001, FR-007, FR-008).

use sea_orm::{ActiveModelTrait, ConnectionTrait, DbBackend, EntityTrait, Set, Statement};
use sea_orm_migration::MigratorTrait;
use trcli_application::ports::unit_of_work::StoreError;
use trcli_application::ports::workspace::{IntegrityCheck, SchemaUpgrade, WorkspaceStore};
use trcli_domain::shared::text::{LongText, Name};
use trcli_domain::workspace::{FormatVersion, Workspace};

use crate::convert::{corrupt, from_millis, store_error, to_id, to_millis};
use crate::entities::workspace::{ActiveModel, Entity, Model};
use crate::migrations::Migrator;
use crate::unit_of_work::SqliteUnit;

/// The workspace a row describes.
fn to_workspace(row: Model) -> Result<Workspace, StoreError> {
    Ok(Workspace {
        id: to_id(&row.id)?,
        name: Name::new(&row.name)
            .map_err(|_| corrupt("the workspace's name is not a valid name"))?,
        description: LongText::new(&row.description)
            .map_err(|_| corrupt("the workspace's description is not valid text"))?,
        format_version: FormatVersion::new(
            u32::try_from(row.format_version)
                .map_err(|_| corrupt("the workspace's format version is not a version"))?,
        ),
        created_at: from_millis(row.created_at)?,
    })
}

impl WorkspaceStore for SqliteUnit {
    async fn workspace(&self) -> Result<Workspace, StoreError> {
        let row = Entity::find()
            .one(&self.transaction)
            .await
            .map_err(store_error)?;
        row.map(to_workspace)
            .unwrap_or_else(|| Err(corrupt("the workspace's own details are missing")))
    }

    async fn save_workspace(&mut self, workspace: &Workspace) -> Result<(), StoreError> {
        let row = ActiveModel {
            id: Set(workspace.id.to_string()),
            name: Set(workspace.name.to_string()),
            description: Set(workspace.description.to_string()),
            format_version: Set(i64::from(workspace.format_version.number())),
            created_at: Set(to_millis(workspace.created_at)),
        };
        let exists = Entity::find_by_id(workspace.id.to_string())
            .one(&self.transaction)
            .await
            .map_err(store_error)?;
        let saved = if exists.is_some() {
            row.update(&self.transaction).await
        } else {
            row.insert(&self.transaction).await
        };
        saved.map(|_| ()).map_err(store_error)
    }
}

impl SchemaUpgrade for SqliteUnit {
    async fn apply_pending_migrations(&mut self) -> Result<(), StoreError> {
        // Inside this unit's transaction: a migration that fails undoes the ones before
        // it, because data definition is transactional in SQLite.
        Migrator::up(&self.transaction, None)
            .await
            .map_err(store_error)
    }
}

impl IntegrityCheck for SqliteUnit {
    async fn integrity_problems(&self) -> Result<Vec<String>, StoreError> {
        let mut problems = Vec::new();
        let check = Statement::from_string(DbBackend::Sqlite, "PRAGMA integrity_check");
        for row in self
            .transaction
            .query_all_raw(check)
            .await
            .map_err(store_error)?
        {
            let line: String = row.try_get_by_index(0).map_err(store_error)?;
            if line != "ok" {
                problems.push(line);
            }
        }
        // Rows that point at a record the index does not hold (FR-017).
        let foreign_keys = Statement::from_string(DbBackend::Sqlite, "PRAGMA foreign_key_check");
        for row in self
            .transaction
            .query_all_raw(foreign_keys)
            .await
            .map_err(store_error)?
        {
            let table: String = row.try_get_by_index(0).map_err(store_error)?;
            problems.push(format!(
                "a row of table `{table}` points at something that does not exist"
            ));
        }
        Ok(problems)
    }
}

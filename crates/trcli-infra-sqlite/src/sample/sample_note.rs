//! The `sample_note` table and its store.

use sea_orm::entity::prelude::*;
use sea_orm::{ActiveModelTrait, Set};
use trcli_application::ports::unit_of_work::StoreError;
use trcli_application::sample::sample_note::{SampleNoteRow, SampleNoteStore};
use trcli_domain::shared::record::RecordId;

use crate::convert::store_error;
use crate::unit_of_work::SqliteUnit;

/// A sample note's own row.
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "sample_note")]
pub struct Model {
    /// The record's identifier, as in the record index.
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    /// The note's text.
    pub body: String,
    /// Whether the note is locked against deletion.
    pub locked: bool,
}

/// `id` is a foreign key to the record index; see the migration.
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

impl SampleNoteStore for SqliteUnit {
    async fn save_sample_note(
        &mut self,
        id: RecordId,
        row: &SampleNoteRow,
    ) -> Result<(), StoreError> {
        let exists = Entity::find_by_id(id.to_string())
            .one(&self.transaction)
            .await
            .map_err(store_error)?
            .is_some();
        let active = ActiveModel {
            id: Set(id.to_string()),
            body: Set(row.body.clone()),
            locked: Set(row.locked),
        };
        let saved = if exists {
            active.update(&self.transaction).await
        } else {
            active.insert(&self.transaction).await
        };
        saved.map(|_| ()).map_err(store_error)
    }

    async fn sample_note(&self, id: RecordId) -> Result<Option<SampleNoteRow>, StoreError> {
        let row = Entity::find_by_id(id.to_string())
            .one(&self.transaction)
            .await
            .map_err(store_error)?;
        Ok(row.map(|row| SampleNoteRow {
            body: row.body,
            locked: row.locked,
        }))
    }

    async fn remove_sample_note(&mut self, id: RecordId) -> Result<(), StoreError> {
        Entity::delete_by_id(id.to_string())
            .exec(&self.transaction)
            .await
            .map(|_| ())
            .map_err(store_error)
    }
}

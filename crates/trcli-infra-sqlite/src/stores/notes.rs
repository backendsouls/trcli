//! The notes of records (FR-015).

use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, NotSet, QueryFilter, QueryOrder, Set};
use trcli_application::ports::records::NoteStore;
use trcli_application::ports::unit_of_work::StoreError;
use trcli_domain::shared::note::Note;
use trcli_domain::shared::record::RecordId;

use crate::convert::{corrupt, from_millis, store_error, to_id, to_millis};
use crate::entities::note::{ActiveModel, Column, Entity, Model};
use crate::unit_of_work::SqliteUnit;

/// The note a row holds.
fn to_note(row: Model) -> Result<Note, StoreError> {
    let body = Note::body(&row.body).map_err(|_| corrupt("a stored note is not valid text"))?;
    Ok(Note::new(
        to_id(&row.record)?,
        body,
        from_millis(row.created_at)?,
    ))
}

impl NoteStore for SqliteUnit {
    async fn add_note(&mut self, note: &Note) -> Result<(), StoreError> {
        let row = ActiveModel {
            id: NotSet,
            record: Set(note.record.to_string()),
            body: Set(note.body.to_string()),
            created_at: Set(to_millis(note.created_at)),
        };
        row.insert(&self.transaction)
            .await
            .map(|_| ())
            .map_err(store_error)
    }

    async fn notes_of(&self, record: RecordId) -> Result<Vec<Note>, StoreError> {
        let rows = Entity::find()
            .filter(Column::Record.eq(record.to_string()))
            .order_by_asc(Column::Id)
            .all(&self.transaction)
            .await
            .map_err(store_error)?;
        rows.into_iter().map(to_note).collect()
    }

    async fn remove_notes_of(&mut self, record: RecordId) -> Result<u64, StoreError> {
        let removed = Entity::delete_many()
            .filter(Column::Record.eq(record.to_string()))
            .exec(&self.transaction)
            .await
            .map_err(store_error)?;
        Ok(removed.rows_affected)
    }
}

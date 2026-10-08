//! The `specimen` table and its store.

use super::super::application::specimen::SpecimenStore;
use sea_orm::entity::prelude::*;
use sea_orm::{ActiveModelTrait, Set};
use trcli_application::ports::unit_of_work::StoreError;
use trcli_domain::shared::record::RecordId;
use trcli_domain::shared::text::Title;

use trcli_cli::compose::AppUnit;

use super::failed;

/// A specimen's own row.
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "specimen")]
pub struct Model {
    /// The record's identifier, as in the record index.
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    /// The specimen's title.
    pub title: String,
}

/// `id` is a foreign key to the record index; see the migration.
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

impl SpecimenStore for AppUnit {
    async fn insert_specimen(&mut self, id: RecordId, title: &Title) -> Result<(), StoreError> {
        let row = ActiveModel {
            id: Set(id.to_string()),
            title: Set(title.to_string()),
        };
        row.insert(self.transaction())
            .await
            .map(|_| ())
            .map_err(failed)
    }

    async fn retitle_specimen(&mut self, id: RecordId, title: &Title) -> Result<(), StoreError> {
        let row = ActiveModel {
            id: Set(id.to_string()),
            title: Set(title.to_string()),
        };
        row.update(self.transaction())
            .await
            .map(|_| ())
            .map_err(failed)
    }

    async fn specimen_title(&self, id: RecordId) -> Result<Option<String>, StoreError> {
        let row = Entity::find_by_id(id.to_string())
            .one(self.transaction())
            .await
            .map_err(failed)?;
        Ok(row.map(|row| row.title))
    }

    async fn remove_specimen(&mut self, id: RecordId) -> Result<(), StoreError> {
        Entity::delete_by_id(id.to_string())
            .exec(self.transaction())
            .await
            .map(|_| ())
            .map_err(failed)
    }
}

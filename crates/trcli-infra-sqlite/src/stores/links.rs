//! The links between records (FR-016).

use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, EntityTrait, NotSet, QueryFilter, QueryOrder, Set,
};
use trcli_application::ports::records::LinkStore;
use trcli_application::ports::unit_of_work::StoreError;
use trcli_domain::shared::link::Link;
use trcli_domain::shared::record::RecordId;
use trcli_domain::shared::text::Relation;

use crate::convert::{corrupt, from_millis, store_error, to_id, to_millis};
use crate::entities::link::{ActiveModel, Column, Entity, Model};
use crate::unit_of_work::SqliteUnit;

/// The link a row holds.
fn to_link(row: Model) -> Result<Link, StoreError> {
    let relation =
        Relation::new(&row.relation).map_err(|_| corrupt("a stored relation is not valid text"))?;
    Link::new(
        to_id(&row.from_record)?,
        to_id(&row.to_record)?,
        relation,
        from_millis(row.created_at)?,
    )
    .map_err(|_| corrupt("a stored link joins a record to itself"))
}

/// The condition "one of the ends is this record".
fn touching(record: RecordId) -> Condition {
    let id = record.to_string();
    Condition::any()
        .add(Column::FromRecord.eq(id.clone()))
        .add(Column::ToRecord.eq(id))
}

impl LinkStore for SqliteUnit {
    async fn add_link(&mut self, link: &Link) -> Result<(), StoreError> {
        let row = ActiveModel {
            id: NotSet,
            from_record: Set(link.from.to_string()),
            to_record: Set(link.to.to_string()),
            relation: Set(link.relation.to_string()),
            created_at: Set(to_millis(link.created_at)),
        };
        row.insert(&self.transaction)
            .await
            .map(|_| ())
            .map_err(store_error)
    }

    async fn remove_link(&mut self, link: &Link) -> Result<bool, StoreError> {
        let (from, to) = (link.from.to_string(), link.to.to_string());
        // The link is the same whichever end it was made from.
        let forward = Condition::all()
            .add(Column::FromRecord.eq(from.clone()))
            .add(Column::ToRecord.eq(to.clone()));
        let reverse = Condition::all()
            .add(Column::FromRecord.eq(to))
            .add(Column::ToRecord.eq(from));
        let removed = Entity::delete_many()
            .filter(Column::Relation.eq(link.relation.as_str()))
            .filter(Condition::any().add(forward).add(reverse))
            .exec(&self.transaction)
            .await
            .map_err(store_error)?;
        Ok(removed.rows_affected > 0)
    }

    async fn links_of(&self, record: RecordId) -> Result<Vec<Link>, StoreError> {
        let rows = Entity::find()
            .filter(touching(record))
            .order_by_asc(Column::Id)
            .all(&self.transaction)
            .await
            .map_err(store_error)?;
        rows.into_iter().map(to_link).collect()
    }

    async fn remove_links_of(&mut self, record: RecordId) -> Result<u64, StoreError> {
        let removed = Entity::delete_many()
            .filter(touching(record))
            .exec(&self.transaction)
            .await
            .map_err(store_error)?;
        Ok(removed.rows_affected)
    }
}

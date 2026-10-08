//! The record index: one row for every record of every kind (FR-010, FR-011, FR-014).

use sea_orm::sea_query::Expr;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder,
    QuerySelect, QueryTrait, Set,
};
use time::OffsetDateTime;
use trcli_application::ports::records::{
    IndexedRecord, ListPage, ListQuery, RecordIndex, RecordResolver, SortField,
};
use trcli_application::ports::unit_of_work::StoreError;
use trcli_domain::shared::record::{Handle, RecordId};
use trcli_domain::shared::text::SearchKey;

use crate::convert::{corrupt, from_millis, store_error, to_id, to_millis};
use crate::entities::record::{ActiveModel, Column, Entity, Model};
use crate::entities::tagging;
use crate::unit_of_work::SqliteUnit;

/// The index's knowledge of a record, from its row.
fn to_record(row: Model) -> Result<IndexedRecord, StoreError> {
    Ok(IndexedRecord {
        id: to_id(&row.id)?,
        kind: row.kind,
        handle: Handle::parse(&row.handle)
            .map_err(|_| corrupt("a stored short name is not a short name"))?,
        display_name: row.display_name,
        search_key: SearchKey::from_text(&row.search_key),
        created_at: from_millis(row.created_at)?,
        updated_at: from_millis(row.updated_at)?,
        deleted_at: row.deleted_at.map(from_millis).transpose()?,
    })
}

/// The condition "not deleted, and of one of these kinds (any kind when empty)".
fn live(kinds: &[String]) -> Condition {
    let condition = Condition::all().add(Column::DeletedAt.is_null());
    if kinds.is_empty() {
        condition
    } else {
        condition.add(Column::Kind.is_in(kinds.iter().map(String::as_str)))
    }
}

/// The conditions a list query stands for.
fn matching(query: &ListQuery) -> Condition {
    let mut condition = live(std::slice::from_ref(&query.kind));
    for tag in &query.tags {
        // One sub-query per tag: the record must carry every tag asked for.
        let carriers = tagging::Entity::find()
            .select_only()
            .column(tagging::Column::Record)
            .filter(tagging::Column::Tag.eq(tag.as_str()))
            .into_query();
        condition = condition.add(Column::Id.in_subquery(carriers));
    }
    if let Some(search) = &query.search {
        // A search key holds only letters, digits, and spaces, so it has no wildcard.
        condition = condition.add(Column::SearchKey.contains(search.as_str()));
    }
    condition
}

impl SqliteUnit {
    /// Sets columns of one index row.
    async fn update_record(
        &mut self,
        id: RecordId,
        changes: Vec<(Column, sea_orm::Value)>,
    ) -> Result<(), StoreError> {
        let mut update = Entity::update_many().filter(Column::Id.eq(id.to_string()));
        for (column, value) in changes {
            update = update.col_expr(column, Expr::value(value));
        }
        let result = update.exec(&self.transaction).await.map_err(store_error)?;
        if result.rows_affected == 0 {
            return Err(StoreError::Constraint(format!(
                "record {id} does not exist"
            )));
        }
        Ok(())
    }
}

impl RecordIndex for SqliteUnit {
    async fn insert_record(&mut self, record: &IndexedRecord) -> Result<(), StoreError> {
        let row = ActiveModel {
            id: Set(record.id.to_string()),
            kind: Set(record.kind.clone()),
            handle: Set(record.handle.to_string()),
            display_name: Set(record.display_name.clone()),
            search_key: Set(record.search_key.as_str().to_owned()),
            created_at: Set(to_millis(record.created_at)),
            updated_at: Set(to_millis(record.updated_at)),
            deleted_at: Set(record.deleted_at.map(to_millis)),
        };
        row.insert(&self.transaction)
            .await
            .map(|_| ())
            .map_err(store_error)
    }

    async fn rename_record(
        &mut self,
        id: RecordId,
        display_name: &str,
        search_key: &SearchKey,
        at: OffsetDateTime,
    ) -> Result<(), StoreError> {
        let changes = vec![
            (Column::DisplayName, display_name.into()),
            (Column::SearchKey, search_key.as_str().into()),
            (Column::UpdatedAt, to_millis(at).into()),
        ];
        self.update_record(id, changes).await
    }

    async fn touch_record(&mut self, id: RecordId, at: OffsetDateTime) -> Result<(), StoreError> {
        self.update_record(id, vec![(Column::UpdatedAt, to_millis(at).into())])
            .await
    }

    async fn mark_deleted(&mut self, id: RecordId, at: OffsetDateTime) -> Result<(), StoreError> {
        self.update_record(id, vec![(Column::DeletedAt, Some(to_millis(at)).into())])
            .await
    }

    async fn handle_is_taken(&self, handle: &Handle) -> Result<bool, StoreError> {
        let found = Entity::find()
            .filter(Column::Handle.eq(handle.as_str()))
            .count(&self.transaction)
            .await;
        found.map(|count| count > 0).map_err(store_error)
    }

    async fn record(&self, id: RecordId) -> Result<Option<IndexedRecord>, StoreError> {
        let row = Entity::find_by_id(id.to_string())
            .one(&self.transaction)
            .await
            .map_err(store_error)?;
        row.map(to_record).transpose()
    }

    async fn list_records(&self, query: &ListQuery) -> Result<ListPage, StoreError> {
        let matching = Entity::find().filter(matching(query));
        let total = matching
            .clone()
            .count(&self.transaction)
            .await
            .map_err(store_error)?;
        let order = if query.descending {
            sea_orm::Order::Desc
        } else {
            sea_orm::Order::Asc
        };
        let sorted = match query.sort {
            SortField::Name => matching.order_by(Column::SearchKey, order.clone()),
            SortField::Created => matching.order_by(Column::CreatedAt, order.clone()),
            SortField::Updated => matching.order_by(Column::UpdatedAt, order.clone()),
            SortField::Handle => matching,
        };
        // The handle breaks ties, so that the same query always gives the same order.
        let rows = sorted
            .order_by(Column::Handle, order)
            .limit(u64::from(query.limit))
            .all(&self.transaction)
            .await
            .map_err(store_error)?;
        Ok(ListPage {
            items: rows.into_iter().map(to_record).collect::<Result<_, _>>()?,
            total,
        })
    }

    async fn count_by_kind(&self) -> Result<Vec<(String, u64)>, StoreError> {
        let counts: Vec<(String, i64)> = Entity::find()
            .select_only()
            .column(Column::Kind)
            .column_as(Column::Id.count(), "records")
            .filter(Column::DeletedAt.is_null())
            .group_by(Column::Kind)
            .order_by_asc(Column::Kind)
            .into_tuple()
            .all(&self.transaction)
            .await
            .map_err(store_error)?;
        Ok(counts
            .into_iter()
            .map(|(kind, count)| (kind, count as u64))
            .collect())
    }
}

impl RecordResolver for SqliteUnit {
    async fn records_starting_with(
        &self,
        beginning: &str,
        kinds: &[String],
    ) -> Result<Vec<IndexedRecord>, StoreError> {
        // What was typed holds only letters, digits, and '-': no wildcard to escape.
        let rows = Entity::find()
            .filter(live(kinds).add(Column::Handle.starts_with(beginning)))
            .order_by_asc(Column::Handle)
            .all(&self.transaction)
            .await
            .map_err(store_error)?;
        rows.into_iter().map(to_record).collect()
    }

    async fn handles(&self, kinds: &[String]) -> Result<Vec<Handle>, StoreError> {
        let handles: Vec<String> = Entity::find()
            .select_only()
            .column(Column::Handle)
            .filter(live(kinds))
            .into_tuple()
            .all(&self.transaction)
            .await
            .map_err(store_error)?;
        Ok(handles
            .iter()
            .filter_map(|handle| Handle::parse(handle).ok())
            .collect())
    }
}

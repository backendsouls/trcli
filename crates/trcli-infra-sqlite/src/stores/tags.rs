//! Which records carry which tags (FR-015).

use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, DbBackend, EntityTrait, PaginatorTrait,
    QueryFilter, QueryOrder, QuerySelect, Set, Statement,
};
use trcli_application::ports::records::TagStore;
use trcli_application::ports::unit_of_work::StoreError;
use trcli_domain::shared::record::RecordId;
use trcli_domain::shared::text::TagName;

use crate::convert::{corrupt, store_error};
use crate::entities::{tag, tagging};
use crate::unit_of_work::SqliteUnit;

/// A tag name read from a table.
fn to_tag(name: &str) -> Result<TagName, StoreError> {
    TagName::new(name).map_err(|_| corrupt("a stored tag is not a valid tag name"))
}

impl SqliteUnit {
    /// Removes tags that no record carries any more, so that `tag list` shows only tags
    /// in use.
    async fn remove_unused_tags(&mut self) -> Result<(), StoreError> {
        let unused = "DELETE FROM tag WHERE name NOT IN (SELECT tag FROM tagging)";
        self.transaction
            .execute_unprepared(unused)
            .await
            .map(|_| ())
            .map_err(store_error)
    }
}

impl TagStore for SqliteUnit {
    async fn attach_tag(&mut self, record: RecordId, tag: &TagName) -> Result<bool, StoreError> {
        let key = (tag.as_str().to_owned(), record.to_string());
        if tagging::Entity::find_by_id(key.clone())
            .one(&self.transaction)
            .await
            .map_err(store_error)?
            .is_some()
        {
            return Ok(false);
        }
        if tag::Entity::find_by_id(key.0.clone())
            .one(&self.transaction)
            .await
            .map_err(store_error)?
            .is_none()
        {
            tag::ActiveModel {
                name: Set(key.0.clone()),
            }
            .insert(&self.transaction)
            .await
            .map_err(store_error)?;
        }
        let tagging = tagging::ActiveModel {
            tag: Set(key.0),
            record: Set(key.1),
        };
        tagging
            .insert(&self.transaction)
            .await
            .map(|_| true)
            .map_err(store_error)
    }

    async fn detach_tag(&mut self, record: RecordId, tag: &TagName) -> Result<bool, StoreError> {
        let key = (tag.as_str().to_owned(), record.to_string());
        let removed = tagging::Entity::delete_by_id(key)
            .exec(&self.transaction)
            .await
            .map_err(store_error)?;
        self.remove_unused_tags().await?;
        Ok(removed.rows_affected > 0)
    }

    async fn tags_of(&self, record: RecordId) -> Result<Vec<TagName>, StoreError> {
        let names: Vec<String> = tagging::Entity::find()
            .select_only()
            .column(tagging::Column::Tag)
            .filter(tagging::Column::Record.eq(record.to_string()))
            .order_by_asc(tagging::Column::Tag)
            .into_tuple()
            .all(&self.transaction)
            .await
            .map_err(store_error)?;
        names.iter().map(|name| to_tag(name)).collect()
    }

    async fn tag_counts(&self, kind: Option<&str>) -> Result<Vec<(TagName, u64)>, StoreError> {
        // A join and a grouped count read more clearly as the SQL they are.
        let sql = "SELECT tagging.tag, COUNT(*) FROM tagging \
                   JOIN record ON record.id = tagging.record \
                   WHERE record.deleted_at IS NULL AND (?1 IS NULL OR record.kind = ?1) \
                   GROUP BY tagging.tag ORDER BY tagging.tag";
        let statement = Statement::from_sql_and_values(
            DbBackend::Sqlite,
            sql,
            [kind.map(str::to_owned).into()],
        );
        let rows = self
            .transaction
            .query_all_raw(statement)
            .await
            .map_err(store_error)?;
        rows.iter()
            .map(|row| {
                let name: String = row.try_get_by_index(0).map_err(store_error)?;
                let count: i64 = row.try_get_by_index(1).map_err(store_error)?;
                Ok((to_tag(&name)?, count as u64))
            })
            .collect()
    }

    async fn remove_taggings_of(&mut self, record: RecordId) -> Result<u64, StoreError> {
        let of_record =
            tagging::Entity::find().filter(tagging::Column::Record.eq(record.to_string()));
        let count = of_record
            .count(&self.transaction)
            .await
            .map_err(store_error)?;
        tagging::Entity::delete_many()
            .filter(tagging::Column::Record.eq(record.to_string()))
            .exec(&self.transaction)
            .await
            .map_err(store_error)?;
        self.remove_unused_tags().await?;
        Ok(count)
    }
}

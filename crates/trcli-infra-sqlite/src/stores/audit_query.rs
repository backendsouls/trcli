//! Reading the audit trail (FR-049).

use sea_orm::{
    ColumnTrait, Condition, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder, QuerySelect,
};
use time::{Date, OffsetDateTime, Time};
use trcli_application::ports::audit::{AuditFilter, AuditPage, AuditQuery};
use trcli_application::ports::unit_of_work::StoreError;
use trcli_domain::governance::audit::AuditEntry;

use super::audit::to_entry;
use crate::convert::{store_error, to_millis};
use crate::entities::audit_entry::{Column, Entity};
use crate::unit_of_work::SqliteUnit;

/// The first millisecond of a day, in UTC.
fn start_of(day: Date) -> i64 {
    to_millis(OffsetDateTime::new_utc(day, Time::MIDNIGHT))
}

/// The conditions a filter stands for.
fn condition(filter: &AuditFilter) -> Condition {
    let mut condition = Condition::all();
    if let Some(record) = filter.record {
        condition = condition.add(Column::RecordId.eq(record.to_string()));
    }
    if let Some(kind) = &filter.kind {
        condition = condition.add(Column::Kind.eq(kind.as_str()));
    }
    if let Some(actor) = &filter.actor {
        condition = condition.add(Column::Actor.eq(actor.as_str()));
    }
    if let Some(action) = &filter.action {
        condition = condition.add(Column::Action.eq(action.as_str()));
    }
    if let Some(from) = filter.from {
        condition = condition.add(Column::At.gte(start_of(from)));
    }
    if let Some(to) = filter.to {
        // The whole of the last day is included: everything before the next day begins.
        let next_day = to.next_day().unwrap_or(to);
        condition = condition.add(Column::At.lt(start_of(next_day)));
    }
    condition
}

impl AuditQuery for SqliteUnit {
    async fn entries(&self, filter: &AuditFilter) -> Result<AuditPage, StoreError> {
        let matching = Entity::find().filter(condition(filter));
        let total = matching
            .clone()
            .count(&self.transaction)
            .await
            .map_err(store_error)?;
        let rows = matching
            .order_by_desc(Column::Sequence)
            .limit(u64::from(filter.limit))
            .all(&self.transaction)
            .await
            .map_err(store_error)?;
        let entries = rows
            .into_iter()
            .map(to_entry)
            .collect::<Result<Vec<AuditEntry>, StoreError>>()?;
        Ok(AuditPage { entries, total })
    }

    async fn entries_after(&self, after: u64, limit: u32) -> Result<Vec<AuditEntry>, StoreError> {
        let rows = Entity::find()
            .filter(Column::Sequence.gt(after as i64))
            .order_by_asc(Column::Sequence)
            .limit(u64::from(limit))
            .all(&self.transaction)
            .await
            .map_err(store_error)?;
        rows.into_iter().map(to_entry).collect()
    }

    async fn entry_count(&self) -> Result<u64, StoreError> {
        Entity::find()
            .count(&self.transaction)
            .await
            .map_err(store_error)
    }
}

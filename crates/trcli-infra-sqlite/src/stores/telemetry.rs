//! Local telemetry (FR-052). Nothing here, or anywhere, sends it out (FR-053).

use sea_orm::{ActiveModelTrait, EntityTrait, NotSet, QueryOrder, Set};
use trcli_application::ports::audit::TelemetryLog;
use trcli_application::ports::unit_of_work::StoreError;
use trcli_domain::governance::telemetry::TelemetryRecord;

use crate::convert::{from_millis, store_error, to_millis};
use crate::entities::telemetry_record::{ActiveModel, Column, Entity};
use crate::unit_of_work::SqliteUnit;

impl TelemetryLog for SqliteUnit {
    async fn append_telemetry(&mut self, record: &TelemetryRecord) -> Result<(), StoreError> {
        let row = ActiveModel {
            id: NotSet,
            at: Set(to_millis(record.at)),
            command: Set(record.command.clone()),
            duration_ms: Set(record.duration_ms as i64),
            outcome: Set(record.outcome.clone()),
        };
        row.insert(&self.transaction)
            .await
            .map(|_| ())
            .map_err(store_error)
    }

    async fn telemetry(&self) -> Result<Vec<TelemetryRecord>, StoreError> {
        let rows = Entity::find()
            .order_by_asc(Column::Id)
            .all(&self.transaction)
            .await
            .map_err(store_error)?;
        rows.into_iter()
            .map(|row| {
                Ok(TelemetryRecord {
                    at: from_millis(row.at)?,
                    command: row.command,
                    duration_ms: row.duration_ms as u64,
                    outcome: row.outcome,
                })
            })
            .collect()
    }
}

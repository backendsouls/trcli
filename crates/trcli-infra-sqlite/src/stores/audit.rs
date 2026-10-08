//! Appending to the audit trail (FR-047, FR-051), and the conversions between an entry
//! and its row.
//!
//! An entry is inserted in the same transaction as the change it describes, with the next
//! sequence and the hash that chains it to the entry before. There is no code here, or
//! anywhere, that updates or deletes a row of this table (FR-048).

use sea_orm::{ActiveModelTrait, EntityTrait, QueryOrder, Set};
use trcli_application::ports::audit::AuditLog;
use trcli_application::ports::unit_of_work::StoreError;
use trcli_domain::governance::audit::{
    AuditAction, AuditDraft, AuditEntry, AuditHead, Change, GENESIS_HASH, Stamp, from_hex, to_hex,
};
use trcli_domain::shared::text::ActorName;

use crate::convert::{corrupt, from_millis, store_error, to_id, to_millis};
use crate::digest::Sha256Digest;
use crate::entities::audit_entry::{ActiveModel, Column, Entity, Model};
use crate::unit_of_work::SqliteUnit;

/// The changes of an entry as the JSON text its row holds.
fn changes_to_json(changes: &[Change]) -> String {
    let items: Vec<serde_json::Value> = changes
        .iter()
        .map(|change| serde_json::json!({ "field": change.field, "before": change.before, "after": change.after }))
        .collect();
    serde_json::Value::Array(items).to_string()
}

/// The changes of an entry read from its row.
fn changes_from_json(text: &str) -> Result<Vec<Change>, StoreError> {
    let unreadable = || corrupt("an audit entry's changes cannot be read");
    let items: Vec<serde_json::Value> = serde_json::from_str(text).map_err(|_| unreadable())?;
    let text_of = |item: &serde_json::Value, key: &str| {
        item.get(key)
            .and_then(|value| value.as_str())
            .map(str::to_owned)
    };
    items
        .iter()
        .map(|item| {
            Ok(Change {
                field: text_of(item, "field").ok_or_else(unreadable)?,
                before: text_of(item, "before"),
                after: text_of(item, "after"),
            })
        })
        .collect()
}

/// The row of an entry.
fn to_row(entry: &AuditEntry) -> ActiveModel {
    ActiveModel {
        sequence: Set(entry.sequence as i64),
        at: Set(to_millis(entry.at)),
        actor: Set(entry.actor.to_string()),
        action: Set(entry.action.to_string()),
        kind: Set(entry.kind.clone()),
        record_id: Set(entry.record_id.map(|id| id.to_string())),
        handle: Set(entry.handle.clone()),
        display_name: Set(entry.display_name.clone()),
        changes: Set(changes_to_json(&entry.changes)),
        previous_hash: Set(to_hex(&entry.previous_hash)),
        hash: Set(to_hex(&entry.hash)),
    }
}

/// The entry of a row.
///
/// A row that cannot be read as an entry is reported as damage: no version of the tool
/// writes one, so it was changed outside the tool.
pub(super) fn to_entry(row: Model) -> Result<AuditEntry, StoreError> {
    let hash =
        |text: &str| from_hex(text).ok_or_else(|| corrupt("an audit entry's hash is not a hash"));
    Ok(AuditEntry {
        sequence: row.sequence as u64,
        at: from_millis(row.at)?,
        actor: ActorName::new(&row.actor)
            .map_err(|_| corrupt("an audit entry's actor is not a name"))?,
        action: AuditAction::named(&row.action)
            .ok_or_else(|| corrupt("an audit entry's action is not an action"))?,
        kind: row.kind,
        record_id: row.record_id.as_deref().map(to_id).transpose()?,
        handle: row.handle,
        display_name: row.display_name,
        changes: changes_from_json(&row.changes)?,
        previous_hash: hash(&row.previous_hash)?,
        hash: hash(&row.hash)?,
    })
}

impl AuditLog for SqliteUnit {
    async fn record(&mut self, stamp: &Stamp, draft: AuditDraft) -> Result<AuditEntry, StoreError> {
        let last = Entity::find()
            .order_by_desc(Column::Sequence)
            .one(&self.transaction)
            .await
            .map_err(store_error)?;
        // Only the sequence and the hash of the last row are needed; a row altered outside
        // the tool is still chained from, and reported by verification, not here.
        let (sequence, previous) = match last {
            Some(row) => (
                row.sequence as u64 + 1,
                from_hex(&row.hash).unwrap_or(GENESIS_HASH),
            ),
            None => (1, GENESIS_HASH),
        };
        let entry = AuditEntry::seal(sequence, stamp, draft, previous, Sha256Digest::of);
        to_row(&entry)
            .insert(&self.transaction)
            .await
            .map_err(store_error)?;
        self.recorded = Some(AuditHead::of(&entry));
        Ok(entry)
    }
}

//! Contract of reading the audit trail and of the telemetry log (T104; FR-049, FR-052).
//!
//! Run it against a storage adapter by calling [`run`] with a function that makes fresh,
//! empty storage each time it is called.

use time::{Date, Duration, Month};
use trcli_domain::governance::audit::{AuditAction, AuditDraft, Change, Stamp};
use trcli_domain::governance::telemetry::TelemetryRecord;
use trcli_domain::shared::text::ActorName;

use super::environment::{SeededIds, stamp};
use crate::ports::audit::{AuditFilter, AuditLog, AuditQuery, TelemetryLog};
use crate::ports::unit_of_work::{Storage, UnitOfWork};

/// A stamp `days` after the fixed moment (2026-10-08), by the named actor.
fn stamp_on(days: i64, actor: &str) -> Stamp {
    Stamp::new(
        stamp().at + Duration::days(days),
        ActorName::new(actor).expect("a valid name"),
    )
}

/// A day of October 2026.
fn october(day: u8) -> Date {
    Date::from_calendar_date(2026, Month::October, day).expect("a valid date")
}

/// Records five entries: two records of two kinds, two actors, three days.
async fn seeded<S>(storage: &S)
where
    S: Storage,
    S::Unit: AuditLog + AuditQuery,
{
    let alpha = |action| AuditDraft::record(action, "alpha", SeededIds::at(0), "alp-0001", "First");
    let beta = |action| AuditDraft::record(action, "beta", SeededIds::at(1), "bet-0002", "Second");
    let change = vec![Change::new("title", Some("Frist"), Some("First"))];
    let mut unit = storage.begin().await.expect("begin");
    unit.record(&stamp_on(0, "ana"), alpha(AuditAction::CREATE))
        .await
        .expect("record");
    unit.record(&stamp_on(0, "bob"), beta(AuditAction::CREATE))
        .await
        .expect("record");
    unit.record(
        &stamp_on(1, "ana"),
        alpha(AuditAction::UPDATE).with_changes(change),
    )
    .await
    .expect("record");
    unit.record(&stamp_on(2, "ana"), alpha(AuditAction::TAG))
        .await
        .expect("record");
    unit.record(&stamp_on(2, "bob"), beta(AuditAction::DELETE))
        .await
        .expect("record");
    unit.commit().await.expect("commit");
}

/// The sequences of the entries a filter gives.
async fn sequences<U: AuditQuery>(unit: &U, filter: AuditFilter) -> Vec<u64> {
    unit.entries(&filter)
        .await
        .expect("entries")
        .entries
        .iter()
        .map(|entry| entry.sequence)
        .collect()
}

/// Runs every case of the contract, each on fresh storage.
pub async fn run<S>(fresh: impl AsyncFn() -> S)
where
    S: Storage,
    S::Unit: AuditLog + AuditQuery + TelemetryLog,
{
    entries_come_newest_first_with_a_limit_and_a_total(&fresh().await).await;
    entries_are_filtered_by_record_kind_actor_action_and_dates(&fresh().await).await;
    an_entry_is_read_back_exactly_as_recorded(&fresh().await).await;
    telemetry_round_trips(&fresh().await).await;
}

/// Newest first; the limit cuts the page, not the total.
async fn entries_come_newest_first_with_a_limit_and_a_total<S>(storage: &S)
where
    S: Storage,
    S::Unit: AuditLog + AuditQuery,
{
    seeded(storage).await;
    let unit = storage.read().await.expect("read");
    assert_eq!(
        sequences(&unit, AuditFilter::everything(10)).await,
        [5, 4, 3, 2, 1]
    );
    let page = unit
        .entries(&AuditFilter::everything(2))
        .await
        .expect("entries");
    assert_eq!((page.entries.len(), page.total), (2, 5));
    assert_eq!(unit.entry_count().await.expect("count"), 5);
    let middle: Vec<u64> = unit
        .entries_after(2, 2)
        .await
        .expect("entries")
        .iter()
        .map(|e| e.sequence)
        .collect();
    assert_eq!(middle, [3, 4]);
}

/// Each filter narrows; filters combine.
async fn entries_are_filtered_by_record_kind_actor_action_and_dates<S>(storage: &S)
where
    S: Storage,
    S::Unit: AuditLog + AuditQuery,
{
    seeded(storage).await;
    let unit = storage.read().await.expect("read");
    let all = || AuditFilter::everything(10);
    assert_eq!(
        sequences(
            &unit,
            AuditFilter {
                record: Some(SeededIds::at(0)),
                ..all()
            }
        )
        .await,
        [4, 3, 1]
    );
    assert_eq!(
        sequences(
            &unit,
            AuditFilter {
                kind: Some("beta".into()),
                ..all()
            }
        )
        .await,
        [5, 2]
    );
    assert_eq!(
        sequences(
            &unit,
            AuditFilter {
                actor: Some("bob".into()),
                ..all()
            }
        )
        .await,
        [5, 2]
    );
    assert_eq!(
        sequences(
            &unit,
            AuditFilter {
                action: Some(AuditAction::CREATE),
                ..all()
            }
        )
        .await,
        [2, 1]
    );
    assert_eq!(
        sequences(
            &unit,
            AuditFilter {
                from: Some(october(9)),
                ..all()
            }
        )
        .await,
        [5, 4, 3]
    );
    assert_eq!(
        sequences(
            &unit,
            AuditFilter {
                to: Some(october(8)),
                ..all()
            }
        )
        .await,
        [2, 1]
    );
    let combined = AuditFilter {
        actor: Some("ana".into()),
        from: Some(october(9)),
        to: Some(october(9)),
        ..all()
    };
    assert_eq!(sequences(&unit, combined).await, [3]);
}

/// Every field of an entry survives storage, so that its hash can be recomputed.
async fn an_entry_is_read_back_exactly_as_recorded<S>(storage: &S)
where
    S: Storage,
    S::Unit: AuditLog + AuditQuery,
{
    let changes = vec![
        Change::new("title", None, Some("Ação — 東京")),
        Change::new("status", Some(""), None),
    ];
    let draft = AuditDraft::record(
        AuditAction::UPDATE,
        "alpha",
        SeededIds::at(4),
        "alp-0005",
        "Ação",
    )
    .with_changes(changes);
    let mut unit = storage.begin().await.expect("begin");
    let workspace_level = unit
        .record(&stamp(), AuditDraft::workspace(AuditAction::UPGRADE))
        .await
        .expect("record");
    let about_a_record = unit.record(&stamp(), draft).await.expect("record");
    unit.commit().await.expect("commit");

    let stored = storage
        .read()
        .await
        .expect("read")
        .entries_after(0, 10)
        .await
        .expect("entries");
    assert_eq!(stored, vec![workspace_level, about_a_record]);
}

/// Telemetry records are kept as given, in order.
async fn telemetry_round_trips<S>(storage: &S)
where
    S: Storage,
    S::Unit: TelemetryLog,
{
    let first = TelemetryRecord::new(stamp().at, &["workspace", "show"], 12, "success");
    let second = TelemetryRecord::new(
        stamp().at + Duration::SECOND,
        &["init"],
        40,
        "invalid_input",
    );
    let mut unit = storage.begin().await.expect("begin");
    unit.append_telemetry(&first).await.expect("append");
    unit.append_telemetry(&second).await.expect("append");
    unit.commit().await.expect("commit");
    assert_eq!(
        storage
            .read()
            .await
            .expect("read")
            .telemetry()
            .await
            .expect("telemetry"),
        vec![first, second]
    );
}

#[cfg(test)]
mod tests {
    //! The fake passes the contract it is used in place of.

    use crate::testing::block_on;
    use crate::testing::unit::FakeStorage;

    #[test]
    fn the_in_memory_storage_passes_the_audit_contract() {
        block_on(super::run(async || FakeStorage::new()));
    }
}

//! Unit tests for local telemetry.

use trcli_domain::governance::telemetry::TelemetryRecord;

use trcli_application::governance::telemetry_summary::{record_use, summarise};
use trcli_application::ports::unit_of_work::Storage;
use trcli_testing::block_on;
use trcli_testing::environment::stamp;
use trcli_testing::unit::FakeStorage;

/// A use of a command.
fn used(command: &[&str], duration_ms: u64, outcome: &str) -> TelemetryRecord {
    TelemetryRecord::new(stamp().at, command, duration_ms, outcome)
}

#[test]
fn nothing_is_recorded_while_telemetry_is_off() {
    block_on(async {
        let mut unit = FakeStorage::new().begin().await.expect("begin");
        record_use(
            &mut unit,
            false,
            &used(&["workspace", "show"], 5, "success"),
        )
        .await
        .expect("skipped");
        assert!(unit.state().telemetry.is_empty());
        record_use(&mut unit, true, &used(&["workspace", "show"], 5, "success"))
            .await
            .expect("recorded");
        assert_eq!(unit.state().telemetry.len(), 1);
    });
}

#[test]
fn the_summary_counts_runs_successes_and_durations_per_command() {
    let summary = block_on(async {
        let mut unit = FakeStorage::new().begin().await.expect("begin");
        for record in [
            used(&["workspace", "show"], 10, "success"),
            used(&["workspace", "show"], 30, "workspace_problem"),
            used(&["init"], 40, "success"),
        ] {
            record_use(&mut unit, true, &record)
                .await
                .expect("recorded");
        }
        summarise(&unit, true).await.expect("summarised")
    });
    assert_eq!(summary.total, 3);
    let show = &summary.items[0];
    assert_eq!(
        (show.command.as_str(), show.runs, show.succeeded),
        ("workspace show", 2, 1)
    );
    assert_eq!((show.mean_ms, show.longest_ms), (20, 30));
}

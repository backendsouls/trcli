//! Local telemetry: recording a command's use, and summarising what was recorded
//! (FR-052, FR-053).
//!
//! A record is written only while telemetry is enabled, and holds only the command path,
//! its duration, and how it ended. Nothing here, or anywhere, sends it out.

use std::collections::BTreeMap;

use serde::Serialize;
use trcli_domain::governance::telemetry::TelemetryRecord;

use crate::outcome::Problem;
use crate::ports::audit::TelemetryLog;
use crate::ports::unit_of_work::StoreError;

/// Records one use of a command, unless telemetry is turned off.
pub async fn record_use<U: TelemetryLog>(
    unit: &mut U,
    enabled: bool,
    record: &TelemetryRecord,
) -> Result<(), StoreError> {
    if !enabled {
        return Ok(());
    }
    unit.append_telemetry(record).await
}

/// How one command has been used.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct CommandUse {
    /// The command path.
    pub command: String,
    /// How many times it ran.
    pub runs: u64,
    /// How many of those succeeded.
    pub succeeded: u64,
    /// The mean duration, in milliseconds.
    pub mean_ms: u64,
    /// The longest duration, in milliseconds.
    pub longest_ms: u64,
}

/// The summary `telemetry show` presents.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct TelemetrySummary {
    /// Whether telemetry is being recorded.
    pub enabled: bool,
    /// How many uses are recorded in all.
    pub total: u64,
    /// One line per command, most used first.
    pub items: Vec<CommandUse>,
}

/// Summarises what was recorded in this workspace.
pub async fn summarise<U: TelemetryLog>(
    unit: &U,
    enabled: bool,
) -> Result<TelemetrySummary, Problem> {
    let records = unit.telemetry().await?;
    let mut by_command: BTreeMap<&str, Vec<&TelemetryRecord>> = BTreeMap::new();
    for record in &records {
        by_command
            .entry(record.command.as_str())
            .or_default()
            .push(record);
    }
    let mut items: Vec<CommandUse> = by_command
        .into_iter()
        .map(|(command, uses)| command_use(command, &uses))
        .collect();
    items.sort_by(|one, other| {
        other
            .runs
            .cmp(&one.runs)
            .then(one.command.cmp(&other.command))
    });
    Ok(TelemetrySummary {
        enabled,
        total: records.len() as u64,
        items,
    })
}

/// The figures of one command.
fn command_use(command: &str, uses: &[&TelemetryRecord]) -> CommandUse {
    let runs = uses.len() as u64;
    let total_ms: u64 = uses.iter().map(|record| record.duration_ms).sum();
    CommandUse {
        command: command.to_owned(),
        runs,
        succeeded: uses
            .iter()
            .filter(|record| record.outcome == "success")
            .count() as u64,
        mean_ms: total_ms.checked_div(runs).unwrap_or(0),
        longest_ms: uses
            .iter()
            .map(|record| record.duration_ms)
            .max()
            .unwrap_or(0),
    }
}

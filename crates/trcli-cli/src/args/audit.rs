//! `trcli audit …` and `trcli telemetry …`, as in `contracts/cli-audit.md`.
//!
//! Note what is absent: `audit` has no `add`, `edit`, or `rm` (FR-048).

use clap::{Args, Subcommand};

/// The example shown by `trcli audit --help`.
pub const AUDIT_EXAMPLE: &str = "\
Example:
  trcli audit list --action create --from 2026-10-01
  trcli audit verify
  trcli audit export --to audit.md

Guide: docs/usage/audit.md";

/// The example shown by `trcli telemetry --help`.
pub const TELEMETRY_EXAMPLE: &str = "\
Example:
  trcli telemetry show
  trcli telemetry off

Guide: docs/usage/audit.md";

/// The verbs of `trcli audit`.
#[derive(Clone, Debug, Subcommand)]
pub enum AuditCommand {
    /// List the entries that match every filter given, newest first
    #[command(
        after_long_help = "Example:\n  trcli audit list --limit 10\n  trcli audit list --record ref-7k3f --action update"
    )]
    List(AuditFilters),

    /// Check that the trail has not been altered or shortened outside the tool
    #[command(after_long_help = "Example:\n  trcli audit verify")]
    Verify,

    /// Write the matching entries to a file as a report
    #[command(
        after_long_help = "Example:\n  trcli audit export --to audit.md\n  trcli audit export --from 2026-10-01 --until 2026-10-31 --to october.csv --format csv"
    )]
    Export(ExportArgs),
}

/// The filters of `trcli audit list`.
#[derive(Clone, Debug, Default, Args)]
pub struct AuditFilters {
    /// Only entries about this record (short name or a unique beginning of it)
    #[arg(long, value_name = "REF")]
    pub record: Option<String>,

    /// Only entries about records of this kind
    #[arg(long, value_name = "KIND")]
    pub kind: Option<String>,

    /// Only entries made by this actor
    #[arg(long, value_name = "NAME")]
    pub actor: Option<String>,

    /// Only entries of this action, for example create, update, delete, tag, setting
    #[arg(long, value_name = "ACTION")]
    pub action: Option<String>,

    /// Only entries made on or after this day (YYYY-MM-DD)
    #[arg(long, value_name = "DATE")]
    pub from: Option<String>,

    /// Only entries made on or before this day (YYYY-MM-DD)
    #[arg(long, visible_alias = "until", value_name = "DATE")]
    pub to: Option<String>,

    /// Show at most this many entries (1 to 1000) [default: the output.page_size setting]
    #[arg(long, value_name = "N")]
    pub limit: Option<String>,
}

/// The arguments of `trcli audit export`.
///
/// The filters are those of `audit list`, with one difference: here `--to` names the file
/// to write, so the last day is given with `--until`.
#[derive(Clone, Debug, Args)]
pub struct ExportArgs {
    /// Only entries about this record (short name or a unique beginning of it)
    #[arg(long, value_name = "REF")]
    pub record: Option<String>,

    /// Only entries about records of this kind
    #[arg(long, value_name = "KIND")]
    pub kind: Option<String>,

    /// Only entries made by this actor
    #[arg(long, value_name = "NAME")]
    pub actor: Option<String>,

    /// Only entries of this action, for example create, update, delete, tag, setting
    #[arg(long, value_name = "ACTION")]
    pub action: Option<String>,

    /// Only entries made on or after this day (YYYY-MM-DD)
    #[arg(long, value_name = "DATE")]
    pub from: Option<String>,

    /// Only entries made on or before this day (YYYY-MM-DD)
    #[arg(long, value_name = "DATE")]
    pub until: Option<String>,

    /// File to write the report to
    #[arg(long, value_name = "FILE")]
    pub to: Option<String>,

    /// Form of the report: markdown, json, or csv [default: markdown]
    #[arg(long, value_name = "FORMAT")]
    pub format: Option<String>,
}

/// The verbs of `trcli telemetry`.
#[derive(Clone, Debug, Subcommand)]
pub enum TelemetryCommand {
    /// Show which commands were used, how often, how long they took, and how they ended
    #[command(after_long_help = "Example:\n  trcli telemetry show")]
    Show,

    /// Turn local telemetry on
    #[command(after_long_help = "Example:\n  trcli telemetry on")]
    On,

    /// Turn local telemetry off; the audit trail continues unchanged
    #[command(after_long_help = "Example:\n  trcli telemetry off")]
    Off,

    /// Say whether local telemetry is on
    #[command(after_long_help = "Example:\n  trcli telemetry status")]
    Status,
}

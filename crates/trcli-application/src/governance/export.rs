//! Handing a range of the trail to someone else (FR-049, FR-046).
//!
//! The use case reads the matching entries and records that an export was made. Turning
//! the entries into Markdown, JSON, or CSV is presentation, and writing the file is the
//! system's business; neither is done here.

use std::path::PathBuf;

use serde::Serialize;
use trcli_domain::governance::audit::{AuditAction, AuditDraft, Change, Stamp};
use trcli_domain::shared::problem::Rejection;

use super::query::{AuditList, EntryView};
use crate::outcome::Problem;
use crate::ports::audit::{AuditFilter, AuditLog, AuditQuery};
use crate::validation::{Checker, Valid, one_of};

/// The forms a report of the trail can take.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExportFormat {
    /// A document for reading.
    Markdown,
    /// One JSON array.
    Json,
    /// Comma-separated values, for a spreadsheet.
    Csv,
}

impl ExportFormat {
    /// The names accepted by `--format`.
    pub const NAMES: [&'static str; 3] = ["markdown", "json", "csv"];

    /// The format's name.
    pub fn name(self) -> &'static str {
        match self {
            Self::Markdown => "markdown",
            Self::Json => "json",
            Self::Csv => "csv",
        }
    }

    /// The format with this name.
    fn named(name: &str) -> Self {
        match name {
            "json" => Self::Json,
            "csv" => Self::Csv,
            _ => Self::Markdown,
        }
    }
}

/// A checked request to export the trail.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ExportCommand {
    /// Where to write the report.
    pub to: PathBuf,
    /// The form of the report.
    pub format: ExportFormat,
}

impl ExportCommand {
    /// Checks the destination and the format with the caller's checker, so that their
    /// problems are reported together with those of the filters. `None` means a problem
    /// was recorded.
    pub fn check(checker: &mut Checker, to: Option<&str>, format: Option<&str>) -> Option<Self> {
        let to = checker.required("--to", to, |path| {
            if path.trim().is_empty() {
                Err(Rejection::new("must not be empty", "the path of a file"))
            } else {
                Ok(PathBuf::from(path))
            }
        });
        let format = checker.optional("--format", format, |format| {
            one_of(format, &ExportFormat::NAMES)
        });
        Some(Self {
            to: to?,
            format: format?.map_or(ExportFormat::Markdown, |name| ExportFormat::named(&name)),
        })
    }

    /// Checks the destination and the format on their own.
    pub fn new(to: Option<&str>, format: Option<&str>) -> Result<Valid<Self>, Problem> {
        let mut checker = Checker::new();
        let command = Self::check(&mut checker, to, format);
        checker.finish(|| command.expect("checked"))
    }
}

/// What `audit export` reports.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct Exported {
    /// The file written.
    pub file: String,
    /// The form of the report.
    pub format: String,
    /// How many entries it holds.
    pub entries: u64,
}

/// Reads every entry that passes the filter, oldest first: the order a report is read in.
pub async fn collect<U: AuditQuery>(
    unit: &U,
    filter: &AuditFilter,
) -> Result<Vec<EntryView>, Problem> {
    // An export is not a page: it takes everything that matches.
    let everything = AuditFilter {
        limit: u32::MAX,
        ..filter.clone()
    };
    let AuditList { mut items, .. } = super::query::list(unit, &everything).await?;
    items.reverse();
    Ok(items)
}

/// Records that the export was made, once the file has been written (FR-046).
pub async fn record_export<U: AuditLog>(
    unit: &mut U,
    stamp: &Stamp,
    command: &ExportCommand,
    entries: u64,
) -> Result<Exported, Problem> {
    let file = command.to.display().to_string();
    let changes = vec![
        Change::set("entries", &entries.to_string()),
        Change::set("format", command.format.name()),
    ];
    let draft = AuditDraft::workspace(AuditAction::EXPORT)
        .named(&format!("audit trail to {file}"))
        .with_changes(changes);
    unit.record(stamp, draft).await?;
    Ok(Exported {
        file,
        format: command.format.name().to_owned(),
        entries,
    })
}

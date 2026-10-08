//! The audit trail as a report to hand to someone else (FR-049): Markdown for reading,
//! JSON for programs, CSV for a spreadsheet.

use trcli_application::governance::export::ExportFormat;
use trcli_application::governance::query::EntryView;

/// The report of these entries in the given form.
pub fn report(entries: &[EntryView], format: ExportFormat, workspace_name: &str) -> String {
    match format {
        ExportFormat::Markdown => markdown(entries, workspace_name),
        ExportFormat::Json => json(entries),
        ExportFormat::Csv => csv(entries),
    }
}

/// A document with one table row per entry.
fn markdown(entries: &[EntryView], workspace_name: &str) -> String {
    let mut text = format!(
        "# Audit trail of \"{workspace_name}\"\n\n{} entries.\n\n",
        entries.len()
    );
    text.push_str("| Seq | When (UTC) | Actor | Action | What |\n|----:|------------|-------|--------|------|\n");
    for entry in entries {
        // A pipe inside a value would end the cell early.
        let cell = |value: &str| value.replace('|', "\\|").replace('\n', " ");
        text.push_str(&format!(
            "| {} | {} | {} | {} | {} |\n",
            entry.sequence,
            entry.at.to_rfc3339(),
            cell(&entry.actor),
            entry.action,
            cell(&entry.what("->")),
        ));
    }
    text
}

/// One JSON array holding every entry in full, hashes included.
fn json(entries: &[EntryView]) -> String {
    let mut text = serde_json::to_string_pretty(entries).unwrap_or_else(|_| "[]".to_owned());
    text.push('\n');
    text
}

/// One line per entry; the changes are joined into one field.
fn csv(entries: &[EntryView]) -> String {
    let mut text = "sequence,at,actor,action,kind,handle,display_name,changes,hash\n".to_owned();
    for entry in entries {
        let changes: Vec<String> = entry
            .changes
            .iter()
            .map(|change| {
                let (before, after) = (
                    change.before.as_deref().unwrap_or(""),
                    change.after.as_deref().unwrap_or(""),
                );
                format!("{}: {before} -> {after}", change.field)
            })
            .collect();
        let fields = [
            entry.sequence.to_string(),
            entry.at.to_rfc3339(),
            entry.actor.clone(),
            entry.action.clone(),
            entry.kind.clone().unwrap_or_default(),
            entry.handle.clone().unwrap_or_default(),
            entry.display_name.clone().unwrap_or_default(),
            changes.join("; "),
            entry.hash.clone(),
        ];
        text.push_str(
            &fields
                .iter()
                .map(|field| csv_field(field))
                .collect::<Vec<_>>()
                .join(","),
        );
        text.push('\n');
    }
    text
}

/// A CSV field: quoted when it holds a comma, a quote, or a line break; quotes doubled.
fn csv_field(value: &str) -> String {
    if value.contains([',', '"', '\n', '\r']) {
        format!("\"{}\"", value.replace('"', "\"\""))
    } else {
        value.to_owned()
    }
}

#[cfg(test)]
mod tests {
    //! Unit tests for the exported report.

    use time::OffsetDateTime;
    use trcli_application::governance::export::ExportFormat;
    use trcli_application::governance::query::{ChangeView, EntryView};

    use super::report;

    /// An entry about a record whose name holds a comma, a quote, and a pipe.
    fn entry() -> EntryView {
        EntryView {
            sequence: 3,
            at: OffsetDateTime::UNIX_EPOCH.into(),
            actor: "ana".into(),
            action: "update".into(),
            kind: Some("reference".into()),
            record_id: None,
            handle: Some("ref-7k3f".into()),
            display_name: Some("Rain, \"heavy\" | cold".into()),
            changes: vec![ChangeView {
                field: "title".into(),
                before: Some("Rain".into()),
                after: None,
            }],
            hash: "ab".repeat(32),
        }
    }

    #[test]
    fn markdown_is_a_table_with_one_row_per_entry() {
        let text = report(&[entry()], ExportFormat::Markdown, "Doctorate");
        assert!(text.starts_with("# Audit trail of \"Doctorate\"\n\n1 entries.\n"));
        assert!(text.contains("| 3 | 1970-01-01T00:00:00Z | ana | update | ref-7k3f \"Rain, \"heavy\" \\| cold\" title: Rain -> (none) |"));
    }

    #[test]
    fn json_is_one_array_with_every_field() {
        let text = report(&[entry()], ExportFormat::Json, "Doctorate");
        let parsed: serde_json::Value = serde_json::from_str(&text).expect("valid JSON");
        assert_eq!(parsed[0]["sequence"], 3);
        assert_eq!(parsed[0]["changes"][0]["after"], serde_json::Value::Null);
        assert_eq!(parsed[0]["hash"].as_str().map(str::len), Some(64));
    }

    #[test]
    fn csv_quotes_what_needs_quoting() {
        let text = report(&[entry()], ExportFormat::Csv, "Doctorate");
        let line = text.lines().nth(1).expect("a line");
        assert!(line.starts_with("3,1970-01-01T00:00:00Z,ana,update,reference,ref-7k3f,\"Rain, \"\"heavy\"\" | cold\",title: Rain -> ,"));
        assert_eq!(
            text.lines().next(),
            Some("sequence,at,actor,action,kind,handle,display_name,changes,hash")
        );
    }
}

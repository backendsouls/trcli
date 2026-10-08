//! The form for people of the audit trail's and the telemetry's views.

use trcli_application::governance::export::Exported;
use trcli_application::governance::query::AuditList;
use trcli_application::governance::telemetry_summary::TelemetrySummary;

use crate::render::Render;
use crate::render::human::{Cell, Human};

impl Render for AuditList {
    fn render(&self, out: &mut Human) {
        if self.items.is_empty() {
            out.notice("No audit entries match.");
            return;
        }
        let arrow = out.symbols.arrow;
        let rows: Vec<Vec<Cell>> = self
            .items
            .iter()
            .map(|entry| {
                vec![
                    Cell::plain(entry.sequence.to_string()),
                    Cell::muted(out.moment(entry.at)),
                    Cell::plain(&entry.actor),
                    Cell::plain(&entry.action),
                    Cell::plain(entry.what(arrow)),
                ]
            })
            .collect();
        out.table(&["SEQ", "WHEN", "ACTOR", "ACTION", "WHAT"], &rows, 4);
        out.limited(self.items.len(), self.total);
    }
}

impl Render for Exported {
    fn render(&self, out: &mut Human) {
        out.success(&format!("Exported {} audit entries to {} ({})", self.entries, self.file, self.format));
    }
}

impl Render for TelemetrySummary {
    fn render(&self, out: &mut Human) {
        let state = if self.enabled { "on" } else { "off" };
        out.line(format!("Telemetry is {state}. It is kept in this workspace only and is never sent anywhere."));
        if self.items.is_empty() {
            out.notice("No use of the tool has been recorded in this workspace.");
            return;
        }
        let rows: Vec<Vec<Cell>> = self
            .items
            .iter()
            .map(|item| {
                vec![
                    Cell::plain(&item.command),
                    Cell::plain(item.runs.to_string()),
                    Cell::plain(item.succeeded.to_string()),
                    Cell::plain(format!("{} ms", item.mean_ms)),
                    Cell::plain(format!("{} ms", item.longest_ms)),
                ]
            })
            .collect();
        out.table(&["COMMAND", "RUNS", "SUCCEEDED", "MEAN", "LONGEST"], &rows, 0);
    }
}

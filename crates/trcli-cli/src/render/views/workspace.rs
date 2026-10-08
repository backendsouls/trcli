//! The form for people of the workspace's views.

use trcli_application::workspace::check::CheckReport;
use trcli_application::workspace::init::WorkspaceCreated;
use trcli_application::workspace::show::WorkspaceView;
use trcli_application::workspace::upgrade::UpgradeReport;

use crate::render::Render;
use crate::render::human::Human;

impl Render for WorkspaceCreated {
    fn render(&self, out: &mut Human) {
        out.success(&format!(
            "Created workspace \"{}\" in {}",
            self.name, self.location
        ));
    }
}

impl Render for WorkspaceView {
    fn render(&self, out: &mut Human) {
        out.heading(&format!("Workspace \"{}\"", self.name));
        let description = if self.description.is_empty() {
            "(none)".to_owned()
        } else {
            self.description.clone()
        };
        out.fields(&[
            ("Description", description),
            ("Location", self.location.clone()),
            ("Format", self.format_version.to_string()),
            ("Created", out.moment_with_zone(self.created_at)),
        ]);
        out.heading("Records");
        if self.records.is_empty() {
            out.line("  (this version of trcli has no kinds of record yet)");
            return;
        }
        let counts: Vec<(&str, String)> = self
            .records
            .iter()
            .map(|count| (count.kind.as_str(), count.count.to_string()))
            .collect();
        out.fields(&counts);
    }
}

impl Render for UpgradeReport {
    fn render(&self, out: &mut Human) {
        let (from, to) = (self.from_format, self.to_format);
        if self.upgraded {
            out.success(&format!(
                "Upgraded the workspace from format {from} to format {to}."
            ));
            if let Some(backup) = &self.backup {
                out.line(format!(
                    "A copy of the workspace as it was is kept in {backup}"
                ));
            }
        } else if self.needed {
            out.line(format!("An upgrade is needed: the workspace is in format {from}; this version writes format {to}."));
            out.line("Run `trcli workspace upgrade` to upgrade; a copy is kept first.");
        } else {
            out.line(format!(
                "No upgrade is needed: the workspace is in format {from}."
            ));
        }
    }
}

impl Render for CheckReport {
    fn render(&self, out: &mut Human) {
        let check = out.symbols.check;
        out.success(&format!("{check} The stored data is consistent."));
        out.success(&format!(
            "{check} The audit trail is intact: {} entries verified.",
            self.audit_entries
        ));
    }
}

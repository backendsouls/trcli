//! The form for people of the settings' views.

use trcli_application::settings::commands::{SettingDetail, SettingsList, SettingsPaths};

use crate::render::Render;
use crate::render::human::{Cell, Human};

/// What stands for a setting that has no value.
const UNSET: &str = "(unset)";

impl Render for SettingsList {
    fn render(&self, out: &mut Human) {
        let rows: Vec<Vec<Cell>> = self
            .items
            .iter()
            .map(|row| {
                let value = row
                    .value
                    .as_ref()
                    .map_or_else(|| UNSET.to_owned(), ToString::to_string);
                vec![
                    Cell::plain(&row.key),
                    Cell::plain(value),
                    Cell::muted(&row.source_detail),
                ]
            })
            .collect();
        out.table(&["SETTING", "VALUE", "SOURCE"], &rows, 2);
    }
}

impl Render for SettingDetail {
    fn render(&self, out: &mut Human) {
        let shown = |value: &Option<_>| {
            value
                .as_ref()
                .map_or_else(|| UNSET.to_owned(), ToString::to_string)
        };
        out.heading(&self.key);
        out.fields(&[
            ("Meaning", self.summary.clone()),
            ("Allowed", self.allowed.clone()),
            ("Default", shown(&self.default)),
            ("Scope", self.scope.clone()),
            ("Value", shown(&self.value)),
            ("Source", self.source.clone()),
        ]);
    }
}

impl Render for SettingsPaths {
    fn render(&self, out: &mut Human) {
        let shown =
            |path: &Option<String>, absent: &str| path.clone().unwrap_or_else(|| absent.to_owned());
        out.fields(&[
            ("User", shown(&self.user, "(not known on this system)")),
            ("Workspace", shown(&self.workspace, "(not in a workspace)")),
        ]);
    }
}

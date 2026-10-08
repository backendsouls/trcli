//! The form for people of what every kind of record shares: lists, one record, tags,
//! notes, links, and deletion.

use trcli_application::records::delete::Deleted;
use trcli_application::records::link::{LinkList, LinkView, Linked};
use trcli_application::records::list_options::{RecordList, RecordRow};
use trcli_application::records::note::Noted;
use trcli_application::records::show::RecordDetail;
use trcli_application::records::tag::{TagList, Tagged};

use crate::render::Render;
use crate::render::human::{Cell, Human};
use crate::render::theme::Meaning;

impl Render for RecordList {
    fn render(&self, out: &mut Human) {
        if self.items.is_empty() {
            // An empty list is said in one line and is not a failure (FR-037).
            out.notice(format!("No {} records match.", self.kind));
            return;
        }
        let rows: Vec<Vec<Cell>> = self
            .items
            .iter()
            .map(|row| {
                vec![
                    Cell::handle(&row.handle),
                    Cell::plain(&row.name),
                    Cell::plain(row.tags.join(", ")),
                    Cell::muted(out.moment(row.updated_at)),
                ]
            })
            .collect();
        let name_heading = self.name_field.to_uppercase();
        out.table(&["HANDLE", &name_heading, "TAGS", "UPDATED"], &rows, 1);
        out.limited(self.items.len(), self.total);
    }
}

impl Render for RecordRow {
    fn render(&self, out: &mut Human) {
        let handle = out.paint(Meaning::Handle, &self.handle);
        out.success(&format!("Saved {} {handle} \"{}\"", self.kind, self.name));
    }
}

/// The lines that list a record's links.
fn link_lines(out: &mut Human, links: &[LinkView]) {
    let link = out.symbols.link;
    for view in links {
        let handle = out.paint(Meaning::Handle, &view.other.handle);
        out.line(format!(
            "  {link} {handle} \"{}\" ({}, {})",
            view.other.name, view.relation, view.other.kind
        ));
    }
}

impl Render for RecordDetail {
    fn render(&self, out: &mut Human) {
        let handle = out.paint(Meaning::Handle, &self.handle);
        out.line(format!("{handle} \"{}\"", self.name));
        let mut fields = vec![("Kind", self.kind.clone()), ("Id", self.id.clone())];
        // The kind's own fields, under the names the kind gives them.
        let own: Vec<(String, String)> = self
            .fields
            .iter()
            .map(|field| {
                (
                    capitalized(field.name),
                    field.value.clone().unwrap_or_else(|| "(none)".to_owned()),
                )
            })
            .collect();
        fields.extend(
            own.iter()
                .map(|(name, value)| (name.as_str(), value.clone())),
        );
        fields.push(("Created", out.moment_with_zone(self.created_at)));
        fields.push(("Updated", out.moment_with_zone(self.updated_at)));
        fields.push((
            "Tags",
            if self.tags.is_empty() {
                "(none)".to_owned()
            } else {
                self.tags.join(", ")
            },
        ));
        out.fields(&fields);
        if !self.notes.is_empty() {
            out.heading("Notes");
            for note in &self.notes {
                let when = out.paint(Meaning::Muted, &out.moment(note.created_at));
                out.line(format!(
                    "  {when}  {}",
                    note.body.replace('\n', "\n                    ")
                ));
            }
        }
        if !self.links.is_empty() {
            out.heading("Links");
            link_lines(out, &self.links);
        }
    }
}

/// A field's name with its first letter in upper case and underscores as spaces.
fn capitalized(name: &str) -> String {
    let spaced = name.replace('_', " ");
    let mut characters = spaced.chars();
    characters.next().map_or_else(String::new, |first| {
        first.to_uppercase().chain(characters).collect()
    })
}

impl Render for Tagged {
    fn render(&self, out: &mut Human) {
        let handle = out.paint(Meaning::Handle, &self.handle);
        let tags = if self.tags.is_empty() {
            "(none)".to_owned()
        } else {
            self.tags.join(", ")
        };
        if self.added.is_empty() && self.removed.is_empty() {
            out.line(format!(
                "Nothing to change: {handle} \"{}\" has tags: {tags}",
                self.name
            ));
        } else if !self.added.is_empty() {
            out.success(&format!(
                "Tagged {handle} \"{}\": {} (now: {tags})",
                self.name,
                self.added.join(", ")
            ));
        } else {
            out.success(&format!(
                "Removed from {handle} \"{}\": {} (now: {tags})",
                self.name,
                self.removed.join(", ")
            ));
        }
    }
}

impl Render for Noted {
    fn render(&self, out: &mut Human) {
        let handle = out.paint(Meaning::Handle, &self.handle);
        out.success(&format!(
            "Added a note to {handle} \"{}\" ({})",
            self.name,
            out.moment(self.created_at)
        ));
    }
}

impl Render for Linked {
    fn render(&self, out: &mut Human) {
        let verb = if self.removed { "Unlinked" } else { "Linked" };
        let (from, to) = (
            out.paint(Meaning::Handle, &self.from.handle),
            out.paint(Meaning::Handle, &self.to.handle),
        );
        let link = out.symbols.link;
        out.success(&format!(
            "{verb} {from} \"{}\" {link} {to} \"{}\" ({})",
            self.from.name, self.to.name, self.relation
        ));
    }
}

/// "1 link", "3 links".
fn counted_links(total: u64) -> String {
    if total == 1 {
        "1 link".to_owned()
    } else {
        format!("{total} links")
    }
}

impl Render for LinkList {
    fn render(&self, out: &mut Human) {
        if self.items.is_empty() {
            out.notice(format!(
                "{} \"{}\" has no links.",
                self.of.handle, self.of.name
            ));
            return;
        }
        let handle = out.paint(Meaning::Handle, &self.of.handle);
        out.line(format!(
            "{} of {} {handle} \"{}\":",
            counted_links(self.total),
            self.of.kind,
            self.of.name
        ));
        link_lines(out, &self.items);
    }
}

impl Render for TagList {
    fn render(&self, out: &mut Human) {
        if self.items.is_empty() {
            out.notice("No tags are in use.");
            return;
        }
        let rows: Vec<Vec<Cell>> = self
            .items
            .iter()
            .map(|item| {
                vec![
                    Cell::plain(&item.tag),
                    Cell::plain(item.records.to_string()),
                ]
            })
            .collect();
        out.table(&["TAG", "RECORDS"], &rows, 0);
    }
}

impl Render for Deleted {
    fn render(&self, out: &mut Human) {
        let handle = out.paint(Meaning::Handle, &self.handle);
        out.success(&format!("Deleted {} {handle} \"{}\"", self.kind, self.name));
        for removed in &self.removed {
            out.line(format!("  removed: {removed}"));
        }
    }
}

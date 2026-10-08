//! Adding a dated note to a record of any kind (FR-015).

use serde::Serialize;
use trcli_domain::governance::audit::{AuditAction, AuditDraft, Change, Stamp};
use trcli_domain::shared::note::Note;
use trcli_domain::shared::text::LongText;

use super::resolve::resolve;
use crate::outcome::Problem;
use crate::ports::audit::AuditLog;
use crate::ports::records::{NoteStore, RecordIndex, RecordResolver};
use crate::validation::{Checker, Valid};
use crate::view::Instant;

/// A checked request to add a note.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NoteCommand {
    /// What was typed to name the record.
    pub reference: String,
    /// The note's text: at least one character.
    pub body: LongText,
}

impl NoteCommand {
    /// Checks the note's text.
    pub fn new(reference: &str, text: &str) -> Result<Valid<Self>, Problem> {
        let mut checker = Checker::new();
        let body = checker.check("<text>", text, Note::body);
        checker.finish(|| Self {
            reference: reference.to_owned(),
            body: body.expect("checked"),
        })
    }
}

/// What adding a note reports.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct Noted {
    /// The record's short name.
    pub handle: String,
    /// What the record is called.
    pub name: String,
    /// The note's text.
    pub body: String,
    /// When the note was added.
    pub created_at: Instant,
}

/// Adds the note and records it in the audit trail.
pub async fn add_note<U>(
    unit: &mut U,
    stamp: &Stamp,
    kinds: &[String],
    command: NoteCommand,
) -> Result<Noted, Problem>
where
    U: RecordResolver + RecordIndex + NoteStore + AuditLog,
{
    let record = resolve(unit, &command.reference, kinds).await?;
    unit.add_note(&Note::new(record.id, command.body.clone(), stamp.at))
        .await?;
    unit.touch_record(record.id, stamp.at).await?;
    let draft = AuditDraft::record(
        AuditAction::NOTE,
        &record.kind,
        record.id,
        record.handle.as_str(),
        &record.display_name,
    )
    .with_changes(vec![Change::set("note", command.body.as_str())]);
    unit.record(stamp, draft).await?;
    Ok(Noted {
        handle: record.handle.to_string(),
        name: record.display_name,
        body: command.body.to_string(),
        created_at: stamp.at.into(),
    })
}

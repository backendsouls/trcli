//! The sample kind `sample-note`: a record with a body of text, which can be locked.
//!
//! It is the second sample kind, so that "the same for every kind" can be shown with two
//! (FR-013). Locking exists to show a deletion that a feature forbids: a locked note
//! cannot be deleted until it is unlocked (FR-012).

use trcli_domain::governance::audit::{Change, Stamp};
use trcli_domain::shared::note::Note;
use trcli_domain::shared::record::RecordId;
use trcli_domain::shared::text::LongText;

use crate::governance::record::ChangeSet;
use crate::kinds::{DeletionBlock, KindBehaviour, KindField, RecordKindDescriptor};
use crate::outcome::Problem;
use crate::ports::audit::AuditLog;
use crate::ports::environment::IdGenerator;
use crate::ports::records::{RecordIndex, RecordResolver, TagStore};
use crate::ports::unit_of_work::StoreError;
use crate::records::create::{register, update};
use crate::records::list_options::{RecordRow, row};
use crate::records::resolve::resolve;
use crate::validation::{Checker, Valid};

/// The kind's name.
pub const KIND: &str = "sample-note";

/// How many characters of the body name the note in lists and messages.
const NAME_LENGTH: usize = 60;

/// What the foundation needs to know about sample notes.
pub fn descriptor() -> RecordKindDescriptor {
    RecordKindDescriptor::new(
        KIND,
        "smp",
        "Sample records with a body of text, used to try the tool out.",
        "body",
    )
}

/// A sample note's own row.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SampleNoteRow {
    /// The note's text.
    pub body: String,
    /// Whether the note is locked against deletion.
    pub locked: bool,
}

/// The rows sample notes keep for themselves.
pub trait SampleNoteStore {
    /// Stores a note's row, replacing the one it had.
    async fn save_sample_note(
        &mut self,
        id: RecordId,
        row: &SampleNoteRow,
    ) -> Result<(), StoreError>;

    /// A note's row.
    async fn sample_note(&self, id: RecordId) -> Result<Option<SampleNoteRow>, StoreError>;

    /// Removes a note's row.
    async fn remove_sample_note(&mut self, id: RecordId) -> Result<(), StoreError>;
}

/// What a note is called: the first line of its body, shortened.
fn name_of(body: &str) -> String {
    body.lines()
        .next()
        .unwrap_or_default()
        .chars()
        .take(NAME_LENGTH)
        .collect()
}

/// What only sample notes know about themselves.
#[derive(Clone, Debug)]
pub struct SampleNotes {
    /// The kind's descriptor.
    descriptor: RecordKindDescriptor,
}

impl SampleNotes {
    /// The behaviour of the kind.
    pub fn new() -> Self {
        Self {
            descriptor: descriptor(),
        }
    }
}

impl Default for SampleNotes {
    fn default() -> Self {
        Self::new()
    }
}

impl<U: SampleNoteStore> KindBehaviour<U> for SampleNotes {
    fn descriptor(&self) -> &RecordKindDescriptor {
        &self.descriptor
    }

    async fn deletion_block(
        &self,
        unit: &U,
        id: RecordId,
    ) -> Result<Option<DeletionBlock>, StoreError> {
        let locked = unit.sample_note(id).await?.is_some_and(|row| row.locked);
        Ok(locked.then(|| DeletionBlock {
            reason: "it is locked, and deleting it would lose what it records".to_owned(),
            alternative:
                "unlock it first with `trcli sample-note edit <ref> --unlock`, then delete it"
                    .to_owned(),
        }))
    }

    async fn dependents(&self, _unit: &U, _id: RecordId) -> Result<Vec<String>, StoreError> {
        Ok(Vec::new())
    }

    async fn remove_rows(&self, unit: &mut U, id: RecordId) -> Result<(), StoreError> {
        unit.remove_sample_note(id).await
    }

    async fn fields(&self, unit: &U, id: RecordId) -> Result<Vec<KindField>, StoreError> {
        let row = unit.sample_note(id).await?;
        Ok(vec![
            KindField {
                name: "body",
                value: row.as_ref().map(|row| row.body.clone()),
            },
            KindField {
                name: "locked",
                value: row.map(|row| row.locked.to_string()),
            },
        ])
    }
}

/// A checked request to add a sample note.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AddSampleNote {
    /// The note's text.
    pub body: LongText,
    /// Lock the note against deletion.
    pub locked: bool,
}

impl AddSampleNote {
    /// Checks the body: required, at least one character.
    pub fn new(body: Option<&str>, locked: bool) -> Result<Valid<Self>, Problem> {
        let mut checker = Checker::new();
        let body = checker.required("--body", body, Note::body);
        checker.finish(|| Self {
            body: body.expect("checked"),
            locked,
        })
    }
}

/// Adds a sample note.
pub async fn add<U>(
    unit: &mut U,
    stamp: &Stamp,
    ids: &impl IdGenerator,
    command: AddSampleNote,
) -> Result<RecordRow, Problem>
where
    U: SampleNoteStore + RecordIndex + TagStore + AuditLog,
{
    let id = ids.next_id();
    let name = name_of(command.body.as_str());
    let changes = vec![
        Change::set("body", command.body.as_str()),
        Change::set("locked", &command.locked.to_string()),
    ];
    let record = register(unit, stamp, &descriptor(), id, (&name, changes)).await?;
    unit.save_sample_note(
        id,
        &SampleNoteRow {
            body: command.body.to_string(),
            locked: command.locked,
        },
    )
    .await?;
    row(unit, &record).await
}

/// A checked request to change a sample note.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EditSampleNote {
    /// What was typed to name the note.
    pub reference: String,
    /// The new body, when given.
    pub body: Option<LongText>,
    /// Lock (`true`) or unlock (`false`) the note, when asked.
    pub locked: Option<bool>,
}

impl EditSampleNote {
    /// Checks the body, when one is given.
    pub fn new(
        reference: &str,
        body: Option<&str>,
        locked: Option<bool>,
    ) -> Result<Valid<Self>, Problem> {
        let mut checker = Checker::new();
        let body = checker.optional("--body", body, Note::body);
        checker.finish(|| Self {
            reference: reference.to_owned(),
            body: body.flatten(),
            locked,
        })
    }
}

/// Changes only what was named; when nothing differs, nothing is recorded.
pub async fn edit<U>(
    unit: &mut U,
    stamp: &Stamp,
    command: EditSampleNote,
) -> Result<RecordRow, Problem>
where
    U: SampleNoteStore + RecordResolver + RecordIndex + TagStore + AuditLog,
{
    let record = resolve(unit, &command.reference, &[KIND.to_owned()]).await?;
    let current = unit
        .sample_note(record.id)
        .await?
        .ok_or_else(|| Problem::internal("the note's own row is missing"))?;
    let next = SampleNoteRow {
        body: command
            .body
            .map_or_else(|| current.body.clone(), |body| body.to_string()),
        locked: command.locked.unwrap_or(current.locked),
    };
    let mut changes = ChangeSet::new();
    changes
        .field("body", Some(&current.body), Some(&next.body))
        .field(
            "locked",
            Some(&current.locked.to_string()),
            Some(&next.locked.to_string()),
        );
    if changes.is_empty() {
        return row(unit, &record).await;
    }
    unit.save_sample_note(record.id, &next).await?;
    let new_name = (next.body != current.body).then(|| name_of(&next.body));
    let updated = update(
        unit,
        stamp,
        record,
        new_name.as_deref(),
        changes.into_changes(),
    )
    .await?;
    row(unit, &updated).await
}

#[cfg(feature = "test-support")]
impl SampleNoteStore for crate::testing::unit::FakeUnit {
    async fn save_sample_note(
        &mut self,
        id: RecordId,
        row: &SampleNoteRow,
    ) -> Result<(), StoreError> {
        let fields = [
            ("body".to_owned(), row.body.clone()),
            ("locked".to_owned(), row.locked.to_string()),
        ];
        self.put_kind_row(KIND, id, fields.into());
        Ok(())
    }

    async fn sample_note(&self, id: RecordId) -> Result<Option<SampleNoteRow>, StoreError> {
        Ok(self.kind_row(KIND, id).map(|row| SampleNoteRow {
            body: row.get("body").cloned().unwrap_or_default(),
            locked: row.get("locked").is_some_and(|locked| locked == "true"),
        }))
    }

    async fn remove_sample_note(&mut self, id: RecordId) -> Result<(), StoreError> {
        self.remove_kind_row(KIND, id);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    //! Unit tests for the second sample kind.

    use super::{AddSampleNote, EditSampleNote, SampleNotes, add, edit};
    use crate::outcome::codes;
    use crate::ports::interaction::Confirmation;
    use crate::ports::unit_of_work::Storage;
    use crate::records::delete::delete;
    use crate::testing::block_on;
    use crate::testing::environment::{SeededIds, stamp};
    use crate::testing::interaction::ScriptedPrompter;
    use crate::testing::unit::{FakeStorage, FakeUnit};

    /// A unit holding one note with this body; returns its handle too.
    fn unit_with(body: &str, locked: bool) -> (FakeUnit, String) {
        let mut unit = block_on(FakeStorage::new().begin()).expect("begin");
        let command = AddSampleNote::new(Some(body), locked)
            .expect("valid")
            .command;
        let row = block_on(add(&mut unit, &stamp(), &SeededIds::new(), command)).expect("added");
        (unit, row.handle)
    }

    #[test]
    fn a_note_is_named_by_the_first_line_of_its_body() {
        let (mut unit, handle) = unit_with("Collected in the rain\nSecond line", false);
        assert!(handle.starts_with("smp-"));
        assert_eq!(
            unit.state().records[0].display_name,
            "Collected in the rain"
        );
        assert!(AddSampleNote::new(Some("  "), false).is_err());
        assert!(AddSampleNote::new(None, false).is_err());
    }

    #[test]
    fn a_locked_note_cannot_be_deleted_until_it_is_unlocked() {
        let (mut unit, handle) = unit_with("Keep me", true);
        let mut prompter = ScriptedPrompter::answering(Confirmation::Yes);
        let blocked = block_on(delete(
            &mut unit,
            &stamp(),
            &SampleNotes::new(),
            &mut prompter,
            &handle,
        ));
        let problem = blocked.expect_err("locked");
        assert_eq!(problem.code, codes::BLOCKED_BY_DEPENDENTS);
        assert!(
            problem
                .next_step
                .expect("an alternative")
                .contains("--unlock")
        );

        let unlock = EditSampleNote::new(&handle, None, Some(false))
            .expect("valid")
            .command;
        block_on(edit(&mut unit, &stamp(), unlock)).expect("unlocked");
        block_on(delete(
            &mut unit,
            &stamp(),
            &SampleNotes::new(),
            &mut prompter,
            &handle,
        ))
        .expect("deleted");
    }

    #[test]
    fn editing_the_body_renames_the_note() {
        let (mut unit, handle) = unit_with("Old body", false);
        let command = EditSampleNote::new(&handle, Some("New body"), None)
            .expect("valid")
            .command;
        let row = block_on(edit(&mut unit, &stamp(), command)).expect("edited");
        assert_eq!(row.name, "New body");
        assert_eq!(unit.state().audit[1].changes.len(), 1);
    }
}

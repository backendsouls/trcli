//! Notes: dated remarks attached to a record (FR-015).
//!
//! A record may have any number of notes; they are removed with the record (FR-017).

use time::OffsetDateTime;

use super::problem::Rejection;
use super::record::RecordId;
use super::text::LongText;

/// A dated remark about one record.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Note {
    /// The record the note is about.
    pub record: RecordId,
    /// What was noted: at least one character.
    pub body: LongText,
    /// When the note was added.
    pub created_at: OffsetDateTime,
}

impl Note {
    /// Checks the text of a note: it is long text that may not be empty.
    pub fn body(raw: &str) -> Result<LongText, Rejection> {
        let body = LongText::new(raw)?;
        if body.as_str().is_empty() {
            return Err(Rejection::new("must not be empty", "1 to 20000 characters"));
        }
        Ok(body)
    }

    /// A note on `record`, made at `created_at`.
    pub fn new(record: RecordId, body: LongText, created_at: OffsetDateTime) -> Self {
        Self {
            record,
            body,
            created_at,
        }
    }
}

#[cfg(test)]
mod tests {
    //! Unit tests for notes.

    use super::Note;

    #[test]
    fn a_note_needs_at_least_one_character() {
        assert!(Note::body("  ").is_err());
        assert_eq!(
            Note::body(" seen in the rain ").expect("valid").as_str(),
            "seen in the rain"
        );
    }
}

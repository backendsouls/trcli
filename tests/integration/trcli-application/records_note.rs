//! Unit tests for notes (T066).

use trcli_application::ports::records::RecordIndex;
use trcli_application::ports::unit_of_work::Storage;
use trcli_application::records::note::{NoteCommand, add_note};
use trcli_testing::block_on;
use trcli_testing::contract_records::indexed;
use trcli_testing::environment::stamp;
use trcli_testing::unit::FakeStorage;

#[test]
fn a_note_needs_at_least_one_character() {
    assert!(NoteCommand::new("alp-1", "   ").is_err());
    assert!(NoteCommand::new("alp-1", "Collected in the rain").is_ok());
}

#[test]
fn a_note_is_stored_with_its_date_and_one_audit_entry() {
    block_on(async {
        let mut unit = FakeStorage::new().begin().await.expect("begin");
        let record = indexed(0, "alpha", "alp", "First");
        unit.insert_record(&record).await.expect("insert");
        let command = NoteCommand::new(record.handle.as_str(), "Collected in the rain")
            .expect("valid")
            .command;
        let noted = add_note(&mut unit, &stamp(), &[], command)
            .await
            .expect("noted");
        assert_eq!(noted.body, "Collected in the rain");
        let state = unit.state();
        assert_eq!(
            (state.notes.len(), state.notes[0].created_at),
            (1, stamp().at)
        );
        assert_eq!(
            (state.audit.len(), state.audit[0].action.as_str()),
            (1, "note")
        );
    });
}

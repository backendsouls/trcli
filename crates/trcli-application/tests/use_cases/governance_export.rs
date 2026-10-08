//! Unit tests for exporting the trail.

use trcli_domain::governance::audit::{AuditAction, AuditDraft};

use trcli_application::governance::export::{ExportCommand, ExportFormat, collect, record_export};
use trcli_application::ports::audit::{AuditFilter, AuditLog};
use trcli_application::ports::unit_of_work::Storage;
use trcli_testing::block_on;
use trcli_testing::environment::stamp;
use trcli_testing::unit::FakeStorage;

#[test]
fn the_destination_is_required_and_the_format_is_one_of_three() {
    assert!(ExportCommand::new(None, None).is_err());
    assert!(ExportCommand::new(Some("audit.md"), Some("pdf")).is_err());
    let command = ExportCommand::new(Some("audit.csv"), Some("csv"))
        .expect("valid")
        .command;
    assert_eq!(command.format, ExportFormat::Csv);
    assert_eq!(
        ExportCommand::new(Some("a.md"), None)
            .expect("valid")
            .command
            .format,
        ExportFormat::Markdown
    );
}

#[test]
fn an_export_holds_every_matching_entry_oldest_first_and_is_itself_recorded() {
    let storage = FakeStorage::new();
    block_on(async {
        let mut unit = storage.begin().await.expect("begin");
        for _ in 0..3 {
            unit.record(&stamp(), AuditDraft::workspace(AuditAction::SETTING))
                .await
                .expect("record");
        }
        let entries = collect(&unit, &AuditFilter::everything(1))
            .await
            .expect("collected");
        let sequences: Vec<u64> = entries.iter().map(|entry| entry.sequence).collect();
        assert_eq!(sequences, [1, 2, 3]);

        let command = ExportCommand::new(Some("audit.md"), None)
            .expect("valid")
            .command;
        let exported = record_export(&mut unit, &stamp(), &command, 3)
            .await
            .expect("recorded");
        assert_eq!(
            (exported.entries, exported.format.as_str()),
            (3, "markdown")
        );
        assert_eq!(unit.state().audit[3].action.as_str(), "export");
    });
}

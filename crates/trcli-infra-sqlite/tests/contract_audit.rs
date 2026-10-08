//! The SQLite adapter passes the audit contract the in-memory fake passes (T110), and its
//! trail verifies with the real digest.

mod support;

use trcli_application::governance::verify::{Fault, TrailVerdict, verify};
use trcli_application::ports::audit::AuditLog;
use trcli_application::ports::unit_of_work::{Storage, UnitOfWork};
use trcli_application::testing::environment::stamp;
use trcli_application::testing::interaction::RecordingProgress;
use trcli_domain::governance::audit::{AuditAction, AuditDraft};
use trcli_infra_sqlite::digest::Sha256Digest;

#[tokio::test(flavor = "current_thread")]
async fn sqlite_passes_the_audit_contract() {
    let databases = support::Databases::new();
    trcli_application::testing::contract_audit::run(async || databases.fresh().await).await;
}

#[tokio::test(flavor = "current_thread")]
async fn a_trail_written_by_sqlite_verifies_with_sha_256_and_detects_a_removed_entry() {
    let databases = support::Databases::new();
    let storage = databases.fresh().await;
    let mut unit = storage.begin().await.expect("begin");
    for _ in 0..3 {
        unit.record(
            &stamp(),
            AuditDraft::workspace(AuditAction::SETTING).named("x"),
        )
        .await
        .expect("record");
    }
    let head = unit.commit().await.expect("commit").expect("a head");

    let reader = storage.read().await.expect("read");
    let verdict = verify(
        &reader,
        Some(head),
        &Sha256Digest,
        &mut RecordingProgress::default(),
    )
    .await;
    assert_eq!(verdict, Ok(TrailVerdict::Intact { entries: 3 }));
    drop(reader);

    // Verification against a head that remembers more than is there: the end was removed.
    let longer = trcli_domain::governance::audit::AuditHead {
        sequence: 4,
        hash: head.hash,
    };
    let reader = storage.read().await.expect("read");
    let verdict = verify(
        &reader,
        Some(longer),
        &Sha256Digest,
        &mut RecordingProgress::default(),
    )
    .await;
    assert!(
        matches!(verdict, Ok(TrailVerdict::Broken(finding)) if finding.fault == Fault::Shortened)
    );
}

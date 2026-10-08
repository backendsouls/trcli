//! Unit tests for reference resolution (T060).

use trcli_domain::shared::record::Handle;

use trcli_application::outcome::{Details, Problem, codes};
use trcli_application::ports::records::{IndexedRecord, RecordIndex};
use trcli_application::ports::unit_of_work::Storage;
use trcli_application::records::resolve::{resolve, resolve_option};
use trcli_application::validation::Checker;
use trcli_testing::block_on;
use trcli_testing::contract_records::indexed;
use trcli_testing::environment::stamp;
use trcli_testing::unit::{FakeStorage, FakeUnit};

/// A record with a handle chosen by the test.
fn record(position: u64, kind: &str, handle: &str, name: &str) -> IndexedRecord {
    let mut record = indexed(position, kind, &handle[..3], name);
    record.handle = Handle::parse(handle).expect("a valid handle");
    record
}

/// A unit holding three `alpha` records and one `beta` record.
fn unit() -> FakeUnit {
    block_on(async {
        let mut unit = FakeStorage::new().begin().await.expect("begin");
        for record in [
            record(0, "alpha", "alp-7k3f", "First"),
            record(1, "alpha", "alp-7k3fz", "Longer"),
            record(2, "alpha", "alp-9abc", "Third"),
            record(3, "beta", "bet-7k3f", "Other kind"),
        ] {
            unit.insert_record(&record).await.expect("insert");
        }
        unit
    })
}

/// Resolves `typed` among the given kinds.
fn found(unit: &FakeUnit, typed: &str, kinds: &[&str]) -> Result<String, Problem> {
    let kinds: Vec<String> = kinds.iter().map(|kind| (*kind).to_owned()).collect();
    block_on(resolve(unit, typed, &kinds)).map(|record| record.display_name)
}

#[test]
fn an_exact_handle_wins_over_longer_ones_that_start_with_it() {
    assert_eq!(found(&unit(), "alp-7k3f", &[]), Ok("First".to_owned()));
}

#[test]
fn a_unique_beginning_is_enough_and_case_does_not_matter() {
    assert_eq!(found(&unit(), "alp-9", &[]), Ok("Third".to_owned()));
    assert_eq!(found(&unit(), "ALP-9A", &[]), Ok("Third".to_owned()));
}

#[test]
fn several_matches_are_listed_and_nothing_is_chosen() {
    let problem = found(&unit(), "alp-7", &[]).expect_err("ambiguous");
    assert_eq!(problem.code, codes::AMBIGUOUS_REFERENCE);
    assert_eq!(
        problem.details,
        Details::Items(vec![
            "alp-7k3f \"First\"".into(),
            "alp-7k3fz \"Longer\"".into()
        ])
    );
}

#[test]
fn only_the_kinds_the_command_accepts_are_considered() {
    assert_eq!(
        found(&unit(), "bet", &["beta"]),
        Ok("Other kind".to_owned())
    );
    assert_eq!(
        found(&unit(), "bet-7k3f", &["alpha"])
            .expect_err("wrong kind")
            .code,
        codes::NOT_FOUND
    );
}

#[test]
fn nothing_found_suggests_up_to_three_close_handles() {
    let problem = found(&unit(), "alp-7k3g", &[]).expect_err("not found");
    assert_eq!(problem.code, codes::NOT_FOUND);
    let Details::Items(close) = problem.details else {
        panic!("suggestions expected")
    };
    assert_eq!(close[0], "alp-7k3f");
    assert!(close.len() <= 3);
    assert_eq!(
        found(&unit(), "zzz-0000", &[])
            .expect_err("not found")
            .details,
        Details::None
    );
}

#[test]
fn deleted_records_are_not_found() {
    let mut unit = unit();
    block_on(async {
        let third = resolve(&unit, "alp-9abc", &[]).await.expect("found");
        unit.mark_deleted(third.id, stamp().at)
            .await
            .expect("delete");
    });
    assert_eq!(
        found(&unit, "alp-9abc", &[]).expect_err("deleted").code,
        codes::NOT_FOUND
    );
}

#[test]
fn something_that_cannot_be_a_handle_is_invalid_input() {
    assert_eq!(
        found(&unit(), "../etc/passwd", &[])
            .expect_err("invalid")
            .code,
        codes::VALIDATION_FAILED
    );
}

#[test]
fn a_record_named_by_an_option_is_reported_like_any_other_invalid_value() {
    let unit = unit();
    let mut checker = Checker::new();
    let missing = block_on(resolve_option(
        &unit,
        &mut checker,
        "--record",
        Some("alp-0000"),
    ))
    .expect("checked");
    assert!(missing.is_none());
    let problem = checker.finish(|| ()).expect_err("invalid");
    assert_eq!(problem.code, codes::VALIDATION_FAILED);
    let Details::Fields(fields) = problem.details else {
        panic!("fields expected")
    };
    assert_eq!(
        (fields[0].field.as_str(), fields[0].value.as_deref()),
        ("--record", Some("alp-0000"))
    );
    assert!(fields[0].problem.starts_with("no record matches"));
}

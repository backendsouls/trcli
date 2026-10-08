//! Finding the record a researcher typed (FR-011).
//!
//! Among records that are not deleted, of the kinds the command accepts, ignoring case:
//! an exact short name is that record; otherwise the records whose short name starts with
//! what was typed — one is that record, several is "ambiguous" with the list, none is
//! "not found" with up to three close short names.

use trcli_domain::shared::problem::Rejection;
use trcli_domain::shared::record::TypedReference;

use super::similar::closest;
use crate::outcome::{Details, Problem, codes};
use crate::ports::records::{IndexedRecord, RecordResolver};
use crate::validation::Checker;

/// How many matches an "ambiguous" problem lists before saying how many more there are.
const LISTED_MATCHES: usize = 10;

/// Resolves what was typed to one record of one of `kinds` (any kind when empty).
pub async fn resolve<U: RecordResolver>(
    unit: &U,
    typed: &str,
    kinds: &[String],
) -> Result<IndexedRecord, Problem> {
    let mut checker = Checker::new();
    let reference = checker.check("<ref>", typed, TypedReference::new);
    let Some(reference) = reference else {
        return Err(checker.finish(|| ()).expect_err("a problem was recorded"));
    };
    let mut matches = unit
        .records_starting_with(reference.as_str(), kinds)
        .await?;
    // An exact short name wins even when it is also the beginning of a longer one.
    if let Some(exact) = matches
        .iter()
        .position(|record| record.handle.as_str() == reference.as_str())
    {
        return Ok(matches.swap_remove(exact));
    }
    match matches.len() {
        1 => Ok(matches.swap_remove(0)),
        0 => Err(not_found(unit, reference.as_str(), kinds).await?),
        _ => Err(ambiguous(reference.as_str(), &matches)),
    }
}

/// Resolves a record named by an option of a command. A record that does not exist is
/// then reported like any other invalid value (User Story 3, scenario 5), under the
/// option's name, in the same report as the command's other problems.
pub async fn resolve_option<U: RecordResolver>(
    unit: &U,
    checker: &mut Checker,
    field: &str,
    typed: Option<&str>,
) -> Result<Option<IndexedRecord>, Problem> {
    let Some(typed) = typed else { return Ok(None) };
    match resolve(unit, typed, &[]).await {
        Ok(record) => Ok(Some(record)),
        Err(problem)
            if problem.code == codes::INTERNAL || problem.code == codes::WORKSPACE_BUSY =>
        {
            Err(problem)
        }
        Err(problem) => {
            let rejection = Rejection::new(
                lower_first(&problem.message),
                "the short name of an existing record",
            );
            let rejection = match &problem.details {
                Details::Items(items) => rejection.with_choices(items.iter().cloned()),
                _ => rejection,
            };
            checker.reject(field, typed, rejection);
            Ok(None)
        }
    }
}

/// A sentence turned into a phrase that can follow a value.
fn lower_first(sentence: &str) -> String {
    let mut characters = sentence.chars();
    characters.next().map_or_else(String::new, |first| {
        first.to_lowercase().chain(characters).collect()
    })
}

/// The problem for something that matches no record, with close short names if any.
async fn not_found<U: RecordResolver>(
    unit: &U,
    typed: &str,
    kinds: &[String],
) -> Result<Problem, Problem> {
    let handles = unit.handles(kinds).await?;
    let close: Vec<String> = closest(typed, handles.iter().map(|handle| handle.as_str()), 3)
        .into_iter()
        .map(str::to_owned)
        .collect();
    let problem = Problem::new(codes::NOT_FOUND, format!("no record matches `{typed}`"));
    Ok(if close.is_empty() {
        problem
    } else {
        let next_step = format!("did you mean {}?", close.join(", "));
        problem.with_items(close).with_next_step(next_step)
    })
}

/// The problem for something that matches several records, listing them.
fn ambiguous(typed: &str, matches: &[IndexedRecord]) -> Problem {
    let mut items: Vec<String> = matches
        .iter()
        .take(LISTED_MATCHES)
        .map(|record| format!("{} \"{}\"", record.handle, record.display_name))
        .collect();
    if matches.len() > LISTED_MATCHES {
        items.push(format!("and {} more", matches.len() - LISTED_MATCHES));
    }
    Problem::new(
        codes::AMBIGUOUS_REFERENCE,
        format!("`{typed}` matches {} records", matches.len()),
    )
    .with_items(items)
    .with_next_step("type more of the short name")
}

#[cfg(test)]
mod tests {
    //! Unit tests for reference resolution (T060).

    use trcli_domain::shared::record::Handle;

    use super::{resolve, resolve_option};
    use crate::outcome::{Details, Problem, codes};
    use crate::ports::records::{IndexedRecord, RecordIndex};
    use crate::ports::unit_of_work::Storage;
    use crate::testing::block_on;
    use crate::testing::contract_records::indexed;
    use crate::testing::environment::stamp;
    use crate::testing::unit::{FakeStorage, FakeUnit};
    use crate::validation::Checker;

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
}

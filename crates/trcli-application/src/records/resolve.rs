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

//! The one way input is checked before anything changes (FR-020 to FR-027).
//!
//! # The pattern every command constructor follows
//!
//! A use case does not take raw input. It takes a *command* that can only be built by a
//! constructor which checks every value and reports every problem at once:
//!
//! ```text
//! let mut checker = Checker::new();
//! let name  = checker.required("--name", raw.name.as_deref(), Name::new);
//! let about = checker.optional("--description", raw.description.as_deref(), LongText::new);
//! checker.finish(|| InitCommand { name: name.unwrap(), description: about.flatten() })
//! ```
//!
//! Each call records a problem under the value's name as the researcher typed it and
//! returns `None`; `finish` builds the command only when no problem was recorded, so the
//! `unwrap` calls inside its closure cannot fail. Checks that need storage ("does that
//! record exist?") are added by the handler to the same [`Checker`] before it finishes, so
//! they land in the same report and nothing is written while any problem remains.
//!
//! Warnings travel beside a valid command and never change a value (FR-025).

use time::Date;
use trcli_domain::shared::problem::{FieldProblem, Rejection, ValidationReport, Warning};

use crate::outcome::Problem;

/// A command that passed every check, with the warnings raised on the way.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Valid<T> {
    /// The command, ready for its use case.
    pub command: T,
    /// What was unusual but allowed.
    pub warnings: Vec<Warning>,
}

/// Collects the problems of one command's input.
#[derive(Debug, Default)]
pub struct Checker {
    /// What has been found so far.
    report: ValidationReport,
}

impl Checker {
    /// A checker with nothing found yet.
    pub fn new() -> Self {
        Self::default()
    }

    /// Checks a value that must be given. A missing value is a problem like any other.
    pub fn required<T>(
        &mut self,
        field: &str,
        value: Option<&str>,
        check: impl FnOnce(&str) -> Result<T, Rejection>,
    ) -> Option<T> {
        match value {
            Some(value) => self.check(field, value, check),
            None => {
                self.reject(field, "", Rejection::new("is required", "a value"));
                None
            }
        }
    }

    /// Checks a value that may be left out. The outer `None` means "a problem was
    /// recorded"; `Some(None)` means "not given".
    pub fn optional<T>(
        &mut self,
        field: &str,
        value: Option<&str>,
        check: impl FnOnce(&str) -> Result<T, Rejection>,
    ) -> Option<Option<T>> {
        match value {
            Some(value) => self.check(field, value, check).map(Some),
            None => Some(None),
        }
    }

    /// Checks one value and records the problem if it is rejected.
    pub fn check<T>(
        &mut self,
        field: &str,
        value: &str,
        check: impl FnOnce(&str) -> Result<T, Rejection>,
    ) -> Option<T> {
        match check(value) {
            Ok(checked) => Some(checked),
            Err(rejection) => {
                self.reject(field, value, rejection);
                None
            }
        }
    }

    /// Records a problem found by the caller: a rule between two values, or something only
    /// storage could tell.
    pub fn reject(&mut self, field: &str, value: &str, rejection: Rejection) {
        self.report
            .reject(FieldProblem::new(field, value, rejection));
    }

    /// Records a problem with a secret value, which is never repeated (FR-054).
    pub fn reject_secret(&mut self, field: &str, rejection: Rejection) {
        self.report
            .reject(FieldProblem::for_secret(field, rejection));
    }

    /// Records something unusual but allowed.
    pub fn warn(&mut self, warning: Warning) {
        self.report.warn(warning);
    }

    /// Whether no problem has been recorded so far.
    pub fn is_clean(&self) -> bool {
        self.report.is_valid()
    }

    /// Ends the checking: the command built by `build` when nothing is wrong, otherwise
    /// one problem that lists every invalid value. `build` runs only in the first case.
    pub fn finish<T>(self, build: impl FnOnce() -> T) -> Result<Valid<T>, Problem> {
        let (errors, warnings) = self.report.into_parts();
        if errors.is_empty() {
            Ok(Valid {
                command: build(),
                warnings,
            })
        } else {
            Err(Problem::validation(errors))
        }
    }
}

/// Accepts a value that is one of `choices`, and lists them when it is not (FR-022).
pub fn one_of(value: &str, choices: &[&str]) -> Result<String, Rejection> {
    if choices.contains(&value) {
        return Ok(value.to_owned());
    }
    Err(Rejection::new(
        "is not one of the allowed values",
        format!("one of: {}", choices.join(", ")),
    )
    .with_choices(choices.iter().copied()))
}

/// Accepts a whole number within a range, inclusive, and shows a valid one when it is not.
pub fn integer_in_range(value: &str, minimum: i64, maximum: i64) -> Result<i64, Rejection> {
    let expected = format!("a whole number from {minimum} to {maximum}");
    match value.trim().parse::<i64>() {
        Ok(number) if (minimum..=maximum).contains(&number) => Ok(number),
        Ok(_) => Err(Rejection::new("is out of range", expected).with_example(minimum.to_string())),
        Err(_) => {
            Err(Rejection::new("is not a whole number", expected).with_example(minimum.to_string()))
        }
    }
}

/// Accepts a calendar date written `YYYY-MM-DD`; an impossible date is rejected (FR-022).
pub fn date(value: &str) -> Result<Date, Rejection> {
    let rejection =
        || Rejection::new("is not a date", "a date written YYYY-MM-DD").with_example("2026-10-08");
    let mut parts = value.trim().split('-');
    let (Some(year), Some(month), Some(day), None) =
        (parts.next(), parts.next(), parts.next(), parts.next())
    else {
        return Err(rejection());
    };
    if year.len() != 4 || month.len() != 2 || day.len() != 2 {
        return Err(rejection());
    }
    let year: i32 = year.parse().map_err(|_| rejection())?;
    let month: u8 = month.parse().map_err(|_| rejection())?;
    let day: u8 = day.parse().map_err(|_| rejection())?;
    let month = time::Month::try_from(month).map_err(|_| rejection())?;
    Date::from_calendar_date(year, month, day).map_err(|_| rejection())
}

/// The rule that an end may not come before its start (FR-021); the rejection names it.
pub fn not_before(start: Date, end: Date) -> Result<(), Rejection> {
    if end < start {
        return Err(Rejection::new(
            format!("is before the start ({start})"),
            "an end on or after the start",
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    //! Unit tests for the command-constructor pattern (T078).

    use trcli_domain::shared::problem::Warning;
    use trcli_domain::shared::text::{LongText, Name, TagName};

    use super::{Checker, date, integer_in_range, not_before, one_of};
    use crate::outcome::{Details, codes};

    /// A command with three fields, built the documented way.
    #[derive(Debug, PartialEq)]
    struct Example {
        /// A required name.
        name: Name,
        /// A required tag.
        tag: TagName,
        /// An optional description.
        about: Option<LongText>,
    }

    /// The constructor of [`Example`], with an optional check a handler would add.
    fn example(
        name: &str,
        tag: &str,
        about: Option<&str>,
        stored_problem: bool,
    ) -> Result<Example, String> {
        let mut checker = Checker::new();
        let name = checker.required("--name", Some(name), Name::new);
        let tag = checker.required("<tag>", Some(tag), TagName::new);
        let about = checker.optional("--about", about, LongText::new);
        if stored_problem {
            checker.reject(
                "--name",
                "taken",
                trcli_domain::shared::problem::Rejection::new("is taken", "a new name"),
            );
        }
        let written = std::cell::Cell::new(false);
        let result = checker.finish(|| {
            written.set(true);
            Example {
                name: name.expect("checked"),
                tag: tag.expect("checked"),
                about: about.flatten(),
            }
        });
        match result {
            Ok(valid) => Ok(valid.command),
            Err(problem) => {
                assert!(
                    !written.get(),
                    "nothing is built when the report has an error"
                );
                assert_eq!(problem.code, codes::VALIDATION_FAILED);
                let Details::Fields(fields) = problem.details else {
                    panic!("fields expected")
                };
                Err(fields
                    .iter()
                    .map(|field| field.field.clone())
                    .collect::<Vec<_>>()
                    .join(","))
            }
        }
    }

    #[test]
    fn three_invalid_fields_give_one_report_in_the_order_given() {
        let long = "x".repeat(20_001);
        assert_eq!(
            example("", "two words", Some(&long), false),
            Err("--name,<tag>,--about".to_owned())
        );
    }

    #[test]
    fn checks_added_by_the_handler_land_in_the_same_report() {
        assert_eq!(
            example("", "ok", None, true),
            Err("--name,--name".to_owned())
        );
    }

    #[test]
    fn a_valid_command_is_built() {
        let command = example("Ana", "Field-Work", None, false).expect("valid");
        assert_eq!(command.tag.as_str(), "field-work");
        assert_eq!(command.about, None);
    }

    #[test]
    fn a_missing_required_value_is_a_problem_like_any_other() {
        let mut checker = Checker::new();
        assert_eq!(checker.required("--name", None, Name::new), None);
        assert!(!checker.is_clean());
    }

    #[test]
    fn warnings_travel_beside_a_valid_command() {
        let mut checker = Checker::new();
        checker.warn(Warning::new("unusual", "that is unusual"));
        let valid = checker.finish(|| 7).expect("valid");
        assert_eq!((valid.command, valid.warnings.len()), (7, 1));
    }

    #[test]
    fn a_secret_value_is_not_repeated() {
        let mut checker = Checker::new();
        checker.reject_secret(
            "--token",
            trcli_domain::shared::problem::Rejection::new("is too short", "40+"),
        );
        let problem = checker.finish(|| ()).expect_err("invalid");
        let Details::Fields(fields) = problem.details else {
            panic!("fields expected")
        };
        assert_eq!(fields[0].value, None);
    }

    #[test]
    fn a_choice_outside_the_set_lists_the_set() {
        assert_eq!(one_of("json", &["human", "json"]), Ok("json".to_owned()));
        assert_eq!(
            one_of("xml", &["human", "json"])
                .expect_err("not a choice")
                .choices,
            ["human", "json"]
        );
    }

    #[test]
    fn a_number_outside_its_range_shows_a_valid_one() {
        assert_eq!(integer_in_range("50", 1, 1000), Ok(50));
        assert!(
            integer_in_range("0", 1, 1000)
                .expect_err("low")
                .example
                .is_some()
        );
        assert!(integer_in_range("ten", 1, 1000).is_err());
    }

    #[test]
    fn dates_are_written_year_month_day_and_must_exist() {
        assert!(date("2026-10-08").is_ok());
        for bad in [
            "2026-02-30",
            "08/10/2026",
            "2026-10",
            "26-10-08",
            "2026-13-01",
            "",
        ] {
            assert_eq!(
                date(bad).expect_err(bad).example.as_deref(),
                Some("2026-10-08")
            );
        }
    }

    #[test]
    fn an_end_may_not_come_before_its_start() {
        let start = date("2026-10-08").expect("valid");
        let end = date("2026-10-01").expect("valid");
        assert!(not_before(start, end).is_err());
        assert!(not_before(start, start).is_ok());
    }
}

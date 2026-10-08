//! How a problem with an input is described (FR-022, FR-023).
//!
//! A [`Rejection`] says what is wrong with a value and what would be right. A
//! [`FieldProblem`] adds which value it was, as the researcher gave it. A
//! [`ValidationReport`] collects every problem of one command, so that all of them are
//! reported together and nothing is changed while any remains.
//!
//! This module does not decide how problems are shown; that is the renderer's job.

/// What is wrong with one value, and what would be right.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Rejection {
    /// What is wrong, as a phrase that follows the value: "must not be empty".
    pub problem: String,
    /// What is expected, as a noun phrase: "1 to 200 characters".
    pub expected: String,
    /// A valid value, when showing one helps (FR-022).
    pub example: Option<String>,
    /// The allowed values, when the value must be one of a set (FR-022).
    pub choices: Vec<String>,
}

impl Rejection {
    /// A rejection with what is wrong and what is expected.
    pub fn new(problem: impl Into<String>, expected: impl Into<String>) -> Self {
        Self {
            problem: problem.into(),
            expected: expected.into(),
            example: None,
            choices: Vec::new(),
        }
    }

    /// Adds an example of a valid value.
    #[must_use]
    pub fn with_example(mut self, example: impl Into<String>) -> Self {
        self.example = Some(example.into());
        self
    }

    /// Adds the list of allowed values.
    #[must_use]
    pub fn with_choices<I, S>(mut self, choices: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.choices = choices.into_iter().map(Into::into).collect();
        self
    }
}

/// One problem with one value of a command.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FieldProblem {
    /// The value's name as the researcher gave it, for example `--year` or `<tag>`.
    pub field: String,
    /// The value that was given; `None` when it is secret and must not be repeated (FR-054).
    pub value: Option<String>,
    /// What is wrong with it.
    pub problem: String,
    /// What is expected instead.
    pub expected: String,
    /// A valid value, when one helps.
    pub example: Option<String>,
    /// The allowed values, when the value must be one of a set.
    pub choices: Vec<String>,
}

impl FieldProblem {
    /// Describes a rejected value of the named field.
    pub fn new(field: impl Into<String>, value: impl Into<String>, rejection: Rejection) -> Self {
        Self::build(field.into(), Some(value.into()), rejection)
    }

    /// Describes a rejected secret value: the value itself is never kept (FR-054).
    pub fn for_secret(field: impl Into<String>, rejection: Rejection) -> Self {
        Self::build(field.into(), None, rejection)
    }

    /// Assembles the problem from its parts.
    fn build(field: String, value: Option<String>, rejection: Rejection) -> Self {
        Self {
            field,
            value,
            problem: rejection.problem,
            expected: rejection.expected,
            example: rejection.example,
            choices: rejection.choices,
        }
    }
}

/// Something unusual but allowed: reported, never a reason to change a value (FR-025).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Warning {
    /// A stable, lower-case name for the kind of warning.
    pub code: String,
    /// One sentence for a person.
    pub message: String,
    /// The value the warning is about, when it is about one.
    pub field: Option<String>,
}

impl Warning {
    /// A warning that is not about a particular value.
    pub fn new(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            code: code.into(),
            message: message.into(),
            field: None,
        }
    }

    /// Names the value the warning is about.
    #[must_use]
    pub fn about(mut self, field: impl Into<String>) -> Self {
        self.field = Some(field.into());
        self
    }
}

/// Every problem and every warning found while checking one command.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ValidationReport {
    /// Errors, in the order the values were checked.
    errors: Vec<FieldProblem>,
    /// Warnings, kept apart from errors: they never stop a command.
    warnings: Vec<Warning>,
}

impl ValidationReport {
    /// An empty report.
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds an error.
    pub fn reject(&mut self, problem: FieldProblem) {
        self.errors.push(problem);
    }

    /// Adds a warning.
    pub fn warn(&mut self, warning: Warning) {
        self.warnings.push(warning);
    }

    /// Whether the command may proceed: there is no error (warnings do not count).
    pub fn is_valid(&self) -> bool {
        self.errors.is_empty()
    }

    /// The errors, in the order they were found.
    pub fn errors(&self) -> &[FieldProblem] {
        &self.errors
    }

    /// The warnings, in the order they were found.
    pub fn warnings(&self) -> &[Warning] {
        &self.warnings
    }

    /// Splits the report into its errors and its warnings.
    pub fn into_parts(self) -> (Vec<FieldProblem>, Vec<Warning>) {
        (self.errors, self.warnings)
    }
}

#[cfg(test)]
mod tests {
    //! Unit tests for the problem types (T013).

    use super::{FieldProblem, Rejection, ValidationReport, Warning};

    /// A problem for the named field.
    fn problem(field: &str) -> FieldProblem {
        FieldProblem::new(field, "x", Rejection::new("is wrong", "something right"))
    }

    #[test]
    fn problems_accumulate_in_the_order_they_were_found() {
        let mut report = ValidationReport::new();
        report.reject(problem("--first"));
        report.reject(problem("--second"));
        report.reject(problem("--third"));
        let fields: Vec<&str> = report.errors().iter().map(|p| p.field.as_str()).collect();
        assert_eq!(fields, ["--first", "--second", "--third"]);
        assert!(!report.is_valid());
    }

    #[test]
    fn warnings_are_a_separate_list_and_do_not_make_a_report_invalid() {
        let mut report = ValidationReport::new();
        report.warn(Warning::new("unusual", "this is unusual").about("--year"));
        assert!(report.is_valid());
        assert_eq!(report.warnings().len(), 1);
        assert!(report.errors().is_empty());
    }

    #[test]
    fn a_field_problem_carries_everything_a_message_needs() {
        let rejection = Rejection::new("is not a year", "a year between 1000 and 2027")
            .with_example("2024")
            .with_choices(["a", "b"]);
        let problem = FieldProblem::new("--year", "20x0", rejection);
        assert_eq!(problem.field, "--year");
        assert_eq!(problem.value.as_deref(), Some("20x0"));
        assert_eq!(problem.problem, "is not a year");
        assert_eq!(problem.expected, "a year between 1000 and 2027");
        assert_eq!(problem.example.as_deref(), Some("2024"));
        assert_eq!(problem.choices, ["a", "b"]);
    }

    #[test]
    fn a_secret_value_is_never_kept_in_a_problem() {
        let problem = FieldProblem::for_secret("--token", Rejection::new("is too short", "40+"));
        assert_eq!(problem.value, None);
    }
}

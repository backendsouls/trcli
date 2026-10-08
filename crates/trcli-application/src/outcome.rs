//! How a command ends, and how a failure is described (FR-032 to FR-034).
//!
//! [`Outcome`] is the fixed set of endings, each with its exit code; it is the same for
//! every command and never changes between versions. A [`Problem`] describes a failure
//! with a stable code, a sentence for a person, the details needed to act, whether
//! anything was changed, and the next step when there is an obvious one.
//!
//! Features add their own problem codes by registering them in a [`ProblemRegistry`];
//! nothing here lists features. This module does not render problems.

use trcli_domain::shared::problem::FieldProblem;

/// The ways a command can end.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Outcome {
    /// It worked. Includes an empty list, and work that paused by design.
    Success,
    /// Something unexpected went wrong.
    Failure,
    /// The input or the way the command was written is invalid.
    InvalidInput,
    /// Nothing matched, or what was typed matches several records.
    NotFound,
    /// There is no workspace, or it exists already, is too new, needs upgrading, is
    /// damaged, or is busy.
    WorkspaceProblem,
    /// A confirmation was required and not given, was declined, or the action is blocked.
    Refused,
    /// A verification did not pass.
    CheckFailed,
    /// Something outside the tool could not be done.
    OperationFailed,
    /// The researcher stopped the command.
    Interrupted,
}

impl Outcome {
    /// The exit code a calling program sees.
    pub fn exit_code(self) -> u8 {
        match self {
            Self::Success => 0,
            Self::Failure => 1,
            Self::InvalidInput => 2,
            Self::NotFound => 3,
            Self::WorkspaceProblem => 4,
            Self::Refused => 5,
            Self::CheckFailed => 6,
            Self::OperationFailed => 7,
            Self::Interrupted => 130,
        }
    }

    /// The stable lower-case name, used in telemetry.
    pub fn name(self) -> &'static str {
        match self {
            Self::Success => "success",
            Self::Failure => "failure",
            Self::InvalidInput => "invalid_input",
            Self::NotFound => "not_found",
            Self::WorkspaceProblem => "workspace_problem",
            Self::Refused => "refused",
            Self::CheckFailed => "check_failed",
            Self::OperationFailed => "operation_failed",
            Self::Interrupted => "interrupted",
        }
    }
}

/// A kind of problem: its stable name and the way a command that meets it ends.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ProblemCode {
    /// The stable, lower-case name shown as `error.code` in the structured form.
    pub name: &'static str,
    /// How a command that meets this problem ends.
    pub outcome: Outcome,
}

/// The foundation's problem codes, as listed in `contracts/output-and-exit-codes.md`.
pub mod codes {
    use super::{Outcome, ProblemCode};

    /// Declares one problem code.
    macro_rules! code {
        ($(#[$doc:meta])* $constant:ident, $name:literal, $outcome:ident) => {
            $(#[$doc])*
            pub const $constant: ProblemCode = ProblemCode { name: $name, outcome: Outcome::$outcome };
        };
    }

    code!(
        /// One or more values are invalid; the details list every one.
        VALIDATION_FAILED, "validation_failed", InvalidInput
    );
    code!(
        /// The command line is malformed.
        USAGE, "usage", InvalidInput
    );
    code!(
        /// A settings file or session variable holds an unknown key or an invalid value.
        SETTINGS_INVALID, "settings_invalid", InvalidInput
    );
    code!(
        /// No record matches what was typed.
        NOT_FOUND, "not_found", NotFound
    );
    code!(
        /// What was typed matches several records.
        AMBIGUOUS_REFERENCE, "ambiguous_reference", NotFound
    );
    code!(
        /// No workspace was found.
        NO_WORKSPACE, "no_workspace", WorkspaceProblem
    );
    code!(
        /// A workspace already exists where one was to be created.
        WORKSPACE_EXISTS, "workspace_exists", WorkspaceProblem
    );
    code!(
        /// The workspace was made by a newer version of the tool.
        WORKSPACE_TOO_NEW, "workspace_too_new", WorkspaceProblem
    );
    code!(
        /// The workspace is in an older format and must be upgraded before it is changed.
        WORKSPACE_NEEDS_UPGRADE, "workspace_needs_upgrade", WorkspaceProblem
    );
    code!(
        /// The stored data cannot be opened or fails its check.
        WORKSPACE_DAMAGED, "workspace_damaged", WorkspaceProblem
    );
    code!(
        /// Another command is changing the workspace and did not finish within the wait.
        WORKSPACE_BUSY, "workspace_busy", WorkspaceProblem
    );
    code!(
        /// A confirmation is needed and nobody can be asked.
        CONFIRMATION_REQUIRED, "confirmation_required", Refused
    );
    code!(
        /// The researcher was asked and did not say yes.
        DECLINED, "declined", Refused
    );
    code!(
        /// Deletion is not allowed while something stands in the way.
        BLOCKED_BY_DEPENDENTS, "blocked_by_dependents", Refused
    );
    code!(
        /// A verification found a difference.
        CHECK_FAILED, "check_failed", CheckFailed
    );
    code!(
        /// Something outside the tool could not be done.
        OPERATION_FAILED, "operation_failed", OperationFailed
    );
    code!(
        /// The researcher stopped the command.
        INTERRUPTED, "interrupted", Interrupted
    );
    code!(
        /// Anything else.
        INTERNAL, "internal", Failure
    );

    /// Every code above, in the order of the contract.
    pub const FOUNDATION: [ProblemCode; 18] = [
        VALIDATION_FAILED,
        USAGE,
        SETTINGS_INVALID,
        NOT_FOUND,
        AMBIGUOUS_REFERENCE,
        NO_WORKSPACE,
        WORKSPACE_EXISTS,
        WORKSPACE_TOO_NEW,
        WORKSPACE_NEEDS_UPGRADE,
        WORKSPACE_DAMAGED,
        WORKSPACE_BUSY,
        CONFIRMATION_REQUIRED,
        DECLINED,
        BLOCKED_BY_DEPENDENTS,
        CHECK_FAILED,
        OPERATION_FAILED,
        INTERRUPTED,
        INTERNAL,
    ];
}

/// The problem codes known to this build: the foundation's, plus what features register.
#[derive(Clone, Debug, Default)]
pub struct ProblemRegistry {
    /// The registered codes, in registration order.
    codes: Vec<ProblemCode>,
}

impl ProblemRegistry {
    /// A registry holding the foundation's codes.
    pub fn foundation() -> Self {
        Self {
            codes: codes::FOUNDATION.to_vec(),
        }
    }

    /// Adds a feature's code; a name may be registered only once, so that a code never
    /// means two things.
    pub fn register(&mut self, code: ProblemCode) -> Result<(), String> {
        if self.find(code.name).is_some() {
            return Err(format!(
                "problem code `{}` is already registered",
                code.name
            ));
        }
        self.codes.push(code);
        Ok(())
    }

    /// The code with this name, if registered.
    pub fn find(&self, name: &str) -> Option<ProblemCode> {
        self.codes.iter().copied().find(|code| code.name == name)
    }

    /// Every registered code.
    pub fn all(&self) -> &[ProblemCode] {
        &self.codes
    }
}

/// What a person or a program needs in order to act on a problem.
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub enum Details {
    /// Nothing beyond the message.
    #[default]
    None,
    /// For `validation_failed`: every invalid value.
    Fields(Vec<FieldProblem>),
    /// A list of things: matching handles, dependents, differences found.
    Items(Vec<String>),
}

/// A failure, described the same way whatever feature met it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Problem {
    /// The kind of problem.
    pub code: ProblemCode,
    /// One sentence for a person.
    pub message: String,
    /// What is needed to act.
    pub details: Details,
    /// Whether anything was changed before the problem was met. Almost always `false`.
    pub changed: bool,
    /// What to do next, when there is an obvious step (FR-034).
    pub next_step: Option<String>,
}

impl Problem {
    /// A problem of the given kind; nothing was changed unless said otherwise.
    pub fn new(code: ProblemCode, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
            details: Details::None,
            changed: false,
            next_step: None,
        }
    }

    /// Invalid input: one problem per invalid value, all reported together (FR-023).
    pub fn validation(errors: Vec<FieldProblem>) -> Self {
        let message = match errors.len() {
            1 => "1 value is invalid".to_owned(),
            count => format!("{count} values are invalid"),
        };
        Self {
            details: Details::Fields(errors),
            ..Self::new(codes::VALIDATION_FAILED, message)
        }
    }

    /// Something unexpected.
    pub fn internal(message: impl Into<String>) -> Self {
        Self::new(codes::INTERNAL, message)
    }

    /// No workspace was found from `searched_from`.
    pub fn no_workspace(searched_from: &str) -> Self {
        Self::new(codes::NO_WORKSPACE, format!("there is no workspace in {searched_from} or above it")).with_next_step(
            "create one here with `trcli init --name <name>`, or point to one with `--workspace <dir>`",
        )
    }

    /// A confirmation was needed and nobody could be asked; lists what would be affected.
    pub fn confirmation_required(what: &str, affected: Vec<String>) -> Self {
        Self::new(
            codes::CONFIRMATION_REQUIRED,
            format!("{what} needs confirmation, and nobody can be asked"),
        )
        .with_items(affected)
        .with_next_step("run the command again with `--yes` to confirm beforehand")
    }

    /// The researcher was asked and did not say yes.
    pub fn declined(what: &str) -> Self {
        Self::new(codes::DECLINED, format!("{what} was not confirmed"))
    }

    /// The outcome of a command that meets this problem.
    pub fn outcome(&self) -> Outcome {
        self.code.outcome
    }

    /// Adds a list of things to the details.
    #[must_use]
    pub fn with_items(mut self, items: Vec<String>) -> Self {
        self.details = if items.is_empty() {
            Details::None
        } else {
            Details::Items(items)
        };
        self
    }

    /// Names the next step.
    #[must_use]
    pub fn with_next_step(mut self, next_step: impl Into<String>) -> Self {
        self.next_step = Some(next_step.into());
        self
    }

    /// Says that something had already been changed when the problem was met.
    #[must_use]
    pub fn after_changes(mut self) -> Self {
        self.changed = true;
        self
    }
}

#[cfg(test)]
mod tests {
    //! Unit tests for outcomes and problems (T014).

    use trcli_domain::shared::problem::{FieldProblem, Rejection};

    use super::{Details, Outcome, Problem, ProblemCode, ProblemRegistry, codes};

    #[test]
    fn exit_codes_are_exactly_those_of_the_contract() {
        let expected = [
            (Outcome::Success, 0),
            (Outcome::Failure, 1),
            (Outcome::InvalidInput, 2),
            (Outcome::NotFound, 3),
            (Outcome::WorkspaceProblem, 4),
            (Outcome::Refused, 5),
            (Outcome::CheckFailed, 6),
            (Outcome::OperationFailed, 7),
            (Outcome::Interrupted, 130),
        ];
        for (outcome, code) in expected {
            assert_eq!(outcome.exit_code(), code, "{outcome:?}");
        }
    }

    #[test]
    fn every_foundation_code_has_a_distinct_name() {
        let registry = ProblemRegistry::foundation();
        for code in registry.all() {
            let same_name = registry
                .all()
                .iter()
                .filter(|other| other.name == code.name)
                .count();
            assert_eq!(same_name, 1, "{}", code.name);
        }
        assert_eq!(registry.find("no_workspace"), Some(codes::NO_WORKSPACE));
    }

    #[test]
    fn a_feature_registers_a_code_once() {
        let mut registry = ProblemRegistry::foundation();
        let code = ProblemCode {
            name: "step_failed",
            outcome: Outcome::OperationFailed,
        };
        assert!(registry.register(code).is_ok());
        assert!(registry.register(code).is_err());
        assert!(registry.register(codes::USAGE).is_err());
    }

    #[test]
    fn a_validation_problem_counts_its_values_and_changes_nothing() {
        let field = FieldProblem::new(
            "--name",
            "",
            Rejection::new("must not be empty", "1 to 200 characters"),
        );
        let one = Problem::validation(vec![field.clone()]);
        assert_eq!(one.message, "1 value is invalid");
        let two = Problem::validation(vec![field.clone(), field]);
        assert_eq!(two.message, "2 values are invalid");
        assert_eq!(two.outcome(), Outcome::InvalidInput);
        assert!(!two.changed);
        assert!(matches!(two.details, Details::Fields(ref fields) if fields.len() == 2));
    }

    #[test]
    fn obvious_next_steps_are_named() {
        assert!(
            Problem::no_workspace("/tmp")
                .next_step
                .expect("a step")
                .contains("trcli init")
        );
        let problem = Problem::confirmation_required("Deleting spc-1", vec!["1 link".into()]);
        assert!(problem.next_step.expect("a step").contains("--yes"));
        assert_eq!(problem.details, Details::Items(vec!["1 link".into()]));
    }
}

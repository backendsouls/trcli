//! How a failure is shown to a person (FR-022, FR-034).
//!
//! Every problem looks the same whatever feature met it: what happened; for invalid
//! input, each value by the name the researcher typed, what is wrong with it, and what is
//! expected, with an example or the allowed choices; that nothing was changed, when so;
//! and the next step, when there is an obvious one.

use trcli_application::outcome::{Details, Problem};
use trcli_domain::shared::problem::{FieldProblem, Warning};

use super::theme::{Meaning, Theme};
use super::width::{display_width, pad};

/// The lines that describe a problem, for standard error.
pub fn render(problem: &Problem, theme: &Theme) -> String {
    let mut lines = vec![format!("{} {}", theme.paint(Meaning::Error, "error:"), problem.message)];
    match &problem.details {
        Details::None => {}
        Details::Fields(fields) => lines.extend(field_lines(fields)),
        Details::Items(items) => lines.extend(items.iter().map(|item| format!("  {item}"))),
    }
    if !problem.changed {
        lines.push("Nothing was changed.".to_owned());
    }
    if let Some(next_step) = &problem.next_step {
        lines.push(format!("{} {next_step}", theme.paint(Meaning::Muted, "Next:")));
    }
    lines.join("\n") + "\n"
}

/// One line per invalid value, with the names and values aligned.
fn field_lines(fields: &[FieldProblem]) -> Vec<String> {
    let named: Vec<String> = fields.iter().map(name_and_value).collect();
    let width = named.iter().map(|name| display_width(name)).max().unwrap_or(0);
    fields.iter().zip(&named).map(|(field, name)| format!("  {}  {}", pad(name, width), explanation(field))).collect()
}

/// The value's name as typed and, in quotes, the value given. A secret value is not shown.
fn name_and_value(field: &FieldProblem) -> String {
    match &field.value {
        Some(value) => format!("{} \"{}\"", field.field, value.replace('\n', " ")),
        None => field.field.clone(),
    }
}

/// What is wrong, what is expected, and an example or the choices.
fn explanation(field: &FieldProblem) -> String {
    let mut text = format!("{}; expected {}", field.problem, field.expected);
    if let Some(example) = &field.example {
        text.push_str(&format!(" (for example: {example})"));
    }
    // The choices are already part of "expected" when the rule lists them itself.
    if !field.choices.is_empty() && !field.expected.contains(&field.choices.join(", ")) {
        text.push_str(&format!(" (one of: {})", field.choices.join(", ")));
    }
    text
}

/// The line that reports a warning: shown, never a reason to change a value (FR-025).
pub fn render_warning(warning: &Warning, theme: &Theme) -> String {
    format!("{} {}", theme.paint(Meaning::Warning, "warning:"), warning.message)
}

#[cfg(test)]
mod tests {
    //! Unit tests for problem rendering (T079).

    use trcli_application::outcome::{Problem, codes};
    use trcli_domain::shared::problem::{FieldProblem, Rejection, Warning};

    use super::{render, render_warning};
    use crate::render::theme::Theme;

    /// A validation problem with a form that has an example and a set that has choices.
    fn invalid() -> Problem {
        Problem::validation(vec![
            FieldProblem::new(
                "--from",
                "08/10/2026",
                Rejection::new("is not a date", "a date written YYYY-MM-DD").with_example("2026-10-08"),
            ),
            FieldProblem::new(
                "--sort",
                "colour",
                Rejection::new("is not one of the allowed values", "a field to sort by").with_choices(["title", "created"]),
            ),
        ])
    }

    #[test]
    fn each_line_shows_the_option_as_typed_the_value_what_is_wrong_and_what_is_expected() {
        let text = render(&invalid(), &Theme::plain());
        let lines: Vec<&str> = text.lines().collect();
        assert_eq!(lines[0], "error: 2 values are invalid");
        assert_eq!(
            lines[1],
            "  --from \"08/10/2026\"  is not a date; expected a date written YYYY-MM-DD (for example: 2026-10-08)"
        );
        assert_eq!(
            lines[2],
            "  --sort \"colour\"      is not one of the allowed values; expected a field to sort by (one of: title, created)"
        );
    }

    #[test]
    fn the_last_line_says_nothing_was_changed() {
        assert_eq!(render(&invalid(), &Theme::plain()).lines().last(), Some("Nothing was changed."));
        let changed = Problem::new(codes::OPERATION_FAILED, "the copy could not be removed").after_changes();
        assert!(!render(&changed, &Theme::plain()).contains("Nothing was changed."));
    }

    #[test]
    fn a_next_step_is_named_and_listed_items_are_indented() {
        let problem = Problem::new(codes::AMBIGUOUS_REFERENCE, "`spc` matches 2 records")
            .with_items(vec!["spc-7k3f \"First\"".into(), "spc-9abc \"Second\"".into()])
            .with_next_step("type more of the short name");
        let text = render(&problem, &Theme::plain());
        assert!(text.contains("\n  spc-7k3f \"First\"\n  spc-9abc \"Second\"\n"));
        assert!(text.ends_with("Next: type more of the short name\n"));
    }

    #[test]
    fn a_secret_value_is_not_shown() {
        let secret = FieldProblem::for_secret("--passphrase", Rejection::new("is too short", "12 or more characters"));
        let text = render(&Problem::validation(vec![secret]), &Theme::plain());
        assert!(text.contains("  --passphrase  is too short; expected 12 or more characters"));
    }

    #[test]
    fn choices_already_in_the_expectation_are_not_repeated() {
        let field = FieldProblem::new(
            "--color",
            "sometimes",
            Rejection::new("is not one of the allowed values", "one of: auto, always, never")
                .with_choices(["auto", "always", "never"]),
        );
        let text = render(&Problem::validation(vec![field]), &Theme::plain());
        assert_eq!(text.matches("auto, always, never").count(), 1);
    }

    #[test]
    fn a_warning_is_one_line() {
        let warning = Warning::new("duplicate_suspected", "another specimen has the same title");
        assert_eq!(render_warning(&warning, &Theme::plain()), "warning: another specimen has the same title");
    }
}

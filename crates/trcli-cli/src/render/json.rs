//! The structured form: one JSON document, the same shape for every command (FR-028,
//! FR-033).
//!
//! Success is `{"ok": true, "data": …, "warnings": […]}`; failure is `{"ok": false,
//! "error": {"code": …, "message": …, "details": …, "next_step": …}}`. Absent values are
//! `null`, never left out, so a program can rely on every key. The document ends with one
//! newline and never contains colour.

use serde_json::{Value, json};
use trcli_application::outcome::{Details, Problem};
use trcli_domain::shared::problem::{FieldProblem, Warning};

/// The envelope of a command that succeeded.
pub fn success(data: Value, warnings: &[Warning]) -> String {
    let warnings: Vec<Value> = warnings
        .iter()
        .map(|warning| json!({ "code": warning.code, "message": warning.message, "field": warning.field }))
        .collect();
    document(&json!({ "ok": true, "data": data, "warnings": warnings }))
}

/// The envelope of a command that failed.
pub fn failure(problem: &Problem) -> String {
    let details = match &problem.details {
        Details::None => Value::Null,
        Details::Fields(fields) => Value::Array(fields.iter().map(field).collect()),
        Details::Items(items) => json!(items),
    };
    document(&json!({
        "ok": false,
        "error": {
            "code": problem.code.name,
            "message": problem.message,
            "details": details,
            "changed": problem.changed,
            "next_step": problem.next_step,
        }
    }))
}

/// One invalid value. A secret value is `null`: it was never kept (FR-054).
fn field(problem: &FieldProblem) -> Value {
    json!({
        "field": problem.field,
        "value": problem.value,
        "problem": problem.problem,
        "expected": problem.expected,
        "example": problem.example,
        "choices": problem.choices,
    })
}

/// A value as indented text ending with exactly one newline.
fn document(value: &Value) -> String {
    // Serializing a `Value` cannot fail.
    let mut text = serde_json::to_string_pretty(value).unwrap_or_default();
    text.push('\n');
    text
}

#[cfg(test)]
mod tests {
    //! Unit tests for the JSON envelope (T015).

    use serde_json::{Value, json};
    use trcli_application::outcome::{Problem, codes};
    use trcli_domain::shared::problem::{FieldProblem, Rejection, Warning};

    use super::{failure, success};

    /// Reads a document back.
    fn parsed(document: &str) -> Value {
        serde_json::from_str(document).expect("valid JSON")
    }

    #[test]
    fn success_is_ok_data_and_warnings() {
        let warning = Warning::new("duplicate_suspected", "seen before");
        let document = parsed(&success(json!({ "handle": "spc-7k3f" }), &[warning]));
        assert_eq!(document["ok"], json!(true));
        assert_eq!(document["data"]["handle"], json!("spc-7k3f"));
        assert_eq!(
            document["warnings"][0]["code"],
            json!("duplicate_suspected")
        );
        assert_eq!(parsed(&success(json!({}), &[]))["warnings"], json!([]));
    }

    #[test]
    fn failure_is_ok_false_and_an_error_with_code_message_and_details() {
        let invalid = FieldProblem::new(
            "--name",
            "",
            Rejection::new("must not be empty", "1 to 200 characters"),
        );
        let document = parsed(&failure(&Problem::validation(vec![invalid])));
        assert_eq!(document["ok"], json!(false));
        assert_eq!(document["error"]["code"], json!("validation_failed"));
        assert_eq!(document["error"]["message"], json!("1 value is invalid"));
        assert_eq!(document["error"]["details"][0]["field"], json!("--name"));
        assert_eq!(
            document["error"]["details"][0]["problem"],
            json!("must not be empty")
        );
        assert!(document.get("data").is_none());
    }

    #[test]
    fn absent_optional_values_are_null_never_omitted() {
        let warning = parsed(&success(
            json!({}),
            &[Warning::new("unusual", "that is unusual")],
        ));
        assert!(
            warning["warnings"][0]
                .as_object()
                .expect("an object")
                .contains_key("field")
        );
        assert_eq!(warning["warnings"][0]["field"], Value::Null);

        let problem = parsed(&failure(&Problem::new(
            codes::NO_WORKSPACE,
            "there is no workspace here",
        )));
        let error = problem["error"].as_object().expect("an object");
        assert_eq!(error["details"], Value::Null);
        assert!(error.contains_key("next_step"));

        let invalid = FieldProblem::new("--name", "", Rejection::new("must not be empty", "text"));
        let field = parsed(&failure(&Problem::validation(vec![invalid])));
        assert_eq!(field["error"]["details"][0]["example"], Value::Null);
    }

    #[test]
    fn a_secret_value_never_reaches_the_document() {
        let secret = FieldProblem::for_secret(
            "--passphrase",
            Rejection::new("is too short", "12 or more characters"),
        );
        let document = failure(&Problem::validation(vec![secret]));
        assert_eq!(
            parsed(&document)["error"]["details"][0]["value"],
            Value::Null
        );
    }

    #[test]
    fn the_document_ends_with_one_newline_and_has_no_colour() {
        let document = success(json!({ "name": "Doctorate" }), &[]);
        assert!(document.ends_with('\n') && !document.ends_with("\n\n"));
        assert!(!document.contains('\u{1b}'));
    }
}

//! The two output forms carry the same content (SC-005).
//!
//! Both forms are produced from one view model, so they cannot hold different facts; this
//! test checks, through the binary, that neither leaves any out. For each command of the
//! sample kind, every value in the structured form's `data` must be findable in the form
//! for people (moments by their date and time, which are written differently on purpose).

mod common;

use common::Sandbox;
use serde_json::Value;

/// Every text and number inside a JSON value, as the form for people would write it.
fn leaves(value: &Value, found: &mut Vec<String>) {
    match value {
        Value::Null => {}
        Value::Bool(answer) => found.push(answer.to_string()),
        Value::Number(number) => found.push(number.to_string()),
        Value::String(text) => found.push(text.clone()),
        Value::Array(items) => items.iter().for_each(|item| leaves(item, found)),
        Value::Object(fields) => fields.values().for_each(|field| leaves(field, found)),
    }
}

/// A value as it appears in the form for people: a moment written `2026-10-08T14:00:00Z`
/// in the structured form is written `2026-10-08 14:00` there.
fn as_shown(value: &str) -> String {
    let is_moment = value.len() == 20 && value.ends_with('Z') && value.as_bytes()[10] == b'T';
    if is_moment {
        format!("{} {}", &value[..10], &value[11..16])
    } else {
        value.to_owned()
    }
}

/// Asserts that everything the structured form of a command says is in its other form.
fn assert_forms_agree(sandbox: &Sandbox, arguments: &[&str]) {
    let human = sandbox.ok(arguments);
    let mut with_json = arguments.to_vec();
    with_json.extend(["--output", "json"]);
    let structured = sandbox.ok(&with_json).json();
    assert_eq!(structured["ok"], Value::Bool(true));
    let mut values = Vec::new();
    leaves(&structured["data"], &mut values);
    assert!(!values.is_empty(), "`{}` has no data", arguments.join(" "));
    let shown = format!("{}{}", human.stdout, human.stderr);
    for value in values {
        assert!(
            shown.contains(&as_shown(&value)),
            "`{value}` is in the structured form of `trcli {}` and not in:\n{shown}",
            arguments.join(" ")
        );
    }
}

/// A workspace with two specimens, one tagged, noted, and linked to the other.
fn workspace() -> (Sandbox, String, String) {
    let sandbox = Sandbox::with_workspace();
    let handle = |title: &str| {
        sandbox
            .ok(&["specimen", "add", "--title", title, "--output", "json"])
            .json()["data"]["handle"]
            .as_str()
            .expect("a handle")
            .to_owned()
    };
    let (first, second) = (handle("Soil sample"), handle("Rain water"));
    sandbox.ok(&["specimen", "tag", &first, "field-work"]);
    sandbox.ok(&["specimen", "note", &first, "Dried overnight"]);
    sandbox.ok(&["link", "add", &first, &second, "--relation", "same site"]);
    (sandbox, first, second)
}

#[test]
fn showing_a_record_says_the_same_in_both_forms() {
    let (sandbox, first, _) = workspace();
    assert_forms_agree(&sandbox, &["specimen", "show", &first]);
}

#[test]
fn listing_records_says_the_same_in_both_forms() {
    let (sandbox, _, _) = workspace();
    let human = sandbox.ok(&["specimen", "list"]);
    let structured = sandbox.ok(&["specimen", "list", "--output", "json"]).json();
    let items = structured["data"]["items"].as_array().expect("items");
    assert_eq!(structured["data"]["total"], 2);
    for item in items {
        for field in ["handle", "name"] {
            let value = item[field].as_str().expect("text");
            assert!(
                human.stdout.contains(value),
                "`{value}` is not in:\n{}",
                human.stdout
            );
        }
        for tag in item["tags"].as_array().expect("tags") {
            assert!(human.stdout.contains(tag.as_str().expect("a tag")));
        }
    }
    assert_eq!(
        human.stdout.lines().count(),
        items.len() + 1,
        "one line per record, and the heading"
    );
}

#[test]
fn links_tags_and_the_workspace_say_the_same_in_both_forms() {
    let (sandbox, first, _) = workspace();
    assert_forms_agree(&sandbox, &["link", "list", &first]);
    assert_forms_agree(&sandbox, &["tag", "list"]);
    assert_forms_agree(&sandbox, &["config", "get", "output.page_size"]);
}

#[test]
fn a_failure_says_the_same_in_both_forms() {
    let (sandbox, _, _) = workspace();
    let human = sandbox.run(&["specimen", "show", "spc"]);
    let structured = sandbox.run(&["specimen", "show", "spc", "--output", "json"]);
    assert_eq!((human.code, structured.code), (3, 3));
    let error = &structured.json()["error"];
    let mut values = Vec::new();
    leaves(&error["message"], &mut values);
    leaves(&error["details"], &mut values);
    leaves(&error["next_step"], &mut values);
    for value in values {
        assert!(
            human.stderr.contains(&value),
            "`{value}` is not in:\n{}",
            human.stderr
        );
    }
}

//! The examples in the usage guides behave as the guides say (FR-058; User Story 7,
//! scenario 5).
//!
//! Every ```` ```console ```` block of every `docs/usage/*.md` is run against the built
//! tool, in order, in one scratch directory per guide, with a fixed clock and seeded
//! identifiers. In a block:
//!
//! - a line starting with `$ trcli` is a command (double or single quotes group words);
//! - the lines after it are what it prints: standard output, then standard error;
//! - `[..]` inside a line stands for any text (a path, for instance);
//! - a line that is just `...` stands for any number of lines;
//! - a last line `[exit N]` says the command ends with exit code N instead of 0.
//!
//! To rewrite the expected output of every block from what the tool prints now, run the
//! test with `TRCLI_BLESS_USAGE=1` and review the difference before committing it.

mod common;

use common::{Finished, Sandbox};

/// One command of a guide with what the guide says it prints.
#[derive(Clone, Debug)]
struct Example {
    /// The line of the guide the command is on, for messages.
    line: usize,
    /// The command line, without the `$ `.
    command: String,
    /// The lines the guide shows after it.
    expected: Vec<String>,
}

/// Splits a command line as a shell would, for what guides need: spaces separate words,
/// and single or double quotes group a word.
fn split(line: &str) -> Vec<String> {
    let mut words = Vec::new();
    let mut word = String::new();
    let (mut quote, mut started) = (None, false);
    for character in line.chars() {
        match (quote, character) {
            (None, '"' | '\'') => {
                quote = Some(character);
                started = true;
            }
            (Some(open), close) if open == close => quote = None,
            (None, ' ') => {
                if started {
                    words.push(std::mem::take(&mut word));
                    started = false;
                }
            }
            (_, other) => {
                word.push(other);
                started = true;
            }
        }
    }
    if started {
        words.push(word);
    }
    words
}

/// The examples of a guide, in order.
fn examples(text: &str) -> Vec<Example> {
    let mut examples: Vec<Example> = Vec::new();
    let mut in_console = false;
    for (index, line) in text.lines().enumerate() {
        if line.trim_start().starts_with("```") {
            in_console = !in_console && line.trim() == "```console";
        } else if in_console {
            if let Some(command) = line.strip_prefix("$ ") {
                examples.push(Example {
                    line: index + 1,
                    command: command.to_owned(),
                    expected: Vec::new(),
                });
            } else if let Some(example) = examples.last_mut() {
                example.expected.push(line.to_owned());
            }
        }
    }
    examples
}

/// Whether a printed line is what an expected line describes, `[..]` standing for any text.
fn line_matches(expected: &str, actual: &str) -> bool {
    let (expected, actual) = (expected.trim_end(), actual.trim_end());
    let parts: Vec<&str> = expected.split("[..]").collect();
    if parts.len() == 1 {
        return expected == actual;
    }
    // The line must begin with what comes before the first wildcard and end with what
    // comes after the last; the parts between must appear in order in what is left.
    let (first, last) = (parts[0], parts[parts.len() - 1]);
    if actual.len() < first.len() + last.len()
        || !actual.starts_with(first)
        || !actual.ends_with(last)
    {
        return false;
    }
    let mut rest = &actual[first.len()..actual.len() - last.len()];
    for part in &parts[1..parts.len() - 1] {
        match rest.find(part) {
            Some(at) => rest = &rest[at + part.len()..],
            None => return false,
        }
    }
    true
}

/// Whether the printed lines are what the expected lines describe, `...` standing for any
/// number of lines.
fn lines_match(expected: &[&str], actual: &[&str]) -> bool {
    match expected.split_first() {
        None => actual.is_empty(),
        Some((&"...", rest)) => {
            (0..=actual.len()).any(|skipped| lines_match(rest, &actual[skipped..]))
        }
        Some((first, rest)) => actual
            .split_first()
            .is_some_and(|(line, others)| line_matches(first, line) && lines_match(rest, others)),
    }
}

/// What a command printed, as the guide would show it: standard output, then standard
/// error, then the exit code when it is not 0.
fn printed(finished: &Finished, home: &str) -> Vec<String> {
    let text = format!("{}{}", finished.stdout, finished.stderr).replace(home, "[..]");
    // Guides are written with `/` in paths; on Windows the tool prints the other separator.
    let text = if cfg!(windows) {
        text.replace(0x5C as char, "/")
    } else {
        text
    };
    let mut lines: Vec<String> = text
        .lines()
        .map(|line| line.trim_end().to_owned())
        .collect();
    if finished.code != 0 {
        lines.push(format!("[exit {}]", finished.code));
    }
    lines
}

/// Runs every example of one guide; returns what each printed.
fn run_guide(text: &str, guide: &str) -> Vec<(Example, Vec<String>)> {
    let sandbox = Sandbox::new();
    let home = sandbox.home().display().to_string();
    let mut results = Vec::new();
    for example in examples(text) {
        let words = split(&example.command);
        assert_eq!(
            words.first().map(String::as_str),
            Some("trcli"),
            "{guide}:{}: only `trcli` commands can be checked",
            example.line
        );
        let arguments: Vec<&str> = words[1..].iter().map(String::as_str).collect();
        let finished = sandbox.run(&arguments);
        results.push((example, printed(&finished, &home)));
    }
    results
}

/// Rewrites a guide so that every console block shows what the tool printed.
fn bless(text: &str, results: &[(Example, Vec<String>)]) -> String {
    let mut blessed = Vec::new();
    let mut results = results.iter();
    let mut in_console = false;
    for line in text.lines() {
        if line.trim_start().starts_with("```") {
            in_console = !in_console && line.trim() == "```console";
            blessed.push(line.to_owned());
        } else if in_console {
            if line.starts_with("$ ") {
                blessed.push(line.to_owned());
                blessed.extend(
                    results
                        .next()
                        .map(|(_, printed)| printed.clone())
                        .unwrap_or_default(),
                );
            }
            // The old expected lines are dropped: the new ones were just written.
        } else {
            blessed.push(line.to_owned());
        }
    }
    blessed.join("\n") + "\n"
}

#[test]
fn every_example_in_every_usage_guide_behaves_as_written() {
    let directory = common::repository().join("docs/usage");
    let blessing = std::env::var_os("TRCLI_BLESS_USAGE").is_some();
    let mut failures = Vec::new();
    let mut checked = 0;
    for guide in common::files_under(&directory, &["md"]) {
        let name = guide
            .file_name()
            .expect("a file name")
            .to_string_lossy()
            .into_owned();
        let text = std::fs::read_to_string(&guide).expect("a guide");
        let results = run_guide(&text, &name);
        checked += results.len();
        if blessing {
            std::fs::write(&guide, bless(&text, &results)).expect("the guide is rewritten");
            continue;
        }
        for (example, actual) in &results {
            let expected: Vec<&str> = example
                .expected
                .iter()
                .map(String::as_str)
                .filter(|line| !line.trim().is_empty())
                .collect();
            let actual: Vec<&str> = actual
                .iter()
                .map(String::as_str)
                .filter(|line| !line.trim().is_empty())
                .collect();
            if !lines_match(&expected, &actual) {
                failures.push(format!(
                    "{name}:{}: $ {}\n  the guide says:\n    {}\n  the tool prints:\n    {}",
                    example.line,
                    example.command,
                    expected.join("\n    "),
                    actual.join("\n    ")
                ));
            }
        }
    }
    assert!(
        checked >= 30,
        "the guides hold the examples this test runs (found {checked})"
    );
    assert!(
        failures.is_empty(),
        "{} example(s) do not behave as written:\n\n{}",
        failures.len(),
        failures.join("\n\n")
    );
}

#[test]
fn the_matching_rules_are_what_the_guides_rely_on() {
    assert!(line_matches(
        "Created workspace \"Doctorate\" in [..]/.trcli",
        "Created workspace \"Doctorate\" in /tmp/x/work/.trcli"
    ));
    assert!(!line_matches(
        "Created workspace \"Doctorate\" in [..]/.trcli",
        "Created workspace \"Other\" in /tmp/x/work/.trcli"
    ));
    assert!(line_matches("exact", "exact") && !line_matches("exact", "exact and more"));
    assert!(line_matches("starts [..]", "starts with anything"));
    assert!(lines_match(
        &["first", "...", "last"],
        &["first", "a", "b", "last"]
    ));
    assert!(lines_match(&["first", "..."], &["first"]));
    assert!(!lines_match(&["first", "last"], &["first", "a", "last"]));
    assert_eq!(
        split("trcli init --name \"My Lab\" --description 'two words'"),
        [
            "trcli",
            "init",
            "--name",
            "My Lab",
            "--description",
            "two words"
        ]
    );
}

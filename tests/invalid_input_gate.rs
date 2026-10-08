//! Every input of every command has a scenario in which an invalid form of it is rejected
//! (constitution, principle V; FR-074; User Story 8, scenario 4).
//!
//! The command tree is walked as clap sees it. For each command, each option that takes a
//! value must appear in the command line of at least one scenario tagged `@invalid` that
//! runs that command; a command that takes only positional arguments must itself appear
//! in one. A command added without such a scenario fails this test.
//!
//! Flags that take no value cannot be given an invalid one and are not looked for.

mod common;

use clap::{ArgAction, Command};

/// The command lines of every scenario tagged `@invalid`, as words. Quotes only group
/// words in a scenario, so they are dropped.
fn rejected_command_lines() -> Vec<Vec<String>> {
    common::scenarios()
        .into_iter()
        .filter(|scenario| scenario.tags.iter().any(|tag| tag == "invalid"))
        .flat_map(|scenario| scenario.commands)
        .map(|line| line.split_whitespace().skip(1).map(|word| word.trim_matches('\'').to_owned()).collect())
        .collect()
}

/// Every command that can be run (one with no sub-commands), with the words that reach it.
fn runnable(command: &Command, path: &[String]) -> Vec<(Vec<String>, Command)> {
    let inner: Vec<&Command> = command.get_subcommands().filter(|inner| inner.get_name() != "help").collect();
    if inner.is_empty() {
        return vec![(path.to_vec(), command.clone())];
    }
    let mut found = Vec::new();
    for inner in inner {
        let mut inner_path = path.to_vec();
        inner_path.push(inner.get_name().to_owned());
        found.extend(runnable(inner, &inner_path));
    }
    found
}

/// The global options that take a value; the word after one of them is that value, not a
/// word of the command.
const GLOBAL_OPTIONS_WITH_A_VALUE: [&str; 3] = ["--workspace", "--output", "--color"];

/// Whether a command line runs the command reached by `path`: once options and the values
/// of global options are set aside, the line begins with the path's words.
fn runs(line: &[String], path: &[String]) -> bool {
    let mut words = Vec::new();
    let mut skip_next = false;
    for word in line {
        if skip_next {
            skip_next = false;
        } else if GLOBAL_OPTIONS_WITH_A_VALUE.contains(&word.as_str()) {
            skip_next = true;
        } else if !word.starts_with('-') {
            words.push(word);
        }
    }
    words.len() >= path.len() && words.iter().zip(path).all(|(word, expected)| *word == expected)
}

/// The long names of the options of a command that take a value.
fn valued_options(command: &Command) -> Vec<String> {
    command
        .get_arguments()
        .filter(|argument| !argument.is_positional())
        .filter(|argument| matches!(argument.get_action(), ArgAction::Set | ArgAction::Append))
        .filter_map(|argument| argument.get_long())
        .map(|long| format!("--{long}"))
        .collect()
}

#[test]
fn every_command_and_every_valued_option_has_a_rejection_scenario() {
    let lines = rejected_command_lines();
    assert!(lines.len() > 50, "the feature files hold the rejection scenarios this test reads");
    let mut missing = Vec::new();
    for (path, command) in runnable(&trcli_cli::cli::command(), &[]) {
        let of_command: Vec<&Vec<String>> = lines.iter().filter(|line| runs(line, &path)).collect();
        let takes_input = command.get_arguments().any(|argument| argument.is_positional()) || !valued_options(&command).is_empty();
        if takes_input && of_command.is_empty() {
            missing.push(format!("trcli {}: no rejection scenario at all", path.join(" ")));
        }
        for option in valued_options(&command) {
            if !of_command.iter().any(|line| line.contains(&option)) {
                missing.push(format!("trcli {}: no rejection scenario gives {option}", path.join(" ")));
            }
        }
    }
    assert!(missing.is_empty(), "inputs with no scenario tagged @invalid:\n{}", missing.join("\n"));
}

#[test]
fn the_walk_sees_the_commands_it_is_meant_to_check() {
    let commands: Vec<String> = runnable(&trcli_cli::cli::command(), &[]).into_iter().map(|(path, _)| path.join(" ")).collect();
    for expected in ["init", "workspace edit", "config set", "link add", "audit list", "audit export", "specimen list", "sample-note add"] {
        assert!(commands.contains(&expected.to_owned()), "`{expected}` is not in {commands:?}");
    }
}

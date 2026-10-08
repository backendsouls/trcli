//! Every command explains itself with an example, and every group of commands has a usage
//! guide (FR-055, FR-058; User Story 8, scenario 5).
//!
//! The command tree is walked as clap sees it, so a command added without help, without
//! an example, or without a guide fails this test instead of reaching a researcher.

mod common;

use clap::Command;

/// Every command of the tree with the words that reach it.
fn all_commands(command: &Command, path: &[String]) -> Vec<(Vec<String>, Command)> {
    let mut found = vec![(path.to_vec(), command.clone())];
    for inner in command.get_subcommands().filter(|inner| inner.get_name() != "help") {
        let mut inner_path = path.to_vec();
        inner_path.push(inner.get_name().to_owned());
        found.extend(all_commands(inner, &inner_path));
    }
    found
}

/// The text shown after a command's options by `--help`.
fn long_help(command: &Command) -> String {
    command.get_after_long_help().map(ToString::to_string).unwrap_or_default()
}

#[test]
fn every_command_says_what_it_is_for() {
    for (path, command) in all_commands(&trcli_cli::cli::command(), &[]) {
        let about = command.get_about().map(ToString::to_string).unwrap_or_default();
        assert!(!about.trim().is_empty(), "`trcli {}` has no description", path.join(" "));
    }
}

#[test]
fn every_command_shows_an_example_that_names_it() {
    for (path, command) in all_commands(&trcli_cli::cli::command(), &[]).into_iter().filter(|(path, _)| !path.is_empty()) {
        let help = long_help(&command);
        let invocation = format!("trcli {}", path.join(" "));
        assert!(help.contains("Example:"), "`{invocation} --help` shows no example");
        assert!(help.contains(&format!("trcli {}", path[0])), "the example of `{invocation}` does not use the command");
    }
}

#[test]
fn every_option_and_argument_is_explained() {
    for (path, command) in all_commands(&trcli_cli::cli::command(), &[]) {
        for argument in command.get_arguments().filter(|argument| !argument.is_hide_set()) {
            let help = argument.get_help().map(ToString::to_string).unwrap_or_default();
            assert!(!help.trim().is_empty(), "`{}` of `trcli {}` has no help", argument.get_id(), path.join(" "));
        }
    }
}

#[test]
fn every_group_of_commands_names_a_usage_guide_that_exists_and_mentions_it() {
    let root = trcli_cli::cli::command();
    for group in root.get_subcommands().filter(|group| group.get_name() != "help") {
        let help = long_help(group);
        let guide = help
            .lines()
            .find_map(|line| line.trim().strip_prefix("Guide: "))
            .unwrap_or_else(|| panic!("`trcli {} --help` names no usage guide", group.get_name()));
        let path = common::repository().join(guide);
        let text = std::fs::read_to_string(&path).unwrap_or_else(|_| panic!("{guide} does not exist"));
        assert!(text.contains(&format!("trcli {}", group.get_name())), "{guide} never mentions `trcli {}`", group.get_name());
    }
}

#[test]
fn every_usage_guide_has_the_parts_a_guide_must_have() {
    let guides = common::files_under(&common::repository().join("docs/usage"), &["md"]);
    assert!(guides.len() >= 4, "the foundation has guides for workspace, config, records, and audit");
    for guide in guides.iter().filter(|guide| !guide.ends_with("README.md")) {
        let text = std::fs::read_to_string(guide).expect("a guide");
        for part in ["## Purpose", "## Commands", "## Examples", "## When it fails"] {
            assert!(text.contains(part), "{} has no `{part}` section", guide.display());
        }
        assert!(text.contains("```console"), "{} has no worked example", guide.display());
    }
}

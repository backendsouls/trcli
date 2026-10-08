//! Every acceptance scenario of the specification has an automated check (constitution,
//! principle IV; specs/000-foundation, User Story 8, scenario 3).
//!
//! The specification's user stories are read from `specs/000-foundation/spec.md`. For
//! each numbered acceptance scenario there must be either a Gherkin scenario tagged with
//! it (`@US3-07` is user story 3, scenario 7) under `tests/features/foundation/`, or an
//! entry in the table below naming the structural test that covers it.

mod common;

use std::collections::{BTreeMap, BTreeSet};

/// Acceptance scenarios that are not shown through the binary: each is covered by the
/// test file named, which must exist.
const STRUCTURAL: [(&str, &str); 6] = [
    ("US8-03", "tests/scenario_coverage.rs"),
    ("US8-04", "tests/invalid_input_gate.rs"),
    ("US8-05", "tests/help_examples.rs"),
    ("US8-06", ".github/workflows/ci.yml"),
    ("US8-07", "tests/layering.rs"),
    ("US8-09", "crates/trcli-infra-sqlite/tests/upgrade.rs"),
];

/// How many acceptance scenarios each user story of the specification has.
fn scenarios_per_story() -> BTreeMap<u32, u32> {
    let spec = std::fs::read_to_string(common::repository().join("specs/000-foundation/spec.md"))
        .expect("the spec");
    let mut counts = BTreeMap::new();
    let mut story = None;
    for line in spec.lines() {
        if let Some(rest) = line.strip_prefix("### User Story ") {
            story = rest
                .split_whitespace()
                .next()
                .and_then(|number| number.parse::<u32>().ok());
        } else if line.starts_with("### ") || line.starts_with("## ") {
            story = None;
        } else if let Some(story) = story {
            // An acceptance scenario is a numbered item that begins with "Given".
            let numbered = line
                .split_once(". **Given**")
                .is_some_and(|(number, _)| number.parse::<u32>().is_ok());
            if numbered {
                *counts.entry(story).or_insert(0) += 1;
            }
        }
    }
    counts
}

/// The scenario tags found in the foundation's feature files.
fn tags_in_features() -> BTreeSet<String> {
    common::scenarios()
        .into_iter()
        .flat_map(|scenario| scenario.tags)
        .filter(|tag| tag.starts_with("US"))
        .collect()
}

#[test]
fn the_specification_has_the_user_stories_this_test_expects() {
    let counts = scenarios_per_story();
    assert_eq!(
        counts.len(),
        8,
        "the foundation has eight user stories: {counts:?}"
    );
    assert_eq!(
        counts.values().sum::<u32>(),
        95,
        "the foundation has 95 acceptance scenarios: {counts:?}"
    );
}

#[test]
fn every_acceptance_scenario_has_an_automated_check() {
    let tags = tags_in_features();
    let structural: BTreeSet<&str> = STRUCTURAL.iter().map(|(tag, _)| *tag).collect();
    let mut missing = Vec::new();
    for (story, count) in scenarios_per_story() {
        for scenario in 1..=count {
            let tag = format!("US{story}-{scenario:02}");
            if !tags.contains(&tag) && !structural.contains(tag.as_str()) {
                missing.push(tag);
            }
        }
    }
    assert!(
        missing.is_empty(),
        "acceptance scenarios with no automated check: {missing:?}"
    );
}

#[test]
fn every_tag_names_a_scenario_that_exists() {
    let counts = scenarios_per_story();
    for tag in tags_in_features() {
        let (story, scenario) = tag[2..]
            .split_once('-')
            .unwrap_or_else(|| panic!("`@{tag}` is not `@US<n>-<mm>`"));
        let (story, scenario): (u32, u32) = (
            story.parse().expect("a story number"),
            scenario.parse().expect("a number"),
        );
        let known = counts
            .get(&story)
            .is_some_and(|count| (1..=*count).contains(&scenario));
        assert!(
            known,
            "`@{tag}` names a scenario the specification does not have"
        );
    }
}

#[test]
fn every_structural_check_named_here_exists() {
    for (tag, file) in STRUCTURAL {
        assert!(
            common::repository().join(file).is_file(),
            "{tag} is said to be covered by {file}, which does not exist"
        );
    }
}

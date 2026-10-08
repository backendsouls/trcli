//! Text in any language and script is stored, shown, found, and sorted (FR-067).

mod common;

use common::Sandbox;

/// The titles added: two spellings of one Portuguese word, a title in Japanese, one in
/// Arabic (written right to left), and one with accents and typographic punctuation.
const TITLES: [&str; 5] = [
    "Ação",
    "acao",
    "東京",
    "مرحبا بالعالم",
    "Ünïcödé — “quoted”",
];

/// A workspace holding one specimen per title.
fn workspace() -> Sandbox {
    let sandbox = Sandbox::with_workspace();
    for title in TITLES {
        sandbox.ok(&["specimen", "add", "--title", title, "--quiet"]);
    }
    sandbox
}

#[test]
fn every_title_is_stored_and_shown_exactly_as_written() {
    let sandbox = workspace();
    let listed = sandbox.ok(&["specimen", "list", "--output", "json"]).json();
    let names: Vec<&str> = listed["data"]["items"]
        .as_array()
        .expect("items")
        .iter()
        .filter_map(|item| item["name"].as_str())
        .collect();
    for title in TITLES {
        assert!(names.contains(&title), "`{title}` is not among {names:?}");
    }
    let human = sandbox.ok(&["specimen", "list"]);
    for title in TITLES {
        assert!(
            human.stdout.contains(title),
            "`{title}` is not in:\n{}",
            human.stdout
        );
    }
}

#[test]
fn a_search_finds_both_spellings_whatever_the_accents_and_case() {
    let sandbox = workspace();
    for typed in ["acao", "AÇÃO", "Acão"] {
        let found = sandbox
            .ok(&["specimen", "list", "--search", typed, "--output", "json"])
            .json();
        assert_eq!(found["data"]["total"], 2, "searching for `{typed}`");
    }
    let japanese = sandbox
        .ok(&["specimen", "list", "--search", "東京", "--output", "json"])
        .json();
    assert_eq!(japanese["data"]["items"][0]["name"], "東京");
    let arabic = sandbox
        .ok(&["specimen", "list", "--search", "مرحبا", "--output", "json"])
        .json();
    assert_eq!(arabic["data"]["total"], 1);
}

#[test]
fn sorting_by_title_orders_every_script_without_error() {
    let sandbox = workspace();
    let ascending = sandbox
        .ok(&["specimen", "list", "--sort", "title", "--output", "json"])
        .json();
    let descending = sandbox
        .ok(&[
            "specimen", "list", "--sort", "title", "--desc", "--output", "json",
        ])
        .json();
    let names = |listing: &serde_json::Value| -> Vec<String> {
        listing["data"]["items"]
            .as_array()
            .expect("items")
            .iter()
            .filter_map(|item| item["name"].as_str())
            .map(str::to_owned)
            .collect()
    };
    let (mut up, down) = (names(&ascending), names(&descending));
    assert_eq!(up.len(), TITLES.len());
    // The two Portuguese spellings sort together, before the title that starts with "u".
    let position = |title: &str| up.iter().position(|name| name == title).expect("listed");
    assert!(position("Ação").abs_diff(position("acao")) == 1);
    assert!(position("acao") < position("Ünïcödé — “quoted”"));
    up.reverse();
    assert_eq!(up, down, "descending is ascending reversed");
}

#[test]
fn a_narrow_terminal_shortens_wide_characters_without_breaking_them() {
    let sandbox = Sandbox::with_workspace();
    sandbox.ok(&[
        "specimen",
        "add",
        "--title",
        "東京都立大学における土壌微生物の研究についての長い題名",
        "--quiet",
    ]);
    let listed = sandbox
        .command(&["specimen", "list"])
        .env("COLUMNS", "40")
        .output()
        .expect("trcli runs");
    let text = String::from_utf8(listed.stdout)
        .expect("the output is valid UTF-8: no character was cut in two");
    assert!(text.contains('…'), "{text}");
    assert!(text.contains("東京"), "{text}");
}

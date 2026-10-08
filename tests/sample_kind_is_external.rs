//! No foundation file names the sample record kinds (FR-068; proof for User Story 8).
//!
//! The sample kinds get everything records share by registering a descriptor. If making
//! them work had needed a change to foundation code, that code would name them. This
//! test reads every source file of every crate outside the `src/sample/` directories and
//! fails if one does.

mod common;

/// The words that would betray a foundation file that knows the sample kinds.
const NAMES: [&str; 3] = ["specimen", "sample-note", "sample_note"];

#[test]
fn no_foundation_source_file_names_a_sample_kind() {
    let crates = common::repository().join("crates");
    let mut offenders = Vec::new();
    for file in common::files_under(&crates, &["rs"]) {
        let relative = file
            .strip_prefix(&crates)
            .expect("under crates/")
            .to_string_lossy()
            .replace('\\', "/");
        // The sample kinds' own code, and the adapter's test of that code.
        let is_sample_code =
            relative.contains("/src/sample/") || relative.ends_with("/tests/sample_kinds.rs");
        let is_source = relative.contains("/src/");
        if is_sample_code || !is_source {
            continue;
        }
        let text = std::fs::read_to_string(&file)
            .expect("a source file")
            .to_lowercase();
        for name in NAMES {
            if text.contains(name) {
                offenders.push(format!("{relative} names `{name}`"));
            }
        }
    }
    assert!(
        offenders.is_empty(),
        "foundation files must not know the sample kinds:\n{}",
        offenders.join("\n")
    );
}

#[test]
fn the_sample_kinds_live_where_this_test_expects() {
    let crates = common::repository().join("crates");
    for directory in [
        "trcli-application/src/sample",
        "trcli-infra-sqlite/src/sample",
        "trcli-cli/src/sample",
    ] {
        assert!(
            crates.join(directory).is_dir(),
            "{directory} is where the sample kinds are kept"
        );
    }
}

#[test]
fn a_build_without_the_feature_has_no_sample_command() {
    // This test target is built without the `sample-kind` feature unless it is asked for.
    if cfg!(feature = "sample-kind") {
        return;
    }
    let root = trcli_cli::cli::command();
    let nouns: Vec<&str> = root
        .get_subcommands()
        .map(clap::Command::get_name)
        .collect();
    assert!(
        !nouns.contains(&"specimen") && !nouns.contains(&"sample-note"),
        "{nouns:?}"
    );
}

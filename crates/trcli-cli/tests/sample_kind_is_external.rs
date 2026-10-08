//! No file of the foundation names the sample record kinds (FR-068; proof for User
//! Story 8).
//!
//! The sample kinds live in an example, outside the crates' sources, and get everything
//! records share by registering a descriptor. If making them work had needed a change to
//! foundation code, that code would name them. This test reads every source file of
//! every crate and fails if one does.

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
        // Only the crates' own sources: their tests and examples are not the foundation.
        if !relative.contains("/src/") {
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
fn the_sample_kinds_live_in_the_example() {
    let example = common::repository().join("crates/trcli-cli/examples/sample_kinds/sample");
    for part in ["application", "storage", "commands"] {
        assert!(
            example.join(part).is_dir(),
            "the sample feature has a `{part}` part"
        );
    }
}

#[test]
fn the_trcli_binary_has_no_sample_command() {
    let root = trcli_cli::cli::command(&trcli_cli::extension::NoExtension);
    let nouns: Vec<&str> = root
        .get_subcommands()
        .map(clap::Command::get_name)
        .collect();
    assert!(
        !nouns.contains(&"specimen") && !nouns.contains(&"sample-note"),
        "{nouns:?}"
    );
    let help = common::Sandbox::new().ok(&["--help"]).stdout;
    assert!(
        !help.contains("specimen") && !help.contains("sample-note"),
        "{help}"
    );
}

//! The dependency graph obeys the layers (FR-070; User Story 8, scenario 7), and nothing
//! in it can reach the network (SC-010).
//!
//! The compiler already refuses an import from a crate that is not a dependency; this
//! test refuses the dependency itself, by reading the manifests through `cargo metadata`.
//! To see it fail, add `sea-orm` to `crates/trcli-domain/Cargo.toml`.

mod common;

use std::collections::BTreeMap;
use std::process::Command;

/// The direct dependencies of each crate of the workspace that are compiled into it
/// (development-only dependencies are left out), with the features asked of each.
fn dependencies() -> BTreeMap<String, BTreeMap<String, Vec<String>>> {
    let cargo = std::env::var("CARGO").unwrap_or_else(|_| "cargo".to_owned());
    let output = Command::new(cargo)
        .args(["metadata", "--format-version", "1", "--no-deps"])
        .current_dir(common::repository())
        .output()
        .expect("cargo metadata runs");
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let metadata: serde_json::Value =
        serde_json::from_slice(&output.stdout).expect("JSON from cargo metadata");
    let mut graph = BTreeMap::new();
    for package in metadata["packages"].as_array().expect("packages") {
        let name = package["name"].as_str().expect("a name").to_owned();
        let mut direct = BTreeMap::new();
        for dependency in package["dependencies"].as_array().expect("dependencies") {
            if dependency["kind"].is_null() {
                let features =
                    dependency["features"]
                        .as_array()
                        .map_or_else(Vec::new, |features| {
                            features
                                .iter()
                                .filter_map(|feature| feature.as_str())
                                .map(str::to_owned)
                                .collect()
                        });
                direct.insert(
                    dependency["name"].as_str().expect("a name").to_owned(),
                    features,
                );
            }
        }
        graph.insert(name, direct);
    }
    graph
}

/// The names of a crate's direct dependencies.
fn names(graph: &BTreeMap<String, BTreeMap<String, Vec<String>>>, package: &str) -> Vec<String> {
    graph
        .get(package)
        .unwrap_or_else(|| panic!("{package} is not in the workspace"))
        .keys()
        .cloned()
        .collect()
}

#[test]
fn the_workspace_has_the_five_crates_of_the_plan_and_one_for_test_support() {
    let graph = dependencies();
    let crates: Vec<&str> = graph.keys().map(String::as_str).collect();
    assert_eq!(
        crates,
        [
            "trcli-application",
            "trcli-cli",
            "trcli-domain",
            "trcli-infra-sqlite",
            "trcli-infra-system",
            "trcli-testing"
        ]
    );
}

#[test]
fn test_support_is_never_compiled_into_the_tool() {
    let graph = dependencies();
    // The graph holds only what is compiled into each crate; development dependencies
    // are left out. The doubles and contract suites may appear in none of them.
    for (package, direct) in &graph {
        assert!(
            !direct.contains_key("trcli-testing"),
            "{package} depends on the test-support crate"
        );
    }
    let allowed = ["time", "trcli-application", "trcli-domain"];
    for dependency in names(&graph, "trcli-testing") {
        assert!(
            allowed.contains(&dependency.as_str()),
            "trcli-testing may not depend on `{dependency}`"
        );
    }
}

#[test]
fn the_domain_depends_on_nothing_that_stores_shows_or_invokes() {
    let allowed = ["thiserror", "time", "unicode-normalization", "uuid"];
    for dependency in names(&dependencies(), "trcli-domain") {
        assert!(
            allowed.contains(&dependency.as_str()),
            "trcli-domain may not depend on `{dependency}`"
        );
    }
}

#[test]
fn the_application_depends_on_the_domain_and_on_no_framework() {
    let allowed = ["serde", "thiserror", "time", "trcli-domain"];
    for dependency in names(&dependencies(), "trcli-application") {
        assert!(
            allowed.contains(&dependency.as_str()),
            "trcli-application may not depend on `{dependency}`"
        );
    }
}

#[test]
fn the_two_adapter_crates_do_not_depend_on_each_other_or_on_the_command_line() {
    let graph = dependencies();
    let sqlite = names(&graph, "trcli-infra-sqlite");
    let system = names(&graph, "trcli-infra-system");
    assert!(!sqlite.contains(&"trcli-infra-system".to_owned()));
    assert!(!system.contains(&"trcli-infra-sqlite".to_owned()));
    for adapter in [&sqlite, &system] {
        assert!(
            !adapter.contains(&"clap".to_owned()) && !adapter.contains(&"trcli-cli".to_owned())
        );
    }
    assert!(
        !system.contains(&"sea-orm".to_owned()),
        "only the SQLite adapter knows SeaORM"
    );
}

#[test]
fn only_the_command_line_crate_names_every_layer() {
    let graph = dependencies();
    let cli = names(&graph, "trcli-cli");
    for layer in [
        "trcli-domain",
        "trcli-application",
        "trcli-infra-sqlite",
        "trcli-infra-system",
    ] {
        assert!(
            cli.contains(&layer.to_owned()),
            "trcli-cli composes `{layer}`"
        );
    }
    for inner in [
        "trcli-domain",
        "trcli-application",
        "trcli-infra-sqlite",
        "trcli-infra-system",
    ] {
        assert!(
            !names(&graph, inner).contains(&"trcli-cli".to_owned()),
            "`{inner}` may not depend on the command line"
        );
    }
}

#[test]
fn nothing_in_the_workspace_can_reach_the_network() {
    let network_crates = [
        "reqwest",
        "hyper",
        "ureq",
        "curl",
        "isahc",
        "surf",
        "tonic",
        "tungstenite",
        "socket2",
        "h2",
    ];
    for (package, direct) in dependencies() {
        for (dependency, features) in direct {
            assert!(
                !network_crates.contains(&dependency.as_str()),
                "{package} depends on the network crate `{dependency}`"
            );
            if dependency == "tokio" {
                let reaches_out = features
                    .iter()
                    .any(|feature| feature == "net" || feature == "full");
                assert!(
                    !reaches_out,
                    "{package} enables tokio's networking: {features:?}"
                );
            }
        }
    }
}

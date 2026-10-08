//! Behaviour tests: the acceptance scenarios of the specifications, written in Gherkin
//! under `tests/features/`, run against the compiled `trcli` binary (constitution,
//! principle IV).
//!
//! The steps in [`steps`] are generic — "Given a workspace", "When I run …", "Then the
//! exit code is …" — so that a later feature adds scenarios, not step code. Each scenario
//! gets its own temporary directory, a fixed clock, and seeded identifiers, so that it
//! gives the same output on every run and on every system.
//!
//! Run with: `cargo test --test bdd --features test-clock,sample-kind`.

mod steps;
mod world;

use cucumber::World as _;

use world::TrcliWorld;

/// Whether a scenario can run on this system: scenarios tagged `@unix` send signals or
/// change file permissions in ways only Unix has.
fn runs_here(tags: &[String]) -> bool {
    cfg!(unix) || !tags.iter().any(|tag| tag == "unix")
}

/// Runs every feature file and fails the test target when a scenario fails or a step is
/// undefined.
#[tokio::main(flavor = "current_thread")]
async fn main() {
    let features = concat!(env!("CARGO_MANIFEST_DIR"), "/../../tests/features");
    TrcliWorld::cucumber()
        .fail_on_skipped()
        .filter_run_and_exit(features, |feature, _rule, scenario| {
            let tags: Vec<String> = feature.tags.iter().chain(&scenario.tags).cloned().collect();
            runs_here(&tags)
        })
        .await;
}

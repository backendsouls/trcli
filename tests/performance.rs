//! The speed a researcher can count on (SC-012; FR-065).
//!
//! - A simple command answers in under a tenth of a second.
//! - Listing or searching 10,000 records of one kind answers in under two seconds.
//!
//! These are timings, so they are ignored by default and run on purpose, on an optimized
//! build: `cargo test --release --test performance --features test-clock,sample-kind --
//! --ignored`. Each figure is the best of several runs, so that one slow start of a
//! process on a busy machine does not fail the build.

mod common;

use std::time::{Duration, Instant};

use common::Sandbox;
use sea_orm::{ConnectionTrait, Database};

/// The shortest time `trcli` takes with these arguments, over a few runs.
fn best_of(sandbox: &Sandbox, arguments: &[&str]) -> Duration {
    (0..5)
        .map(|_| {
            let started = Instant::now();
            sandbox.ok(arguments);
            started.elapsed()
        })
        .min()
        .expect("at least one run")
}

/// Puts 10,000 specimens into the workspace directly, as a script loading data would.
async fn load_ten_thousand(sandbox: &Sandbox) {
    let database = sandbox.work().join(".trcli/trcli.db");
    let mut options = sea_orm::ConnectOptions::new("sqlite:trcli-test");
    options
        .sqlx_logging(false)
        .map_sqlx_sqlite_opts(move |sqlite| sqlite.filename(&database));
    let connection = Database::connect(options).await.expect("the database");
    let mut statements = String::from("BEGIN;");
    for number in 0..10_000_u32 {
        let id = format!("01920000-0000-7000-9000-{number:012x}");
        let word = ["soil", "rain", "leaf", "root", "seed"][number as usize % 5];
        statements.push_str(&format!(
            "INSERT INTO record (id, kind, handle, display_name, search_key, created_at, updated_at) \
             VALUES ('{id}', 'specimen', 'spc-{number:08x}', 'Sample {number} of {word}', 'sample {number} of {word}', {number}, {number});\
             INSERT INTO specimen (id, title) VALUES ('{id}', 'Sample {number} of {word}');"
        ));
    }
    statements.push_str("COMMIT;");
    connection
        .execute_unprepared(&statements)
        .await
        .expect("the records are loaded");
    connection.close().await.expect("closed");
}

#[test]
#[ignore = "a timing; run on purpose with --ignored on a release build"]
fn a_simple_command_answers_in_under_a_tenth_of_a_second() {
    let sandbox = Sandbox::with_workspace();
    let limit = Duration::from_millis(100);
    for arguments in [
        &["--version"][..],
        &["workspace", "show"][..],
        &["config", "list"][..],
    ] {
        let took = best_of(&sandbox, arguments);
        assert!(
            took < limit,
            "`trcli {}` took {took:?}",
            arguments.join(" ")
        );
    }
}

#[tokio::test(flavor = "current_thread")]
#[ignore = "a timing; run on purpose with --ignored on a release build"]
async fn listing_and_searching_ten_thousand_records_answers_in_under_two_seconds() {
    let sandbox = Sandbox::with_workspace();
    load_ten_thousand(&sandbox).await;
    let limit = Duration::from_secs(2);
    let listed = sandbox
        .ok(&["specimen", "list", "--limit", "1000", "--output", "json"])
        .json();
    assert_eq!(listed["data"]["total"], 10_000);
    for arguments in [
        &["specimen", "list"][..],
        &["specimen", "list", "--limit", "1000"][..],
        &[
            "specimen", "list", "--search", "rain", "--sort", "created", "--desc",
        ][..],
        &["specimen", "show", "spc-00001f40"][..],
    ] {
        let took = best_of(&sandbox, arguments);
        assert!(
            took < limit,
            "`trcli {}` took {took:?}",
            arguments.join(" ")
        );
    }
}

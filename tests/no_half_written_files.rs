//! No file the tool writes is ever left half-written (FR-066).
//!
//! Three kinds of file are written outside the database: settings files, the audit head,
//! and the copy kept before an upgrade. Each is written under a temporary name and then
//! renamed. Here the rename — the last step — is made to fail, as an interruption or a
//! full disk would, and the original must be intact with nothing left beside it.

mod common;

use std::io;
use std::path::Path;

use trcli_infra_system::atomic::{copy_atomically_with, write_atomically, write_atomically_with};

/// A rename that fails, as when the process is stopped just before it.
fn interrupted(_from: &Path, _to: &Path) -> io::Result<()> {
    Err(io::Error::other("stopped before the rename"))
}

/// The names of the files in a directory.
fn files_in(directory: &Path) -> Vec<String> {
    let mut names: Vec<String> = std::fs::read_dir(directory)
        .expect("the directory")
        .filter_map(Result::ok)
        .map(|entry| entry.file_name().to_string_lossy().into_owned())
        .collect();
    names.sort();
    names
}

#[test]
fn an_interrupted_write_of_a_settings_file_leaves_the_file_as_it_was() {
    let directory = tempfile::tempdir().expect("a temporary directory");
    let settings = directory.path().join("config.toml");
    write_atomically(&settings, b"[output]\ncolor = \"never\"\n").expect("the first write");
    let failed = write_atomically_with(&settings, b"[output]\ncolor = \"alw", interrupted);
    assert!(failed.is_err());
    assert_eq!(std::fs::read_to_string(&settings).expect("readable"), "[output]\ncolor = \"never\"\n");
    assert_eq!(files_in(directory.path()), ["config.toml"]);
}

#[test]
fn an_interrupted_write_of_the_audit_head_leaves_the_head_as_it_was() {
    let directory = tempfile::tempdir().expect("a temporary directory");
    let head = directory.path().join("audit.head");
    let original = format!("12 {}\n", "ab".repeat(32));
    write_atomically(&head, original.as_bytes()).expect("the first write");
    let failed = write_atomically_with(&head, b"13 cd", interrupted);
    assert!(failed.is_err());
    assert_eq!(std::fs::read_to_string(&head).expect("readable"), original);
    assert_eq!(files_in(directory.path()), ["audit.head"]);
}

#[test]
fn an_interrupted_copy_before_an_upgrade_leaves_no_partial_copy() {
    let directory = tempfile::tempdir().expect("a temporary directory");
    let database = directory.path().join("trcli.db");
    std::fs::write(&database, vec![7_u8; 64 * 1024]).expect("the database");
    let copy = directory.path().join("trcli-copy.db");
    let failed = copy_atomically_with(&database, &copy, interrupted);
    assert!(failed.is_err());
    assert_eq!(files_in(directory.path()), ["trcli.db"], "no copy, whole or partial, is left");
    assert_eq!(std::fs::read(&database).expect("readable").len(), 64 * 1024);
}

#[test]
fn the_tool_itself_leaves_no_temporary_file_in_a_workspace() {
    let sandbox = common::Sandbox::with_workspace();
    sandbox.ok(&["config", "set", "output.page_size", "20"]);
    sandbox.ok(&["workspace", "edit", "--researcher", "Ana Souza"]);
    sandbox.ok(&["config", "unset", "output.page_size"]);
    let files = files_in(&sandbox.work().join(".trcli"));
    // Between commands a workspace is exactly these: no journal, no temporary file.
    assert_eq!(files, ["audit.head", "backups", "config.toml", "trcli.db"]);
}

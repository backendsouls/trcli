//! Unit tests for creating a workspace (T048).

use std::path::{Path, PathBuf};

use trcli_application::outcome::{Details, codes};
use trcli_application::workspace::init::{InitCommand, InitInput, InitServices, init};
use trcli_testing::audit::MemoryHead;
use trcli_testing::block_on;
use trcli_testing::environment::{SeededIds, stamp};
use trcli_testing::workspace::{FakeFiles, FakeOpener, FakeProbe};

/// The input `trcli init --name <name>` gives in `/research`.
fn input(name: Option<&str>) -> InitInput {
    InitInput {
        directory: PathBuf::from("/research"),
        name: name.map(str::to_owned),
        description: None,
    }
}

#[test]
fn an_empty_name_and_an_unwritable_directory_are_reported_together() {
    let files = FakeFiles::with_unwritable("/research");
    let problem =
        InitCommand::new(input(Some("")), &files, &FakeProbe::default()).expect_err("invalid");
    let Details::Fields(fields) = problem.details else {
        panic!("fields expected")
    };
    let named: Vec<&str> = fields.iter().map(|field| field.field.as_str()).collect();
    assert_eq!(named, ["--name", "<dir>"]);
}

#[test]
fn a_missing_name_is_invalid_input() {
    let problem = InitCommand::new(input(None), &FakeFiles::new(), &FakeProbe::default())
        .expect_err("invalid");
    assert_eq!(problem.code, codes::VALIDATION_FAILED);
}

#[test]
fn a_workspace_is_not_created_where_one_exists() {
    let probe = FakeProbe::with_workspaces(&["/research"]);
    let problem =
        InitCommand::new(input(Some("Again")), &FakeFiles::new(), &probe).expect_err("exists");
    assert_eq!(problem.code, codes::WORKSPACE_EXISTS);
    // Joined, not written out: the separator is the system's.
    let place = PathBuf::from("/research")
        .join(".trcli")
        .display()
        .to_string();
    assert!(problem.message.contains(&place), "{}", problem.message);
}

#[test]
fn creating_stores_the_workspace_with_its_first_audit_entry_and_head() {
    let (opener, files, ids, head) = (
        FakeOpener::new(),
        FakeFiles::new(),
        SeededIds::new(),
        MemoryHead::new(),
    );
    let services = InitServices {
        opener: &opener,
        files: &files,
        ids: &ids,
        head: &head,
    };
    let command = InitCommand::new(input(Some("Doctorate")), &files, &FakeProbe::default())
        .expect("valid")
        .command;
    let database = Path::new("/research/.trcli/trcli.db");
    let created = block_on(init(&services, &stamp(), database, command)).expect("created");
    assert_eq!(
        (created.name.as_str(), PathBuf::from(&created.location)),
        ("Doctorate", PathBuf::from("/research").join(".trcli"))
    );

    let state = opener.storage(database).expect("storage").snapshot();
    assert_eq!(state.workspace.expect("row").name.as_str(), "Doctorate");
    assert_eq!(
        (state.audit.len(), state.audit[0].action.as_str()),
        (1, "create")
    );
    assert_eq!(head.current().expect("head").sequence, 1);
}

#[test]
fn a_failure_part_way_leaves_nothing_behind() {
    let (opener, files, ids, head) = (
        FakeOpener::failing(),
        FakeFiles::new(),
        SeededIds::new(),
        MemoryHead::new(),
    );
    let services = InitServices {
        opener: &opener,
        files: &files,
        ids: &ids,
        head: &head,
    };
    let command = InitCommand::new(input(Some("Doctorate")), &files, &FakeProbe::default())
        .expect("valid")
        .command;
    assert!(
        block_on(init(
            &services,
            &stamp(),
            Path::new("/research/.trcli/trcli.db"),
            command
        ))
        .is_err()
    );
    assert!(files.prepared.borrow().is_empty());
    assert_eq!(head.current(), None);
}

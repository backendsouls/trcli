//! Creating a workspace (FR-001, FR-002).
//!
//! The name is required, the directory must be writable, and no workspace may be there
//! already. Everything is checked before anything is created; if creation fails part-way,
//! what was created is removed, so nothing half-made is left behind.

use std::path::{Path, PathBuf};

use serde::Serialize;
use trcli_domain::governance::audit::{AuditAction, AuditDraft, Change, Stamp};
use trcli_domain::shared::problem::Rejection;
use trcli_domain::shared::text::{LongText, Name};
use trcli_domain::workspace::Workspace;

use crate::governance::record::commit;
use crate::outcome::{Problem, codes};
use crate::ports::audit::{AuditHeadStore, AuditLog};
use crate::ports::environment::IdGenerator;
use crate::ports::unit_of_work::Storage;
use crate::ports::workspace::{StorageOpener, WorkspaceFiles, WorkspaceProbe, WorkspaceStore};
use crate::validation::{Checker, Valid};

/// What the researcher gave to `trcli init`, unchecked.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct InitInput {
    /// The directory to create the workspace in.
    pub directory: PathBuf,
    /// `--name`.
    pub name: Option<String>,
    /// `--description`.
    pub description: Option<String>,
}

/// A checked request to create a workspace.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InitCommand {
    /// The directory that will hold `.trcli/`.
    pub root: PathBuf,
    /// The workspace's name.
    pub name: Name,
    /// What it is for.
    pub description: LongText,
}

impl InitCommand {
    /// Checks the name, the description, and the directory together, then that no
    /// workspace is there already.
    pub fn new(
        input: InitInput,
        files: &impl WorkspaceFiles,
        probe: &impl WorkspaceProbe,
    ) -> Result<Valid<Self>, Problem> {
        let mut checker = Checker::new();
        let name = checker.required("--name", input.name.as_deref(), Name::new);
        let description =
            checker.optional("--description", input.description.as_deref(), LongText::new);
        if !files.is_writable(&input.directory) {
            let rejection = Rejection::new(
                "is not a directory that can be written to",
                "an existing, writable directory",
            );
            checker.reject("<dir>", &input.directory.display().to_string(), rejection);
        }
        let valid = checker.finish(|| Self {
            root: input.directory.clone(),
            name: name.expect("checked"),
            description: description
                .flatten()
                .unwrap_or_else(|| LongText::new("").expect("empty text is valid")),
        })?;
        if probe.holds_workspace(&valid.command.root) {
            let place = valid.command.root.join(".trcli");
            return Err(Problem::new(
                codes::WORKSPACE_EXISTS,
                format!("a workspace already exists in {}", place.display()),
            ));
        }
        Ok(valid)
    }
}

/// What `init` reports: the workspace that now exists.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct WorkspaceCreated {
    /// The workspace's name.
    pub name: String,
    /// Where it is: the `.trcli` directory.
    pub location: String,
    /// The format it is stored in.
    pub format_version: u32,
}

/// The outside things `init` needs.
#[derive(Debug)]
pub struct InitServices<'a, O, F, I, H> {
    /// Creates the storage.
    pub opener: &'a O,
    /// Creates the other files.
    pub files: &'a F,
    /// Gives the workspace its identifier.
    pub ids: &'a I,
    /// Keeps the end of the audit trail.
    pub head: &'a H,
}

/// Creates the workspace: its files, its storage, its row, and the first audit entry.
pub async fn init<O, F, I, H>(
    services: &InitServices<'_, O, F, I, H>,
    stamp: &Stamp,
    database: &Path,
    command: InitCommand,
) -> Result<WorkspaceCreated, Problem>
where
    O: StorageOpener,
    <O::Storage as Storage>::Unit: WorkspaceStore + AuditLog,
    F: WorkspaceFiles,
    I: IdGenerator,
    H: AuditHeadStore,
{
    services.files.prepare(&command.root)?;
    let workspace = Workspace::new(
        services.ids.next_id(),
        command.name,
        command.description,
        stamp.at,
    );
    match store(services, stamp, database, &workspace).await {
        Ok(()) => Ok(WorkspaceCreated {
            name: workspace.name.to_string(),
            location: command.root.join(".trcli").display().to_string(),
            format_version: workspace.format_version.number(),
        }),
        Err(problem) => {
            // Nothing half-made may be left: a later `init` must find the directory as it
            // was (FR-009).
            services.files.discard(&command.root);
            Err(problem)
        }
    }
}

/// Creates the storage and writes the workspace's row with its audit entry.
async fn store<O, F, I, H>(
    services: &InitServices<'_, O, F, I, H>,
    stamp: &Stamp,
    database: &Path,
    workspace: &Workspace,
) -> Result<(), Problem>
where
    O: StorageOpener,
    <O::Storage as Storage>::Unit: WorkspaceStore + AuditLog,
    H: AuditHeadStore,
{
    let storage = services.opener.create(database).await?;
    let mut unit = storage.begin().await?;
    unit.save_workspace(workspace).await?;
    let changes = vec![Change::set("name", workspace.name.as_str())];
    let draft = AuditDraft::workspace(AuditAction::CREATE)
        .named(workspace.name.as_str())
        .with_changes(changes);
    unit.record(stamp, draft).await?;
    commit(unit, services.head).await?;
    // A new workspace is left at rest: one database file, nothing beside it to carry along.
    Ok(storage.close().await?)
}

#[cfg(test)]
mod tests {
    //! Unit tests for creating a workspace (T048).

    use std::path::{Path, PathBuf};

    use super::{InitCommand, InitInput, InitServices, init};
    use crate::outcome::{Details, codes};
    use crate::testing::audit::MemoryHead;
    use crate::testing::block_on;
    use crate::testing::environment::{SeededIds, stamp};
    use crate::testing::workspace::{FakeFiles, FakeOpener, FakeProbe};

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
        assert!(problem.message.contains("/research/.trcli"));
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
            (created.name.as_str(), created.location.as_str()),
            ("Doctorate", "/research/.trcli")
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
}

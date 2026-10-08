//! Changing a workspace's details (FR-001): its name, its description, and the name under
//! which the researcher's actions are recorded. Only what was named is changed.

use trcli_domain::governance::audit::{AuditAction, AuditDraft, Change, Stamp};
use trcli_domain::settings::{Place, SettingKey, SettingValue};
use trcli_domain::shared::problem::Rejection;
use trcli_domain::shared::text::{ActorName, LongText, Name};
use trcli_domain::workspace::Workspace;

use crate::outcome::{Problem, codes};
use crate::ports::audit::AuditLog;
use crate::ports::settings::SettingsFiles;
use crate::ports::workspace::WorkspaceStore;
use crate::settings::foundation::RESEARCHER_NAME;
use crate::validation::{Checker, Valid};
use crate::view::Done;

/// What the researcher gave to `trcli workspace edit`, unchecked.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct EditInput {
    /// `--name`.
    pub name: Option<String>,
    /// `--description`.
    pub description: Option<String>,
    /// `--researcher`.
    pub researcher: Option<String>,
}

/// A checked request to change a workspace's details.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EditCommand {
    /// The new name, when given.
    pub name: Option<Name>,
    /// The new description, when given.
    pub description: Option<LongText>,
    /// The new name for the researcher's actions, when given.
    pub researcher: Option<ActorName>,
}

impl EditCommand {
    /// Checks every value given, and that at least one was.
    pub fn new(input: EditInput) -> Result<Valid<Self>, Problem> {
        let mut checker = Checker::new();
        let name = checker.optional("--name", input.name.as_deref(), Name::new);
        let description =
            checker.optional("--description", input.description.as_deref(), LongText::new);
        let researcher =
            checker.optional("--researcher", input.researcher.as_deref(), ActorName::new);
        if input.name.is_none() && input.description.is_none() && input.researcher.is_none() {
            let rejection = Rejection::new(
                "nothing to change was given",
                "at least one of the three options",
            );
            checker.reject("--name, --description, --researcher", "", rejection);
        }
        checker.finish(|| Self {
            name: name.flatten(),
            description: description.flatten(),
            researcher: researcher.flatten(),
        })
    }
}

/// Changes what was named, records one audit entry listing each change, and says what
/// changed. `current_researcher` is the name in effect before, for the entry.
pub async fn edit<U>(
    unit: &mut U,
    stamp: &Stamp,
    files: &impl SettingsFiles,
    current_researcher: Option<&str>,
    command: EditCommand,
) -> Result<Done, Problem>
where
    U: WorkspaceStore + AuditLog,
{
    let mut workspace = unit.workspace().await?;
    let mut changes = apply_details(&mut workspace, command.name, command.description);
    if let Some(researcher) = command
        .researcher
        .filter(|name| Some(name.as_str()) != current_researcher)
    {
        changes.push(Change::new(
            RESEARCHER_NAME,
            current_researcher,
            Some(researcher.as_str()),
        ));
        store_researcher(files, &researcher)?;
    }
    if changes.is_empty() {
        return Ok(Done::new(
            "Nothing to change: the workspace already has these details",
        ));
    }
    let fields: Vec<&str> = changes.iter().map(|change| change.field.as_str()).collect();
    let message = format!(
        "Updated workspace \"{}\": {}",
        workspace.name,
        fields.join(", ")
    );
    unit.save_workspace(&workspace).await?;
    let draft = AuditDraft::workspace(AuditAction::UPDATE)
        .named(workspace.name.as_str())
        .with_changes(changes);
    unit.record(stamp, draft).await?;
    Ok(Done::new(message))
}

/// Puts a new name and description in the workspace when they differ from what it has,
/// and lists what changed.
fn apply_details(
    workspace: &mut Workspace,
    name: Option<Name>,
    description: Option<LongText>,
) -> Vec<Change> {
    let mut changes = Vec::new();
    if let Some(name) = name.filter(|name| name != &workspace.name) {
        changes.push(Change::new(
            "name",
            Some(workspace.name.as_str()),
            Some(name.as_str()),
        ));
        workspace.name = name;
    }
    if let Some(description) =
        description.filter(|description| description != &workspace.description)
    {
        changes.push(Change::new(
            "description",
            Some(workspace.description.as_str()),
            Some(description.as_str()),
        ));
        workspace.description = description;
    }
    changes
}

/// Writes the researcher's name to the workspace's settings.
fn store_researcher(files: &impl SettingsFiles, researcher: &ActorName) -> Result<(), Problem> {
    let key = SettingKey::new(RESEARCHER_NAME).expect("a valid key");
    files
        .store(
            Place::Workspace,
            &key,
            &SettingValue::Text(researcher.to_string()),
        )
        .map_err(|error| Problem::new(codes::OPERATION_FAILED, error.to_string()))
}

#[cfg(test)]
mod tests {
    //! Unit tests for changing a workspace's details (T048).

    use trcli_domain::settings::{Place, SettingValue};
    use trcli_domain::shared::text::{LongText, Name};
    use trcli_domain::workspace::Workspace;

    use super::{EditCommand, EditInput, edit};
    use crate::ports::unit_of_work::Storage;
    use crate::ports::workspace::WorkspaceStore;
    use crate::settings::foundation::RESEARCHER_NAME;
    use crate::testing::block_on;
    use crate::testing::environment::{SeededIds, stamp};
    use crate::testing::settings::MemoryFiles;
    use crate::testing::unit::{FakeStorage, FakeUnit};

    /// A unit holding a workspace named "Doctorate" described as "Thesis".
    fn unit() -> FakeUnit {
        block_on(async {
            let mut unit = FakeStorage::new().begin().await.expect("begin");
            let (name, about) = (
                Name::new("Doctorate").expect("valid"),
                LongText::new("Thesis").expect("valid"),
            );
            unit.save_workspace(&Workspace::new(SeededIds::at(0), name, about, stamp().at))
                .await
                .expect("save");
            unit
        })
    }

    /// The checked command for the given options.
    fn command(
        name: Option<&str>,
        description: Option<&str>,
        researcher: Option<&str>,
    ) -> EditCommand {
        let owned = |value: Option<&str>| value.map(str::to_owned);
        let input = EditInput {
            name: owned(name),
            description: owned(description),
            researcher: owned(researcher),
        };
        EditCommand::new(input).expect("valid").command
    }

    #[test]
    fn edit_changes_only_what_was_named_and_records_one_entry() {
        let (mut unit, files) = (unit(), MemoryFiles::new());
        let done = block_on(edit(
            &mut unit,
            &stamp(),
            &files,
            None,
            command(Some("Renamed"), None, None),
        ))
        .expect("ok");
        assert_eq!(done.message, "Updated workspace \"Renamed\": name");
        let state = unit.state();
        let workspace = state.workspace.clone().expect("row");
        assert_eq!(
            (workspace.name.as_str(), workspace.description.as_str()),
            ("Renamed", "Thesis")
        );
        assert_eq!(state.audit.len(), 1);
        assert_eq!(
            state.audit[0].changes[0].before.as_deref(),
            Some("Doctorate")
        );
    }

    #[test]
    fn the_researcher_name_goes_to_the_workspace_settings() {
        let (mut unit, files) = (unit(), MemoryFiles::new());
        block_on(edit(
            &mut unit,
            &stamp(),
            &files,
            Some("ana"),
            command(None, None, Some("Ana Souza")),
        ))
        .expect("ok");
        assert_eq!(
            files.value(Place::Workspace, RESEARCHER_NAME),
            Some(SettingValue::Text("Ana Souza".into()))
        );
        assert_eq!(unit.state().audit[0].changes[0].field, RESEARCHER_NAME);
    }

    #[test]
    fn giving_the_same_details_changes_nothing_and_records_nothing() {
        let (mut unit, files) = (unit(), MemoryFiles::new());
        let done = block_on(edit(
            &mut unit,
            &stamp(),
            &files,
            None,
            command(Some("Doctorate"), None, None),
        ))
        .expect("ok");
        assert!(done.message.starts_with("Nothing to change"));
        assert!(unit.state().audit.is_empty());
    }

    #[test]
    fn giving_nothing_or_an_invalid_value_is_rejected() {
        assert!(EditCommand::new(EditInput::default()).is_err());
        let invalid = EditInput {
            name: Some(String::new()),
            ..EditInput::default()
        };
        assert!(EditCommand::new(invalid).is_err());
    }
}

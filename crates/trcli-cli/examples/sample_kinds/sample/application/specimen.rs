//! The sample kind `specimen`: a record with one field, a title.
//!
//! This file is everything a feature writes for a kind of record at this layer: a
//! descriptor, a port for its own rows, the behaviour only it can know, and its `add` and
//! `edit` use cases. Short names, tags, notes, links, listing, showing, and guarded
//! deletion are not here: they come from the foundation.

use trcli_domain::governance::audit::{Change, Stamp};
use trcli_domain::shared::problem::Warning;
use trcli_domain::shared::record::RecordId;
use trcli_domain::shared::text::{SearchKey, Title};

use trcli_application::governance::record::ChangeSet;
use trcli_application::kinds::{DeletionBlock, KindBehaviour, KindField, RecordKindDescriptor};
use trcli_application::outcome::Problem;
use trcli_application::ports::audit::AuditLog;
use trcli_application::ports::environment::IdGenerator;
use trcli_application::ports::records::{
    ListQuery, RecordIndex, RecordResolver, SortField, TagStore,
};
use trcli_application::ports::unit_of_work::StoreError;
use trcli_application::records::create::{register, update};
use trcli_application::records::list_options::{RecordRow, row};
use trcli_application::records::resolve::resolve;
use trcli_application::validation::{Checker, Valid};

/// The kind's name.
pub const KIND: &str = "specimen";

/// What the foundation needs to know about specimens.
pub fn descriptor() -> RecordKindDescriptor {
    RecordKindDescriptor::new(
        KIND,
        "spc",
        "Sample records with a title, used to try the tool out.",
        "title",
    )
}

/// The rows specimens keep for themselves.
pub trait SpecimenStore {
    /// Stores a new specimen's title.
    async fn insert_specimen(&mut self, id: RecordId, title: &Title) -> Result<(), StoreError>;

    /// Replaces a specimen's title.
    async fn retitle_specimen(&mut self, id: RecordId, title: &Title) -> Result<(), StoreError>;

    /// A specimen's title.
    async fn specimen_title(&self, id: RecordId) -> Result<Option<String>, StoreError>;

    /// Removes a specimen's row.
    async fn remove_specimen(&mut self, id: RecordId) -> Result<(), StoreError>;
}

/// What only specimens know about themselves.
#[derive(Clone, Debug)]
pub struct Specimens {
    /// The kind's descriptor.
    descriptor: RecordKindDescriptor,
}

impl Specimens {
    /// The behaviour of the kind.
    pub fn new() -> Self {
        Self {
            descriptor: descriptor(),
        }
    }
}

impl Default for Specimens {
    fn default() -> Self {
        Self::new()
    }
}

impl<U: SpecimenStore> KindBehaviour<U> for Specimens {
    fn descriptor(&self) -> &RecordKindDescriptor {
        &self.descriptor
    }

    async fn deletion_block(
        &self,
        _unit: &U,
        _id: RecordId,
    ) -> Result<Option<DeletionBlock>, StoreError> {
        // Nothing forbids deleting a specimen.
        Ok(None)
    }

    async fn dependents(&self, _unit: &U, _id: RecordId) -> Result<Vec<String>, StoreError> {
        Ok(Vec::new())
    }

    async fn remove_rows(&self, unit: &mut U, id: RecordId) -> Result<(), StoreError> {
        unit.remove_specimen(id).await
    }

    async fn fields(&self, unit: &U, id: RecordId) -> Result<Vec<KindField>, StoreError> {
        Ok(vec![KindField {
            name: "title",
            value: unit.specimen_title(id).await?,
        }])
    }
}

/// A checked request to add a specimen.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AddSpecimen {
    /// The specimen's title.
    pub title: Title,
}

impl AddSpecimen {
    /// Checks the title.
    pub fn new(title: Option<&str>) -> Result<Valid<Self>, Problem> {
        let mut checker = Checker::new();
        let title = checker.required("--title", title, Title::new);
        checker.finish(|| Self {
            title: title.expect("checked"),
        })
    }
}

/// Adds a specimen. A title another specimen already has is unusual but allowed: the
/// specimen is added as given, with a warning (FR-025).
pub async fn add<U>(
    unit: &mut U,
    stamp: &Stamp,
    ids: &impl IdGenerator,
    command: AddSpecimen,
) -> Result<Valid<RecordRow>, Problem>
where
    U: SpecimenStore + RecordIndex + TagStore + AuditLog,
{
    let warnings = duplicate_warning(unit, &command.title).await?;
    let id = ids.next_id();
    let changes = vec![Change::set("title", command.title.as_str())];
    // The index row comes first: the kind's own row refers to it.
    let record = register(
        unit,
        stamp,
        &descriptor(),
        id,
        (command.title.as_str(), changes),
    )
    .await?;
    unit.insert_specimen(id, &command.title).await?;
    Ok(Valid {
        command: row(unit, &record).await?,
        warnings,
    })
}

/// A warning when another specimen is found by the same title.
async fn duplicate_warning<U: RecordIndex>(
    unit: &U,
    title: &Title,
) -> Result<Vec<Warning>, Problem> {
    let key = SearchKey::from_text(title.as_str());
    let query = ListQuery {
        kind: KIND.to_owned(),
        tags: Vec::new(),
        search: Some(key.clone()),
        sort: SortField::Name,
        descending: false,
        limit: 1000,
    };
    let same = unit
        .list_records(&query)
        .await?
        .items
        .into_iter()
        .find(|record| record.search_key == key);
    Ok(same
        .map(|record| {
            let message = format!(
                "another specimen, {}, has the same title; added anyway",
                record.handle
            );
            Warning::new("duplicate_suspected", message).about("--title")
        })
        .into_iter()
        .collect())
}

/// A checked request to change a specimen.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EditSpecimen {
    /// What was typed to name the specimen.
    pub reference: String,
    /// The new title, when given.
    pub title: Option<Title>,
}

impl EditSpecimen {
    /// Checks the title, when one is given.
    pub fn new(reference: &str, title: Option<&str>) -> Result<Valid<Self>, Problem> {
        let mut checker = Checker::new();
        let title = checker.optional("--title", title, Title::new);
        checker.finish(|| Self {
            reference: reference.to_owned(),
            title: title.flatten(),
        })
    }
}

/// Changes only what was named; when nothing differs, nothing is recorded.
pub async fn edit<U>(
    unit: &mut U,
    stamp: &Stamp,
    command: EditSpecimen,
) -> Result<RecordRow, Problem>
where
    U: SpecimenStore + RecordResolver + RecordIndex + TagStore + AuditLog,
{
    let record = resolve(unit, &command.reference, &[KIND.to_owned()]).await?;
    let mut changes = ChangeSet::new();
    if let Some(title) = &command.title {
        changes.field("title", Some(&record.display_name), Some(title.as_str()));
    }
    if changes.is_empty() {
        return row(unit, &record).await;
    }
    let title = command.title.expect("a change implies a title");
    unit.retitle_specimen(record.id, &title).await?;
    let updated = update(
        unit,
        stamp,
        record,
        Some(title.as_str()),
        changes.into_changes(),
    )
    .await?;
    row(unit, &updated).await
}

#[cfg(test)]
impl SpecimenStore for trcli_testing::unit::FakeUnit {
    async fn insert_specimen(&mut self, id: RecordId, title: &Title) -> Result<(), StoreError> {
        self.put_kind_row(KIND, id, [("title".to_owned(), title.to_string())].into());
        Ok(())
    }

    async fn retitle_specimen(&mut self, id: RecordId, title: &Title) -> Result<(), StoreError> {
        self.insert_specimen(id, title).await
    }

    async fn specimen_title(&self, id: RecordId) -> Result<Option<String>, StoreError> {
        Ok(self
            .kind_row(KIND, id)
            .and_then(|row| row.get("title").cloned()))
    }

    async fn remove_specimen(&mut self, id: RecordId) -> Result<(), StoreError> {
        self.remove_kind_row(KIND, id);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    //! Unit tests for the sample kind's own use cases.

    use super::{AddSpecimen, EditSpecimen, SpecimenStore, Specimens, add, edit};
    use trcli_application::kinds::KindBehaviour;
    use trcli_application::ports::unit_of_work::Storage;
    use trcli_testing::block_on;
    use trcli_testing::environment::{SeededIds, stamp};
    use trcli_testing::unit::{FakeStorage, FakeUnit};

    /// Adds a specimen with this title to the unit.
    fn added(
        unit: &mut FakeUnit,
        ids: &SeededIds,
        title: &str,
    ) -> trcli_application::validation::Valid<trcli_application::records::list_options::RecordRow>
    {
        let command = AddSpecimen::new(Some(title)).expect("valid").command;
        block_on(add(unit, &stamp(), ids, command)).expect("added")
    }

    #[test]
    fn a_specimen_gets_a_short_name_its_row_and_a_create_entry() {
        let (mut unit, ids) = (
            block_on(FakeStorage::new().begin()).expect("begin"),
            SeededIds::new(),
        );
        let row = added(&mut unit, &ids, "First").command;
        assert!(row.handle.starts_with("spc-"));
        assert_eq!(
            (row.kind.as_str(), row.name.as_str()),
            ("specimen", "First")
        );
        let id = unit.state().records[0].id;
        assert_eq!(
            block_on(unit.specimen_title(id)).expect("read"),
            Some("First".to_owned())
        );
        assert_eq!(unit.state().audit[0].action.as_str(), "create");
    }

    #[test]
    fn a_title_is_required_and_checked() {
        assert!(AddSpecimen::new(None).is_err());
        assert!(AddSpecimen::new(Some("")).is_err());
        assert!(AddSpecimen::new(Some(&"t".repeat(501))).is_err());
    }

    #[test]
    fn a_repeated_title_is_allowed_with_a_warning_and_kept_as_given() {
        let (mut unit, ids) = (
            block_on(FakeStorage::new().begin()).expect("begin"),
            SeededIds::new(),
        );
        assert!(added(&mut unit, &ids, "Ação").warnings.is_empty());
        let second = added(&mut unit, &ids, "acao");
        assert_eq!(second.warnings[0].code, "duplicate_suspected");
        assert_eq!(second.command.name, "acao");
        assert_eq!(unit.state().records.len(), 2);
    }

    #[test]
    fn editing_changes_only_what_was_named_and_records_before_and_after() {
        let (mut unit, ids) = (
            block_on(FakeStorage::new().begin()).expect("begin"),
            SeededIds::new(),
        );
        let handle = added(&mut unit, &ids, "First").command.handle;
        let command = EditSpecimen::new(&handle, Some("Renamed"))
            .expect("valid")
            .command;
        let row = block_on(edit(&mut unit, &stamp(), command)).expect("edited");
        assert_eq!(
            (row.handle.as_str(), row.name.as_str()),
            (handle.as_str(), "Renamed")
        );
        let change = &unit.state().audit[1].changes[0];
        assert_eq!(
            (change.before.as_deref(), change.after.as_deref()),
            (Some("First"), Some("Renamed"))
        );

        let unchanged = EditSpecimen::new(&handle, None).expect("valid").command;
        block_on(edit(&mut unit, &stamp(), unchanged)).expect("nothing to do");
        assert_eq!(unit.state().audit.len(), 2);
    }

    #[test]
    fn the_kind_shows_its_own_field_and_removes_its_own_row() {
        let (mut unit, ids) = (
            block_on(FakeStorage::new().begin()).expect("begin"),
            SeededIds::new(),
        );
        added(&mut unit, &ids, "First");
        let id = unit.state().records[0].id;
        let kind = Specimens::new();
        let fields = block_on(KindBehaviour::<FakeUnit>::fields(&kind, &unit, id)).expect("fields");
        assert_eq!(
            (fields[0].name, fields[0].value.as_deref()),
            ("title", Some("First"))
        );
        block_on(kind.remove_rows(&mut unit, id)).expect("removed");
        assert_eq!(block_on(unit.specimen_title(id)).expect("read"), None);
    }
}

//! How a feature plugs a kind of record into the foundation (FR-068).
//!
//! A feature registers a [`RecordKindDescriptor`] for each kind it owns, and implements
//! [`KindBehaviour`] for what only it can know: what stands in the way of deleting one of
//! its records, what else refers to it, how its own rows are removed, and which fields it
//! shows. From that alone the foundation provides short names, resolving what was typed,
//! tags, notes, links, guarded deletion, listing options, both output forms for the common
//! fields, counts, and audit entries.
//!
//! The registry has no list of features: it knows only what was registered (open/closed).

use trcli_domain::shared::record::{RecordId, RecordKind};

use crate::ports::unit_of_work::StoreError;

/// What the foundation needs to know about a kind of record.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RecordKindDescriptor {
    /// The kind's name and handle prefix. The name is also the command noun.
    pub kind: RecordKind,
    /// One line saying what records of this kind are, shown in help.
    pub summary: &'static str,
    /// What the kind calls the text its records are named and found by (`title`, `name`);
    /// accepted by `--sort` and used as the column heading.
    pub name_field: &'static str,
}

impl RecordKindDescriptor {
    /// A descriptor. An invalid name or prefix is a programming error and panics at
    /// start-up, where a contributor sees it at once.
    pub fn new(name: &str, prefix: &str, summary: &'static str, name_field: &'static str) -> Self {
        let kind = RecordKind::new(name, prefix).unwrap_or_else(|rejection| {
            panic!("record kind `{name}` ({prefix}): {}", rejection.expected)
        });
        Self {
            kind,
            summary,
            name_field,
        }
    }

    /// The kind's name.
    pub fn name(&self) -> &str {
        self.kind.name()
    }
}

/// Why a kind cannot be registered.
#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]
pub enum KindError {
    /// Two kinds share a name.
    #[error("record kinds `{0}` and `{1}` have the same name")]
    SameName(String, String),
    /// Two kinds share a handle prefix.
    #[error("record kinds `{0}` and `{1}` have the same handle prefix `{2}`")]
    SamePrefix(String, String, String),
}

/// The kinds of record this build knows.
#[derive(Clone, Debug, Default)]
pub struct KindRegistry {
    /// The registered kinds, in registration order.
    kinds: Vec<RecordKindDescriptor>,
}

impl KindRegistry {
    /// An empty registry. The foundation itself owns no kind of record.
    pub fn new() -> Self {
        Self::default()
    }

    /// Registers a kind. Names and prefixes are unique among kinds, so that a short name
    /// always says which kind it belongs to (FR-010).
    pub fn register(&mut self, descriptor: RecordKindDescriptor) -> Result<(), KindError> {
        for existing in &self.kinds {
            if existing.name() == descriptor.name() {
                return Err(KindError::SameName(
                    existing.name().to_owned(),
                    descriptor.name().to_owned(),
                ));
            }
            if existing.kind.prefix() == descriptor.kind.prefix() {
                return Err(KindError::SamePrefix(
                    existing.name().to_owned(),
                    descriptor.name().to_owned(),
                    descriptor.kind.prefix().to_owned(),
                ));
            }
        }
        self.kinds.push(descriptor);
        Ok(())
    }

    /// The kind with this name.
    pub fn by_name(&self, name: &str) -> Option<&RecordKindDescriptor> {
        self.kinds
            .iter()
            .find(|descriptor| descriptor.name() == name)
    }

    /// The kind whose handles start with this prefix.
    pub fn by_prefix(&self, prefix: &str) -> Option<&RecordKindDescriptor> {
        self.kinds
            .iter()
            .find(|descriptor| descriptor.kind.prefix() == prefix)
    }

    /// Every registered kind.
    pub fn all(&self) -> &[RecordKindDescriptor] {
        &self.kinds
    }

    /// The names of every registered kind.
    pub fn names(&self) -> Vec<&str> {
        self.kinds.iter().map(RecordKindDescriptor::name).collect()
    }
}

/// What stands in the way of deleting a record, and what to do instead (FR-012).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DeletionBlock {
    /// Why the record may not be deleted.
    pub reason: String,
    /// The alternative the feature provides.
    pub alternative: String,
}

/// One of a record's own fields, as its kind shows it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct KindField {
    /// The field's name, in `snake_case`.
    pub name: &'static str,
    /// Its value; `None` when it has none.
    pub value: Option<String>,
}

/// What only a kind's own feature can say about its records. `U` is the unit of work the
/// feature's stores are reached through.
pub trait KindBehaviour<U> {
    /// The kind this behaviour belongs to.
    fn descriptor(&self) -> &RecordKindDescriptor;

    /// What, if anything, forbids deleting this record. Must not change anything.
    async fn deletion_block(
        &self,
        unit: &U,
        id: RecordId,
    ) -> Result<Option<DeletionBlock>, StoreError>;

    /// What else refers to this record and would be affected by deleting it, beyond the
    /// links, tags, and notes the foundation already knows. Must not change anything.
    async fn dependents(&self, unit: &U, id: RecordId) -> Result<Vec<String>, StoreError>;

    /// Removes the kind's own rows for this record.
    async fn remove_rows(&self, unit: &mut U, id: RecordId) -> Result<(), StoreError>;

    /// The kind's own fields of this record, in the order they are shown.
    async fn fields(&self, unit: &U, id: RecordId) -> Result<Vec<KindField>, StoreError>;
}

#[cfg(test)]
mod tests {
    //! Unit tests for the kind registry (T065).

    use super::{KindError, KindRegistry, RecordKindDescriptor};

    /// A descriptor with the given name and prefix.
    fn kind(name: &str, prefix: &str) -> RecordKindDescriptor {
        RecordKindDescriptor::new(name, prefix, "A kind used in tests.", "title")
    }

    #[test]
    fn a_registered_kind_is_found_by_name_and_by_prefix() {
        let mut registry = KindRegistry::new();
        registry
            .register(kind("reference", "ref"))
            .expect("registered");
        assert_eq!(
            registry
                .by_name("reference")
                .map(RecordKindDescriptor::name),
            Some("reference")
        );
        assert_eq!(
            registry.by_prefix("ref").map(RecordKindDescriptor::name),
            Some("reference")
        );
        assert!(registry.by_name("task").is_none());
        assert_eq!(registry.names(), ["reference"]);
    }

    #[test]
    fn two_kinds_with_the_same_name_are_refused_naming_both() {
        let mut registry = KindRegistry::new();
        registry
            .register(kind("reference", "ref"))
            .expect("registered");
        let error = registry
            .register(kind("reference", "rfc"))
            .expect_err("same name");
        assert_eq!(
            error,
            KindError::SameName("reference".into(), "reference".into())
        );
    }

    #[test]
    fn two_kinds_with_the_same_prefix_are_refused_naming_both() {
        let mut registry = KindRegistry::new();
        registry
            .register(kind("reference", "ref"))
            .expect("registered");
        let error = registry
            .register(kind("referee", "ref"))
            .expect_err("same prefix");
        assert_eq!(
            error.to_string(),
            "record kinds `reference` and `referee` have the same handle prefix `ref`"
        );
    }

    #[test]
    #[should_panic(expected = "record kind")]
    fn a_kind_with_an_invalid_name_or_prefix_stops_at_start_up() {
        let _ = kind("Reference", "r");
    }
}

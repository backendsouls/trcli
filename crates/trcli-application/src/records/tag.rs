//! Tagging and untagging a record of any kind (FR-015).

use serde::Serialize;
use trcli_domain::governance::audit::{AuditAction, AuditDraft, Change, Stamp};
use trcli_domain::shared::problem::Rejection;
use trcli_domain::shared::text::TagName;

use super::resolve::resolve;
use crate::kinds::KindRegistry;
use crate::outcome::Problem;
use crate::ports::audit::AuditLog;
use crate::ports::records::{IndexedRecord, RecordIndex, RecordResolver, TagStore};
use crate::validation::{Checker, Valid};

/// A checked request to add tags to a record, or remove them.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TagCommand {
    /// What was typed to name the record.
    pub reference: String,
    /// The tags, normalized.
    pub tags: Vec<TagName>,
    /// Remove the tags instead of adding them.
    pub remove: bool,
}

impl TagCommand {
    /// Checks every tag name; all invalid ones are reported together.
    pub fn new(reference: &str, tags: &[String], remove: bool) -> Result<Valid<Self>, Problem> {
        let mut checker = Checker::new();
        if tags.is_empty() {
            checker.reject(
                "<tag>",
                "",
                Rejection::new("is required", "at least one tag"),
            );
        }
        let checked: Vec<Option<TagName>> = tags
            .iter()
            .map(|tag| checker.check("<tag>", tag, TagName::new))
            .collect();
        checker.finish(|| Self {
            reference: reference.to_owned(),
            tags: checked.into_iter().flatten().collect(),
            remove,
        })
    }
}

/// What tagging reports.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct Tagged {
    /// The record's short name.
    pub handle: String,
    /// What the record is called.
    pub name: String,
    /// The tags this command added.
    pub added: Vec<String>,
    /// The tags this command removed.
    pub removed: Vec<String>,
    /// The tags the record now carries.
    pub tags: Vec<String>,
}

/// Adds or removes the tags, recording one audit entry for each tag that changed. A tag
/// the record already carries (or, when removing, does not carry) changes nothing.
pub async fn tag<U>(
    unit: &mut U,
    stamp: &Stamp,
    kinds: &[String],
    command: TagCommand,
) -> Result<Tagged, Problem>
where
    U: RecordResolver + RecordIndex + TagStore + AuditLog,
{
    let record = resolve(unit, &command.reference, kinds).await?;
    let mut changed = Vec::new();
    for tag in &command.tags {
        if apply(unit, stamp, &record, tag, command.remove).await? {
            changed.push(tag.to_string());
        }
    }
    if !changed.is_empty() {
        unit.touch_record(record.id, stamp.at).await?;
    }
    let tags = unit
        .tags_of(record.id)
        .await?
        .iter()
        .map(ToString::to_string)
        .collect();
    let (added, removed) = if command.remove {
        (Vec::new(), changed)
    } else {
        (changed, Vec::new())
    };
    Ok(Tagged {
        handle: record.handle.to_string(),
        name: record.display_name,
        added,
        removed,
        tags,
    })
}

/// Adds or removes one tag; says whether anything changed, and records it when so.
async fn apply<U>(
    unit: &mut U,
    stamp: &Stamp,
    record: &IndexedRecord,
    tag: &TagName,
    remove: bool,
) -> Result<bool, Problem>
where
    U: TagStore + AuditLog,
{
    let changed = if remove {
        unit.detach_tag(record.id, tag).await?
    } else {
        unit.attach_tag(record.id, tag).await?
    };
    if changed {
        unit.record(stamp, entry(record, tag, remove)).await?;
    }
    Ok(changed)
}

/// The audit entry for one tag added to, or removed from, a record.
fn entry(record: &IndexedRecord, tag: &TagName, removed: bool) -> AuditDraft {
    let (action, change) = if removed {
        (
            AuditAction::UNTAG,
            Change::new("tag", Some(tag.as_str()), None),
        )
    } else {
        (
            AuditAction::TAG,
            Change::new("tag", None, Some(tag.as_str())),
        )
    };
    AuditDraft::record(
        action,
        &record.kind,
        record.id,
        record.handle.as_str(),
        &record.display_name,
    )
    .with_changes(vec![change])
}

/// One tag and how many records carry it.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct TagCount {
    /// The tag.
    pub tag: String,
    /// How many records carry it.
    pub records: u64,
}

/// Every tag in use.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct TagList {
    /// The tags, in alphabetical order.
    pub items: Vec<TagCount>,
    /// How many tags there are.
    pub total: u64,
}

/// Lists every tag with the number of records carrying it, of one kind when given. An
/// unknown kind is invalid input, with the kinds that exist.
pub async fn list_tags<U: TagStore>(
    unit: &U,
    kinds: &KindRegistry,
    kind: Option<&str>,
) -> Result<TagList, Problem> {
    if let Some(kind) = kind
        && kinds.by_name(kind).is_none()
    {
        let mut checker = Checker::new();
        let rejection = Rejection::new(
            "is not a kind of record",
            "a kind of record known to this version",
        );
        checker.reject("--kind", kind, rejection.with_choices(kinds.names()));
        return Err(checker.finish(|| ()).expect_err("a problem was recorded"));
    }
    let counts = unit.tag_counts(kind).await?;
    let items: Vec<TagCount> = counts
        .into_iter()
        .map(|(tag, records)| TagCount {
            tag: tag.to_string(),
            records,
        })
        .collect();
    Ok(TagList {
        total: items.len() as u64,
        items,
    })
}

#[cfg(test)]
mod tests {
    //! Unit tests for tagging (T066).

    use super::{TagCommand, list_tags, tag};
    use crate::kinds::{KindRegistry, RecordKindDescriptor};
    use crate::outcome::{Details, codes};
    use crate::ports::records::RecordIndex;
    use crate::ports::unit_of_work::Storage;
    use crate::testing::block_on;
    use crate::testing::contract_records::indexed;
    use crate::testing::environment::stamp;
    use crate::testing::unit::{FakeStorage, FakeUnit};

    /// A unit holding one `alpha` record; returns its handle too.
    fn unit() -> (FakeUnit, String) {
        block_on(async {
            let mut unit = FakeStorage::new().begin().await.expect("begin");
            let record = indexed(0, "alpha", "alp", "First");
            unit.insert_record(&record).await.expect("insert");
            (unit, record.handle.to_string())
        })
    }

    /// The checked command tagging `handle` with `tags`.
    fn command(handle: &str, tags: &[&str], remove: bool) -> TagCommand {
        let tags: Vec<String> = tags.iter().map(|tag| (*tag).to_owned()).collect();
        TagCommand::new(handle, &tags, remove)
            .expect("valid")
            .command
    }

    #[test]
    fn tagging_twice_leaves_one_tagging_and_one_audit_entry() {
        let (mut unit, handle) = unit();
        block_on(async {
            let first = tag(
                &mut unit,
                &stamp(),
                &[],
                command(&handle, &["Field-Work"], false),
            )
            .await
            .expect("tagged");
            assert_eq!(first.added, ["field-work"]);
            let again = tag(
                &mut unit,
                &stamp(),
                &[],
                command(&handle, &["field-work"], false),
            )
            .await
            .expect("tagged");
            assert!(again.added.is_empty());
            assert_eq!(again.tags, ["field-work"]);
        });
        assert_eq!(unit.state().taggings.len(), 1);
        assert_eq!(unit.state().audit.len(), 1);
        assert_eq!(unit.state().audit[0].action.as_str(), "tag");
    }

    #[test]
    fn each_tag_changed_records_one_entry_and_removal_is_recorded_as_untag() {
        let (mut unit, handle) = unit();
        block_on(async {
            tag(
                &mut unit,
                &stamp(),
                &[],
                command(&handle, &["a", "b"], false),
            )
            .await
            .expect("tagged");
            let removed = tag(
                &mut unit,
                &stamp(),
                &[],
                command(&handle, &["a", "zzz"], true),
            )
            .await
            .expect("untagged");
            assert_eq!(
                (removed.removed, removed.tags),
                (vec!["a".to_owned()], vec!["b".to_owned()])
            );
        });
        let actions: Vec<&str> = unit
            .state()
            .audit
            .iter()
            .map(|entry| entry.action.as_str())
            .collect();
        assert_eq!(actions, ["tag", "tag", "untag"]);
    }

    #[test]
    fn every_invalid_tag_name_is_reported_with_the_rule() {
        let tags = vec!["two words".to_owned(), "ok".to_owned(), "ação".to_owned()];
        let problem = TagCommand::new("alp-1", &tags, false).expect_err("invalid");
        let Details::Fields(fields) = problem.details else {
            panic!("fields expected")
        };
        assert_eq!(fields.len(), 2);
        assert!(fields[0].expected.contains("a-z"));
        assert!(TagCommand::new("alp-1", &[], false).is_err());
    }

    #[test]
    fn tags_are_listed_with_counts_and_an_unknown_kind_is_invalid() {
        let (mut unit, handle) = unit();
        let mut kinds = KindRegistry::new();
        kinds
            .register(RecordKindDescriptor::new("alpha", "alp", "Alpha.", "title"))
            .expect("registered");
        block_on(async {
            tag(
                &mut unit,
                &stamp(),
                &[],
                command(&handle, &["b", "a"], false),
            )
            .await
            .expect("tagged");
            let listed = list_tags(&unit, &kinds, Some("alpha"))
                .await
                .expect("listed");
            let names: Vec<&str> = listed.items.iter().map(|item| item.tag.as_str()).collect();
            assert_eq!((names, listed.total), (vec!["a", "b"], 2));
            let problem = list_tags(&unit, &kinds, Some("gamma"))
                .await
                .expect_err("unknown kind");
            assert_eq!(problem.code, codes::VALIDATION_FAILED);
        });
    }
}

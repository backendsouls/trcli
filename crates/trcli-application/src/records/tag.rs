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

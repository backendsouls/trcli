//! Linking any two records, removing a link, and listing a record's links (FR-016).
//!
//! A link is visible from both ends and exists at most once for a pair and a relation,
//! whichever end it was made from.

use serde::Serialize;
use trcli_domain::governance::audit::{AuditAction, AuditDraft, Change, Stamp};
use trcli_domain::shared::link::Link;
use trcli_domain::shared::problem::Rejection;
use trcli_domain::shared::text::Relation;

use super::resolve::resolve;
use crate::outcome::{Problem, codes};
use crate::ports::audit::AuditLog;
use crate::ports::records::{IndexedRecord, LinkStore, RecordIndex, RecordResolver};
use crate::validation::{Checker, Valid};

/// A checked request to link two records, or to remove their link.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LinkCommand {
    /// What was typed to name the first record.
    pub one: String,
    /// What was typed to name the second record.
    pub other: String,
    /// How the two relate; `related` unless said otherwise.
    pub relation: Relation,
}

impl LinkCommand {
    /// Checks the relation.
    pub fn new(one: &str, other: &str, relation: Option<&str>) -> Result<Valid<Self>, Problem> {
        let mut checker = Checker::new();
        let relation = checker.optional("--relation", relation, Relation::new);
        checker.finish(|| Self {
            one: one.to_owned(),
            other: other.to_owned(),
            relation: relation.flatten().unwrap_or_else(Relation::related),
        })
    }
}

/// One end of a link, in a view.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct LinkEnd {
    /// The record's short name.
    pub handle: String,
    /// What the record is called.
    pub name: String,
    /// The record's kind.
    pub kind: String,
}

impl From<&IndexedRecord> for LinkEnd {
    fn from(record: &IndexedRecord) -> Self {
        Self {
            handle: record.handle.to_string(),
            name: record.display_name.clone(),
            kind: record.kind.clone(),
        }
    }
}

/// A link between two records, as reported by `link add` and `link rm`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct Linked {
    /// One end.
    pub from: LinkEnd,
    /// The other end.
    pub to: LinkEnd,
    /// How they relate.
    pub relation: String,
    /// `true` when the link was removed by this command.
    pub removed: bool,
}

/// Resolves both ends and builds the link; linking a record to itself is invalid input.
async fn ends<U: RecordResolver>(
    unit: &U,
    stamp: &Stamp,
    command: &LinkCommand,
) -> Result<(IndexedRecord, IndexedRecord, Link), Problem> {
    let one = resolve(unit, &command.one, &[]).await?;
    let other = resolve(unit, &command.other, &[]).await?;
    match Link::new(one.id, other.id, command.relation.clone(), stamp.at) {
        Ok(link) => Ok((one, other, link)),
        Err(error) => {
            let mut checker = Checker::new();
            checker.reject(
                "<ref>",
                &command.other,
                Rejection::new(error.to_string(), "two different records"),
            );
            Err(checker.finish(|| ()).expect_err("a problem was recorded"))
        }
    }
}

/// The audit entry for a link made or removed, recorded about the first record.
fn entry(
    action: AuditAction,
    one: &IndexedRecord,
    other: &IndexedRecord,
    relation: &Relation,
) -> AuditDraft {
    let described = format!("{} \"{}\" ({relation})", other.handle, other.display_name);
    let change = if action == AuditAction::LINK {
        Change::new("link", None, Some(&described))
    } else {
        Change::new("link", Some(&described), None)
    };
    AuditDraft::record(
        action,
        &one.kind,
        one.id,
        one.handle.as_str(),
        &one.display_name,
    )
    .with_changes(vec![change])
}

/// Links two records. The same link made twice, from either end, is invalid input.
pub async fn add_link<U>(
    unit: &mut U,
    stamp: &Stamp,
    command: LinkCommand,
) -> Result<Linked, Problem>
where
    U: RecordResolver + RecordIndex + LinkStore + AuditLog,
{
    let (one, other, link) = ends(unit, stamp, &command).await?;
    if unit
        .links_of(one.id)
        .await?
        .iter()
        .any(|existing| existing.is_same_as(&link))
    {
        let mut checker = Checker::new();
        let rejection = Rejection::new(
            format!(
                "is already linked to {} as \"{}\"",
                one.handle, command.relation
            ),
            "a link that does not exist yet",
        );
        checker.reject("<ref>", &command.other, rejection);
        return Err(checker.finish(|| ()).expect_err("a problem was recorded"));
    }
    unit.add_link(&link).await?;
    unit.record(
        stamp,
        entry(AuditAction::LINK, &one, &other, &command.relation),
    )
    .await?;
    Ok(Linked {
        from: (&one).into(),
        to: (&other).into(),
        relation: command.relation.to_string(),
        removed: false,
    })
}

/// Removes the link between two records with the given relation.
pub async fn remove_link<U>(
    unit: &mut U,
    stamp: &Stamp,
    command: LinkCommand,
) -> Result<Linked, Problem>
where
    U: RecordResolver + RecordIndex + LinkStore + AuditLog,
{
    let (one, other, link) = ends(unit, stamp, &command).await?;
    if !unit.remove_link(&link).await? {
        let message = format!(
            "{} and {} are not linked as \"{}\"",
            one.handle, other.handle, command.relation
        );
        return Err(
            Problem::new(codes::NOT_FOUND, message).with_next_step(format!(
                "see the links of a record with `trcli link list {}`",
                one.handle
            )),
        );
    }
    unit.record(
        stamp,
        entry(AuditAction::UNLINK, &one, &other, &command.relation),
    )
    .await?;
    Ok(Linked {
        from: (&one).into(),
        to: (&other).into(),
        relation: command.relation.to_string(),
        removed: true,
    })
}

/// One link of a record, seen from that record.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct LinkView {
    /// How the two relate.
    pub relation: String,
    /// The record at the other end.
    #[serde(flatten)]
    pub other: LinkEnd,
}

/// The links of one record.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct LinkList {
    /// The record whose links these are.
    pub of: LinkEnd,
    /// Its links, grouped by relation.
    pub items: Vec<LinkView>,
    /// How many links it has.
    pub total: u64,
}

/// The links of a record, from either end, ordered by relation.
pub async fn links_of<U>(unit: &U, record: &IndexedRecord) -> Result<Vec<LinkView>, Problem>
where
    U: RecordIndex + LinkStore,
{
    let mut views = Vec::new();
    for link in unit.links_of(record.id).await? {
        let Some(other_id) = link.other_end(record.id) else {
            continue;
        };
        // The other end always exists: storage refuses links to missing records (FR-017).
        if let Some(other) = unit.record(other_id).await? {
            views.push(LinkView {
                relation: link.relation.to_string(),
                other: (&other).into(),
            });
        }
    }
    views.sort_by(|one, other| {
        one.relation
            .cmp(&other.relation)
            .then(one.other.handle.cmp(&other.other.handle))
    });
    Ok(views)
}

/// Lists the links of the record that was typed.
pub async fn list_links<U>(unit: &U, reference: &str) -> Result<LinkList, Problem>
where
    U: RecordResolver + RecordIndex + LinkStore,
{
    let record = resolve(unit, reference, &[]).await?;
    let items = links_of(unit, &record).await?;
    Ok(LinkList {
        of: (&record).into(),
        total: items.len() as u64,
        items,
    })
}

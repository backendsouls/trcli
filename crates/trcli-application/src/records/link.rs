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

#[cfg(test)]
mod tests {
    //! Unit tests for links (T066).

    use super::{LinkCommand, add_link, list_links, remove_link};
    use crate::outcome::codes;
    use crate::ports::records::RecordIndex;
    use crate::ports::unit_of_work::Storage;
    use crate::testing::block_on;
    use crate::testing::contract_records::indexed;
    use crate::testing::environment::stamp;
    use crate::testing::unit::{FakeStorage, FakeUnit};

    /// A unit holding an `alpha` and a `beta` record; returns their handles too.
    fn unit() -> (FakeUnit, String, String) {
        block_on(async {
            let mut unit = FakeStorage::new().begin().await.expect("begin");
            let (first, second) = (
                indexed(0, "alpha", "alp", "First"),
                indexed(1, "beta", "bet", "Second"),
            );
            unit.insert_record(&first).await.expect("insert");
            unit.insert_record(&second).await.expect("insert");
            (unit, first.handle.to_string(), second.handle.to_string())
        })
    }

    /// The checked command linking two handles.
    fn command(one: &str, other: &str, relation: Option<&str>) -> LinkCommand {
        LinkCommand::new(one, other, relation)
            .expect("valid")
            .command
    }

    #[test]
    fn a_link_between_two_kinds_is_visible_from_both_ends() {
        let (mut unit, first, second) = unit();
        block_on(async {
            let linked = add_link(
                &mut unit,
                &stamp(),
                command(&first, &second, Some("same site")),
            )
            .await
            .expect("linked");
            assert_eq!(
                (linked.relation.as_str(), linked.to.kind.as_str()),
                ("same site", "beta")
            );
            let from_first = list_links(&unit, &first).await.expect("listed");
            let from_second = list_links(&unit, &second).await.expect("listed");
            assert_eq!(from_first.items[0].other.handle, second);
            assert_eq!(from_second.items[0].other.handle, first);
        });
        assert_eq!(unit.state().audit[0].action.as_str(), "link");
    }

    #[test]
    fn a_record_cannot_be_linked_to_itself() {
        let (mut unit, first, _) = unit();
        let problem = block_on(add_link(&mut unit, &stamp(), command(&first, &first, None)))
            .expect_err("same");
        assert_eq!(problem.code, codes::VALIDATION_FAILED);
        assert!(unit.state().links.is_empty() && unit.state().audit.is_empty());
    }

    #[test]
    fn the_same_link_cannot_be_made_twice_from_either_end() {
        let (mut unit, first, second) = unit();
        block_on(async {
            add_link(&mut unit, &stamp(), command(&first, &second, None))
                .await
                .expect("linked");
            let again = add_link(&mut unit, &stamp(), command(&second, &first, None))
                .await
                .expect_err("duplicate");
            assert_eq!(again.code, codes::VALIDATION_FAILED);
            add_link(&mut unit, &stamp(), command(&first, &second, Some("cites")))
                .await
                .expect("another relation");
        });
        assert_eq!(unit.state().links.len(), 2);
    }

    #[test]
    fn removing_a_link_records_it_and_a_missing_link_is_not_found() {
        let (mut unit, first, second) = unit();
        block_on(async {
            add_link(&mut unit, &stamp(), command(&first, &second, None))
                .await
                .expect("linked");
            let removed = remove_link(&mut unit, &stamp(), command(&second, &first, None))
                .await
                .expect("removed");
            assert!(removed.removed);
            let missing = remove_link(&mut unit, &stamp(), command(&first, &second, None))
                .await
                .expect_err("gone");
            assert_eq!(missing.code, codes::NOT_FOUND);
        });
        let actions: Vec<&str> = unit
            .state()
            .audit
            .iter()
            .map(|entry| entry.action.as_str())
            .collect();
        assert_eq!(actions, ["link", "unlink"]);
    }

    #[test]
    fn a_relation_is_at_most_fifty_characters() {
        assert!(LinkCommand::new("a", "b", Some(&"r".repeat(51))).is_err());
        assert_eq!(command("a", "b", None).relation.as_str(), "related");
    }
}

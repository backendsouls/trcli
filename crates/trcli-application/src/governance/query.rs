//! Looking through the audit trail (FR-049): by record, kind of record, actor, kind of
//! action, and range of dates; newest first.

use serde::Serialize;
use trcli_domain::governance::audit::{AuditAction, AuditEntry, Change, to_hex};
use trcli_domain::shared::problem::Rejection;
use trcli_domain::shared::record::RecordId;

use crate::kinds::KindRegistry;
use crate::outcome::Problem;
use crate::ports::audit::{AuditFilter, AuditQuery};
use crate::validation::{Checker, Valid, date, integer_in_range, not_before};
use crate::view::Instant;

/// The filters of `audit list` and `audit export`, unchecked.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AuditInput {
    /// `--kind`.
    pub kind: Option<String>,
    /// `--actor`.
    pub actor: Option<String>,
    /// `--action`.
    pub action: Option<String>,
    /// `--from`.
    pub from: Option<String>,
    /// The last day: `--to`, or `--until` where `--to` means something else.
    pub to: Option<String>,
    /// The name of the option the last day was given with, for messages.
    pub to_name: String,
    /// `--limit`.
    pub limit: Option<String>,
}

impl Default for AuditInput {
    fn default() -> Self {
        Self { kind: None, actor: None, action: None, from: None, to: None, to_name: "--to".to_owned(), limit: None }
    }
}

/// Checks the filters with the caller's checker, so that their problems are reported
/// together with those of the command's other values. `record` is the record named with
/// `--record`, already resolved by the caller (resolving needs storage); `default_limit`
/// applies without `--limit`. `None` means a problem was recorded.
pub fn check_filters(
    checker: &mut Checker,
    input: &AuditInput,
    record: Option<RecordId>,
    kinds: &KindRegistry,
    default_limit: u32,
) -> Option<AuditFilter> {
    let kind = checker.optional("--kind", input.kind.as_deref(), |kind| known_kind(kind, kinds));
    let actor = checker.optional("--actor", input.actor.as_deref(), non_empty);
    let action = checker.optional("--action", input.action.as_deref(), known_action);
    let from = checker.optional("--from", input.from.as_deref(), date);
    let to = checker.optional(&input.to_name, input.to.as_deref(), date);
    let limit = checker.optional("--limit", input.limit.as_deref(), |limit| integer_in_range(limit, 1, 1000));
    // A rule between two values, each valid alone (FR-027).
    if let (Some(Some(from)), Some(Some(to)), Some(raw)) = (from, to, input.to.as_deref())
        && let Err(rejection) = not_before(from, to)
    {
        checker.reject(&input.to_name, raw, rejection);
        return None;
    }
    Some(AuditFilter {
        record,
        kind: kind?,
        actor: actor?,
        action: action?,
        from: from?,
        to: to?,
        limit: limit?.map_or(default_limit, |limit| limit as u32),
    })
}

/// Checks the filters on their own.
pub fn filter(
    input: &AuditInput,
    record: Option<RecordId>,
    kinds: &KindRegistry,
    default_limit: u32,
) -> Result<Valid<AuditFilter>, Problem> {
    let mut checker = Checker::new();
    let filter = check_filters(&mut checker, input, record, kinds, default_limit);
    checker.finish(|| filter.expect("checked"))
}

/// Accepts the name of a registered kind, listing the kinds when it is not one.
fn known_kind(kind: &str, kinds: &KindRegistry) -> Result<String, Rejection> {
    if kinds.by_name(kind).is_some() {
        return Ok(kind.to_owned());
    }
    Err(Rejection::new(
        "is not a kind of record",
        "a kind of record known to this version",
    )
    .with_choices(kinds.names()))
}

/// Accepts text that is not empty.
fn non_empty(text: &str) -> Result<String, Rejection> {
    let trimmed = text.trim();
    if trimmed.is_empty() {
        return Err(Rejection::new("must not be empty", "a name"));
    }
    Ok(trimmed.to_owned())
}

/// Accepts the name of an action, listing the foundation's when it is not one.
fn known_action(action: &str) -> Result<AuditAction, Rejection> {
    AuditAction::named(action)
        .filter(|_| AuditAction::FOUNDATION.contains(&action))
        .ok_or_else(|| {
            Rejection::new(
                "is not a kind of action",
                "one of the actions the trail records",
            )
            .with_choices(AuditAction::FOUNDATION)
        })
}

/// One changed field, in a view.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct ChangeView {
    /// The field's name.
    pub field: String,
    /// Its value before.
    pub before: Option<String>,
    /// Its value after.
    pub after: Option<String>,
}

impl From<&Change> for ChangeView {
    fn from(change: &Change) -> Self {
        Self {
            field: change.field.clone(),
            before: change.before.clone(),
            after: change.after.clone(),
        }
    }
}

/// One entry of the trail, in a view.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct EntryView {
    /// Position in the trail.
    pub sequence: u64,
    /// When it happened.
    pub at: Instant,
    /// Who acted.
    pub actor: String,
    /// What was done.
    pub action: String,
    /// The kind of the record concerned.
    pub kind: Option<String>,
    /// The record concerned.
    pub record_id: Option<String>,
    /// The record's short name at the time.
    pub handle: Option<String>,
    /// What the record was called at the time, even if it has since been deleted.
    pub display_name: Option<String>,
    /// The fields that changed.
    pub changes: Vec<ChangeView>,
    /// The entry's hash, in hexadecimal.
    pub hash: String,
}

impl From<&AuditEntry> for EntryView {
    fn from(entry: &AuditEntry) -> Self {
        Self {
            sequence: entry.sequence,
            at: entry.at.into(),
            actor: entry.actor.to_string(),
            action: entry.action.to_string(),
            kind: entry.kind.clone(),
            record_id: entry.record_id.map(|id| id.to_string()),
            handle: entry.handle.clone(),
            display_name: entry.display_name.clone(),
            changes: entry.changes.iter().map(ChangeView::from).collect(),
            hash: to_hex(&entry.hash),
        }
    }
}

impl EntryView {
    /// What the entry is about, in a few words: the record's short name and name, then
    /// each change as `field: before → after`.
    pub fn what(&self, arrow: &str) -> String {
        let mut parts = Vec::new();
        if let Some(handle) = &self.handle {
            parts.push(handle.clone());
        }
        if let Some(name) = &self.display_name {
            parts.push(format!("\"{name}\""));
        }
        for change in &self.changes {
            let before = change.before.as_deref().unwrap_or("(none)");
            let after = change.after.as_deref().unwrap_or("(none)");
            parts.push(format!("{}: {before} {arrow} {after}", change.field));
        }
        parts.join(" ")
    }
}

/// A page of the trail.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct AuditList {
    /// The entries shown, newest first.
    pub items: Vec<EntryView>,
    /// How many entries match, shown or not (FR-037).
    pub total: u64,
}

/// Reads the entries that pass the filter, newest first.
pub async fn list<U: AuditQuery>(unit: &U, filter: &AuditFilter) -> Result<AuditList, Problem> {
    let page = unit.entries(filter).await?;
    Ok(AuditList {
        items: page.entries.iter().map(EntryView::from).collect(),
        total: page.total,
    })
}

#[cfg(test)]
mod tests {
    //! Unit tests for the trail's filters and views.

    use trcli_domain::governance::audit::{AuditAction, AuditDraft, Change};

    use super::{AuditInput, filter, list};
    use crate::kinds::{KindRegistry, RecordKindDescriptor};
    use crate::outcome::Details;
    use crate::ports::audit::AuditLog;
    use crate::ports::unit_of_work::Storage;
    use crate::testing::block_on;
    use crate::testing::environment::{SeededIds, stamp};
    use crate::testing::unit::FakeStorage;

    /// A registry with the kind `alpha`.
    fn kinds() -> KindRegistry {
        let mut registry = KindRegistry::new();
        registry
            .register(RecordKindDescriptor::new("alpha", "alp", "Alpha.", "title"))
            .expect("registered");
        registry
    }

    /// The names of the fields a set of filters is rejected for.
    fn rejected(input: AuditInput) -> Vec<String> {
        let problem = filter(&input, None, &kinds(), 50).expect_err("invalid");
        let Details::Fields(fields) = problem.details else {
            panic!("fields expected")
        };
        fields.into_iter().map(|field| field.field).collect()
    }

    #[test]
    fn valid_filters_are_accepted_with_the_default_limit() {
        let input = AuditInput {
            kind: Some("alpha".into()),
            action: Some("create".into()),
            ..AuditInput::default()
        };
        let checked = filter(&input, Some(SeededIds::at(1)), &kinds(), 50)
            .expect("valid")
            .command;
        assert_eq!(
            (checked.limit, checked.kind.as_deref()),
            (50, Some("alpha"))
        );
        assert_eq!(checked.record, Some(SeededIds::at(1)));
    }

    #[test]
    fn every_invalid_filter_is_reported_at_once() {
        let input = AuditInput {
            kind: Some("gamma".into()),
            action: Some("explode".into()),
            from: Some("yesterday".into()),
            limit: Some("0".into()),
            ..AuditInput::default()
        };
        assert_eq!(rejected(input), ["--kind", "--action", "--from", "--limit"]);
    }

    #[test]
    fn an_end_before_its_start_is_reported_with_the_rule() {
        let input = AuditInput {
            from: Some("2026-10-08".into()),
            to: Some("2026-10-01".into()),
            ..AuditInput::default()
        };
        assert_eq!(rejected(input), ["--to"]);
    }

    #[test]
    fn an_entry_says_what_it_is_about() {
        let storage = FakeStorage::new();
        let listed = block_on(async {
            let mut unit = storage.begin().await.expect("begin");
            let draft = AuditDraft::record(
                AuditAction::UPDATE,
                "alpha",
                SeededIds::at(0),
                "alp-7k3f",
                "A title",
            )
            .with_changes(vec![Change::new(
                "status",
                Some("to_read"),
                Some("reading"),
            )]);
            unit.record(&stamp(), draft).await.expect("record");
            list(
                &unit,
                &filter(&AuditInput::default(), None, &kinds(), 50)
                    .expect("valid")
                    .command,
            )
            .await
        })
        .expect("listed");
        assert_eq!(listed.total, 1);
        assert_eq!(
            listed.items[0].what("->"),
            "alp-7k3f \"A title\" status: to_read -> reading"
        );
        assert_eq!(listed.items[0].hash.len(), 64);
    }
}

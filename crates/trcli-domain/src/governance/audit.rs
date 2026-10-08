//! The audit trail: what an entry is and how entries are chained (FR-046 to FR-051).
//!
//! Every change in a workspace is described by an [`AuditEntry`]. Entries are numbered
//! from 1 with no gap, and each carries the hash of the one before it, so that an entry
//! altered or removed outside the tool can be detected (FR-048).
//!
//! The rule is: `hash = digest(previous_hash ‖ canonical form)`, where the canonical form
//! is every other field in a fixed order, each with a length prefix so that no two
//! different entries share a form.
//!
//! This module does not compute the digest itself: the hash function is supplied by an
//! adapter, which keeps cryptography out of the domain's dependencies. It also has no
//! operation that changes or removes an entry — there is none anywhere (FR-048).

use std::borrow::Cow;

use time::OffsetDateTime;

use crate::shared::record::RecordId;
use crate::shared::text::ActorName;

/// The output of the hash function that chains the trail: 32 bytes.
pub type Digest = [u8; 32];

/// The `previous_hash` of the first entry: all zeros.
pub const GENESIS_HASH: Digest = [0; 32];

/// What kind of thing was done.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct AuditAction(Cow<'static, str>);

impl AuditAction {
    /// A record was created.
    pub const CREATE: AuditAction = AuditAction(Cow::Borrowed("create"));
    /// A record's fields were changed.
    pub const UPDATE: AuditAction = AuditAction(Cow::Borrowed("update"));
    /// A record was deleted.
    pub const DELETE: AuditAction = AuditAction(Cow::Borrowed("delete"));
    /// A record moved from one status to another.
    pub const STATUS: AuditAction = AuditAction(Cow::Borrowed("status"));
    /// Two records were linked.
    pub const LINK: AuditAction = AuditAction(Cow::Borrowed("link"));
    /// A link was removed.
    pub const UNLINK: AuditAction = AuditAction(Cow::Borrowed("unlink"));
    /// A tag was added to a record.
    pub const TAG: AuditAction = AuditAction(Cow::Borrowed("tag"));
    /// A tag was removed from a record.
    pub const UNTAG: AuditAction = AuditAction(Cow::Borrowed("untag"));
    /// A note was added to a record.
    pub const NOTE: AuditAction = AuditAction(Cow::Borrowed("note"));
    /// Records were brought in from elsewhere.
    pub const IMPORT: AuditAction = AuditAction(Cow::Borrowed("import"));
    /// Something was written out for someone else.
    pub const EXPORT: AuditAction = AuditAction(Cow::Borrowed("export"));
    /// Something was run.
    pub const RUN: AuditAction = AuditAction(Cow::Borrowed("run"));
    /// The researcher confirmed something.
    pub const CONFIRM: AuditAction = AuditAction(Cow::Borrowed("confirm"));
    /// A workspace setting was changed.
    pub const SETTING: AuditAction = AuditAction(Cow::Borrowed("setting"));
    /// The workspace was brought to a newer format.
    pub const UPGRADE: AuditAction = AuditAction(Cow::Borrowed("upgrade"));

    /// The names of the actions the foundation defines. Features may add more.
    pub const FOUNDATION: [&'static str; 15] = [
        "create", "update", "delete", "status", "link", "unlink", "tag", "untag", "note", "import",
        "export", "run", "confirm", "setting", "upgrade",
    ];

    /// An action read from storage or registered by a feature: a lower-case word.
    pub fn named(name: &str) -> Option<Self> {
        let is_word = !name.is_empty()
            && name.len() <= 40
            && name
                .chars()
                .all(|character| character.is_ascii_lowercase() || character == '_');
        is_word.then(|| Self(Cow::Owned(name.to_owned())))
    }

    /// The action's name.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for AuditAction {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.0)
    }
}

/// One field that changed, with its value before and after.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Change {
    /// The field's name.
    pub field: String,
    /// The value before; `None` when there was none.
    pub before: Option<String>,
    /// The value after; `None` when there is none.
    pub after: Option<String>,
}

impl Change {
    /// A field that went from one value to another.
    pub fn new(field: &str, before: Option<&str>, after: Option<&str>) -> Self {
        Self {
            field: field.to_owned(),
            before: before.map(str::to_owned),
            after: after.map(str::to_owned),
        }
    }

    /// A field that received its first value.
    pub fn set(field: &str, after: &str) -> Self {
        Self::new(field, None, Some(after))
    }
}

/// What a use case says about a change it is making. The trail adds the rest: the
/// sequence, the moment, who acted, and the hashes.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AuditDraft {
    /// What was done.
    pub action: AuditAction,
    /// The kind of the record concerned; `None` for workspace-level actions.
    pub kind: Option<String>,
    /// The record concerned. Not a reference that storage enforces: the entry outlives it.
    pub record_id: Option<RecordId>,
    /// The record's handle as it was at the time.
    pub handle: Option<String>,
    /// What the record was called at the time (FR-046).
    pub display_name: Option<String>,
    /// The fields that changed; empty for actions that change no field.
    pub changes: Vec<Change>,
}

impl AuditDraft {
    /// A draft about the workspace as a whole.
    pub fn workspace(action: AuditAction) -> Self {
        Self {
            action,
            kind: None,
            record_id: None,
            handle: None,
            display_name: None,
            changes: Vec::new(),
        }
    }

    /// A draft about one record.
    pub fn record(
        action: AuditAction,
        kind: &str,
        id: RecordId,
        handle: &str,
        display_name: &str,
    ) -> Self {
        Self {
            action,
            kind: Some(kind.to_owned()),
            record_id: Some(id),
            handle: Some(handle.to_owned()),
            display_name: Some(display_name.to_owned()),
            changes: Vec::new(),
        }
    }

    /// Sets what the entry calls its subject, for drafts that are not about a record.
    #[must_use]
    pub fn named(mut self, display_name: &str) -> Self {
        self.display_name = Some(display_name.to_owned());
        self
    }

    /// Adds the fields that changed.
    #[must_use]
    pub fn with_changes(mut self, changes: Vec<Change>) -> Self {
        self.changes = changes;
        self
    }
}

/// One entry of the trail. Append-only: there is no way to change or remove one.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AuditEntry {
    /// Position in the trail: starts at 1, increases by 1, no gaps. This, not the clock,
    /// is the order of the trail (FR-051).
    pub sequence: u64,
    /// When it happened, to the millisecond. Informative only.
    pub at: OffsetDateTime,
    /// Who acted (FR-050).
    pub actor: ActorName,
    /// What was done.
    pub action: AuditAction,
    /// The kind of the record concerned, if any.
    pub kind: Option<String>,
    /// The record concerned, if any.
    pub record_id: Option<RecordId>,
    /// The record's handle at the time.
    pub handle: Option<String>,
    /// What the record was called at the time.
    pub display_name: Option<String>,
    /// The fields that changed.
    pub changes: Vec<Change>,
    /// The hash of the entry before; zeros for the first.
    pub previous_hash: Digest,
    /// The digest of `previous_hash` followed by this entry's canonical form.
    pub hash: Digest,
}

/// The end of the trail as last seen by this copy of the workspace: kept outside the
/// database so that removing the last entries can be noticed (FR-048).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AuditHead {
    /// The sequence of the last entry.
    pub sequence: u64,
    /// The hash of the last entry.
    pub hash: Digest,
}

impl AuditHead {
    /// The head that corresponds to an entry.
    pub fn of(entry: &AuditEntry) -> Self {
        Self {
            sequence: entry.sequence,
            hash: entry.hash,
        }
    }

    /// The one-line text form kept in the head file: the sequence, a space, the hash.
    pub fn to_line(&self) -> String {
        format!("{} {}\n", self.sequence, to_hex(&self.hash))
    }

    /// Reads the text form written by [`AuditHead::to_line`].
    pub fn parse(line: &str) -> Option<Self> {
        let (sequence, hash) = line.trim().split_once(' ')?;
        Some(Self {
            sequence: sequence.parse().ok()?,
            hash: from_hex(hash)?,
        })
    }
}

/// Who acted and when: stamped on every entry of one command.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Stamp {
    /// The moment of the command, to the millisecond.
    pub at: OffsetDateTime,
    /// The name the researcher's actions are recorded under.
    pub actor: ActorName,
}

impl Stamp {
    /// A stamp; the moment is cut to whole milliseconds so that what is hashed is exactly
    /// what storage gives back.
    pub fn new(at: OffsetDateTime, actor: ActorName) -> Self {
        let nanosecond = at.nanosecond() / 1_000_000 * 1_000_000;
        let at = at.replace_nanosecond(nanosecond).unwrap_or(at);
        Self {
            at: at.to_offset(time::UtcOffset::UTC),
            actor,
        }
    }
}

impl AuditEntry {
    /// Completes a draft into the entry at position `sequence`, following `previous_hash`.
    pub fn seal(
        sequence: u64,
        stamp: &Stamp,
        draft: AuditDraft,
        previous_hash: Digest,
        digest: impl Fn(&[u8]) -> Digest,
    ) -> Self {
        let mut entry = Self {
            sequence,
            at: stamp.at,
            actor: stamp.actor.clone(),
            action: draft.action,
            kind: draft.kind,
            record_id: draft.record_id,
            handle: draft.handle,
            display_name: draft.display_name,
            changes: draft.changes,
            previous_hash,
            hash: GENESIS_HASH,
        };
        entry.hash = entry.expected_hash(digest);
        entry
    }

    /// The hash this entry must have, given its content and its `previous_hash`.
    pub fn expected_hash(&self, digest: impl Fn(&[u8]) -> Digest) -> Digest {
        let mut input = Vec::with_capacity(32 + 256);
        input.extend_from_slice(&self.previous_hash);
        input.extend_from_slice(&self.canonical_form());
        digest(&input)
    }

    /// Every field but the two hashes, in the documented order, each as UTF-8 with a
    /// length prefix.
    pub fn canonical_form(&self) -> Vec<u8> {
        let mut form = Vec::with_capacity(256);
        let milliseconds = self.at.unix_timestamp_nanos() / 1_000_000;
        put(&mut form, Some(&self.sequence.to_string()));
        put(&mut form, Some(&milliseconds.to_string()));
        put(&mut form, Some(self.actor.as_str()));
        put(&mut form, Some(self.action.as_str()));
        put(&mut form, self.kind.as_deref());
        put(
            &mut form,
            self.record_id.map(|id| id.to_string()).as_deref(),
        );
        put(&mut form, self.handle.as_deref());
        put(&mut form, self.display_name.as_deref());
        put(&mut form, Some(&self.changes.len().to_string()));
        for change in &self.changes {
            put(&mut form, Some(&change.field));
            put(&mut form, change.before.as_deref());
            put(&mut form, change.after.as_deref());
        }
        form
    }
}

/// Appends one field to a canonical form.
///
/// A marker byte tells an absent value from an empty one, and the length prefix keeps a
/// value from running into the next, so different entries always give different bytes.
fn put(form: &mut Vec<u8>, value: Option<&str>) {
    match value {
        None => form.push(0),
        Some(text) => {
            form.push(1);
            form.extend_from_slice(&(text.len() as u64).to_be_bytes());
            form.extend_from_slice(text.as_bytes());
        }
    }
}

/// Writes a digest as 64 lower-case hexadecimal characters.
pub fn to_hex(digest: &Digest) -> String {
    digest.iter().map(|byte| format!("{byte:02x}")).collect()
}

/// Reads a digest written by [`to_hex`]; `None` when the text is not one.
pub fn from_hex(text: &str) -> Option<Digest> {
    if text.len() != 64 || !text.is_ascii() {
        return None;
    }
    let mut digest = GENESIS_HASH;
    for (index, byte) in digest.iter_mut().enumerate() {
        *byte = u8::from_str_radix(&text[index * 2..index * 2 + 2], 16).ok()?;
    }
    Some(digest)
}

#[cfg(test)]
mod tests {
    //! Unit tests for the hashing rule of the trail (T019).

    use time::OffsetDateTime;

    use super::{
        AuditAction, AuditDraft, AuditEntry, Change, Digest, GENESIS_HASH, Stamp, from_hex, to_hex,
    };
    use crate::shared::record::RecordId;
    use crate::shared::text::ActorName;

    /// A stand-in digest: not cryptographic, but any change of input changes the output
    /// in these tests, which is all the rule's tests need.
    fn toy_digest(bytes: &[u8]) -> Digest {
        let mut digest = [0_u8; 32];
        for (index, byte) in bytes.iter().enumerate() {
            let slot = index % 32;
            digest[slot] = digest[slot]
                .wrapping_mul(31)
                .wrapping_add(*byte)
                .wrapping_add(index as u8);
        }
        digest
    }

    /// The stamp used by these tests.
    fn stamp() -> Stamp {
        Stamp::new(
            OffsetDateTime::UNIX_EPOCH,
            ActorName::new("ana").expect("valid"),
        )
    }

    /// A draft about one record with one changed field.
    fn draft() -> AuditDraft {
        AuditDraft::record(
            AuditAction::UPDATE,
            "reference",
            RecordId::from_u128(7),
            "ref-7k3f",
            "A title",
        )
        .with_changes(vec![Change::new(
            "status",
            Some("to_read"),
            Some("reading"),
        )])
    }

    #[test]
    fn the_hash_covers_the_previous_hash_and_the_canonical_form() {
        let entry = AuditEntry::seal(1, &stamp(), draft(), GENESIS_HASH, toy_digest);
        let mut input = GENESIS_HASH.to_vec();
        input.extend_from_slice(&entry.canonical_form());
        assert_eq!(entry.hash, toy_digest(&input));
        assert_eq!(entry.expected_hash(toy_digest), entry.hash);
    }

    #[test]
    fn the_first_entry_follows_zeros() {
        let entry = AuditEntry::seal(1, &stamp(), draft(), GENESIS_HASH, toy_digest);
        assert_eq!(entry.previous_hash, [0; 32]);
    }

    #[test]
    fn changing_any_field_changes_the_hash() {
        let original = AuditEntry::seal(1, &stamp(), draft(), GENESIS_HASH, toy_digest);
        let mut altered = Vec::new();
        altered.push(AuditEntry {
            sequence: 2,
            ..original.clone()
        });
        altered.push(AuditEntry {
            at: OffsetDateTime::UNIX_EPOCH + time::Duration::SECOND,
            ..original.clone()
        });
        altered.push(AuditEntry {
            actor: ActorName::new("bob").expect("valid"),
            ..original.clone()
        });
        altered.push(AuditEntry {
            action: AuditAction::DELETE,
            ..original.clone()
        });
        altered.push(AuditEntry {
            kind: None,
            ..original.clone()
        });
        altered.push(AuditEntry {
            record_id: Some(RecordId::from_u128(8)),
            ..original.clone()
        });
        altered.push(AuditEntry {
            handle: Some("ref-0000".into()),
            ..original.clone()
        });
        altered.push(AuditEntry {
            display_name: Some("Another".into()),
            ..original.clone()
        });
        altered.push(AuditEntry {
            changes: Vec::new(),
            ..original.clone()
        });
        altered.push(AuditEntry {
            previous_hash: [9; 32],
            ..original.clone()
        });
        for entry in altered {
            assert_ne!(entry.expected_hash(toy_digest), original.hash);
        }
    }

    #[test]
    fn two_different_entries_never_share_a_canonical_form() {
        // Without length prefixes these two would both read "ab" then "c" / "a" then "bc".
        let mut first = AuditEntry::seal(1, &stamp(), draft(), GENESIS_HASH, toy_digest);
        let mut second = first.clone();
        first.handle = Some("ab".into());
        first.display_name = Some("c".into());
        second.handle = Some("a".into());
        second.display_name = Some("bc".into());
        assert_ne!(first.canonical_form(), second.canonical_form());
    }

    #[test]
    fn an_absent_value_differs_from_an_empty_one() {
        let mut absent = AuditEntry::seal(1, &stamp(), draft(), GENESIS_HASH, toy_digest);
        let mut empty = absent.clone();
        absent.changes[0].before = None;
        empty.changes[0].before = Some(String::new());
        assert_ne!(absent.canonical_form(), empty.canonical_form());
    }

    #[test]
    fn a_stamp_is_cut_to_whole_milliseconds_in_utc() {
        let moment = OffsetDateTime::UNIX_EPOCH + time::Duration::nanoseconds(1_234_567_890);
        let stamp = Stamp::new(moment, ActorName::unknown());
        assert_eq!(stamp.at.nanosecond(), 234_000_000);
    }

    #[test]
    fn a_digest_round_trips_through_hexadecimal() {
        let digest = toy_digest(b"some bytes");
        assert_eq!(from_hex(&to_hex(&digest)), Some(digest));
        assert_eq!(to_hex(&GENESIS_HASH), "0".repeat(64));
        assert_eq!(from_hex("zz"), None);
    }

    #[test]
    fn the_head_round_trips_through_its_line() {
        let entry = AuditEntry::seal(7, &stamp(), draft(), GENESIS_HASH, toy_digest);
        let head = super::AuditHead::of(&entry);
        assert_eq!(super::AuditHead::parse(&head.to_line()), Some(head));
        assert_eq!(super::AuditHead::parse("seven abc"), None);
    }

    #[test]
    fn an_action_is_a_lower_case_word() {
        assert_eq!(AuditAction::named("create"), Some(AuditAction::CREATE));
        assert_eq!(AuditAction::named("Create"), None);
        assert_eq!(AuditAction::named(""), None);
        assert!(AuditAction::FOUNDATION.contains(&AuditAction::UPGRADE.as_str()));
    }
}

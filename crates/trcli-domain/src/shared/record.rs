//! The identity of records (FR-010, FR-011, FR-019).
//!
//! Every record of every kind has two identifiers: a [`RecordId`] that never collides
//! across copies of a workspace, and a [`Handle`], the short name a researcher types. A
//! [`RecordKind`] says what a record is; a [`RecordRef`] is the only way one feature points
//! at another feature's record (FR-072).
//!
//! This module does not assign handles or resolve what was typed: both need to know which
//! records exist, which is the application layer's business.

use uuid::Uuid;

use super::problem::Rejection;

/// The permanent identifier of a record: a version 7 UUID.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct RecordId(Uuid);

impl RecordId {
    /// Wraps an existing UUID.
    pub fn from_uuid(uuid: Uuid) -> Self {
        Self(uuid)
    }

    /// Builds an identifier from a number; used by seeded generators and by tests.
    pub fn from_u128(value: u128) -> Self {
        Self(Uuid::from_u128(value))
    }

    /// Reads an identifier written in the usual hyphenated form.
    pub fn parse(text: &str) -> Option<Self> {
        Uuid::parse_str(text).ok().map(Self)
    }

    /// The UUID itself.
    pub fn as_uuid(&self) -> Uuid {
        self.0
    }

    /// The random part of the identifier, from which handle codes are derived.
    ///
    /// A version 7 UUID begins with a timestamp; only its last 60 bits are used so that
    /// records made in the same millisecond still get different codes.
    fn random_bits(&self) -> u64 {
        (self.0.as_u128() & 0x0FFF_FFFF_FFFF_FFFF) as u64
    }
}

impl std::fmt::Display for RecordId {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}", self.0.as_hyphenated())
    }
}

/// A kind of record: its name and the prefix of its handles.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct RecordKind {
    /// What the kind is called in output and filters: 2 to 40 characters of `a-z -`.
    name: String,
    /// The start of its records' handles: 2 to 4 lower-case letters.
    prefix: String,
}

impl RecordKind {
    /// Checks a kind's name and handle prefix.
    pub fn new(name: &str, prefix: &str) -> Result<Self, Rejection> {
        let name_is_valid = (2..=40).contains(&name.len())
            && name
                .chars()
                .all(|character| character.is_ascii_lowercase() || character == '-');
        if !name_is_valid {
            return Err(Rejection::new(
                "is not a valid kind name",
                "2 to 40 characters of a-z and '-'",
            ));
        }
        let prefix_is_valid = (2..=4).contains(&prefix.len())
            && prefix
                .chars()
                .all(|character| character.is_ascii_lowercase());
        if !prefix_is_valid {
            return Err(Rejection::new(
                "is not a valid handle prefix",
                "2 to 4 letters a-z",
            ));
        }
        Ok(Self {
            name: name.to_owned(),
            prefix: prefix.to_owned(),
        })
    }

    /// The kind's name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// The prefix of the kind's handles.
    pub fn prefix(&self) -> &str {
        &self.prefix
    }
}

/// A pointer to a record of any kind: its kind's name and its identifier.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct RecordRef {
    /// The name of the record's kind.
    pub kind: String,
    /// The record's identifier.
    pub id: RecordId,
}

/// The alphabet of handle codes: Crockford base 32, which has no `i`, `l`, `o`, or `u`,
/// so that a code read aloud or copied by hand is not mistaken.
const CODE_ALPHABET: &[u8; 32] = b"0123456789abcdefghjkmnpqrstvwxyz";

/// The short name of a record: `<prefix>-<code>`, for example `ref-7k3f`.
///
/// Handles are lower case and compared ignoring case; they never change and are never
/// given to another record (FR-010).
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Handle(String);

impl Handle {
    /// The fewest characters a code may have.
    pub const MINIMUM_CODE_LENGTH: usize = 4;
    /// The most characters a code may have: the 60 random bits of an identifier.
    pub const MAXIMUM_CODE_LENGTH: usize = 12;

    /// Derives the handle of a record from its kind and identifier, with a code of
    /// `code_length` characters (kept within the allowed range).
    pub fn derive(kind: &RecordKind, id: RecordId, code_length: usize) -> Self {
        let length = code_length.clamp(Self::MINIMUM_CODE_LENGTH, Self::MAXIMUM_CODE_LENGTH);
        let bits = id.random_bits();
        let code: String = (0..length)
            .map(|position| {
                // Five bits per character, most significant first.
                let shift = 5 * (Self::MAXIMUM_CODE_LENGTH - 1 - position);
                CODE_ALPHABET[((bits >> shift) & 0b1_1111) as usize] as char
            })
            .collect();
        Self(format!("{}-{code}", kind.prefix()))
    }

    /// Reads a complete handle, ignoring letter case.
    pub fn parse(text: &str) -> Result<Self, Rejection> {
        let lowered = text.trim().to_lowercase();
        let rejection = || {
            Rejection::new("is not a short name", "a prefix, a hyphen, and a code")
                .with_example("ref-7k3f")
        };
        let (prefix, code) = lowered.split_once('-').ok_or_else(rejection)?;
        let prefix_is_valid = (2..=4).contains(&prefix.len())
            && prefix
                .chars()
                .all(|character| character.is_ascii_lowercase());
        let code_is_valid = (Self::MINIMUM_CODE_LENGTH..=Self::MAXIMUM_CODE_LENGTH)
            .contains(&code.len())
            && code.bytes().all(|byte| CODE_ALPHABET.contains(&byte));
        if prefix_is_valid && code_is_valid {
            Ok(Self(lowered))
        } else {
            Err(rejection())
        }
    }

    /// The handle as text.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// The prefix that says what kind of record this is.
    pub fn prefix(&self) -> &str {
        self.0.split_once('-').map_or("", |(prefix, _)| prefix)
    }
}

impl std::fmt::Display for Handle {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.0)
    }
}

/// What a researcher typed to name a record: a handle or the beginning of one (FR-011).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TypedReference(String);

impl TypedReference {
    /// Checks what was typed: lowered, and made only of what a handle can contain.
    pub fn new(raw: &str) -> Result<Self, Rejection> {
        let lowered = raw.trim().to_lowercase();
        let is_valid = !lowered.is_empty()
            && lowered.len() <= 4 + 1 + Handle::MAXIMUM_CODE_LENGTH
            && lowered
                .chars()
                .all(|character| character.is_ascii_alphanumeric() || character == '-');
        if !is_valid {
            return Err(Rejection::new(
                "is not a short name or the beginning of one",
                "letters, digits, and '-'",
            )
            .with_example("ref-7k3f"));
        }
        Ok(Self(lowered))
    }

    /// What was typed, in lower case.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    //! Unit tests for record identity (T026).

    use super::{Handle, RecordId, RecordKind, TypedReference};

    /// The kind used throughout these tests.
    fn reference_kind() -> RecordKind {
        RecordKind::new("reference", "ref").expect("valid kind")
    }

    #[test]
    fn a_kind_name_is_two_to_forty_lower_case_letters_or_hyphens() {
        assert!(RecordKind::new("sample-note", "smp").is_ok());
        assert!(RecordKind::new("x", "ab").is_err());
        assert!(RecordKind::new("Reference", "ref").is_err());
        assert!(RecordKind::new(&"k".repeat(41), "ref").is_err());
    }

    #[test]
    fn a_handle_prefix_is_two_to_four_letters() {
        assert!(RecordKind::new("question", "rq").is_ok());
        assert!(RecordKind::new("question", "q").is_err());
        assert!(RecordKind::new("question", "quest").is_err());
        assert!(RecordKind::new("question", "r2").is_err());
    }

    #[test]
    fn a_handle_is_the_prefix_a_hyphen_and_a_code() {
        let handle = Handle::derive(
            &reference_kind(),
            RecordId::from_u128(0x1234_5678_9abc_def0),
            4,
        );
        assert!(handle.as_str().starts_with("ref-"));
        assert_eq!(handle.as_str().len(), "ref-".len() + 4);
        assert_eq!(handle.prefix(), "ref");
    }

    #[test]
    fn a_longer_code_extends_the_shorter_one() {
        let id = RecordId::from_u128(0x0abc_def0_1234_5678);
        let short = Handle::derive(&reference_kind(), id, 4);
        let long = Handle::derive(&reference_kind(), id, 5);
        assert!(long.as_str().starts_with(short.as_str()));
        assert_eq!(
            Handle::derive(&reference_kind(), id, 99).as_str().len(),
            "ref-".len() + 12
        );
    }

    #[test]
    fn handles_never_contain_the_letters_that_are_easily_misread() {
        for seed in 0..500_u128 {
            let handle = Handle::derive(
                &reference_kind(),
                RecordId::from_u128(seed * 0x9e37_79b9_7f4a_7c15),
                12,
            );
            let code = handle.as_str().trim_start_matches("ref-");
            assert!(!code.contains(['i', 'l', 'o', 'u']), "{handle}");
        }
    }

    #[test]
    fn handles_are_read_ignoring_case() {
        assert_eq!(
            Handle::parse("REF-7K3F").expect("valid").as_str(),
            "ref-7k3f"
        );
        assert!(Handle::parse("ref7k3f").is_err());
        assert!(Handle::parse("ref-7k").is_err());
        assert!(Handle::parse("ref-7k3l").is_err());
    }

    #[test]
    fn a_typed_reference_is_lowered_and_may_be_only_a_beginning() {
        assert_eq!(
            TypedReference::new(" SPC-7 ").expect("valid").as_str(),
            "spc-7"
        );
        assert!(TypedReference::new("spc").is_ok());
        assert!(TypedReference::new("").is_err());
        assert!(TypedReference::new("spc 7").is_err());
        assert!(TypedReference::new("../etc").is_err());
    }

    #[test]
    fn a_record_id_round_trips_through_text() {
        let id = RecordId::from_u128(42);
        assert_eq!(RecordId::parse(&id.to_string()), Some(id));
        assert_eq!(RecordId::parse("not an id"), None);
    }
}

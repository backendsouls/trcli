//! Checked text: the only way text enters the domain (FR-021, FR-026, FR-067).
//!
//! Every text type here is built through a constructor that trims surrounding whitespace,
//! rejects control characters other than line break and tab, and enforces a length counted
//! in characters. Apart from that trimming, text is kept exactly as written, in any script.
//!
//! [`SearchKey`] is the one derived form: it is used to search and sort without regard to
//! case or accents, and is never shown.

use unicode_normalization::UnicodeNormalization;
use unicode_normalization::char::is_combining_mark;

use super::problem::Rejection;

/// Trims `raw` and checks it against the rules shared by all text.
///
/// `minimum` and `maximum` are counted in characters, not bytes, so that a limit means the
/// same in every script (FR-067).
fn checked(raw: &str, minimum: usize, maximum: usize) -> Result<String, Rejection> {
    let trimmed = raw.trim();
    let expected = expected_length(minimum, maximum);
    if trimmed.chars().any(is_forbidden_control) {
        return Err(Rejection::new(
            "contains control characters that cannot be stored",
            expected,
        ));
    }
    let length = trimmed.chars().count();
    if length == 0 && minimum > 0 {
        return Err(Rejection::new("must not be empty", expected));
    }
    if length < minimum {
        return Err(Rejection::new(
            format!("is too short ({length} characters)"),
            expected,
        ));
    }
    if length > maximum {
        return Err(Rejection::new(
            format!("is too long ({length} characters)"),
            expected,
        ));
    }
    Ok(trimmed.to_owned())
}

/// Whether a character is a control character that text may not contain (FR-026).
fn is_forbidden_control(character: char) -> bool {
    character.is_control() && character != '\n' && character != '\t'
}

/// The phrase that states a length rule: "1 to 200 characters".
fn expected_length(minimum: usize, maximum: usize) -> String {
    if minimum == 0 {
        format!("at most {maximum} characters")
    } else {
        format!("{minimum} to {maximum} characters")
    }
}

/// Declares a text value object with a length rule and the shared accessors.
macro_rules! text_value {
    ($(#[$doc:meta])* $name:ident, $minimum:expr, $maximum:expr) => {
        $(#[$doc])*
        #[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
        pub struct $name(String);

        impl $name {
            /// The fewest characters allowed.
            pub const MINIMUM: usize = $minimum;
            /// The most characters allowed.
            pub const MAXIMUM: usize = $maximum;

            /// Checks `raw` and keeps it, trimmed, when it follows the rule.
            pub fn new(raw: &str) -> Result<Self, Rejection> {
                checked(raw, Self::MINIMUM, Self::MAXIMUM).map(Self)
            }

            /// The text as written.
            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str(&self.0)
            }
        }
    };
}

text_value!(
    /// The main text of a record: 1 to 500 characters.
    Title, 1, 500
);
text_value!(
    /// The name of something: 1 to 200 characters.
    Name, 1, 200
);
text_value!(
    /// Free text of any length up to 20,000 characters; may be empty.
    LongText, 0, 20_000
);
text_value!(
    /// How two linked records relate: 1 to 50 characters.
    Relation, 1, 50
);
text_value!(
    /// The name under which a researcher's actions are recorded: 1 to 200 characters.
    ActorName, 1, 200
);

impl Relation {
    /// The relation used when the researcher does not say how two records relate.
    pub const DEFAULT: &'static str = "related";

    /// The default relation, `related`.
    pub fn related() -> Self {
        Self(Self::DEFAULT.to_owned())
    }
}

impl ActorName {
    /// The placeholder recorded when nobody's name can be determined (FR-050).
    pub const UNKNOWN: &'static str = "unknown";

    /// The placeholder actor.
    pub fn unknown() -> Self {
        Self(Self::UNKNOWN.to_owned())
    }

    /// Whether this is the placeholder, so that the tool can suggest setting a name.
    pub fn is_unknown(&self) -> bool {
        self.0 == Self::UNKNOWN
    }
}

/// The name of a tag: 1 to 50 characters of `a-z 0-9 - _`.
///
/// Normalization is stated and nothing else is altered (FR-025): surrounding whitespace is
/// removed and upper case is lowered. Inner spaces and any other character are rejected
/// with the rule.
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct TagName(String);

impl TagName {
    /// The most characters a tag name may have.
    pub const MAXIMUM: usize = 50;
    /// The rule, as shown to the researcher.
    pub const RULE: &'static str = "1 to 50 characters of a-z, 0-9, '-' and '_' (no spaces)";

    /// Normalizes and checks a tag name.
    pub fn new(raw: &str) -> Result<Self, Rejection> {
        let name = raw.trim().to_lowercase();
        let rejection =
            |problem: &str| Rejection::new(problem, Self::RULE).with_example("field-work");
        if name.is_empty() {
            return Err(rejection("must not be empty"));
        }
        if name.chars().count() > Self::MAXIMUM {
            return Err(rejection("is too long"));
        }
        if !name.chars().all(Self::is_allowed) {
            return Err(rejection("contains characters a tag name may not have"));
        }
        Ok(Self(name))
    }

    /// Whether a character may appear in a tag name.
    fn is_allowed(character: char) -> bool {
        character.is_ascii_lowercase()
            || character.is_ascii_digit()
            || matches!(character, '-' | '_')
    }

    /// The normalized name.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for TagName {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.0)
    }
}

/// Text reduced to what matters for finding it: lower case, no accents, single spaces.
///
/// "Ação" and "acao" have the same key; "東京" is preserved (FR-067). Used for searching
/// and sorting, never shown.
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SearchKey(String);

impl SearchKey {
    /// Derives the key of a piece of text.
    pub fn from_text(text: &str) -> Self {
        let mut key = String::with_capacity(text.len());
        // Compatibility decomposition separates a letter from its accents, which are then
        // dropped as combining marks.
        for character in text
            .nfkd()
            .filter(|character| !is_combining_mark(*character))
        {
            if character.is_alphanumeric() {
                key.extend(character.to_lowercase());
            } else if !key.ends_with(' ') {
                // Punctuation and whitespace collapse to one space between words.
                key.push(' ');
            }
        }
        Self(key.trim().to_owned())
    }

    /// The key itself.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Whether the key holds no searchable character.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

#[cfg(test)]
mod tests {
    //! Unit tests for the text value objects and the search key (T011, T012).

    use super::{ActorName, LongText, Name, Relation, SearchKey, TagName, Title};

    #[test]
    fn title_is_one_to_five_hundred_characters() {
        assert!(Title::new("").is_err());
        assert!(Title::new("a").is_ok());
        assert!(Title::new(&"é".repeat(500)).is_ok());
        assert!(Title::new(&"é".repeat(501)).is_err());
    }

    #[test]
    fn name_is_one_to_two_hundred_characters() {
        assert!(Name::new("   ").is_err());
        assert!(Name::new(&"n".repeat(200)).is_ok());
        assert!(Name::new(&"n".repeat(201)).is_err());
    }

    #[test]
    fn long_text_may_be_empty_and_holds_twenty_thousand_characters() {
        assert!(LongText::new("").is_ok());
        assert!(LongText::new(&"x".repeat(20_000)).is_ok());
        assert!(LongText::new(&"x".repeat(20_001)).is_err());
    }

    #[test]
    fn surrounding_whitespace_is_trimmed() {
        assert_eq!(
            Title::new("  Field notes \n").expect("valid").as_str(),
            "Field notes"
        );
    }

    #[test]
    fn control_characters_other_than_line_break_and_tab_are_rejected() {
        assert!(Title::new("bell\u{7}").is_err());
        assert!(Title::new("escape\u{1b}[0m").is_err());
        assert!(LongText::new("line one\nline two\tindented").is_ok());
    }

    #[test]
    fn text_is_kept_exactly_as_written_in_any_script() {
        for text in [
            "Ação e reação",
            "東京の研究",
            "مرحبا بالعالم",
            "Ünïcödé — “quoted”",
        ] {
            assert_eq!(Title::new(text).expect("valid").as_str(), text);
        }
    }

    #[test]
    fn a_rejection_states_the_limit() {
        let rejection = Name::new("").expect_err("empty");
        assert_eq!(rejection.problem, "must not be empty");
        assert_eq!(rejection.expected, "1 to 200 characters");
    }

    #[test]
    fn tag_names_are_lowered_and_trimmed_and_nothing_else() {
        assert_eq!(
            TagName::new(" Field-Work ").expect("valid").as_str(),
            "field-work"
        );
        assert_eq!(
            TagName::new("to_read2").expect("valid").as_str(),
            "to_read2"
        );
    }

    #[test]
    fn tag_names_with_inner_spaces_or_other_characters_are_rejected_with_the_rule() {
        for raw in ["two words", "ação", "a/b", "", &"t".repeat(51)] {
            let rejection = TagName::new(raw).expect_err("invalid tag");
            assert_eq!(rejection.expected, TagName::RULE);
        }
    }

    #[test]
    fn relation_defaults_to_related() {
        assert_eq!(Relation::related().as_str(), "related");
        assert!(Relation::new(&"r".repeat(51)).is_err());
    }

    #[test]
    fn the_unknown_actor_is_recognizable() {
        assert!(ActorName::unknown().is_unknown());
        assert!(!ActorName::new("ana").expect("valid").is_unknown());
    }

    #[test]
    fn search_key_is_lower_case_without_accents() {
        assert_eq!(SearchKey::from_text("Ação"), SearchKey::from_text("acao"));
        assert_eq!(SearchKey::from_text("Ação").as_str(), "acao");
    }

    #[test]
    fn search_key_collapses_punctuation_to_single_spaces() {
        assert_eq!(
            SearchKey::from_text("  Deep-Learning:  a   survey! ").as_str(),
            "deep learning a survey"
        );
    }

    #[test]
    fn search_key_preserves_scripts_without_case_or_accents() {
        assert_eq!(SearchKey::from_text("東京").as_str(), "東京");
        assert!(SearchKey::from_text("…").is_empty());
    }
}

//! Settings as the domain sees them (FR-039 to FR-045).
//!
//! A setting exists because a feature registered its [`SettingDefinition`]: its key, the
//! kind of value it takes, its default, and where it may be set. Values are checked against
//! the definition's [`ValueKind`] wherever they come from.
//!
//! Where values are read from, and which source wins, is the application layer's business.
//! No setting may hold a secret (FR-045).

use crate::shared::problem::Rejection;

/// The dotted name of a setting, for example `output.color`.
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SettingKey(String);

impl SettingKey {
    /// Checks a key: one or more segments of `a-z 0-9 _` joined by dots.
    pub fn new(raw: &str) -> Result<Self, Rejection> {
        let is_segment = |segment: &str| {
            !segment.is_empty()
                && segment.chars().all(|character| {
                    character.is_ascii_lowercase() || character.is_ascii_digit() || character == '_'
                })
        };
        if raw.is_empty() || !raw.split('.').all(is_segment) {
            return Err(Rejection::new(
                "is not a setting name",
                "words of a-z, 0-9, '_' joined by dots",
            )
            .with_example("output.color"));
        }
        Ok(Self(raw.to_owned()))
    }

    /// The key as text.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// The name of the session variable that overrides this setting: `output.color` is
    /// overridden by `TRCLI_OUTPUT_COLOR`.
    pub fn variable_name(&self) -> String {
        format!("TRCLI_{}", self.0.replace('.', "_").to_uppercase())
    }
}

impl std::fmt::Display for SettingKey {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.0)
    }
}

/// One of the two places a researcher stores settings.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Place {
    /// The researcher's own file: applies in every workspace they use.
    User,
    /// The workspace's file: applies there only.
    Workspace,
}

/// Where a setting may be stored (FR-044).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Scope {
    /// Only for a person.
    User,
    /// Only for a workspace.
    Workspace,
    /// Either.
    Both,
}

impl Scope {
    /// Whether a setting of this scope may be stored in `place`.
    pub fn allows(self, place: Place) -> bool {
        match self {
            Self::Both => true,
            Self::User => place == Place::User,
            Self::Workspace => place == Place::Workspace,
        }
    }

    /// The scope as shown to the researcher.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::User => "user",
            Self::Workspace => "workspace",
            Self::Both => "both",
        }
    }
}

/// The value of a setting.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SettingValue {
    /// Text, including a choice from a list, a path, and a style.
    Text(String),
    /// A whole number.
    Integer(i64),
    /// Yes or no.
    Boolean(bool),
}

impl SettingValue {
    /// The text of a text value.
    pub fn as_text(&self) -> Option<&str> {
        match self {
            Self::Text(text) => Some(text),
            _ => None,
        }
    }

    /// The number of a whole-number value.
    pub fn as_integer(&self) -> Option<i64> {
        match self {
            Self::Integer(number) => Some(*number),
            _ => None,
        }
    }

    /// The answer of a yes/no value.
    pub fn as_boolean(&self) -> Option<bool> {
        match self {
            Self::Boolean(answer) => Some(*answer),
            _ => None,
        }
    }
}

impl std::fmt::Display for SettingValue {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Text(text) => formatter.write_str(text),
            Self::Integer(number) => write!(formatter, "{number}"),
            Self::Boolean(answer) => write!(formatter, "{answer}"),
        }
    }
}

/// The kind of value a setting takes, with its limits.
///
/// The foundation needs these six. Lists of text and spans of time arrive with the first
/// feature that registers a setting of that kind.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ValueKind {
    /// Text of a length between the two limits, in characters.
    Text {
        /// Fewest characters.
        minimum: usize,
        /// Most characters.
        maximum: usize,
    },
    /// A whole number between the two limits, inclusive.
    Integer {
        /// Smallest value.
        minimum: i64,
        /// Largest value.
        maximum: i64,
    },
    /// Yes or no.
    Boolean,
    /// One of a fixed list of words.
    OneOf(&'static [&'static str]),
    /// A path to a file or directory.
    Path,
    /// A style: a colour name with optional `bold`, `dim`, `underline`.
    Style,
}

impl ValueKind {
    /// Checks a value that arrived as text (a session variable, `config set`).
    pub fn parse(&self, text: &str) -> Result<SettingValue, Rejection> {
        self.accept(&SettingValue::Text(text.to_owned()))
    }

    /// Checks a value of any form against this kind and returns it in the kind's form.
    pub fn accept(&self, raw: &SettingValue) -> Result<SettingValue, Rejection> {
        match self {
            Self::Text { minimum, maximum } => self.accept_text(raw, *minimum, *maximum),
            Self::Integer { minimum, maximum } => self.accept_integer(raw, *minimum, *maximum),
            Self::Boolean => self.accept_boolean(raw),
            Self::OneOf(choices) => self.accept_choice(raw, choices),
            Self::Path => self.accept_text(raw, 1, 4096),
            Self::Style => self.accept_style(raw),
        }
    }

    /// What this kind allows, as shown by `config get` and in rejections.
    pub fn describe(&self) -> String {
        match self {
            Self::Text { minimum, maximum } => format!("text of {minimum} to {maximum} characters"),
            Self::Integer { minimum, maximum } => {
                format!("a whole number from {minimum} to {maximum}")
            }
            Self::Boolean => "true or false".to_owned(),
            Self::OneOf(choices) => format!("one of: {}", choices.join(", ")),
            Self::Path => "a path".to_owned(),
            Self::Style => format!(
                "a colour ({}) optionally followed by bold, dim, underline",
                StyleSpec::COLORS.join(", ")
            ),
        }
    }

    /// A rejection that states what this kind allows.
    fn rejection(&self, problem: &str) -> Rejection {
        let rejection = Rejection::new(problem, self.describe());
        match self {
            Self::OneOf(choices) => rejection.with_choices(choices.iter().copied()),
            Self::Integer { minimum, .. } => rejection.with_example(minimum.to_string()),
            Self::Boolean => rejection.with_choices(["true", "false"]),
            Self::Style => rejection.with_example("red bold"),
            Self::Text { .. } | Self::Path => rejection,
        }
    }

    /// Accepts text within the length limits.
    fn accept_text(
        &self,
        raw: &SettingValue,
        minimum: usize,
        maximum: usize,
    ) -> Result<SettingValue, Rejection> {
        let text = raw
            .as_text()
            .ok_or_else(|| self.rejection("is not text"))?
            .trim();
        let length = text.chars().count();
        if length < minimum || length > maximum || text.chars().any(char::is_control) {
            return Err(self.rejection("is not acceptable text"));
        }
        Ok(SettingValue::Text(text.to_owned()))
    }

    /// Accepts a whole number within the range, given as a number or as text.
    fn accept_integer(
        &self,
        raw: &SettingValue,
        minimum: i64,
        maximum: i64,
    ) -> Result<SettingValue, Rejection> {
        let number = match raw {
            SettingValue::Integer(number) => Some(*number),
            SettingValue::Text(text) => text.trim().parse::<i64>().ok(),
            SettingValue::Boolean(_) => None,
        };
        match number {
            Some(number) if (minimum..=maximum).contains(&number) => {
                Ok(SettingValue::Integer(number))
            }
            Some(_) => Err(self.rejection("is out of range")),
            None => Err(self.rejection("is not a whole number")),
        }
    }

    /// Accepts yes or no, given as such or as one of the usual words.
    fn accept_boolean(&self, raw: &SettingValue) -> Result<SettingValue, Rejection> {
        let answer = match raw {
            SettingValue::Boolean(answer) => Some(*answer),
            SettingValue::Text(text) => match text.trim().to_lowercase().as_str() {
                "true" | "yes" | "on" | "1" => Some(true),
                "false" | "no" | "off" | "0" => Some(false),
                _ => None,
            },
            SettingValue::Integer(_) => None,
        };
        answer
            .map(SettingValue::Boolean)
            .ok_or_else(|| self.rejection("is not true or false"))
    }

    /// Accepts one of the listed words, exactly as listed.
    fn accept_choice(
        &self,
        raw: &SettingValue,
        choices: &[&str],
    ) -> Result<SettingValue, Rejection> {
        match raw.as_text().map(str::trim) {
            Some(text) if choices.contains(&text) => Ok(SettingValue::Text(text.to_owned())),
            _ => Err(self.rejection("is not one of the allowed values")),
        }
    }

    /// Accepts a style that can be read as a [`StyleSpec`].
    fn accept_style(&self, raw: &SettingValue) -> Result<SettingValue, Rejection> {
        let text = raw
            .as_text()
            .ok_or_else(|| self.rejection("is not a style"))?
            .trim();
        match StyleSpec::parse(text) {
            Some(_) => Ok(SettingValue::Text(text.to_lowercase())),
            None => Err(self.rejection("is not a style")),
        }
    }
}

/// A style as a researcher writes it: a colour name and optional emphasis.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct StyleSpec {
    /// The colour's name, one of [`StyleSpec::COLORS`]; `None` keeps the terminal's own.
    pub color: Option<&'static str>,
    /// Heavier weight.
    pub bold: bool,
    /// Lighter weight.
    pub dim: bool,
    /// Underlined.
    pub underline: bool,
}

impl StyleSpec {
    /// The colour names a style may use.
    pub const COLORS: [&'static str; 8] = [
        "black", "red", "green", "yellow", "blue", "magenta", "cyan", "white",
    ];

    /// Reads a style such as `red bold` or `dim`; `None` when a word is not understood.
    pub fn parse(text: &str) -> Option<Self> {
        let mut style = Self::default();
        let mut words = 0;
        for word in text.split_whitespace() {
            words += 1;
            match word.to_lowercase().as_str() {
                "bold" => style.bold = true,
                "dim" => style.dim = true,
                "underline" => style.underline = true,
                other => {
                    style.color = Some(Self::COLORS.iter().copied().find(|color| *color == other)?)
                }
            }
        }
        (words > 0).then_some(style)
    }
}

/// Why a definition cannot be registered.
#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]
pub enum DefinitionError {
    /// The definition is marked secret: settings must not hold secrets (FR-045).
    #[error("setting `{0}` is marked secret; settings must not hold secrets")]
    Secret(String),
    /// The default does not follow the definition's own rule.
    #[error("the default of setting `{0}` is not a valid value for it")]
    InvalidDefault(String),
}

/// What a feature declares about one setting.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SettingDefinition {
    /// The setting's name.
    pub key: SettingKey,
    /// The kind of value it takes.
    pub kind: ValueKind,
    /// The value in effect when no source gives one; `None` means "unset" (FR-042).
    pub default: Option<SettingValue>,
    /// Where it may be stored.
    pub scope: Scope,
    /// One line saying what it is for, shown by `config get` (FR-041).
    pub summary: &'static str,
    /// Always `false`: the field exists so that a secret setting can be refused by rule
    /// rather than by convention (FR-045).
    pub secret: bool,
}

impl SettingDefinition {
    /// A definition; `key` must be a valid key, which is a programming error otherwise.
    pub fn new(key: &str, kind: ValueKind, scope: Scope, summary: &'static str) -> Self {
        let key =
            SettingKey::new(key).unwrap_or_else(|_| panic!("`{key}` is not a valid setting key"));
        Self {
            key,
            kind,
            default: None,
            scope,
            summary,
            secret: false,
        }
    }

    /// Sets the default from its text form.
    #[must_use]
    pub fn with_default(mut self, default: &str) -> Self {
        self.default = Some(SettingValue::Text(default.to_owned()));
        self
    }

    /// Checks that the definition may be registered and puts its default in its kind's form.
    pub fn checked(mut self) -> Result<Self, DefinitionError> {
        if self.secret {
            return Err(DefinitionError::Secret(self.key.to_string()));
        }
        if let Some(default) = &self.default {
            let accepted = self.kind.accept(default);
            self.default =
                Some(accepted.map_err(|_| DefinitionError::InvalidDefault(self.key.to_string()))?);
        }
        Ok(self)
    }
}

#[cfg(test)]
mod tests {
    //! Unit tests for setting keys, kinds, and definitions.

    use super::{
        DefinitionError, Place, Scope, SettingDefinition, SettingKey, SettingValue, StyleSpec,
        ValueKind,
    };

    #[test]
    fn a_key_is_dotted_lower_case_segments() {
        assert!(SettingKey::new("output.color").is_ok());
        assert!(SettingKey::new("storage.busy_timeout_ms").is_ok());
        for bad in [
            "",
            "Output.color",
            "output..color",
            ".color",
            "output color",
        ] {
            assert!(SettingKey::new(bad).is_err(), "{bad}");
        }
    }

    #[test]
    fn a_key_names_its_session_variable() {
        let key = SettingKey::new("storage.busy_timeout_ms").expect("valid");
        assert_eq!(key.variable_name(), "TRCLI_STORAGE_BUSY_TIMEOUT_MS");
    }

    #[test]
    fn scope_decides_where_a_setting_may_be_stored() {
        assert!(Scope::Both.allows(Place::User) && Scope::Both.allows(Place::Workspace));
        assert!(Scope::User.allows(Place::User) && !Scope::User.allows(Place::Workspace));
        assert!(!Scope::Workspace.allows(Place::User));
    }

    #[test]
    fn a_whole_number_is_checked_against_its_range() {
        let kind = ValueKind::Integer {
            minimum: 1,
            maximum: 1000,
        };
        assert_eq!(kind.parse("20"), Ok(SettingValue::Integer(20)));
        assert_eq!(
            kind.accept(&SettingValue::Integer(1000)),
            Ok(SettingValue::Integer(1000))
        );
        assert!(kind.parse("0").is_err());
        assert!(kind.parse("twenty").is_err());
    }

    #[test]
    fn a_choice_lists_what_is_allowed_when_refused() {
        let kind = ValueKind::OneOf(&["auto", "always", "never"]);
        assert!(kind.parse("never").is_ok());
        let rejection = kind.parse("sometimes").expect_err("not a choice");
        assert_eq!(rejection.choices, ["auto", "always", "never"]);
    }

    #[test]
    fn yes_and_no_are_read_from_the_usual_words() {
        assert_eq!(
            ValueKind::Boolean.parse("off"),
            Ok(SettingValue::Boolean(false))
        );
        assert_eq!(
            ValueKind::Boolean.accept(&SettingValue::Boolean(true)),
            Ok(SettingValue::Boolean(true))
        );
        assert!(ValueKind::Boolean.parse("maybe").is_err());
    }

    #[test]
    fn a_style_is_a_colour_with_optional_emphasis() {
        let style = StyleSpec::parse("red bold").expect("valid");
        assert_eq!(style.color, Some("red"));
        assert!(style.bold && !style.dim);
        assert_eq!(
            StyleSpec::parse("dim"),
            Some(StyleSpec {
                dim: true,
                ..StyleSpec::default()
            })
        );
        assert_eq!(StyleSpec::parse("crimson"), None);
        assert_eq!(StyleSpec::parse(""), None);
    }

    #[test]
    fn a_definition_marked_secret_cannot_be_registered() {
        let mut definition =
            SettingDefinition::new("api.token", ValueKind::Path, Scope::User, "A token.");
        definition.secret = true;
        assert_eq!(
            definition.checked(),
            Err(DefinitionError::Secret("api.token".into()))
        );
    }

    #[test]
    fn a_default_is_put_in_the_form_of_its_kind() {
        let definition = SettingDefinition::new(
            "output.page_size",
            ValueKind::Integer {
                minimum: 1,
                maximum: 1000,
            },
            Scope::Both,
            "Rows per list.",
        )
        .with_default("50")
        .checked()
        .expect("valid");
        assert_eq!(definition.default, Some(SettingValue::Integer(50)));
    }

    #[test]
    fn a_default_outside_its_own_rule_is_refused() {
        let definition = SettingDefinition::new(
            "output.format",
            ValueKind::OneOf(&["human", "json"]),
            Scope::Both,
            "Form.",
        )
        .with_default("xml");
        assert!(matches!(
            definition.checked(),
            Err(DefinitionError::InvalidDefault(_))
        ));
    }
}

//! The settings the foundation registers: where the workspace is, how output looks, who
//! is acting, and whether local telemetry is kept. Their allowed values and defaults are
//! those of `contracts/configuration.md`.

use trcli_domain::settings::{Scope, SettingDefinition, ValueKind};

use super::registry::SettingsRegistry;

/// The workspace used when none is found from the current directory.
pub const DEFAULT_WORKSPACE: &str = "default_workspace";
/// Where the workspace's database is, relative to the workspace root.
pub const STORAGE_PATH: &str = "storage.path";
/// How long to wait for another command that is changing the workspace.
pub const STORAGE_BUSY_TIMEOUT_MS: &str = "storage.busy_timeout_ms";
/// The default output form.
pub const OUTPUT_FORMAT: &str = "output.format";
/// When to use colour.
pub const OUTPUT_COLOR: &str = "output.color";
/// Special symbols, or plain characters only.
pub const OUTPUT_SYMBOLS: &str = "output.symbols";
/// How many rows a list shows by default.
pub const OUTPUT_PAGE_SIZE: &str = "output.page_size";
/// How dates are shown.
pub const OUTPUT_DATE_FORMAT: &str = "output.date_format";
/// The name recorded as the actor.
pub const RESEARCHER_NAME: &str = "researcher.name";
/// Whether local telemetry is recorded.
pub const TELEMETRY_ENABLED: &str = "telemetry.enabled";

/// The meanings a theme gives a style to, with their default styles.
pub const THEME_STYLES: [(&str, &str, &str); 6] = [
    (
        "theme.success",
        "green",
        "Style of messages that say something worked.",
    ),
    ("theme.warning", "yellow", "Style of warnings."),
    ("theme.error", "red bold", "Style of error messages."),
    ("theme.handle", "cyan", "Style of short names of records."),
    (
        "theme.heading",
        "bold",
        "Style of headings and column titles.",
    ),
    ("theme.muted", "dim", "Style of secondary text."),
];

/// Where the workspace and its storage are.
fn storage_definitions() -> Vec<SettingDefinition> {
    vec![
        SettingDefinition::new(
            DEFAULT_WORKSPACE,
            ValueKind::Path,
            Scope::User,
            "Workspace used when none is found from the current directory.",
        ),
        SettingDefinition::new(
            STORAGE_PATH,
            ValueKind::Path,
            Scope::Workspace,
            "Location of the workspace's database; relative paths are relative to the workspace root.",
        )
        .with_default(".trcli/trcli.db"),
        SettingDefinition::new(
            STORAGE_BUSY_TIMEOUT_MS,
            ValueKind::Integer { minimum: 0, maximum: 60_000 },
            Scope::Both,
            "How long to wait, in milliseconds, when another trcli command is changing the workspace.",
        )
        .with_default("5000"),
    ]
}

/// How output looks.
fn output_definitions() -> Vec<SettingDefinition> {
    let choice = |key, choices, summary| {
        SettingDefinition::new(key, ValueKind::OneOf(choices), Scope::Both, summary)
    };
    vec![
        choice(OUTPUT_FORMAT, &["human", "json"], "Default output form.").with_default("human"),
        choice(
            OUTPUT_COLOR,
            &["auto", "always", "never"],
            "Coloured output.",
        )
        .with_default("auto"),
        choice(
            OUTPUT_SYMBOLS,
            &["unicode", "ascii"],
            "Special symbols, or plain characters only.",
        )
        .with_default("unicode"),
        SettingDefinition::new(
            OUTPUT_PAGE_SIZE,
            ValueKind::Integer {
                minimum: 1,
                maximum: 1000,
            },
            Scope::Both,
            "How many rows a list shows unless --limit says otherwise.",
        )
        .with_default("50"),
        choice(
            OUTPUT_DATE_FORMAT,
            &["iso"],
            "Dates are shown as YYYY-MM-DD.",
        )
        .with_default("iso"),
    ]
}

/// Who is acting, and whether the tool's own use is recorded.
fn governance_definitions() -> Vec<SettingDefinition> {
    vec![
        SettingDefinition::new(
            RESEARCHER_NAME,
            ValueKind::Text {
                minimum: 1,
                maximum: 200,
            },
            Scope::Both,
            "Name recorded as the actor in the audit trail (default: your system user name).",
        ),
        SettingDefinition::new(
            TELEMETRY_ENABLED,
            ValueKind::Boolean,
            Scope::Workspace,
            "Records local telemetry about the tool's use. Nothing is ever transmitted.",
        )
        .with_default("true"),
    ]
}

/// The foundation's definitions other than the theme, in the order `config list` shows.
fn definitions() -> Vec<SettingDefinition> {
    let mut definitions = storage_definitions();
    definitions.extend(output_definitions());
    definitions.extend(governance_definitions());
    definitions
}

/// Registers the foundation's settings. Failing here is a programming error in the
/// definitions above, found the first time the tool starts.
pub fn register(registry: &mut SettingsRegistry) {
    let theme = THEME_STYLES.iter().map(|(key, default, summary)| {
        SettingDefinition::new(key, ValueKind::Style, Scope::Both, summary).with_default(default)
    });
    for definition in definitions().into_iter().chain(theme) {
        let key = definition.key.to_string();
        registry
            .register(definition)
            .unwrap_or_else(|error| panic!("foundation setting `{key}`: {error}"));
    }
}

#[cfg(test)]
mod tests {
    //! Unit tests for the foundation's own settings.

    use trcli_domain::settings::SettingValue;

    use super::{OUTPUT_PAGE_SIZE, STORAGE_PATH, TELEMETRY_ENABLED, register};
    use crate::settings::registry::SettingsRegistry;

    #[test]
    fn the_foundation_registers_its_settings_with_the_contract_defaults() {
        let mut registry = SettingsRegistry::new();
        register(&mut registry);
        let default = |key: &str| {
            registry
                .find(key)
                .and_then(|definition| definition.default.clone())
        };
        assert_eq!(
            default(STORAGE_PATH),
            Some(SettingValue::Text(".trcli/trcli.db".into()))
        );
        assert_eq!(default(OUTPUT_PAGE_SIZE), Some(SettingValue::Integer(50)));
        assert_eq!(
            default(TELEMETRY_ENABLED),
            Some(SettingValue::Boolean(true))
        );
        assert_eq!(
            default("theme.error"),
            Some(SettingValue::Text("red bold".into()))
        );
        assert_eq!(registry.all().len(), 16);
    }

    #[test]
    fn no_foundation_setting_is_secret() {
        let mut registry = SettingsRegistry::new();
        register(&mut registry);
        assert!(registry.all().iter().all(|definition| !definition.secret));
    }
}

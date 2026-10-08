//! The settings registry: every setting this build knows, registered by its owner.
//!
//! The registry has no list of features; a feature adds its definitions at start-up and
//! the foundation treats them like its own (open/closed, FR-068).

use trcli_domain::settings::{DefinitionError, SettingDefinition};

/// Why a definition cannot be registered.
#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]
pub enum RegistryError {
    /// The definition itself is not acceptable.
    #[error(transparent)]
    Definition(#[from] DefinitionError),
    /// The key is registered already.
    #[error("setting `{0}` is already registered")]
    Duplicate(String),
}

/// Every setting definition known to this build.
#[derive(Clone, Debug, Default)]
pub struct SettingsRegistry {
    /// The definitions, in registration order: the order `config list` shows them in.
    definitions: Vec<SettingDefinition>,
}

impl SettingsRegistry {
    /// An empty registry.
    pub fn new() -> Self {
        Self::default()
    }

    /// Registers a definition. A secret setting, an invalid default, and a key registered
    /// twice are all refused.
    pub fn register(&mut self, definition: SettingDefinition) -> Result<(), RegistryError> {
        let definition = definition.checked()?;
        if self.find(definition.key.as_str()).is_some() {
            return Err(RegistryError::Duplicate(definition.key.to_string()));
        }
        self.definitions.push(definition);
        Ok(())
    }

    /// The definition of a key, if registered.
    pub fn find(&self, key: &str) -> Option<&SettingDefinition> {
        self.definitions
            .iter()
            .find(|definition| definition.key.as_str() == key)
    }

    /// Every definition.
    pub fn all(&self) -> &[SettingDefinition] {
        &self.definitions
    }
}

#[cfg(test)]
mod tests {
    //! Unit tests for the settings registry.

    use trcli_domain::settings::{Scope, SettingDefinition, ValueKind};

    use super::{RegistryError, SettingsRegistry};

    /// A yes/no definition with the given key.
    fn definition(key: &str) -> SettingDefinition {
        SettingDefinition::new(key, ValueKind::Boolean, Scope::Both, "A switch.")
            .with_default("true")
    }

    #[test]
    fn a_definition_is_found_once_registered() {
        let mut registry = SettingsRegistry::new();
        registry
            .register(definition("feature.enabled"))
            .expect("registered");
        assert!(registry.find("feature.enabled").is_some());
        assert!(registry.find("feature.other").is_none());
    }

    #[test]
    fn a_key_is_registered_only_once() {
        let mut registry = SettingsRegistry::new();
        registry
            .register(definition("feature.enabled"))
            .expect("registered");
        assert_eq!(
            registry.register(definition("feature.enabled")),
            Err(RegistryError::Duplicate("feature.enabled".into()))
        );
    }

    #[test]
    fn a_secret_definition_is_refused() {
        let mut secret = definition("feature.token");
        secret.secret = true;
        assert!(matches!(
            SettingsRegistry::new().register(secret),
            Err(RegistryError::Definition(_))
        ));
    }
}

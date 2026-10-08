//! Who is acting (FR-050): the name the researcher set for themselves, else the name they
//! are known by on their machine, else the placeholder `unknown`.

use trcli_application::ports::environment::ActorProvider;
use trcli_domain::shared::text::ActorName;

/// The actor, decided once per command.
#[derive(Clone, Debug)]
pub struct SystemActor {
    /// The name under which actions are recorded.
    name: ActorName,
}

impl SystemActor {
    /// Decides the actor from the `researcher.name` setting and the system's variables.
    pub fn new(configured: Option<&str>) -> Self {
        Self::from_environment(configured, |name| std::env::var(name).ok())
    }

    /// As [`SystemActor::new`], given a way to read environment variables.
    pub fn from_environment(
        configured: Option<&str>,
        variable: impl Fn(&str) -> Option<String>,
    ) -> Self {
        // `USER` on Linux and macOS, `USERNAME` on Windows. A value that is not a valid
        // name (empty, or with control characters) is passed over, not recorded.
        let system_user = ["USER", "USERNAME"].into_iter().filter_map(variable);
        let name = configured
            .map(str::to_owned)
            .into_iter()
            .chain(system_user)
            .find_map(|candidate| ActorName::new(&candidate).ok())
            .unwrap_or_else(ActorName::unknown);
        Self { name }
    }
}

impl ActorProvider for SystemActor {
    fn actor(&self) -> ActorName {
        self.name.clone()
    }
}

#[cfg(test)]
mod tests {
    //! Unit tests for the actor (T021).

    use trcli_application::ports::environment::ActorProvider;

    use super::SystemActor;

    /// The actor's name given a setting and the two variables.
    fn actor(configured: Option<&str>, user: Option<&str>, username: Option<&str>) -> String {
        let variable = |name: &str| match name {
            "USER" => user.map(str::to_owned),
            "USERNAME" => username.map(str::to_owned),
            _ => None,
        };
        SystemActor::from_environment(configured, variable)
            .actor()
            .to_string()
    }

    #[test]
    fn the_name_the_researcher_set_comes_first() {
        assert_eq!(actor(Some("Ana Souza"), Some("ana"), None), "Ana Souza");
    }

    #[test]
    fn otherwise_the_system_user_name_is_used() {
        assert_eq!(actor(None, Some("ana"), Some("ANA-PC")), "ana");
        assert_eq!(actor(None, None, Some("ana.souza")), "ana.souza");
        assert_eq!(actor(Some("   "), Some("ana"), None), "ana");
    }

    #[test]
    fn when_nothing_is_known_the_placeholder_is_recorded() {
        assert_eq!(actor(None, None, None), "unknown");
        assert_eq!(actor(None, Some(""), Some("")), "unknown");
        assert!(
            SystemActor::from_environment(None, |_| None)
                .actor()
                .is_unknown()
        );
    }
}

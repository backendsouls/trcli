//! Where the researcher's own settings file is, on each operating system.
//!
//! | System  | Directory                                   |
//! |---------|---------------------------------------------|
//! | Linux   | `$XDG_CONFIG_HOME`, else `~/.config`        |
//! | macOS   | `~/Library/Application Support`             |
//! | Windows | `%APPDATA%`                                 |
//!
//! The file is `trcli/config.toml` inside that directory. These three rules replace a
//! platform-directories library; if a system's edge case is handled badly here, that
//! library is the stated fallback (research.md, section 3).

use std::path::PathBuf;

/// The operating systems whose conventions differ.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Platform {
    /// Linux and other systems that follow the XDG convention.
    Linux,
    /// macOS.
    MacOs,
    /// Windows.
    Windows,
}

impl Platform {
    /// The system this binary was built for.
    pub fn current() -> Self {
        if cfg!(target_os = "windows") {
            Self::Windows
        } else if cfg!(target_os = "macos") {
            Self::MacOs
        } else {
            Self::Linux
        }
    }
}

/// The researcher's settings file, given a way to read environment variables. `None` when
/// the system does not say where the researcher's home is; then there is simply no user
/// file, and defaults apply.
pub fn user_settings_file(
    platform: Platform,
    variable: impl Fn(&str) -> Option<String>,
) -> Option<PathBuf> {
    let set = |name: &str| {
        variable(name)
            .filter(|value| !value.is_empty())
            .map(PathBuf::from)
    };
    let directory = match platform {
        // A relative XDG_CONFIG_HOME is invalid by the convention and is ignored. On
        // Linux "has a root" is "is absolute".
        Platform::Linux => set("XDG_CONFIG_HOME")
            .filter(|path| path.has_root())
            .or_else(|| set("HOME").map(|home| home.join(".config"))),
        Platform::MacOs => set("HOME").map(|home| home.join("Library").join("Application Support")),
        Platform::Windows => set("APPDATA"),
    }?;
    Some(directory.join("trcli").join("config.toml"))
}

/// The researcher's settings file on this system.
pub fn current_user_settings_file() -> Option<PathBuf> {
    user_settings_file(Platform::current(), |name| std::env::var(name).ok())
}

#[cfg(test)]
mod tests {
    //! Unit tests for the platform paths (T036).

    use std::path::PathBuf;

    use super::{Platform, user_settings_file};

    /// An environment holding the given variables.
    fn environment(
        variables: &'static [(&'static str, &'static str)],
    ) -> impl Fn(&str) -> Option<String> {
        move |name| {
            variables
                .iter()
                .find(|(key, _)| *key == name)
                .map(|(_, value)| (*value).to_owned())
        }
    }

    #[test]
    fn linux_uses_xdg_config_home_when_it_is_set_and_absolute() {
        let variables =
            environment(&[("XDG_CONFIG_HOME", "/custom/config"), ("HOME", "/home/ana")]);
        let path = user_settings_file(Platform::Linux, variables);
        assert_eq!(
            path,
            Some(PathBuf::from("/custom/config/trcli/config.toml"))
        );
    }

    #[test]
    fn linux_falls_back_to_dot_config_in_home() {
        for variables in [
            &[("HOME", "/home/ana")][..],
            &[("XDG_CONFIG_HOME", ""), ("HOME", "/home/ana")][..],
        ] {
            let lookup = |name: &str| {
                variables
                    .iter()
                    .find(|(key, _)| *key == name)
                    .map(|(_, v)| (*v).to_owned())
            };
            assert_eq!(
                user_settings_file(Platform::Linux, lookup),
                Some(PathBuf::from("/home/ana/.config/trcli/config.toml"))
            );
        }
        let relative = environment(&[("XDG_CONFIG_HOME", "relative"), ("HOME", "/home/ana")]);
        assert_eq!(
            user_settings_file(Platform::Linux, relative),
            Some(PathBuf::from("/home/ana/.config/trcli/config.toml"))
        );
    }

    #[test]
    fn macos_uses_application_support() {
        let path = user_settings_file(Platform::MacOs, environment(&[("HOME", "/Users/ana")]));
        assert_eq!(
            path,
            Some(PathBuf::from(
                "/Users/ana/Library/Application Support/trcli/config.toml"
            ))
        );
    }

    #[test]
    fn windows_uses_appdata() {
        let variables = environment(&[("APPDATA", "C:\\Users\\ana\\AppData\\Roaming")]);
        let path = user_settings_file(Platform::Windows, variables).expect("a path");
        assert!(path.starts_with("C:\\Users\\ana\\AppData\\Roaming"));
        assert!(path.ends_with(PathBuf::from("trcli").join("config.toml")));
    }

    #[test]
    fn without_a_home_there_is_no_user_file() {
        assert_eq!(user_settings_file(Platform::Linux, environment(&[])), None);
        assert_eq!(
            user_settings_file(Platform::Windows, environment(&[("HOME", "/home/ana")])),
            None
        );
    }
}

//! Colour (FR-029, FR-030): whether to use it, and which style each meaning gets.
//!
//! A [`Theme`] maps meanings — success, warning, error, handle, heading, muted — to
//! styles the researcher can change (`theme.*`). When colour is off, painting returns the
//! text unchanged, so no escape code ever reaches a file or a pipe.
//!
//! Whether colour is on: `--color always` forces it and `--color never` removes it; else
//! `NO_COLOR` (any value) or `CLICOLOR=0` turn it off; else the `output.color` setting
//! decides, where `auto` means "only when the stream is a terminal".

use anstyle::{AnsiColor, Color, Style};
use trcli_application::settings::foundation::THEME_STYLES;
use trcli_application::settings::layers::Settings;
use trcli_domain::settings::StyleSpec;

/// When to use colour, as a flag or a setting says it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ColorChoice {
    /// Only when the stream is a terminal.
    Auto,
    /// Always.
    Always,
    /// Never.
    Never,
}

impl ColorChoice {
    /// The choice with this name; anything unknown is `auto`.
    pub fn named(name: &str) -> Self {
        match name {
            "always" => Self::Always,
            "never" => Self::Never,
            _ => Self::Auto,
        }
    }
}

/// What decides whether a stream gets colour.
#[derive(Clone, Copy, Debug)]
pub struct ColorContext {
    /// `--color`, when given on the command line.
    pub flag: Option<ColorChoice>,
    /// The `output.color` setting from the files and the session.
    pub setting: ColorChoice,
    /// Whether `NO_COLOR` is set, to any value.
    pub no_color: bool,
    /// Whether `CLICOLOR` is `0`.
    pub clicolor_off: bool,
}

impl ColorContext {
    /// Whether a stream gets colour, given whether it is a terminal.
    pub fn enabled(&self, stream_is_terminal: bool) -> bool {
        match self.flag {
            Some(ColorChoice::Always) => return true,
            Some(ColorChoice::Never) => return false,
            Some(ColorChoice::Auto) | None => {}
        }
        // The common conventions for turning colour off win over the setting (FR-030).
        if self.no_color || self.clicolor_off {
            return false;
        }
        match self.setting {
            ColorChoice::Always => true,
            ColorChoice::Never => false,
            ColorChoice::Auto => stream_is_terminal,
        }
    }
}

/// The meanings a theme gives a style to.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Meaning {
    /// Something worked.
    Success,
    /// Something is unusual.
    Warning,
    /// Something failed.
    Error,
    /// The short name of a record.
    Handle,
    /// A heading or a column title.
    Heading,
    /// Secondary text.
    Muted,
}

impl Meaning {
    /// The position of this meaning in [`THEME_STYLES`].
    fn index(self) -> usize {
        match self {
            Self::Success => 0,
            Self::Warning => 1,
            Self::Error => 2,
            Self::Handle => 3,
            Self::Heading => 4,
            Self::Muted => 5,
        }
    }
}

/// The styles in effect for one stream.
#[derive(Clone, Debug)]
pub struct Theme {
    /// Whether colour is on; when off, nothing is painted.
    enabled: bool,
    /// One style per meaning, in the order of [`THEME_STYLES`].
    styles: [Style; 6],
}

impl Theme {
    /// The theme the settings describe, on or off.
    pub fn from_settings(settings: &Settings, enabled: bool) -> Self {
        let mut styles = [Style::new(); 6];
        for (index, (key, default, _)) in THEME_STYLES.iter().enumerate() {
            let spec = StyleSpec::parse(settings.text(key).unwrap_or(default)).unwrap_or_default();
            styles[index] = to_style(spec);
        }
        Self { enabled, styles }
    }

    /// A theme that paints nothing.
    pub fn plain() -> Self {
        Self {
            enabled: false,
            styles: [Style::new(); 6],
        }
    }

    /// Whether colour is on.
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// The text in the style of a meaning; unchanged when colour is off.
    pub fn paint(&self, meaning: Meaning, text: &str) -> String {
        if !self.enabled || text.is_empty() {
            return text.to_owned();
        }
        let style = self.styles[meaning.index()];
        format!("{}{text}{}", style.render(), style.render_reset())
    }
}

/// The terminal style a researcher's style describes.
fn to_style(spec: StyleSpec) -> Style {
    let color = spec.color.map(|name| {
        Color::Ansi(match name {
            "black" => AnsiColor::Black,
            "red" => AnsiColor::Red,
            "green" => AnsiColor::Green,
            "yellow" => AnsiColor::Yellow,
            "blue" => AnsiColor::Blue,
            "magenta" => AnsiColor::Magenta,
            "cyan" => AnsiColor::Cyan,
            _ => AnsiColor::White,
        })
    });
    let mut style = Style::new().fg_color(color);
    if spec.bold {
        style = style.bold();
    }
    if spec.dim {
        style = style.dimmed();
    }
    if spec.underline {
        style = style.underline();
    }
    style
}

#[cfg(test)]
mod tests {
    //! Unit tests for colour selection (T017).

    use trcli_application::settings::layers::Settings;

    use super::{ColorChoice, ColorContext, Meaning, Theme};

    /// A context with nothing set: the setting is `auto`.
    fn context() -> ColorContext {
        ColorContext {
            flag: None,
            setting: ColorChoice::Auto,
            no_color: false,
            clicolor_off: false,
        }
    }

    #[test]
    fn auto_gives_colour_only_on_a_terminal() {
        assert!(context().enabled(true));
        assert!(!context().enabled(false));
    }

    #[test]
    fn no_color_and_clicolor_zero_turn_colour_off_even_on_a_terminal() {
        assert!(
            !ColorContext {
                no_color: true,
                ..context()
            }
            .enabled(true)
        );
        assert!(
            !ColorContext {
                clicolor_off: true,
                ..context()
            }
            .enabled(true)
        );
        let setting_always = ColorContext {
            setting: ColorChoice::Always,
            no_color: true,
            ..context()
        };
        assert!(
            !setting_always.enabled(true),
            "the conventions win over the setting"
        );
    }

    #[test]
    fn always_forces_colour_and_never_removes_it() {
        let forced = ColorContext {
            flag: Some(ColorChoice::Always),
            no_color: true,
            ..context()
        };
        assert!(
            forced.enabled(false),
            "--color always wins over NO_COLOR and over a pipe"
        );
        let removed = ColorContext {
            flag: Some(ColorChoice::Never),
            setting: ColorChoice::Always,
            ..context()
        };
        assert!(!removed.enabled(true));
        assert!(
            ColorContext {
                setting: ColorChoice::Always,
                ..context()
            }
            .enabled(false)
        );
        assert!(
            !ColorContext {
                setting: ColorChoice::Never,
                ..context()
            }
            .enabled(true)
        );
    }

    #[test]
    fn painting_adds_codes_only_when_colour_is_on() {
        let on = Theme::from_settings(&Settings::default(), true);
        let painted = on.paint(Meaning::Error, "failed");
        assert!(
            painted.starts_with("\u{1b}[")
                && painted.contains("failed")
                && painted.ends_with("\u{1b}[0m")
        );
        let off = Theme::from_settings(&Settings::default(), false);
        assert_eq!(off.paint(Meaning::Error, "failed"), "failed");
        assert_eq!(
            Theme::plain().paint(Meaning::Handle, "spc-7k3f"),
            "spc-7k3f"
        );
    }

    #[test]
    fn a_choice_is_read_from_its_name() {
        assert_eq!(ColorChoice::named("always"), ColorChoice::Always);
        assert_eq!(ColorChoice::named("never"), ColorChoice::Never);
        assert_eq!(ColorChoice::named("auto"), ColorChoice::Auto);
    }
}

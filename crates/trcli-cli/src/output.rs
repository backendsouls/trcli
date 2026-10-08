//! Where results and messages go (FR-028, FR-031).
//!
//! Results go to standard output and nothing else does; problems, warnings, remarks, and
//! diagnostics go to standard error. With `--output json`, standard output carries one
//! JSON document for success and for failure alike, so that a script reads one place and
//! tells them apart by the exit code.

use std::io::{IsTerminal, Write};

use time::UtcOffset;
use trcli_application::outcome::Problem;
use trcli_application::settings::foundation::{OUTPUT_COLOR, OUTPUT_FORMAT, OUTPUT_SYMBOLS};
use trcli_application::settings::layers::Settings;
use trcli_domain::shared::problem::Warning;

use crate::args::global::GlobalArgs;
use crate::render::human::Human;
use crate::render::symbols::SymbolSet;
use crate::render::theme::{ColorChoice, ColorContext, Theme};
use crate::render::width::{reported_width, terminal_width};
use crate::render::{View, json, problem};

/// What a command hands back to be shown.
pub struct Reply {
    /// The result, in both of its forms.
    pub view: Box<dyn View>,
    /// What was unusual but allowed.
    pub warnings: Vec<Warning>,
    /// Remarks for the researcher that are not part of the result.
    pub notes: Vec<String>,
}

impl Reply {
    /// A reply that shows a view.
    pub fn new(view: impl View + 'static) -> Self {
        Self { view: Box::new(view), warnings: Vec::new(), notes: Vec::new() }
    }

    /// Adds warnings.
    #[must_use]
    pub fn with_warnings(mut self, warnings: Vec<Warning>) -> Self {
        self.warnings.extend(warnings);
        self
    }

    /// Adds a remark.
    #[must_use]
    pub fn with_note(mut self, note: impl Into<String>) -> Self {
        self.notes.push(note.into());
        self
    }
}

/// How this command's output is presented.
#[derive(Clone, Debug)]
pub struct Presentation {
    /// Whether the structured form was asked for.
    pub json: bool,
    /// Whether only results and errors are wanted.
    pub quiet: bool,
    /// The styles for standard output.
    pub out_theme: Theme,
    /// The styles for standard error.
    pub err_theme: Theme,
    /// The special characters in effect.
    pub symbols: SymbolSet,
    /// The width output is laid out for.
    pub width: usize,
    /// The researcher's offset from UTC.
    pub zone: UtcOffset,
}

impl Presentation {
    /// The presentation the settings in effect ask for.
    pub fn new(global: &GlobalArgs, settings: &Settings, zone: UtcOffset) -> Self {
        let context = ColorContext {
            flag: global.color.as_deref().map(ColorChoice::named),
            setting: ColorChoice::named(settings.text(OUTPUT_COLOR).unwrap_or("auto")),
            no_color: std::env::var_os("NO_COLOR").is_some(),
            clicolor_off: std::env::var("CLICOLOR").is_ok_and(|value| value == "0"),
        };
        let json = settings.text(OUTPUT_FORMAT) == Some("json");
        Self {
            json,
            quiet: global.quiet,
            // The structured form is never coloured, whatever was asked.
            out_theme: Theme::from_settings(settings, !json && context.enabled(std::io::stdout().is_terminal())),
            err_theme: Theme::from_settings(settings, context.enabled(std::io::stderr().is_terminal())),
            symbols: SymbolSet::named(settings.text(OUTPUT_SYMBOLS).unwrap_or("unicode")),
            width: terminal_width(std::env::var("COLUMNS").ok().as_deref(), reported_width()),
            zone,
        }
    }

    /// The presentation used before the settings are known: what the command line and
    /// the built-in defaults say.
    pub fn before_settings(global: &GlobalArgs, zone: UtcOffset) -> Self {
        let mut presentation = Self::new(global, &Settings::default(), zone);
        presentation.json = global.output.as_deref() == Some("json");
        presentation
    }

    /// Shows the result of a command that succeeded.
    pub fn show(&self, reply: &Reply) {
        if self.json {
            write_out(&json::success(reply.view.to_json(), &reply.warnings));
        } else {
            let mut page = Human::new(&self.out_theme, self.symbols, self.width, self.zone);
            reply.view.draw(&mut page);
            let (body, notices) = page.finish();
            write_out(&body);
            self.remark(&notices);
        }
        self.warn(&reply.warnings);
        self.remark(&reply.notes);
    }

    /// Shows a failure.
    pub fn fail(&self, failure: &Problem) {
        if self.json {
            // Also on standard output, so that a script reads one place; a plain line
            // still tells a person watching what happened.
            write_out(&json::failure(failure));
            write_error(&format!("error: {}\n", failure.message));
        } else {
            write_error(&problem::render(failure, &self.err_theme));
        }
    }

    /// Shows warnings on standard error, unless only results and errors are wanted.
    pub fn warn(&self, warnings: &[Warning]) {
        if self.quiet {
            return;
        }
        for warning in warnings {
            write_error(&format!("{}\n", problem::render_warning(warning, &self.err_theme)));
        }
    }

    /// Shows remarks on standard error, unless only results and errors are wanted.
    fn remark(&self, remarks: &[String]) {
        if self.quiet {
            return;
        }
        for remark in remarks {
            write_error(&format!("{remark}\n"));
        }
    }
}

/// Writes to standard output. A reader that stopped early is not an error: the tool ends
/// quietly (spec, edge cases).
pub fn write_out(text: &str) {
    let mut out = std::io::stdout().lock();
    let _ = out.write_all(text.as_bytes());
    let _ = out.flush();
}

/// Writes to standard error, likewise without failing when nobody reads.
pub fn write_error(text: &str) {
    let mut error = std::io::stderr().lock();
    let _ = error.write_all(text.as_bytes());
    let _ = error.flush();
}

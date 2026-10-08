//! Views that belong to no one command group: a plain message, a block of text, and the
//! result of a verification.

use serde::Serialize;
use trcli_application::view::Done;

use crate::render::Render;
use crate::render::human::Human;

impl Render for Done {
    fn render(&self, out: &mut Human) {
        out.success(&self.message);
    }
}

/// A block of text shown as it is: help, a completion script.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct Text {
    /// The text.
    pub text: String,
}

impl Render for Text {
    fn render(&self, out: &mut Human) {
        out.line(self.text.trim_end_matches('\n'));
    }
}

/// The result of `audit verify` when the trail is intact.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct Verified {
    /// Always `true`: a trail that is not intact is a problem, not a view.
    pub intact: bool,
    /// How many entries were verified.
    pub entries: u64,
}

impl Render for Verified {
    fn render(&self, out: &mut Human) {
        let check = out.symbols.check;
        out.success(&format!("{check} The audit trail is intact: {} entries verified.", self.entries));
    }
}

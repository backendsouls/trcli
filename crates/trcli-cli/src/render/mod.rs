//! Presentation: turning what a use case returns into what the researcher sees
//! (FR-028 to FR-031, FR-038).
//!
//! A use case returns a *view model*: a plain value that can be serialized. This module
//! gives each one a second form, for people, through the [`Render`] trait. Both forms come
//! from the same value, so their content cannot drift apart (SC-005).
//!
//! - [`human`]: the layout helpers a view draws itself with.
//! - [`json`]: the one envelope of the structured form.
//! - [`theme`], [`symbols`], [`width`]: colour, special characters, and fitting the terminal.
//! - [`problem`]: how a failure is shown, in both forms.
//! - [`export`]: the audit trail as Markdown, JSON, or CSV.
//! - [`views`]: the form for people of every view model.
//!
//! Colour and symbols are never the only carrier of meaning: everything is also in words.

pub mod export;
pub mod human;
pub mod json;
pub mod problem;
pub mod symbols;
pub mod theme;
pub mod views;
pub mod width;

use serde::Serialize;

use self::human::Human;

/// The form for people of a view model.
pub trait Render {
    /// Draws the view.
    fn render(&self, out: &mut Human);
}

/// A view model in both of its forms. Implemented for everything that is both
/// [`Render`] and `Serialize`, so a handler can return any view model as one type.
pub trait View {
    /// Draws the form for people.
    fn draw(&self, out: &mut Human);

    /// The structured form.
    fn to_json(&self) -> serde_json::Value;
}

impl<T: Render + Serialize> View for T {
    fn draw(&self, out: &mut Human) {
        self.render(out);
    }

    fn to_json(&self) -> serde_json::Value {
        // A view model holds only text, numbers, and lists of them: it always serializes.
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
}

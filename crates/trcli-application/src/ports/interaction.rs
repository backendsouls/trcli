//! Ports for the person at the terminal: questions and progress (FR-035, FR-036).
//!
//! No use case reads standard input or draws on the terminal itself. A question goes
//! through [`Prompter`], which answers at once when nobody can be asked; long work reports
//! through [`Progress`], which shows nothing when nobody is watching.

/// The answer to a yes/no question.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Confirmation {
    /// The researcher said yes, or said beforehand that the answer is yes.
    Yes,
    /// The researcher said anything else. The default answer is always no.
    No,
    /// Nobody can be asked: there is no terminal, or prompts were turned off.
    CannotAsk,
}

/// Asks the researcher to confirm something.
pub trait Prompter {
    /// Asks a yes/no question. Never waits when nobody can answer (SC-007).
    async fn confirm(&mut self, question: &str) -> Confirmation;
}

/// Shows that long work is progressing.
pub trait Progress {
    /// Work described by `label` has started.
    fn start(&mut self, label: &str);

    /// `done` units are finished, out of `total` when the total is known.
    fn advance(&mut self, done: u64, total: Option<u64>);

    /// The work is over; any indication is removed.
    fn finish(&mut self);
}

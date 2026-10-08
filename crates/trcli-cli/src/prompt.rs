//! Asking the researcher to confirm something (FR-035).
//!
//! There are three ways to "ask", chosen once at start-up: ask at the terminal; assume
//! yes, because the researcher said so beforehand (`--yes`); or refuse to ask, because
//! nobody can answer (no terminal, or `--no-input`). The third answers at once, so no
//! command ever waits for an answer nobody can give (SC-007).
//!
//! This is the only code that reads standard input.

use std::io::{BufRead, Write};

use trcli_application::ports::interaction::{Confirmation, Prompter};

/// How questions are answered for one command.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TerminalPrompter {
    /// Ask at the terminal; anything but yes is no.
    Ask,
    /// Answer yes without asking.
    AssumeYes,
    /// Do not ask: say that nobody can be asked.
    Refuse,
}

impl TerminalPrompter {
    /// Chooses how to answer from `--yes`, `--no-input`, and whether standard input is a
    /// terminal. Saying yes beforehand wins: it is the one way to confirm where nobody
    /// can be asked.
    pub fn choose(yes: bool, no_input: bool, input_is_terminal: bool) -> Self {
        if yes {
            Self::AssumeYes
        } else if no_input || !input_is_terminal {
            Self::Refuse
        } else {
            Self::Ask
        }
    }

    /// Reads what was typed as an answer: only `y` or `yes`, in any letter case, is yes.
    /// No answer at all (the input ended) is no.
    pub fn interpret(answer: Option<&str>) -> Confirmation {
        match answer.map(|answer| answer.trim().to_lowercase()) {
            Some(answer) if answer == "y" || answer == "yes" => Confirmation::Yes,
            _ => Confirmation::No,
        }
    }
}

/// Writes the question to standard error and reads one line from standard input.
fn ask_at_terminal(question: String) -> Option<String> {
    let mut error = std::io::stderr().lock();
    // The default answer is No, and the prompt says so.
    let _ = write!(error, "{question} [y/N] ");
    let _ = error.flush();
    let mut answer = String::new();
    match std::io::stdin().lock().read_line(&mut answer) {
        Ok(read) if read > 0 => Some(answer),
        _ => None,
    }
}

impl Prompter for TerminalPrompter {
    async fn confirm(&mut self, question: &str) -> Confirmation {
        match self {
            Self::AssumeYes => Confirmation::Yes,
            Self::Refuse => Confirmation::CannotAsk,
            Self::Ask => {
                // Reading blocks, so it is done off the runtime's thread: Ctrl-C can then
                // still be noticed while the question is open.
                let question = question.to_owned();
                let answer = tokio::task::spawn_blocking(move || ask_at_terminal(question)).await.ok().flatten();
                Self::interpret(answer.as_deref())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    //! Unit tests for the three ways of asking (T086).

    use trcli_application::ports::interaction::{Confirmation, Prompter};

    use super::TerminalPrompter;

    /// Runs a future on a small runtime.
    fn block_on<F: std::future::Future>(future: F) -> F::Output {
        tokio::runtime::Builder::new_current_thread().build().expect("a runtime").block_on(future)
    }

    #[test]
    fn at_a_terminal_anything_but_yes_is_no() {
        for yes in ["y", "Y", "yes", " YES \n"] {
            assert_eq!(TerminalPrompter::interpret(Some(yes)), Confirmation::Yes, "{yes:?}");
        }
        for no in ["", "\n", "n", "no", "yep", "sure", "1"] {
            assert_eq!(TerminalPrompter::interpret(Some(no)), Confirmation::No, "{no:?}");
        }
        assert_eq!(TerminalPrompter::interpret(None), Confirmation::No);
    }

    #[test]
    fn assume_yes_answers_yes_without_asking() {
        let mut prompter = TerminalPrompter::choose(true, false, false);
        assert_eq!(prompter, TerminalPrompter::AssumeYes);
        assert_eq!(block_on(prompter.confirm("Delete?")), Confirmation::Yes);
    }

    #[test]
    fn refuse_answers_at_once_without_reading_input() {
        let mut without_terminal = TerminalPrompter::choose(false, false, false);
        let mut no_input = TerminalPrompter::choose(false, true, true);
        assert_eq!((without_terminal, no_input), (TerminalPrompter::Refuse, TerminalPrompter::Refuse));
        // If either read standard input, this test would hang.
        assert_eq!(block_on(without_terminal.confirm("Delete?")), Confirmation::CannotAsk);
        assert_eq!(block_on(no_input.confirm("Delete?")), Confirmation::CannotAsk);
    }

    #[test]
    fn a_terminal_without_flags_is_asked_and_yes_beforehand_wins_over_no_input() {
        assert_eq!(TerminalPrompter::choose(false, false, true), TerminalPrompter::Ask);
        assert_eq!(TerminalPrompter::choose(true, true, false), TerminalPrompter::AssumeYes);
    }
}

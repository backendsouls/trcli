//! Fakes of the person at the terminal.

use trcli_application::ports::interaction::{Confirmation, Progress, Prompter};

/// A prompter that gives a prepared answer and remembers what it was asked.
#[derive(Clone, Debug)]
pub struct ScriptedPrompter {
    /// The answer to every question.
    answer: Confirmation,
    /// The questions asked, in order.
    pub questions: Vec<String>,
}

impl ScriptedPrompter {
    /// A prompter that always gives `answer`.
    pub fn answering(answer: Confirmation) -> Self {
        Self {
            answer,
            questions: Vec::new(),
        }
    }
}

impl Prompter for ScriptedPrompter {
    async fn confirm(&mut self, question: &str) -> Confirmation {
        self.questions.push(question.to_owned());
        self.answer
    }
}

/// A progress indication that only remembers what it was told.
#[derive(Clone, Debug, Default)]
pub struct RecordingProgress {
    /// The labels of the work started.
    pub started: Vec<String>,
    /// How many times progress advanced.
    pub advances: u64,
    /// Whether the work was reported finished.
    pub finished: bool,
}

impl Progress for RecordingProgress {
    fn start(&mut self, label: &str) {
        self.started.push(label.to_owned());
    }

    fn advance(&mut self, _done: u64, _total: Option<u64>) {
        self.advances += 1;
    }

    fn finish(&mut self) {
        self.finished = true;
    }
}

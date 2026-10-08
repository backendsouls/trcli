//! Showing that long work is progressing (FR-036).
//!
//! The indication is a spinner and a counter on standard error. It appears only when
//! standard error is a terminal, only once the work has lasted about a fifth of a second
//! (so quick commands never flicker), and it erases itself when the work is done.

use std::io::Write;
use std::time::{Duration, Instant};

use trcli_application::ports::interaction::Progress;

use crate::render::symbols::SymbolSet;

/// How long work must last before anything is drawn.
pub const DELAY: Duration = Duration::from_millis(200);

/// The indication of progress on a terminal.
#[derive(Debug)]
pub struct TerminalProgress<W: Write> {
    /// Where the indication is drawn; standard error in the tool.
    out: W,
    /// Whether anything may be drawn: the stream is a terminal and `--quiet` was not given.
    enabled: bool,
    /// The frames of the spinner.
    symbols: SymbolSet,
    /// What the work is, and when it started; `None` when no work is in progress.
    work: Option<(String, Instant)>,
    /// How many times the indication was drawn; picks the spinner's frame.
    frames: usize,
}

impl<W: Write> TerminalProgress<W> {
    /// An indication on `out`, drawn only when `enabled`.
    pub fn new(out: W, enabled: bool, symbols: SymbolSet) -> Self {
        Self {
            out,
            enabled,
            symbols,
            work: None,
            frames: 0,
        }
    }

    /// Draws the indication for work that has lasted `elapsed`.
    fn draw(&mut self, elapsed: Duration, done: u64, total: Option<u64>) {
        let Some((label, _)) = &self.work else { return };
        if !self.enabled || elapsed < DELAY {
            return;
        }
        let frame = self.symbols.spinner[self.frames % self.symbols.spinner.len()];
        let counter = match total {
            Some(total) => format!(" {done}/{total}"),
            None => format!(" {done}"),
        };
        // A carriage return redraws over the previous indication.
        let _ = write!(self.out, "\r{frame} {label}{counter}");
        let _ = self.out.flush();
        self.frames += 1;
    }

    /// Erases whatever was drawn.
    fn erase(&mut self) {
        if self.frames > 0 {
            // Return to the start of the line and clear it.
            let _ = write!(self.out, "\r\u{1b}[2K");
            let _ = self.out.flush();
            self.frames = 0;
        }
    }
}

impl<W: Write> Progress for TerminalProgress<W> {
    fn start(&mut self, label: &str) {
        self.work = Some((label.to_owned(), Instant::now()));
    }

    fn advance(&mut self, done: u64, total: Option<u64>) {
        let elapsed = self
            .work
            .as_ref()
            .map_or(Duration::ZERO, |(_, started)| started.elapsed());
        self.draw(elapsed, done, total);
    }

    fn finish(&mut self) {
        self.erase();
        self.work = None;
    }
}

#[cfg(test)]
mod tests {
    //! Unit tests for the indication of progress (T087).

    use std::time::Duration;

    use trcli_application::ports::interaction::Progress;

    use super::{DELAY, TerminalProgress};
    use crate::render::symbols::SymbolSet;

    /// An indication that draws into memory.
    fn progress(enabled: bool) -> TerminalProgress<Vec<u8>> {
        TerminalProgress::new(Vec::new(), enabled, SymbolSet::ASCII)
    }

    #[test]
    fn nothing_is_drawn_when_the_stream_is_not_a_terminal() {
        let mut progress = progress(false);
        progress.start("Verifying");
        progress.draw(Duration::from_secs(5), 10, Some(100));
        progress.finish();
        assert!(progress.out.is_empty());
    }

    #[test]
    fn nothing_is_drawn_before_the_delay() {
        let mut progress = progress(true);
        progress.start("Verifying");
        progress.draw(DELAY - Duration::from_millis(1), 1, Some(100));
        assert!(progress.out.is_empty());
        progress.advance(1, Some(100));
        assert!(
            progress.out.is_empty(),
            "work that has just started draws nothing"
        );
    }

    #[test]
    fn after_the_delay_a_spinner_and_a_counter_are_drawn() {
        let mut progress = progress(true);
        progress.start("Verifying");
        progress.draw(DELAY, 10, Some(100));
        progress.draw(DELAY, 20, None);
        assert_eq!(
            String::from_utf8_lossy(&progress.out),
            "\r| Verifying 10/100\r/ Verifying 20"
        );
    }

    #[test]
    fn the_indication_is_erased_when_done() {
        let mut progress = progress(true);
        progress.start("Verifying");
        progress.draw(DELAY, 10, Some(100));
        progress.finish();
        assert!(String::from_utf8_lossy(&progress.out).ends_with("\r\u{1b}[2K"));

        let mut quick = super::TerminalProgress::new(Vec::new(), true, SymbolSet::ASCII);
        quick.start("Quick work");
        quick.finish();
        assert!(
            quick.out.is_empty(),
            "work that drew nothing erases nothing"
        );
    }
}

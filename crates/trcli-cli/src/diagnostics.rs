//! Diagnostic detail for `--verbose` (FR-031): plain lines on standard error, written
//! only when asked for. This small writer stands in for a logging framework.

/// Writes diagnostic lines when the researcher asked for them.
#[derive(Clone, Copy, Debug, Default)]
pub struct Diagnostics {
    /// How many times `--verbose` was given.
    level: u8,
}

impl Diagnostics {
    /// Diagnostics at the given level; 0 writes nothing.
    pub fn new(level: u8) -> Self {
        Self { level }
    }

    /// Whether anything is written.
    pub fn is_on(&self) -> bool {
        self.level > 0
    }

    /// Writes one line, when diagnostics are on. The text is built only then.
    pub fn line(&self, text: impl FnOnce() -> String) {
        if self.is_on() {
            eprintln!("debug: {}", text());
        }
    }
}

#[cfg(test)]
mod tests {
    //! Unit tests for the diagnostics writer.

    use super::Diagnostics;

    #[test]
    fn nothing_is_built_or_written_unless_asked() {
        Diagnostics::new(0).line(|| panic!("the text must not be built when diagnostics are off"));
        assert!(Diagnostics::new(1).is_on());
    }
}

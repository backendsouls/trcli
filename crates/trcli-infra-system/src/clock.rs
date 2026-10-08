//! The clock (FR-071).
//!
//! In a build with the `test-clock` feature — used only to run behaviour scenarios — the
//! time can be fixed by the `TRCLI_TEST_NOW` variable (RFC 3339), so that scenarios give
//! the same output on every run. A release build has no such variable.

use time::OffsetDateTime;
use trcli_application::ports::environment::Clock;

/// The machine's clock.
#[derive(Clone, Copy, Debug, Default)]
pub struct SystemClock;

impl SystemClock {
    /// The time fixed by the environment, when this build allows it and it is set.
    #[cfg(feature = "test-clock")]
    fn fixed() -> Option<OffsetDateTime> {
        let text = std::env::var("TRCLI_TEST_NOW").ok()?;
        OffsetDateTime::parse(&text, &time::format_description::well_known::Rfc3339).ok()
    }

    /// A release build never takes the time from the environment.
    #[cfg(not(feature = "test-clock"))]
    fn fixed() -> Option<OffsetDateTime> {
        None
    }
}

impl Clock for SystemClock {
    fn now(&self) -> OffsetDateTime {
        Self::fixed().unwrap_or_else(OffsetDateTime::now_utc)
    }
}

#[cfg(test)]
mod tests {
    //! Unit tests for the clock.

    use time::OffsetDateTime;
    use trcli_application::ports::environment::Clock;

    use super::SystemClock;

    #[test]
    fn the_clock_tells_the_present() {
        let before = OffsetDateTime::now_utc();
        let now = SystemClock.now();
        // With the test clock fixed by the environment this test says nothing; without it
        // the clock must not run backwards.
        if std::env::var("TRCLI_TEST_NOW").is_err() {
            assert!(now >= before);
        }
    }
}

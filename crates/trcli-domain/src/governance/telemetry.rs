//! Local telemetry: what the tool records about its own use (FR-052, FR-053).
//!
//! A record says which command ran, how long it took, and how it ended. It never holds
//! the command's arguments or any value, and nothing here can send it anywhere.

use time::OffsetDateTime;

/// One use of one command.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TelemetryRecord {
    /// When the command ran.
    pub at: OffsetDateTime,
    /// The command path without its arguments, for example `workspace show`.
    pub command: String,
    /// How long it took, in milliseconds.
    pub duration_ms: u64,
    /// How it ended, as the stable name of the outcome, for example `success`.
    pub outcome: String,
}

impl TelemetryRecord {
    /// Builds a record from the words of the command path.
    ///
    /// Only the path is taken; a caller cannot pass arguments by mistake because anything
    /// that is not a plain command word is dropped (FR-054).
    pub fn new(at: OffsetDateTime, path: &[&str], duration_ms: u64, outcome: &str) -> Self {
        let command = path
            .iter()
            .copied()
            .filter(|word| Self::is_command_word(word))
            .collect::<Vec<_>>()
            .join(" ");
        Self {
            at,
            command,
            duration_ms,
            outcome: outcome.to_owned(),
        }
    }

    /// Whether a word can be the name of a command: it starts with a lower-case letter
    /// (so an option such as `--name` is not one) and continues with letters and hyphens.
    fn is_command_word(word: &str) -> bool {
        word.starts_with(|character: char| character.is_ascii_lowercase())
            && word
                .chars()
                .all(|character| character.is_ascii_lowercase() || character == '-')
    }
}

#[cfg(test)]
mod tests {
    //! Unit tests for telemetry records.

    use time::OffsetDateTime;

    use super::TelemetryRecord;

    #[test]
    fn only_the_command_path_is_recorded() {
        let record = TelemetryRecord::new(
            OffsetDateTime::UNIX_EPOCH,
            &["workspace", "show"],
            12,
            "success",
        );
        assert_eq!(record.command, "workspace show");
    }

    #[test]
    fn anything_that_is_not_a_command_word_is_dropped() {
        let record = TelemetryRecord::new(
            OffsetDateTime::UNIX_EPOCH,
            &["init", "--name", "My Secret Project"],
            3,
            "success",
        );
        assert_eq!(record.command, "init");
    }
}

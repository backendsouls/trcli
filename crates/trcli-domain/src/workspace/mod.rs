//! The workspace: the one place a researcher's records are kept (FR-001, FR-007, FR-008).
//!
//! Contains the workspace's own details, the version of the format it is stored in, and
//! the rule that decides what may be done with a workspace of a given format. Where a
//! workspace is, and how it is found, is not known here (FR-005).

use time::OffsetDateTime;

use crate::shared::record::RecordId;
use crate::shared::text::{LongText, Name};

/// The version of the format a workspace is stored in: a positive whole number that
/// increases with each released change to storage (FR-007).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FormatVersion(u32);

impl FormatVersion {
    /// The format this version of the tool reads and writes.
    ///
    /// It is raised when a release changes how workspaces are stored; migrations added
    /// between releases belong to the next format.
    pub const CURRENT: FormatVersion = FormatVersion(1);

    /// Wraps a stored number. Zero is kept as it is: it can only come from a workspace
    /// older than the first release, and is then simply "older than known".
    pub fn new(number: u32) -> Self {
        Self(number)
    }

    /// The number itself.
    pub fn number(self) -> u32 {
        self.0
    }
}

impl std::fmt::Display for FormatVersion {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}", self.0)
    }
}

/// What may be done with a workspace, decided when it is opened.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum OpeningState {
    /// The format is the one this tool knows: everything may be done.
    Usable,
    /// The format is older: reading works; nothing may change until it is upgraded.
    NeedsUpgrade {
        /// The format the workspace is stored in.
        stored: FormatVersion,
        /// The format this tool reads and writes.
        known: FormatVersion,
    },
    /// The format is newer than this tool knows: nothing may be changed.
    TooNew {
        /// The format the workspace is stored in.
        stored: FormatVersion,
        /// The newest format this tool knows.
        known: FormatVersion,
    },
    /// The stored data cannot be opened or fails its check: nothing is written.
    Damaged {
        /// What is wrong.
        what: String,
        /// Where: the file or table concerned.
        location: String,
    },
}

impl OpeningState {
    /// Compares the stored format with the one this tool knows.
    pub fn of(stored: FormatVersion, known: FormatVersion) -> Self {
        match stored.cmp(&known) {
            std::cmp::Ordering::Equal => Self::Usable,
            std::cmp::Ordering::Less => Self::NeedsUpgrade { stored, known },
            std::cmp::Ordering::Greater => Self::TooNew { stored, known },
        }
    }

    /// Whether commands that only read may run.
    pub fn allows_reading(&self) -> bool {
        matches!(self, Self::Usable | Self::NeedsUpgrade { .. })
    }

    /// Whether commands that change something may run.
    pub fn allows_writing(&self) -> bool {
        matches!(self, Self::Usable)
    }
}

/// A workspace's own details: the one row every workspace has.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Workspace {
    /// Identifies the workspace across copies of itself.
    pub id: RecordId,
    /// What the researcher calls it; required (FR-001).
    pub name: Name,
    /// What it is for; may be empty.
    pub description: LongText,
    /// The format it is stored in.
    pub format_version: FormatVersion,
    /// When it was created.
    pub created_at: OffsetDateTime,
}

impl Workspace {
    /// A new workspace in the current format.
    pub fn new(
        id: RecordId,
        name: Name,
        description: LongText,
        created_at: OffsetDateTime,
    ) -> Self {
        Self {
            id,
            name,
            description,
            format_version: FormatVersion::CURRENT,
            created_at,
        }
    }

    /// What may be done with this workspace by a tool that knows `known`.
    pub fn opening_state(&self, known: FormatVersion) -> OpeningState {
        OpeningState::of(self.format_version, known)
    }
}

#[cfg(test)]
mod tests {
    //! Unit tests for the workspace's opening states (T046).

    use super::{FormatVersion, OpeningState};
    use crate::shared::text::Name;

    #[test]
    fn a_workspace_in_the_known_format_is_usable() {
        let state = OpeningState::of(FormatVersion::new(3), FormatVersion::new(3));
        assert_eq!(state, OpeningState::Usable);
        assert!(state.allows_reading() && state.allows_writing());
    }

    #[test]
    fn an_older_workspace_can_be_read_and_not_changed() {
        let state = OpeningState::of(FormatVersion::new(2), FormatVersion::new(3));
        assert!(matches!(state, OpeningState::NeedsUpgrade { .. }));
        assert!(state.allows_reading());
        assert!(!state.allows_writing());
    }

    #[test]
    fn a_newer_workspace_may_not_be_changed() {
        let state = OpeningState::of(FormatVersion::new(4), FormatVersion::new(3));
        assert!(matches!(state, OpeningState::TooNew { .. }));
        assert!(!state.allows_writing());
    }

    #[test]
    fn a_damaged_workspace_allows_nothing() {
        let state = OpeningState::Damaged {
            what: "x".into(),
            location: "y".into(),
        };
        assert!(!state.allows_reading() && !state.allows_writing());
    }

    #[test]
    fn a_workspace_name_is_required_and_at_most_two_hundred_characters() {
        assert!(Name::new("").is_err());
        assert!(Name::new("Doctorate").is_ok());
        assert!(Name::new(&"d".repeat(201)).is_err());
    }
}

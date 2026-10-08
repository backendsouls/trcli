//! The TRCLI system adapter: everything that differs between operating systems, or that
//! touches the file system, the environment, or the clock.
//!
//! - [`paths`]: where the researcher's own settings file is on each system.
//! - [`locator`]: whether a directory holds a workspace, and the workspace's own files.
//! - [`settings_files`]: reading and writing settings as TOML, and the session's variables.
//! - [`clock`], [`ids`], [`actor`]: the time, new identifiers, and who is acting.
//! - [`audit_head`]: the end of the audit trail, kept beside the database.
//! - [`backup_copy`]: the copy of the database taken before an upgrade.
//! - [`atomic`]: writing a file so that it is never left half-written (FR-066).
//!
//! It deliberately knows nothing about SQL, the command line, or rendering. With the
//! `test-clock` feature, the clock and the identifiers can be fixed from the environment;
//! no release build enables it.

// Tests are straight lists of steps and assertions; see the application crate's note.
#![cfg_attr(test, allow(clippy::cognitive_complexity, clippy::too_many_lines))]

pub mod actor;
pub mod atomic;
pub mod audit_head;
pub mod backup_copy;
pub mod clock;
pub mod ids;
pub mod locator;
pub mod paths;
pub mod settings_files;

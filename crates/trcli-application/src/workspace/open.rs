//! The guard at the door: what may be done with a workspace of a given format (FR-007,
//! FR-008).
//!
//! It runs before any handler, so a handler is never called with a workspace that is too
//! new, needs upgrading (when it would change something), or is damaged.

use trcli_domain::workspace::OpeningState;

use crate::outcome::{Problem, codes};

/// What a command does to the workspace.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Access {
    /// It only reads.
    Read,
    /// It changes something.
    Write,
}

/// Lets a command through, or says why the workspace cannot be used that way.
pub fn guard(state: &OpeningState, access: Access) -> Result<(), Problem> {
    match state {
        OpeningState::Usable => Ok(()),
        OpeningState::NeedsUpgrade { .. } if access == Access::Read => Ok(()),
        OpeningState::NeedsUpgrade { stored, known } => Err(Problem::new(
            codes::WORKSPACE_NEEDS_UPGRADE,
            format!("this workspace is in format {stored}; this version of trcli writes format {known}"),
        )
        .with_next_step("run `trcli workspace upgrade` first; a copy of the workspace is kept before upgrading")),
        OpeningState::TooNew { stored, known } => Err(Problem::new(
            codes::WORKSPACE_TOO_NEW,
            format!(
                "this workspace was made by a newer version of trcli (format {stored}; this version knows up to {known})"
            ),
        )
        .with_next_step("install a newer version of trcli")),
        OpeningState::Damaged { what, location } => {
            Err(Problem::new(codes::WORKSPACE_DAMAGED, format!("the workspace is damaged: {what}"))
                .with_items(vec![format!("where: {location}")])
                .with_next_step("restore a copy from `.trcli/backups/` if there is one; nothing was written"))
        }
    }
}

/// A note for the researcher when a reading command ran on a workspace that needs an
/// upgrade; `None` otherwise.
pub fn upgrade_notice(state: &OpeningState) -> Option<String> {
    match state {
        OpeningState::NeedsUpgrade { stored, known } => Some(format!(
            "this workspace is in format {stored} and needs an upgrade to format {known}: run `trcli workspace upgrade`"
        )),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    //! Unit tests for the guard.

    use trcli_domain::workspace::{FormatVersion, OpeningState};

    use super::{Access, guard, upgrade_notice};
    use crate::outcome::{Outcome, codes};

    /// The state of a workspace stored in `stored` opened by a tool that knows format 2.
    fn state(stored: u32) -> OpeningState {
        OpeningState::of(FormatVersion::new(stored), FormatVersion::new(2))
    }

    #[test]
    fn a_usable_workspace_allows_everything() {
        assert!(guard(&state(2), Access::Read).is_ok());
        assert!(guard(&state(2), Access::Write).is_ok());
    }

    #[test]
    fn an_older_workspace_can_be_read_and_asks_for_an_upgrade_before_any_change() {
        assert!(guard(&state(1), Access::Read).is_ok());
        let problem = guard(&state(1), Access::Write).expect_err("needs upgrade");
        assert_eq!(problem.code, codes::WORKSPACE_NEEDS_UPGRADE);
        assert!(
            problem
                .next_step
                .expect("a step")
                .contains("trcli workspace upgrade")
        );
        assert!(upgrade_notice(&state(1)).is_some());
        assert!(upgrade_notice(&state(2)).is_none());
    }

    #[test]
    fn a_newer_workspace_refuses_even_reading_commands_and_explains() {
        for access in [Access::Read, Access::Write] {
            let problem = guard(&state(3), access).expect_err("too new");
            assert_eq!(problem.code, codes::WORKSPACE_TOO_NEW);
            assert_eq!(problem.outcome(), Outcome::WorkspaceProblem);
        }
    }

    #[test]
    fn a_damaged_workspace_says_what_and_where() {
        let damaged = OpeningState::Damaged {
            what: "not a database".into(),
            location: "/w/.trcli/trcli.db".into(),
        };
        let problem = guard(&damaged, Access::Read).expect_err("damaged");
        assert!(problem.message.contains("not a database"));
        assert_eq!(
            problem.details,
            crate::outcome::Details::Items(vec!["where: /w/.trcli/trcli.db".into()])
        );
    }
}

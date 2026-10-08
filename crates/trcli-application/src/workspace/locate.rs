//! Finding the workspace (FR-003, FR-004).
//!
//! The order is: the workspace named for this command (`--workspace`); the one named for
//! the session (`TRCLI_WORKSPACE`); the nearest one at or above the current directory;
//! the researcher's default. The result says how the workspace was found, so that the tool
//! can say when it used the default.
//!
//! This module decides the order; whether a directory holds a workspace is asked of a
//! [`WorkspaceProbe`].

use std::path::{Path, PathBuf};

use crate::outcome::{Problem, codes};
use crate::ports::workspace::WorkspaceProbe;

/// Everything that can point at a workspace, for one command.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct LocateRequest {
    /// The directory named with `--workspace`.
    pub option: Option<PathBuf>,
    /// The directory named by `TRCLI_WORKSPACE`.
    pub session: Option<PathBuf>,
    /// Where the researcher stands.
    pub current_directory: PathBuf,
    /// The researcher's `default_workspace` setting.
    pub default: Option<PathBuf>,
}

/// How a workspace was found.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FoundBy {
    /// Named for this command.
    Option,
    /// Named for the session.
    Session,
    /// At or above the current directory.
    Ancestor,
    /// The researcher's default.
    Default,
}

/// A workspace that was found.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Located {
    /// The directory that holds the workspace's `.trcli/` directory.
    pub root: PathBuf,
    /// How it was found.
    pub found_by: FoundBy,
}

/// Finds the workspace for one command, or explains that there is none.
pub fn locate(request: &LocateRequest, probe: &impl WorkspaceProbe) -> Result<Located, Problem> {
    // A workspace that was named must be there: falling back to another one would act on
    // data the researcher did not mean.
    if let Some(named) = &request.option {
        return named_workspace(named, FoundBy::Option, "--workspace", probe);
    }
    if let Some(named) = &request.session {
        return named_workspace(named, FoundBy::Session, "TRCLI_WORKSPACE", probe);
    }
    if let Some(root) = nearest(&request.current_directory, probe) {
        return Ok(Located {
            root,
            found_by: FoundBy::Ancestor,
        });
    }
    match &request.default {
        Some(default) => named_workspace(
            default,
            FoundBy::Default,
            "the default_workspace setting",
            probe,
        ),
        None => Err(Problem::no_workspace(
            &request.current_directory.display().to_string(),
        )),
    }
}

/// The workspace in a directory that was named, or the problem that says it is not there.
fn named_workspace(
    directory: &Path,
    found_by: FoundBy,
    named_by: &str,
    probe: &impl WorkspaceProbe,
) -> Result<Located, Problem> {
    if probe.holds_workspace(directory) {
        return Ok(Located {
            root: directory.to_path_buf(),
            found_by,
        });
    }
    let message = format!(
        "there is no workspace in {} (named by {named_by})",
        directory.display()
    );
    Err(Problem::new(codes::NO_WORKSPACE, message).with_next_step(
        "check the directory, or create a workspace there with `trcli init <dir> --name <name>`",
    ))
}

/// The nearest directory, at or above `start`, that holds a workspace. With nested
/// workspaces the inner one wins (FR-003).
fn nearest(start: &Path, probe: &impl WorkspaceProbe) -> Option<PathBuf> {
    start
        .ancestors()
        .find(|directory| probe.holds_workspace(directory))
        .map(Path::to_path_buf)
}

#[cfg(test)]
mod tests {
    //! Unit tests for the locator (T045).

    use std::path::PathBuf;

    use super::{FoundBy, LocateRequest, locate};
    use crate::outcome::codes;
    use crate::testing::workspace::FakeProbe;

    /// A request made from `current`, with nothing else set.
    fn from(current: &str) -> LocateRequest {
        LocateRequest {
            current_directory: PathBuf::from(current),
            ..LocateRequest::default()
        }
    }

    #[test]
    fn a_workspace_is_found_from_any_directory_beneath_it() {
        let probe = FakeProbe::with_workspaces(&["/research"]);
        let located = locate(&from("/research/a/b/c"), &probe).expect("found");
        assert_eq!(
            (located.root, located.found_by),
            (PathBuf::from("/research"), FoundBy::Ancestor)
        );
    }

    #[test]
    fn nested_workspaces_resolve_to_the_nearest() {
        let probe = FakeProbe::with_workspaces(&["/research", "/research/inner"]);
        assert_eq!(
            locate(&from("/research/inner/deep"), &probe)
                .expect("found")
                .root,
            PathBuf::from("/research/inner")
        );
        assert_eq!(
            locate(&from("/research/other"), &probe)
                .expect("found")
                .root,
            PathBuf::from("/research")
        );
    }

    #[test]
    fn the_order_is_option_then_session_then_ancestor_then_default() {
        let probe = FakeProbe::with_workspaces(&["/option", "/session", "/here", "/default"]);
        let mut request = LocateRequest {
            option: Some("/option".into()),
            session: Some("/session".into()),
            current_directory: "/here/below".into(),
            default: Some("/default".into()),
        };
        assert_eq!(
            locate(&request, &probe).expect("found").found_by,
            FoundBy::Option
        );
        request.option = None;
        assert_eq!(
            locate(&request, &probe).expect("found").found_by,
            FoundBy::Session
        );
        request.session = None;
        assert_eq!(
            locate(&request, &probe).expect("found").found_by,
            FoundBy::Ancestor
        );
        request.current_directory = "/elsewhere".into();
        let located = locate(&request, &probe).expect("found");
        assert_eq!(
            (located.root, located.found_by),
            (PathBuf::from("/default"), FoundBy::Default)
        );
    }

    #[test]
    fn outside_any_workspace_the_problem_says_how_to_create_or_point_to_one() {
        let problem = locate(&from("/tmp"), &FakeProbe::default()).expect_err("none");
        assert_eq!(problem.code, codes::NO_WORKSPACE);
        let next = problem.next_step.expect("a next step");
        assert!(next.contains("trcli init") && next.contains("--workspace"));
    }

    #[test]
    fn a_named_workspace_that_is_not_there_is_not_replaced_by_another() {
        let probe = FakeProbe::with_workspaces(&["/here"]);
        let request = LocateRequest {
            option: Some("/missing".into()),
            ..from("/here")
        };
        let problem = locate(&request, &probe).expect_err("missing");
        assert!(problem.message.contains("/missing"));
    }
}

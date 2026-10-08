//! Unit tests for the locator (T045).

use std::path::PathBuf;

use trcli_application::outcome::codes;
use trcli_application::workspace::locate::{FoundBy, LocateRequest, locate};
use trcli_testing::workspace::FakeProbe;

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

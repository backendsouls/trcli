//! Everything works with no network at all (SC-010, SC-011): the foundation never reaches
//! outside the machine.
//!
//! Two checks. Where the system can run a process in a network namespace of its own
//! (Linux, with `unshare`), the commands are run there and must work. Everywhere, the
//! commands are run with every proxy variable pointing nowhere, which would make any
//! attempt to reach the network fail. That no crate able to reach the network is even
//! linked is checked by `tests/layering.rs`.

mod common;

use std::process::Command;

use common::Sandbox;

/// The commands of the foundation that a researcher uses day to day.
const COMMANDS: [&[&str]; 9] = [
    &["workspace", "show"],
    &["workspace", "edit", "--researcher", "Ana Souza"],
    &["workspace", "check"],
    &["config", "list"],
    &["config", "set", "output.page_size", "20"],
    &["audit", "list"],
    &["audit", "verify"],
    &["telemetry", "show"],
    &["tag", "list"],
];

#[test]
fn every_foundation_command_works_when_nothing_outside_can_be_reached() {
    let sandbox = Sandbox::with_workspace();
    for arguments in COMMANDS {
        let mut command = sandbox.command(arguments);
        for proxy in [
            "http_proxy",
            "https_proxy",
            "HTTP_PROXY",
            "HTTPS_PROXY",
            "ALL_PROXY",
        ] {
            // Nothing listens there: a request sent through it could only fail.
            command.env(proxy, "http://127.0.0.1:9");
        }
        let output = command.output().expect("trcli runs");
        assert!(
            output.status.success(),
            "`trcli {}` failed: {}",
            arguments.join(" "),
            String::from_utf8_lossy(&output.stderr)
        );
    }
}

/// Whether this system lets an ordinary user run a process without any network.
#[cfg(target_os = "linux")]
fn can_unshare_the_network() -> bool {
    Command::new("unshare")
        .args(["--user", "--net", "true"])
        .output()
        .is_ok_and(|output| output.status.success())
}

#[cfg(target_os = "linux")]
#[test]
fn every_foundation_command_works_in_a_process_with_no_network() {
    if !can_unshare_the_network() {
        eprintln!("skipped: this system does not allow an unprivileged network namespace");
        return;
    }
    let sandbox = Sandbox::with_workspace();
    for arguments in COMMANDS {
        let inner = sandbox.command(arguments);
        let mut isolated = Command::new("unshare");
        isolated
            .args(["--user", "--net", "--"])
            .arg(inner.get_program())
            .args(inner.get_args());
        isolated.current_dir(sandbox.work()).env_clear();
        for (name, value) in inner.get_envs() {
            if let Some(value) = value {
                isolated.env(name, value);
            }
        }
        let output = isolated.output().expect("unshare runs");
        assert!(
            output.status.success(),
            "`trcli {}` failed with no network: {}",
            arguments.join(" "),
            String::from_utf8_lossy(&output.stderr)
        );
    }
}

//! A workspace on a disk that cannot be written to (spec, edge cases): commands that only
//! read keep working; commands that change say why they cannot; nothing is half-written.

// The one test here is a straight list of steps and assertions.
#![allow(clippy::cognitive_complexity)]

mod common;

#[cfg(unix)]
mod unix {
    use std::os::unix::fs::PermissionsExt;
    use std::path::Path;

    use super::common::Sandbox;

    /// Makes the workspace's directory and files read-only, or writable again.
    fn set_read_only(workspace: &Path, read_only: bool) {
        let directory = workspace.join(".trcli");
        let (directory_mode, file_mode) = if read_only {
            (0o555, 0o444)
        } else {
            (0o755, 0o644)
        };
        for entry in std::fs::read_dir(&directory)
            .expect("the workspace")
            .filter_map(Result::ok)
        {
            let mode = if entry.path().is_dir() {
                directory_mode
            } else {
                file_mode
            };
            std::fs::set_permissions(entry.path(), std::fs::Permissions::from_mode(mode))
                .expect("permissions");
        }
        std::fs::set_permissions(&directory, std::fs::Permissions::from_mode(directory_mode))
            .expect("permissions");
    }

    /// Whether this process can write there anyway (it runs as root, as in some containers).
    fn can_still_write(workspace: &Path) -> bool {
        let probe = workspace.join(".trcli").join("probe");
        let written = std::fs::write(&probe, b"").is_ok();
        let _ = std::fs::remove_file(probe);
        written
    }

    #[test]
    fn reading_works_and_changing_is_refused_with_an_explanation() {
        let sandbox = Sandbox::with_workspace();
        sandbox.ok(&[
            "workspace",
            "edit",
            "--description",
            "Before the disk became read-only",
        ]);
        let before = std::fs::read(sandbox.work().join(".trcli/trcli.db")).expect("the database");
        set_read_only(&sandbox.work(), true);
        if can_still_write(&sandbox.work()) {
            set_read_only(&sandbox.work(), false);
            eprintln!("skipped: this process may write anywhere");
            return;
        }

        let shown = sandbox.run(&["workspace", "show"]);
        let listed = sandbox.run(&["audit", "list"]);
        let refused = sandbox.run(&["workspace", "edit", "--name", "Changed"]);
        let after = std::fs::read(sandbox.work().join(".trcli/trcli.db")).expect("the database");
        let files: Vec<_> = std::fs::read_dir(sandbox.work().join(".trcli"))
            .expect("the workspace")
            .collect();
        set_read_only(&sandbox.work(), false);

        assert_eq!(shown.code, 0, "{}", shown.stderr);
        assert!(shown.stdout.contains("Before the disk became read-only"));
        assert_eq!(listed.code, 0, "{}", listed.stderr);
        assert!(listed.stdout.contains("update"));
        assert_eq!(refused.code, 7, "{}", refused.stderr);
        assert!(
            refused.stderr.contains("cannot be written to"),
            "{}",
            refused.stderr
        );
        assert!(
            refused.stderr.contains("Nothing was changed."),
            "{}",
            refused.stderr
        );
        assert_eq!(before, after, "the database is untouched");
        assert_eq!(
            files.len(),
            4,
            "nothing was left beside the workspace's own files"
        );
    }
}

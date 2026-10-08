//! Shared by the black-box tests at the repository root: a scratch directory and the
//! `trcli` binary set up so that nothing outside the directory is read or written.

#![allow(dead_code)] // each test file uses a part of this module

use std::path::{Path, PathBuf};
use std::process::{Command, Output};

use tempfile::TempDir;

/// The root of the repository.
pub fn repository() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..").canonicalize().expect("the repository root")
}

/// What a command printed and how it ended.
#[derive(Clone, Debug)]
pub struct Finished {
    /// Standard output.
    pub stdout: String,
    /// Standard error.
    pub stderr: String,
    /// The exit code; -1 when the process was killed by a signal.
    pub code: i32,
}

impl From<Output> for Finished {
    fn from(output: Output) -> Self {
        Self {
            stdout: String::from_utf8_lossy(&output.stdout).into_owned(),
            stderr: String::from_utf8_lossy(&output.stderr).into_owned(),
            code: output.status.code().unwrap_or(-1),
        }
    }
}

impl Finished {
    /// The JSON document on standard output.
    pub fn json(&self) -> serde_json::Value {
        serde_json::from_str(&self.stdout).unwrap_or_else(|error| panic!("not JSON ({error}):\n{}", self.stdout))
    }
}

/// A scratch directory with a workspace-to-be in `work`.
pub struct Sandbox {
    /// The directory; removed when this value is dropped.
    directory: TempDir,
    /// How many commands were run; seeds the identifiers.
    runs: std::cell::Cell<u64>,
}

impl Sandbox {
    /// An empty scratch directory.
    pub fn new() -> Self {
        let directory = tempfile::tempdir().expect("a temporary directory");
        std::fs::create_dir_all(directory.path().join("work")).expect("the working directory");
        Self { directory, runs: std::cell::Cell::new(0) }
    }

    /// A scratch directory holding a workspace named "Lab".
    pub fn with_workspace() -> Self {
        let sandbox = Self::new();
        let created = sandbox.run(&["init", "--name", "Lab"]);
        assert_eq!(created.code, 0, "{}", created.stderr);
        sandbox
    }

    /// The scratch directory, in the form the tool prints paths in.
    pub fn home(&self) -> PathBuf {
        self.directory.path().canonicalize().expect("the temporary directory exists")
    }

    /// The directory commands are run in.
    pub fn work(&self) -> PathBuf {
        self.home().join("work")
    }

    /// The `trcli` command, with a fixed clock, seeded identifiers, and the researcher's
    /// own settings inside the scratch directory on every system.
    pub fn command(&self, arguments: &[&str]) -> Command {
        self.runs.set(self.runs.get() + 1);
        let settings = self.home().join("user-settings");
        let mut command = Command::new(env!("CARGO_BIN_EXE_trcli"));
        command
            .args(arguments)
            .current_dir(self.work())
            .env_clear()
            .env("XDG_CONFIG_HOME", &settings)
            .env("HOME", &settings)
            .env("APPDATA", &settings)
            .env("USER", "ana")
            .env("USERNAME", "ana")
            .env("TZ", "UTC")
            .env("TRCLI_TEST_NOW", "2026-10-08T14:00:00Z")
            .env("TRCLI_TEST_ID_SEED", self.runs.get().to_string())
            .stdin(std::process::Stdio::null());
        for name in ["SystemRoot", "PATH", "TMPDIR", "TEMP", "TMP"] {
            if let Ok(value) = std::env::var(name) {
                command.env(name, value);
            }
        }
        command
    }

    /// Runs `trcli` with these arguments.
    pub fn run(&self, arguments: &[&str]) -> Finished {
        self.command(arguments).output().expect("trcli starts").into()
    }

    /// Runs `trcli` and requires it to succeed.
    pub fn ok(&self, arguments: &[&str]) -> Finished {
        let finished = self.run(arguments);
        assert_eq!(finished.code, 0, "`trcli {}` failed:\n{}", arguments.join(" "), finished.stderr);
        finished
    }
}

/// Every file under a directory with one of the given extensions, recursively.
pub fn files_under(directory: &Path, extensions: &[&str]) -> Vec<PathBuf> {
    let mut found = Vec::new();
    let Ok(entries) = std::fs::read_dir(directory) else { return found };
    for entry in entries.filter_map(Result::ok) {
        let path = entry.path();
        if path.is_dir() {
            found.extend(files_under(&path, extensions));
        } else if path.extension().and_then(|extension| extension.to_str()).is_some_and(|e| extensions.contains(&e)) {
            found.push(path);
        }
    }
    found.sort();
    found
}

/// The feature files of every specification.
pub fn feature_files() -> Vec<PathBuf> {
    files_under(&repository().join("tests/features"), &["feature"])
}

/// A scenario of a feature file: its tags and the command lines it runs.
#[derive(Clone, Debug, Default)]
pub struct Scenario {
    /// The scenario's tags, without the `@`.
    pub tags: Vec<String>,
    /// The `trcli …` command lines of its steps.
    pub commands: Vec<String>,
}

/// The scenarios of every feature file, read simply: tag lines, then steps until the next
/// scenario. This is all the structural tests need to know about Gherkin.
pub fn scenarios() -> Vec<Scenario> {
    let mut scenarios = Vec::new();
    for file in feature_files() {
        let text = std::fs::read_to_string(&file).expect("a feature file");
        let mut pending_tags: Vec<String> = Vec::new();
        for line in text.lines().map(str::trim) {
            if line.starts_with('@') {
                pending_tags.extend(line.split_whitespace().map(|tag| tag.trim_start_matches('@').to_owned()));
            } else if line.starts_with("Scenario") {
                scenarios.push(Scenario { tags: std::mem::take(&mut pending_tags), commands: Vec::new() });
            } else if line.starts_with("Feature") || line.starts_with("Background") {
                pending_tags.clear();
            } else if let Some(scenario) = scenarios.last_mut() {
                scenario.commands.extend(commands_in(line));
            }
        }
    }
    scenarios
}

/// The `trcli …` command lines quoted in a step.
fn commands_in(step: &str) -> Vec<String> {
    step.split('"').filter(|part| part.starts_with("trcli")).map(str::to_owned).collect()
}

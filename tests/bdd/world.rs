//! The world of one scenario: a temporary directory, the `trcli` binary, and what the
//! last command printed.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};

use sea_orm::{ConnectionTrait, Database, DatabaseConnection, DbBackend, Statement};
use tempfile::TempDir;

/// The moment every scenario starts at, unless it says otherwise.
pub const START: &str = "2026-10-08T14:00:00Z";

/// What a command printed and how it ended.
#[derive(Clone, Debug, Default)]
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

/// What a workspace holds, as far as "nothing was changed" is concerned.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Snapshot {
    /// Whether there is a workspace at all.
    pub exists: bool,
    /// Every row of every shared table, as text.
    pub rows: Vec<String>,
    /// How many audit entries there are.
    pub audit_entries: usize,
    /// The bytes of the workspace's settings file.
    pub settings: Vec<u8>,
}

/// The world of one scenario.
#[derive(Debug, cucumber::World)]
#[world(init = Self::new)]
pub struct TrcliWorld {
    /// The scenario's own directory; removed when the scenario ends.
    home: TempDir,
    /// Where commands are run, inside `home`.
    pub directory: PathBuf,
    /// Variables set for every command of the scenario.
    pub environment: BTreeMap<String, String>,
    /// What the last command printed.
    pub last: Finished,
    /// The workspace as it was before the last command.
    pub before: Snapshot,
    /// How many commands were run; seeds the identifiers so that each command makes
    /// different ones.
    runs: u64,
    /// Short names of records, remembered under a name the scenario chose.
    pub handles: BTreeMap<String, String>,
}

impl TrcliWorld {
    /// A fresh world: an empty directory `work` to stand in, and nothing else.
    fn new() -> Self {
        let home = tempfile::tempdir().expect("a temporary directory");
        // Canonical, so that paths printed by the tool match paths built here on systems
        // where the temporary directory is reached through a link.
        let root = printed_form(home.path());
        let directory = root.join("work");
        std::fs::create_dir_all(&directory).expect("the working directory");
        Self {
            home,
            directory,
            environment: BTreeMap::new(),
            last: Finished::default(),
            before: Snapshot::default(),
            runs: 0,
            handles: BTreeMap::new(),
        }
    }

    /// The scenario's own directory.
    pub fn home(&self) -> PathBuf {
        printed_form(self.home.path())
    }

    /// A path inside the scenario's directory; `~` alone is the directory itself.
    pub fn path(&self, relative: &str) -> PathBuf {
        if relative == "~" {
            self.home()
        } else {
            self.home().join(relative)
        }
    }

    /// The `trcli` command, set up so that nothing outside the scenario's directory is
    /// read or written and so that time and identifiers are fixed.
    pub fn trcli(&mut self, arguments: &[String]) -> Command {
        self.runs += 1;
        let mut command = Command::new(env!("CARGO_BIN_EXE_trcli"));
        let settings = self.home().join("user-settings");
        command
            .args(arguments)
            .current_dir(&self.directory)
            .env_clear()
            // The researcher's own settings live in the scenario's directory on every system.
            .env("XDG_CONFIG_HOME", &settings)
            .env("HOME", &settings)
            .env("APPDATA", &settings)
            .env("USER", "ana")
            .env("USERNAME", "ana")
            .env("TZ", "UTC")
            .env("TRCLI_TEST_NOW", START)
            .env("TRCLI_TEST_ID_SEED", self.runs.to_string())
            .envs(&self.environment)
            .stdin(Stdio::null());
        // A few variables some systems need for a process to start at all.
        for name in ["SystemRoot", "PATH", "TMPDIR", "TEMP", "TMP"] {
            if let Ok(value) = std::env::var(name) {
                command.env(name, value);
            }
        }
        command
    }

    /// Runs `trcli` with these arguments, remembering the workspace as it was before.
    pub async fn run(&mut self, arguments: &[String]) {
        self.before = self.snapshot().await;
        let output = self.trcli(arguments).output().expect("trcli starts");
        self.last = output.into();
    }

    /// The workspace's database, in the directory commands are run in or above it.
    pub fn database(&self) -> Option<PathBuf> {
        self.directory
            .ancestors()
            .map(|directory| directory.join(".trcli").join("trcli.db"))
            .find(|path| path.exists())
    }

    /// The root of the workspace commands are run in.
    pub fn workspace_root(&self) -> Option<PathBuf> {
        self.directory
            .ancestors()
            .find(|directory| directory.join(".trcli").is_dir())
            .map(Path::to_path_buf)
    }

    /// Opens the workspace's database directly, as something outside the tool would.
    pub async fn connect(&self, writable: bool) -> Option<DatabaseConnection> {
        Database::connect(direct(self.database()?, writable, None))
            .await
            .ok()
    }

    /// Whether another process holds the workspace for writing right now.
    async fn is_held(&self) -> bool {
        let Some(database) = self.database() else {
            return false;
        };
        // Asked without waiting, so that the answer is about this moment.
        let options = direct(database, true, Some(std::time::Duration::ZERO));
        let Ok(connection) = Database::connect(options).await else {
            return false;
        };
        let held = connection
            .execute_unprepared("BEGIN IMMEDIATE")
            .await
            .is_err();
        if !held {
            let _ = connection.execute_unprepared("ROLLBACK").await;
        }
        let _ = connection.close().await;
        held
    }

    /// Waits until another process holds the workspace for writing. Scenarios about two
    /// commands at once, or about stopping a command, start from this known point
    /// instead of from a guess at how long a process takes to start.
    ///
    /// A command also writes for an instant while it starts, so the workspace must be
    /// seen held several times in a row before it counts as held.
    pub async fn wait_until_held(&self) {
        let started = std::time::Instant::now();
        let mut seen = 0;
        while seen < 4 {
            assert!(
                started.elapsed() < std::time::Duration::from_secs(20),
                "no command took the workspace"
            );
            seen = if self.is_held().await { seen + 1 } else { 0 };
            std::thread::sleep(std::time::Duration::from_millis(75));
        }
    }

    /// Runs one statement directly against the workspace's database.
    pub async fn execute(&self, sql: &str) {
        let connection = self.connect(true).await.expect("a workspace database");
        connection
            .execute_unprepared(sql)
            .await
            .expect("the statement runs");
        connection.close().await.expect("the connection closes");
    }

    /// The rows of one table as text, in a stable order.
    async fn rows(connection: &DatabaseConnection, table: &str) -> Vec<String> {
        let query = |sql: String| Statement::from_string(DbBackend::Sqlite, sql);
        let columns = connection
            .query_all_raw(query(format!("PRAGMA table_info({table})")))
            .await
            .unwrap_or_default();
        let quoted: Vec<String> = columns
            .iter()
            .filter_map(|row| row.try_get_by_index::<String>(1).ok())
            .map(|name| format!("quote({name})"))
            .collect();
        if quoted.is_empty() {
            return Vec::new();
        }
        let select = format!(
            "SELECT {} FROM {table} ORDER BY rowid",
            quoted.join(" || '|' || ")
        );
        let rows = connection
            .query_all_raw(query(select))
            .await
            .unwrap_or_default();
        rows.iter()
            .filter_map(|row| row.try_get_by_index::<String>(0).ok())
            .map(|row| format!("{table}: {row}"))
            .collect()
    }

    /// What the workspace holds now. Local tables (telemetry) are left out: every command
    /// adds to them, and they are not part of what a command "changes".
    pub async fn snapshot(&self) -> Snapshot {
        let Some(root) = self.workspace_root() else {
            return Snapshot::default();
        };
        let settings = std::fs::read(root.join(".trcli").join("config.toml")).unwrap_or_default();
        let Some(connection) = self.connect(false).await else {
            return Snapshot {
                exists: true,
                settings,
                ..Snapshot::default()
            };
        };
        let mut rows = Vec::new();
        for table in [
            "workspace",
            "record",
            "tag",
            "tagging",
            "note",
            "link",
            "specimen",
            "sample_note",
        ] {
            rows.extend(Self::rows(&connection, table).await);
        }
        let audit = Self::rows(&connection, "audit_entry").await;
        let _ = connection.close().await;
        Snapshot {
            exists: true,
            audit_entries: audit.len(),
            rows: rows.into_iter().chain(audit).collect(),
            settings,
        }
    }

    /// Replaces `<name>` with the short name remembered under that name, `<name:n>` with
    /// its first `n` characters, `{home}` with the scenario's directory, `{bell}` with a
    /// control character, and `{long title}` with 501 characters.
    pub fn expand(&self, text: &str) -> String {
        let mut expanded = text.replace("{home}", &self.home().display().to_string());
        // What a feature file cannot hold comfortably: a control character, and very long text.
        expanded = expanded
            .replace("{bell}", "\u{7}")
            .replace("{long title}", &"t".repeat(501));
        for (name, handle) in &self.handles {
            // `{name}` is the spelling for scenario outlines, where `<…>` already means a
            // column of the examples table.
            expanded = expanded
                .replace(&format!("<{name}>"), handle)
                .replace(&format!("{{{name}}}"), handle);
            for length in 1..=handle.len() {
                expanded = expanded.replace(&format!("<{name}:{length}>"), &handle[..length]);
            }
        }
        expanded
    }
}

/// A directory in the form the tool prints it: with links resolved (on some systems the
/// temporary directory is reached through one), and without the prefix Windows puts on
/// resolved paths.
pub fn printed_form(directory: &Path) -> PathBuf {
    let resolved = directory.canonicalize().expect("the directory exists");
    let text = resolved.display().to_string();
    text.strip_prefix(WINDOWS_VERBATIM_PREFIX)
        .map_or(resolved.clone(), PathBuf::from)
}

/// What Windows puts in front of a resolved path; the tool never prints it.
const WINDOWS_VERBATIM_PREFIX: &str = r"\\?\";

/// How a test opens a workspace's database directly: one connection on the file, given as
/// a path so that no character of it needs escaping on any system.
fn direct(
    database: PathBuf,
    writable: bool,
    busy_timeout: Option<std::time::Duration>,
) -> sea_orm::ConnectOptions {
    let mut options = sea_orm::ConnectOptions::new("sqlite:trcli-test");
    options
        .sqlx_logging(false)
        .max_connections(1)
        .map_sqlx_sqlite_opts(move |sqlite| {
            let sqlite = sqlite
                .filename(&database)
                .create_if_missing(false)
                .read_only(!writable);
            match busy_timeout {
                Some(timeout) => sqlite.busy_timeout(timeout),
                None => sqlite,
            }
        });
    options
}

/// A text with the path separators of this system written as `/`, so that an expected
/// text written once in a feature file matches what the tool prints on every system.
pub fn portable(text: &str) -> String {
    if cfg!(windows) {
        text.replace(BACKSLASH, "/")
    } else {
        text.to_owned()
    }
}

/// The path separator of Windows.
const BACKSLASH: char = 0x5C as char;

/// Splits a command line as a shell would, for the little that scenarios need: words
/// separated by spaces, and single quotes around a word that holds spaces or is empty.
pub fn split(line: &str) -> Vec<String> {
    let mut words = Vec::new();
    let mut word = String::new();
    let (mut quoted, mut started) = (false, false);
    for character in line.chars() {
        match character {
            '\'' => {
                quoted = !quoted;
                started = true;
            }
            ' ' if !quoted => {
                if started {
                    words.push(std::mem::take(&mut word));
                    started = false;
                }
            }
            other => {
                word.push(other);
                started = true;
            }
        }
    }
    if started {
        words.push(word);
    }
    words
}

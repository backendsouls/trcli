//! The composition root: the one place where concrete adapters are named and wired
//! together (dependency inversion, FR-070).
//!
//! A [`Session`] is everything one command needs: the settings in effect, the workspace
//! that was found (or why none was), the adapters for storage, files, the clock, and
//! identifiers, and how the researcher is asked and shown things. Commands use the
//! session and the aliases below; no other module mentions SQLite or the file system.

use std::io::{IsTerminal, Stderr};
use std::path::{Path, PathBuf};

use time::UtcOffset;
use trcli_application::kinds::KindRegistry;
use trcli_application::outcome::{Problem, codes};
use trcli_application::ports::environment::{ActorProvider, Clock};
use trcli_application::ports::settings::SettingsSource;
use trcli_application::ports::unit_of_work::Storage;
use trcli_application::ports::workspace::{StorageOpener, WorkspaceStore};
use trcli_application::settings::foundation::{
    self, DEFAULT_WORKSPACE, OUTPUT_COLOR, OUTPUT_FORMAT, OUTPUT_PAGE_SIZE, RESEARCHER_NAME,
    STORAGE_BUSY_TIMEOUT_MS, STORAGE_PATH, TELEMETRY_ENABLED,
};
use trcli_application::settings::layers::{Resolved, Settings, resolve};
use trcli_application::settings::registry::SettingsRegistry;
use trcli_application::workspace::locate::{FoundBy, LocateRequest, Located, locate};
use trcli_application::workspace::open::{Access, guard, upgrade_notice};
use trcli_domain::governance::audit::Stamp;
use trcli_domain::settings::SettingKey;
use trcli_domain::shared::problem::Warning;
use trcli_domain::workspace::FormatVersion;
use trcli_infra_sqlite::connection::{SqliteOpener, SqliteStorage};
use trcli_infra_sqlite::digest::Sha256Digest;
use trcli_infra_sqlite::unit_of_work::SqliteUnit;
use trcli_infra_system::actor::SystemActor;
use trcli_infra_system::atomic::write_atomically;
use trcli_infra_system::audit_head::FileHead;
use trcli_infra_system::backup_copy::FileBackup;
use trcli_infra_system::clock::SystemClock;
use trcli_infra_system::ids::UuidGenerator;
use trcli_infra_system::locator::{
    FileSystemProbe, FileSystemWorkspaceFiles, audit_head_file, backups_directory, database_file,
    workspace_settings_file,
};
use trcli_infra_system::paths::current_user_settings_file;
use trcli_infra_system::settings_files::{
    CommandLineSource, EnvironmentSource, FileSettings, TomlFileSource,
};

use crate::args::global::GlobalArgs;
use crate::diagnostics::Diagnostics;
use crate::output::Presentation;
use crate::progress::TerminalProgress;
use crate::prompt::TerminalPrompter;

/// The storage every command uses.
pub type AppStorage = SqliteStorage;
/// The unit of work every command uses.
pub type AppUnit = SqliteUnit;
/// Where the end of the audit trail is kept.
pub type AppHead = FileHead;
/// The hash function of the audit trail.
pub type AppDigest = Sha256Digest;
/// The copy kept before an upgrade.
pub type AppBackup = FileBackup;
/// The settings files.
pub type AppSettingsFiles = FileSettings;
/// The opener of workspaces' storage.
pub type AppOpener = SqliteOpener;
/// The workspace's files other than its storage.
pub type AppWorkspaceFiles = FileSystemWorkspaceFiles;
/// The file system, asked whether a directory holds a workspace.
pub type AppProbe = FileSystemProbe;
/// The generator of identifiers.
pub type AppIds = UuidGenerator;
/// The indication of progress.
pub type AppProgress = TerminalProgress<Stderr>;

/// The registries features plug into, filled once at start-up.
#[derive(Debug)]
pub struct Registries {
    /// Every setting this build knows.
    pub settings: SettingsRegistry,
    /// Every kind of record this build knows.
    pub kinds: KindRegistry,
}

impl Registries {
    /// The registries of this build: the foundation's settings, and the record kinds of
    /// the features compiled in. Adding a feature adds a line here and nowhere else in
    /// the foundation.
    pub fn of_this_build() -> Self {
        let mut settings = SettingsRegistry::new();
        foundation::register(&mut settings);
        #[allow(unused_mut)]
        let mut kinds = KindRegistry::new();
        #[cfg(feature = "sample-kind")]
        trcli_application::sample::register(&mut kinds)
            .unwrap_or_else(|error| panic!("sample kinds: {error}"));
        Self { settings, kinds }
    }
}

/// Everything one command needs.
pub struct Session {
    /// The global options given.
    pub global: GlobalArgs,
    /// The registries of this build.
    pub registries: Registries,
    /// The settings in effect.
    pub settings: Settings,
    /// What was ignored while reading settings (FR-044); shown with every command.
    pub setting_warnings: Vec<Warning>,
    /// The workspace that was found, or why none was.
    located: Result<Located, Problem>,
    /// The settings files.
    pub files: AppSettingsFiles,
    /// The clock.
    pub clock: SystemClock,
    /// The generator of identifiers.
    pub ids: AppIds,
    /// How questions are answered.
    pub prompter: TerminalPrompter,
    /// Where progress is shown.
    pub progress: AppProgress,
    /// Where `--verbose` detail goes.
    pub diagnostics: Diagnostics,
    /// How output is presented.
    pub presentation: Presentation,
    /// The workspace's storage, once a command has opened it.
    storage: Option<AppStorage>,
    /// Remarks gathered on the way, shown with the result.
    pub notes: Vec<String>,
}

/// The path named by a variable of the session, when it is set and not empty.
fn session_path(name: &str) -> Option<PathBuf> {
    std::env::var_os(name)
        .filter(|value| !value.is_empty())
        .map(PathBuf::from)
}

/// The sources of settings, in order of increasing precedence, as trait objects.
struct Sources {
    /// The researcher's own file, when the system says where it is.
    user: Option<TomlFileSource>,
    /// The workspace's file, inside a workspace.
    workspace: Option<TomlFileSource>,
    /// The session's variables.
    session: EnvironmentSource,
    /// This command's options.
    command: CommandLineSource,
}

/// What starting a session found: the settings, the workspace, and the settings files.
struct Found {
    /// The settings in effect.
    resolved: Resolved,
    /// The workspace, or why there is none.
    located: Result<Located, Problem>,
    /// The settings files.
    files: AppSettingsFiles,
}

impl Sources {
    /// Resolves the settings from every source there is.
    fn resolve(&self, registry: &SettingsRegistry) -> Result<Resolved, Problem> {
        let mut layers: Vec<&dyn SettingsSource> = Vec::new();
        if let Some(user) = &self.user {
            layers.push(user);
        }
        if let Some(workspace) = &self.workspace {
            layers.push(workspace);
        }
        layers.push(&self.session);
        layers.push(&self.command);
        resolve(registry, &layers)
    }
}

impl Session {
    /// Builds the session of one command: reads the settings, finds the workspace, and
    /// chooses how to ask and show. On failure, the presentation to report it with is
    /// returned too, since the settings that would decide it could not be read.
    pub fn start(
        global: GlobalArgs,
        zone: UtcOffset,
    ) -> Result<Self, Box<(Problem, Presentation)>> {
        let fallback = Presentation::before_settings(&global, zone);
        let registries = Registries::of_this_build();
        let keys: Vec<SettingKey> = registries
            .settings
            .all()
            .iter()
            .map(|definition| definition.key.clone())
            .collect();
        let user_file = current_user_settings_file();
        let mut sources = Sources {
            user: user_file.clone().map(TomlFileSource::user),
            workspace: None,
            session: EnvironmentSource::new(&keys),
            command: CommandLineSource::new()
                .with(OUTPUT_FORMAT, global.output.as_deref())
                .with(OUTPUT_COLOR, global.color.as_deref()),
        };
        // First without a workspace, to learn the researcher's default workspace; then
        // again with the file of the workspace that was found.
        let personal = sources
            .resolve(&registries.settings)
            .map_err(|problem| Box::new((problem, fallback.clone())))?;
        let located = Self::locate(&global, &personal.settings);
        let workspace_file = located
            .as_ref()
            .ok()
            .map(|located| workspace_settings_file(&located.root));
        sources.workspace = workspace_file.clone().map(TomlFileSource::workspace);
        let resolved = sources
            .resolve(&registries.settings)
            .map_err(|problem| Box::new((problem, fallback)))?;
        let found = Found {
            resolved,
            located,
            files: FileSettings::new(user_file, workspace_file),
        };
        Ok(Self::assemble(global, registries, found, zone))
    }

    /// Puts the parts of a session together.
    fn assemble(global: GlobalArgs, registries: Registries, found: Found, zone: UtcOffset) -> Self {
        let presentation = Presentation::new(&global, &found.resolved.settings, zone);
        // The tool says which workspace it used when that was the default (FR-003).
        let notes: Vec<String> = found
            .located
            .iter()
            .filter(|located| located.found_by == FoundBy::Default)
            .map(|located| format!("Using your default workspace in {}", located.root.display()))
            .collect();
        let draws = !global.quiet && std::io::stderr().is_terminal();
        Self {
            prompter: TerminalPrompter::choose(
                global.yes,
                global.no_input,
                std::io::stdin().is_terminal(),
            ),
            progress: TerminalProgress::new(std::io::stderr(), draws, presentation.symbols),
            diagnostics: Diagnostics::new(global.verbose),
            global,
            registries,
            settings: found.resolved.settings,
            setting_warnings: found.resolved.warnings,
            located: found.located,
            files: found.files,
            clock: SystemClock,
            ids: UuidGenerator::new(),
            presentation,
            storage: None,
            notes,
        }
    }

    /// Finds the workspace for this command (FR-003).
    fn locate(global: &GlobalArgs, personal: &Settings) -> Result<Located, Problem> {
        let request = LocateRequest {
            option: global.workspace.clone(),
            session: session_path("TRCLI_WORKSPACE"),
            current_directory: std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")),
            default: personal.text(DEFAULT_WORKSPACE).map(PathBuf::from),
        };
        locate(&request, &FileSystemProbe)
    }

    /// The workspace this command acts in, or the problem that says there is none.
    pub fn located(&self) -> Result<&Located, Problem> {
        self.located.as_ref().map_err(Clone::clone)
    }

    /// Who is acting and when, for this command's audit entries.
    pub fn stamp(&self) -> Stamp {
        Stamp::new(
            self.clock.now(),
            SystemActor::new(self.settings.text(RESEARCHER_NAME)).actor(),
        )
    }

    /// The opener of storage, waiting as long as the settings say for another writer.
    pub fn opener(&self) -> AppOpener {
        let timeout = self
            .settings
            .integer(STORAGE_BUSY_TIMEOUT_MS)
            .unwrap_or(5000);
        SqliteOpener::new(u64::try_from(timeout).unwrap_or(5000))
    }

    /// Where the workspace's database is.
    pub fn database(&self) -> Result<PathBuf, Problem> {
        let path = self
            .settings
            .text(STORAGE_PATH)
            .unwrap_or(".trcli/trcli.db");
        Ok(database_file(&self.located()?.root, path))
    }

    /// Where the database of a workspace about to be created in `root` will be: at the
    /// default location, since a new workspace has no settings of its own yet.
    pub fn new_database_in(&self, root: &Path) -> PathBuf {
        let default = self
            .registries
            .settings
            .find(STORAGE_PATH)
            .and_then(|definition| definition.default.clone());
        let path = default
            .as_ref()
            .and_then(|value| value.as_text())
            .unwrap_or(".trcli/trcli.db")
            .to_owned();
        database_file(root, &path)
    }

    /// The file that keeps the end of the audit trail.
    pub fn head(&self) -> Result<AppHead, Problem> {
        Ok(Self::head_in(&self.located()?.root))
    }

    /// The head file of the workspace in `root`.
    pub fn head_in(root: &Path) -> AppHead {
        FileHead::new(audit_head_file(root))
    }

    /// The file system, asked whether a directory holds a workspace.
    pub fn probe(&self) -> AppProbe {
        FileSystemProbe
    }

    /// Writes a file the researcher asked for, whole or not at all (FR-066).
    pub fn write_file(&self, path: &Path, content: &str) -> Result<(), Problem> {
        write_atomically(path, content.as_bytes()).map_err(|error| {
            Problem::new(
                codes::OPERATION_FAILED,
                format!("{} could not be written: {error}", path.display()),
            )
        })
    }

    /// The directory a path given on the command line stands for, made absolute so that
    /// messages say where things are whatever the current directory.
    pub fn absolute(path: &Path) -> PathBuf {
        std::path::absolute(path).unwrap_or_else(|_| path.to_path_buf())
    }

    /// The copy to keep before an upgrade from the given format.
    pub fn backup(&self, from_format: u32) -> Result<AppBackup, Problem> {
        let root = &self.located()?.root;
        let moment = self.clock.now();
        let (date, time) = (moment.date(), moment.time());
        let label = format!(
            "{:04}{:02}{:02}T{:02}{:02}{:02}Z-format{from_format}",
            date.year(),
            u8::from(date.month()),
            date.day(),
            time.hour(),
            time.minute(),
            time.second()
        );
        Ok(FileBackup::new(
            self.database()?,
            backups_directory(root),
            &label,
        ))
    }

    /// The workspace's storage, opened without asking what state the workspace is in.
    /// Only `workspace upgrade` and `workspace check` want that.
    pub async fn storage_unguarded(&mut self) -> Result<AppStorage, Problem> {
        if let Some(storage) = &self.storage {
            return Ok(storage.clone());
        }
        let database = self.database()?;
        self.diagnostics
            .line(|| format!("opening {}", database.display()));
        let storage = self.opener().open(&database).await?;
        self.storage = Some(storage.clone());
        Ok(storage)
    }

    /// The workspace's storage, for a command that reads or writes. A workspace that is
    /// too new, needs an upgrade (for writing), or is damaged is refused here, before any
    /// handler runs (FR-007, FR-008).
    pub async fn storage(&mut self, access: Access) -> Result<AppStorage, Problem> {
        let storage = self.storage_unguarded().await?;
        let workspace = storage.read().await?.workspace().await?;
        let state = workspace.opening_state(FormatVersion::CURRENT);
        guard(&state, access)?;
        if let Some(notice) = upgrade_notice(&state) {
            self.notes.push(notice);
        }
        Ok(storage)
    }

    /// Closes the workspace's storage at the end of the command. Closing folds the
    /// write-ahead journal back into the database file, so that between commands the
    /// workspace is that one file and can be copied as such.
    pub async fn close(&mut self) {
        if let Some(storage) = self.storage.take()
            && let Err(error) = storage.close().await
        {
            self.diagnostics
                .line(|| format!("the workspace's storage did not close cleanly: {error}"));
        }
    }

    /// How many rows a list shows unless `--limit` says otherwise.
    pub fn page_size(&self) -> u32 {
        self.settings
            .integer(OUTPUT_PAGE_SIZE)
            .and_then(|size| u32::try_from(size).ok())
            .unwrap_or(50)
    }

    /// Whether local telemetry is recorded in this workspace.
    pub fn telemetry_enabled(&self) -> bool {
        self.settings.boolean(TELEMETRY_ENABLED).unwrap_or(true)
    }

    /// The workspace's files other than its storage.
    pub fn workspace_files(&self) -> AppWorkspaceFiles {
        FileSystemWorkspaceFiles
    }
}

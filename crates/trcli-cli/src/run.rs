//! The path of one command, from the arguments to the exit code (FR-032, FR-036).
//!
//! ```text
//! arguments ─▶ clap ─▶ session (settings, workspace) ─▶ handler ─▶ view or problem
//!           ─▶ standard output / standard error ─▶ telemetry ─▶ exit code
//! ```
//!
//! One single-threaded runtime drives the handler; Ctrl-C is awaited beside it. When the
//! researcher interrupts, the handler is dropped where it stands, which drops its unit of
//! work and so undoes whatever it had not committed.

use std::process::ExitCode;
use std::time::Instant;

use time::UtcOffset;
use trcli_application::governance::telemetry_summary::record_use;
use trcli_application::outcome::{Outcome, Problem, codes};
use trcli_application::ports::environment::Clock;
use trcli_application::ports::unit_of_work::{Storage, UnitOfWork};
use trcli_application::workspace::open::Access;
use trcli_domain::governance::telemetry::TelemetryRecord;

use crate::args::global::GlobalArgs;
use crate::cli::{self, Invocation};
use crate::commands;
use crate::compose::Session;
use crate::extension::{Extension, NoExtension};
use crate::output::{Presentation, write_error, write_out};

/// Runs the command given to this process and returns its exit code.
pub fn main() -> ExitCode {
    main_with(&NoExtension)
}

/// Runs the command given to this process, with the kinds of record and the commands an
/// extension adds, and returns its exit code.
pub fn main_with(extension: &impl Extension) -> ExitCode {
    // Asked before any other thread exists: on some systems the local offset cannot be
    // read safely afterwards. Without it, times are shown in UTC and say so.
    let zone = UtcOffset::current_local_offset().unwrap_or(UtcOffset::UTC);
    let arguments: Vec<String> = std::env::args().collect();
    let outcome = match cli::command(extension).try_get_matches_from(&arguments) {
        Ok(matches) => match cli::parse(&matches) {
            Ok(invocation) => execute(invocation, zone, extension),
            Err(error) => reject(&error, &arguments, zone),
        },
        Err(error) => reject(&error, &arguments, zone),
    };
    ExitCode::from(outcome.exit_code())
}

/// Handles what clap would not accept: a request for help or the version is shown; a
/// malformed command line becomes the `usage` problem (FR-024).
fn reject(error: &clap::Error, arguments: &[String], zone: UtcOffset) -> Outcome {
    if cli::is_information(error) {
        write_out(&error.render().to_string());
        return Outcome::Success;
    }
    // The command line could not be read, so whether the structured form was asked for
    // is looked for in the raw arguments.
    let wants_json = arguments
        .windows(2)
        .any(|pair| pair[0] == "--output" && pair[1] == "json")
        || arguments.iter().any(|argument| argument == "--output=json");
    let global = GlobalArgs {
        output: wants_json.then(|| "json".to_owned()),
        ..GlobalArgs::default()
    };
    let problem = cli::usage_problem(error);
    Presentation::before_settings(&global, zone).fail(&problem);
    problem.outcome()
}

/// Runs a parsed command on a single-threaded runtime.
fn execute(invocation: Invocation, zone: UtcOffset, extension: &impl Extension) -> Outcome {
    let runtime = match tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
    {
        Ok(runtime) => runtime,
        Err(error) => {
            write_error(&format!("error: the tool could not start: {error}\n"));
            return Outcome::Failure;
        }
    };
    let outcome = runtime.block_on(run(invocation, zone, extension));
    // A question left open when the researcher interrupted is still waiting for input on
    // its own thread; the runtime is not allowed to wait for it.
    runtime.shutdown_background();
    outcome
}

/// The problem that says the researcher stopped the command.
fn interrupted() -> Problem {
    Problem::new(
        codes::INTERRUPTED,
        "interrupted before the command finished",
    )
    .with_next_step("run the command again; what it had not finished was undone")
}

/// Builds the session, runs the handler beside Ctrl-C, shows the result, and records the
/// use of the command.
async fn run(invocation: Invocation, zone: UtcOffset, extension: &impl Extension) -> Outcome {
    let started = Instant::now();
    let mut session = match Session::start(invocation.global.clone(), zone, extension) {
        Ok(session) => session,
        Err(failed) => {
            let (problem, presentation) = *failed;
            presentation.fail(&problem);
            return problem.outcome();
        }
    };
    let result = tokio::select! {
        result = commands::run(&mut session, extension, &invocation) => result,
        _ = tokio::signal::ctrl_c() => Err(interrupted()),
    };
    let outcome = match result {
        Ok(mut reply) => {
            reply.warnings.extend(session.setting_warnings.clone());
            reply.notes.extend(session.notes.clone());
            session.presentation.show(&reply);
            Outcome::Success
        }
        Err(problem) => {
            session.presentation.warn(&session.setting_warnings);
            session.presentation.fail(&problem);
            problem.outcome()
        }
    };
    remember(&mut session, &invocation.path, started, outcome).await;
    session.close().await;
    outcome
}

/// Records which command ran, how long it took, and how it ended — locally, only while
/// telemetry is on, and never the command's arguments (FR-052 to FR-054).
///
/// Telemetry must never get in the way: when it cannot be written (no workspace, a
/// workspace that may not be changed, a disk that cannot be written to) nothing is said
/// unless `--verbose` was given.
async fn remember(session: &mut Session, path: &[String], started: Instant, outcome: Outcome) {
    if !session.telemetry_enabled() || path.is_empty() || session.located().is_err() {
        return;
    }
    let words: Vec<&str> = path.iter().map(String::as_str).collect();
    let duration = u64::try_from(started.elapsed().as_millis()).unwrap_or(u64::MAX);
    let record = TelemetryRecord::new(session.clock.now(), &words, duration, outcome.name());
    let written = async {
        let storage = session.storage(Access::Write).await?;
        let mut unit = storage.begin().await?;
        record_use(&mut unit, true, &record).await?;
        unit.commit().await?;
        Ok::<(), Problem>(())
    }
    .await;
    if let Err(problem) = written {
        session
            .diagnostics
            .line(|| format!("telemetry was not recorded: {}", problem.message));
    }
}

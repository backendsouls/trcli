//! The handlers: parsed arguments in, a view model out (FR-069).
//!
//! Each handler does the same four things and nothing else: builds a validated command
//! from what was typed; opens a unit of work; calls one use case of the application
//! layer; commits. Deciding is the application's business and showing is the renderer's.

pub mod audit;
pub mod completions;
pub mod config;
pub mod first_steps;
pub mod link;
pub mod workspace;

use trcli_application::governance::record::commit;
use trcli_application::outcome::Problem;

use crate::args::Commands;
use crate::cli::{Invocation, Parsed};
use crate::compose::{AppUnit, Session};
use crate::output::Reply;

/// Runs the command that was parsed.
pub async fn run(session: &mut Session, invocation: &Invocation) -> Result<Reply, Problem> {
    match &invocation.parsed {
        Parsed::Nothing => first_steps::run(session).await,
        Parsed::Foundation(command) => foundation(session, command).await,
        Parsed::Kind { noun, matches } => kind(session, noun, matches).await,
    }
}

/// Runs one of the foundation's own commands.
async fn foundation(session: &mut Session, command: &Commands) -> Result<Reply, Problem> {
    match command {
        Commands::Init(arguments) => workspace::init(session, arguments).await,
        Commands::Workspace(command) => workspace::run(session, command).await,
        Commands::Config(command) => config::run(session, command).await,
        Commands::Link(command) => link::run(session, command).await,
        Commands::Tag(command) => link::tags(session, command).await,
        Commands::Audit(command) => audit::run(session, command).await,
        Commands::Telemetry(command) => audit::telemetry(session, command).await,
        Commands::Completions(arguments) => Ok(completions::run(arguments)),
    }
}

/// Runs a command of a record kind, handing it to the feature that owns the kind.
#[cfg(feature = "sample-kind")]
async fn kind(session: &mut Session, noun: &str, matches: &clap::ArgMatches) -> Result<Reply, Problem> {
    crate::sample::run(session, noun, matches).await
}

/// This build has no kind of record: clap accepts no such command, so this is never
/// reached; it exists so that the dispatch above is the same in every build.
#[cfg(not(feature = "sample-kind"))]
async fn kind(_session: &mut Session, noun: &str, _matches: &clap::ArgMatches) -> Result<Reply, Problem> {
    Err(Problem::internal(format!("no feature of this build owns the command `{noun}`")))
}

/// Commits a unit of work and stores the new end of the audit trail.
pub async fn finish(session: &Session, unit: AppUnit) -> Result<(), Problem> {
    commit(unit, &session.head()?).await
}

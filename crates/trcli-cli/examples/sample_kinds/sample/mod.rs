//! The sample feature: what it adds to the tool, in the three parts every feature has.

pub mod application;
pub mod commands;
pub mod storage;

use clap::{ArgMatches, Command};
use trcli_application::kinds::{KindError, KindRegistry};
use trcli_application::outcome::Problem;
use trcli_application::workspace::open::Access;
use trcli_cli::compose::Session;
use trcli_cli::extension::Extension;
use trcli_cli::output::Reply;

/// The two sample kinds of record, as an extension of the tool.
#[derive(Clone, Copy, Debug, Default)]
pub struct SampleKinds;

impl Extension for SampleKinds {
    fn register_kinds(&self, kinds: &mut KindRegistry) -> Result<(), KindError> {
        application::register(kinds)
    }

    fn commands(&self) -> Vec<Command> {
        vec![
            commands::specimen::command(),
            commands::sample_note::command(),
        ]
    }

    async fn run(
        &self,
        session: &mut Session,
        noun: &str,
        matches: &ArgMatches,
    ) -> Result<Reply, Problem> {
        // A feature built into the tool has its tables created by a migration. This one
        // lives outside, so it creates its own the first time it is used in a workspace
        // (one that may be opened at all: too new or damaged is refused here).
        let storage = session.storage(Access::Read).await?;
        storage::ensure_tables(&storage).await?;
        commands::run(session, noun, matches).await
    }
}

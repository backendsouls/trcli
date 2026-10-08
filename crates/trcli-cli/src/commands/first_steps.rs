//! `trcli` with no arguments: the short help, and — in a workspace with nothing in it
//! yet — the first things to do (FR-057, FR-060).

use trcli_application::outcome::Problem;
use trcli_application::ports::records::RecordIndex;
use trcli_application::ports::unit_of_work::Storage;
use trcli_application::workspace::open::Access;

use crate::cli::command;
use crate::compose::Session;
use crate::output::Reply;
use crate::render::views::Text;

/// What a researcher can do first in a new workspace.
const FIRST_STEPS: &str = "\
First steps in this workspace:
  trcli workspace show                            see what the workspace holds
  trcli workspace edit --researcher \"<your name>\"  record your actions under your name
  trcli config list                               see every setting and where it comes from
  trcli audit list                                see everything that has happened so far";

/// What a researcher can do first without a workspace.
const NO_WORKSPACE: &str = "\
There is no workspace here yet. Create one in this directory with:
  trcli init --name \"<name>\"";

/// Whether the workspace that was found has no record of any kind.
async fn workspace_is_empty(session: &mut Session) -> Result<bool, Problem> {
    let storage = session.storage(Access::Read).await?;
    let counts = storage.read().await?.count_by_kind().await?;
    Ok(counts.iter().all(|(_, count)| *count == 0))
}

/// Shows the short help, never failing: asking what the tool can do must always work.
pub async fn run(session: &mut Session) -> Result<Reply, Problem> {
    let mut text = command().render_help().to_string();
    let hint = if session.located().is_err() {
        Some(NO_WORKSPACE)
    } else {
        // A workspace that cannot be opened is reported by the commands that need it;
        // here it only means there are no first steps to suggest.
        workspace_is_empty(session).await.unwrap_or(false).then_some(FIRST_STEPS)
    };
    if let Some(hint) = hint {
        text.push('\n');
        text.push_str(hint);
        text.push('\n');
    }
    Ok(Reply::new(Text { text }))
}

//! Viewing a workspace: its details and how many records of each kind it holds (FR-006).

use serde::Serialize;

use super::locate::{FoundBy, Located};
use crate::kinds::KindRegistry;
use crate::outcome::Problem;
use crate::ports::records::RecordIndex;
use crate::ports::workspace::WorkspaceStore;
use crate::view::Instant;

/// How many records of one kind a workspace holds.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct KindCount {
    /// The kind's name.
    pub kind: String,
    /// How many records of it exist.
    pub count: u64,
}

/// A workspace as `workspace show` presents it.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct WorkspaceView {
    /// The workspace's name.
    pub name: String,
    /// What it is for; empty when not said.
    pub description: String,
    /// The directory that holds it.
    pub location: String,
    /// The format it is stored in.
    pub format_version: u32,
    /// When it was created.
    pub created_at: Instant,
    /// Whether it was found because it is the researcher's default (FR-003).
    pub is_default: bool,
    /// The number of records of each kind, registered kinds first.
    pub records: Vec<KindCount>,
}

/// Reads the workspace's details and counts its records.
pub async fn show<U>(
    unit: &U,
    located: &Located,
    kinds: &KindRegistry,
) -> Result<WorkspaceView, Problem>
where
    U: WorkspaceStore + RecordIndex,
{
    let workspace = unit.workspace().await?;
    let stored = unit.count_by_kind().await?;
    let count_of = |kind: &str| {
        stored
            .iter()
            .find(|(name, _)| name == kind)
            .map_or(0, |(_, count)| *count)
    };
    // Every registered kind is shown, with zero when it has no record; kinds found in
    // storage that this build does not know come after, so that nothing is hidden.
    let mut records: Vec<KindCount> = kinds
        .names()
        .into_iter()
        .map(|kind| KindCount {
            kind: kind.to_owned(),
            count: count_of(kind),
        })
        .collect();
    for (kind, count) in &stored {
        if kinds.by_name(kind).is_none() {
            records.push(KindCount {
                kind: kind.clone(),
                count: *count,
            });
        }
    }
    Ok(WorkspaceView {
        name: workspace.name.to_string(),
        description: workspace.description.to_string(),
        location: located.root.display().to_string(),
        format_version: workspace.format_version.number(),
        created_at: workspace.created_at.into(),
        is_default: located.found_by == FoundBy::Default,
        records,
    })
}

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

#[cfg(test)]
mod tests {
    //! Unit tests for viewing a workspace (T048, and the counts added by T075).

    use std::path::PathBuf;

    use trcli_domain::shared::text::{LongText, Name};
    use trcli_domain::workspace::Workspace;

    use super::{KindCount, show};
    use crate::kinds::{KindRegistry, RecordKindDescriptor};
    use crate::ports::records::RecordIndex;
    use crate::ports::unit_of_work::Storage;
    use crate::ports::workspace::WorkspaceStore;
    use crate::testing::block_on;
    use crate::testing::contract_records::indexed;
    use crate::testing::environment::{SeededIds, stamp};
    use crate::testing::unit::{FakeStorage, FakeUnit};
    use crate::workspace::locate::{FoundBy, Located};

    /// A unit holding a workspace named "Doctorate".
    fn unit() -> FakeUnit {
        block_on(async {
            let mut unit = FakeStorage::new().begin().await.expect("begin");
            let (name, about) = (
                Name::new("Doctorate").expect("valid"),
                LongText::new("Thesis work").expect("valid"),
            );
            unit.save_workspace(&Workspace::new(SeededIds::at(0), name, about, stamp().at))
                .await
                .expect("save");
            unit
        })
    }

    /// Where the workspace was found.
    fn located(found_by: FoundBy) -> Located {
        Located {
            root: PathBuf::from("/research"),
            found_by,
        }
    }

    /// A registry with the kinds `alpha` and `beta`.
    fn kinds() -> KindRegistry {
        let mut registry = KindRegistry::new();
        registry
            .register(RecordKindDescriptor::new("alpha", "alp", "Alpha.", "title"))
            .expect("registered");
        registry
            .register(RecordKindDescriptor::new("beta", "bet", "Beta.", "title"))
            .expect("registered");
        registry
    }

    #[test]
    fn show_returns_name_description_location_and_format_version() {
        let view = block_on(show(
            &unit(),
            &located(FoundBy::Ancestor),
            &KindRegistry::new(),
        ))
        .expect("shown");
        assert_eq!(
            (view.name.as_str(), view.description.as_str()),
            ("Doctorate", "Thesis work")
        );
        assert_eq!(
            (view.location.as_str(), view.format_version),
            ("/research", 1)
        );
        assert!(!view.is_default);
        assert!(view.records.is_empty());
    }

    #[test]
    fn show_says_when_the_default_workspace_was_used() {
        let view = block_on(show(
            &unit(),
            &located(FoundBy::Default),
            &KindRegistry::new(),
        ))
        .expect("shown");
        assert!(view.is_default);
    }

    #[test]
    fn show_counts_records_per_kind_including_kinds_with_none() {
        let mut unit = unit();
        block_on(async {
            unit.insert_record(&indexed(0, "alpha", "alp", "One"))
                .await
                .expect("insert");
            unit.insert_record(&indexed(1, "alpha", "alp", "Two"))
                .await
                .expect("insert");
            unit.insert_record(&indexed(2, "gamma", "gam", "Unknown kind"))
                .await
                .expect("insert");
        });
        let view = block_on(show(&unit, &located(FoundBy::Ancestor), &kinds())).expect("shown");
        let count = |kind: &str, count| KindCount {
            kind: kind.to_owned(),
            count,
        };
        assert_eq!(
            view.records,
            vec![count("alpha", 2), count("beta", 0), count("gamma", 1)]
        );
    }
}

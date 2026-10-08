//! Checking the audit trail for tampering (FR-048).
//!
//! Verification walks the trail from its first entry. For each entry: its sequence must
//! be the one after the last (no gap), its `previous_hash` must equal the hash before it,
//! and its `hash` must equal the one recomputed from its content. At the end, the trail
//! must reach at least as far as the head file says, and the entry the head names must
//! have the hash the head holds. The first failure is reported with its sequence and
//! which check failed.
//!
//! The limit, stated honestly: someone who rewrites the whole chain *and* the head file
//! consistently is not detected; that needs a reference kept outside the workspace.

use trcli_domain::governance::audit::{AuditEntry, AuditHead, Digest, GENESIS_HASH};

use crate::outcome::{Problem, codes};
use crate::ports::audit::{AuditDigest, AuditQuery};
use crate::ports::interaction::Progress;
use crate::ports::unit_of_work::StoreError;

/// How many entries are read at a time, so that a long trail is never held in memory.
const BATCH: u32 = 500;

/// Which check an entry failed.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fault {
    /// The entry's content does not match its hash, or it does not follow the entry before
    /// it: something was altered.
    Altered,
    /// An entry that should be here is not: the sequence has a gap.
    Missing,
    /// The trail ends before the point the head file remembers: its end was removed.
    Shortened,
    /// There are entries and no head file: whether the end was removed cannot be told.
    HeadMissing,
}

/// The first thing found wrong with the trail.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Finding {
    /// The sequence of the entry where the problem is.
    pub sequence: u64,
    /// Which check failed.
    pub fault: Fault,
}

impl Finding {
    /// The finding in words.
    pub fn describe(&self) -> String {
        let sequence = self.sequence;
        match self.fault {
            Fault::Altered => format!("entry {sequence} was altered: its content no longer matches the chain"),
            Fault::Missing => format!("entry {sequence} is missing: the trail has a gap"),
            Fault::Shortened => format!("the trail was shortened: it should reach entry {sequence}"),
            Fault::HeadMissing => {
                "the file .trcli/audit.head is missing, so a removal at the end of the trail cannot be ruled out"
                    .to_owned()
            }
        }
    }

    /// The problem that reports this finding (exit code 6).
    pub fn into_problem(self) -> Problem {
        Problem::new(
            codes::CHECK_FAILED,
            "the audit trail has been tampered with",
        )
        .with_items(vec![self.describe()])
        .with_next_step(
            "compare with a backup of the workspace; `trcli audit list` still shows what remains",
        )
    }
}

/// The result of verifying the trail.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TrailVerdict {
    /// Every check passed.
    Intact {
        /// How many entries were verified.
        entries: u64,
    },
    /// A check failed; this is the first failure.
    Broken(Finding),
}

/// The walk's memory of the entry before the one being checked.
struct Walk {
    /// The sequence of the last entry checked; 0 before the first.
    sequence: u64,
    /// The hash of the last entry checked; zeros before the first.
    hash: Digest,
    /// The hash of the entry the head names, once the walk has passed it.
    hash_at_head: Option<Digest>,
}

impl Walk {
    /// Checks one entry against the one before it; returns the fault if it fails.
    fn step(
        &mut self,
        entry: &AuditEntry,
        head: Option<&AuditHead>,
        digest: &impl AuditDigest,
    ) -> Option<Finding> {
        let expected = self.sequence + 1;
        if entry.sequence != expected {
            // A lower or repeated sequence is an alteration; a higher one is a gap.
            let fault = if entry.sequence > expected {
                Fault::Missing
            } else {
                Fault::Altered
            };
            return Some(Finding {
                sequence: expected,
                fault,
            });
        }
        let follows = entry.previous_hash == self.hash;
        let matches = entry.expected_hash(|bytes| digest.digest(bytes)) == entry.hash;
        if !(follows && matches) {
            return Some(Finding {
                sequence: entry.sequence,
                fault: Fault::Altered,
            });
        }
        self.sequence = entry.sequence;
        self.hash = entry.hash;
        if head.is_some_and(|head| head.sequence == entry.sequence) {
            self.hash_at_head = Some(entry.hash);
        }
        None
    }

    /// Compares the end of the walk with the head file.
    fn conclude(&self, head: Option<&AuditHead>) -> TrailVerdict {
        let intact = TrailVerdict::Intact {
            entries: self.sequence,
        };
        match head {
            None if self.sequence == 0 => intact,
            None => TrailVerdict::Broken(Finding {
                sequence: self.sequence,
                fault: Fault::HeadMissing,
            }),
            Some(head) if head.sequence > self.sequence => TrailVerdict::Broken(Finding {
                sequence: head.sequence,
                fault: Fault::Shortened,
            }),
            // The trail may be longer than the head says: a command was stopped between
            // committing and writing the head. The entry the head names must still match.
            Some(head) if self.hash_at_head == Some(head.hash) => intact,
            Some(head) => TrailVerdict::Broken(Finding {
                sequence: head.sequence,
                fault: Fault::Altered,
            }),
        }
    }
}

/// Verifies the whole trail against `head`, reporting progress for a long one.
pub async fn verify<U, D, P>(
    unit: &U,
    head: Option<AuditHead>,
    digest: &D,
    progress: &mut P,
) -> Result<TrailVerdict, StoreError>
where
    U: AuditQuery,
    D: AuditDigest,
    P: Progress,
{
    let total = unit.entry_count().await?;
    let mut walk = Walk {
        sequence: 0,
        hash: GENESIS_HASH,
        hash_at_head: None,
    };
    let mut checked = 0_u64;
    progress.start("Verifying the audit trail");
    loop {
        // Entries are read by position after the last one *read*, so that a gap in the
        // sequences is seen as a gap rather than ending the walk early.
        let batch = unit.entries_after(walk.sequence, BATCH).await?;
        if batch.is_empty() {
            break;
        }
        for entry in &batch {
            if let Some(finding) = walk.step(entry, head.as_ref(), digest) {
                progress.finish();
                return Ok(TrailVerdict::Broken(finding));
            }
        }
        checked += batch.len() as u64;
        progress.advance(checked, Some(total));
    }
    progress.finish();
    Ok(walk.conclude(head.as_ref()))
}

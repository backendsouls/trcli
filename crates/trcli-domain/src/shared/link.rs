//! Links: free connections between any two records (FR-016).
//!
//! A link has no direction for the researcher: it is shown from both ends, and a link
//! between two records with a given relation exists at most once whichever end it was made
//! from. Relationships that carry rules of their own belong to the features that define
//! them, not here.

use time::OffsetDateTime;

use super::record::RecordId;
use super::text::Relation;

/// Why two records cannot be linked.
#[derive(Clone, Copy, Debug, PartialEq, Eq, thiserror::Error)]
pub enum LinkError {
    /// Both ends are the same record.
    #[error("a record cannot be linked to itself")]
    SameRecord,
}

/// A connection between two different records.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Link {
    /// The end the link was made from.
    pub from: RecordId,
    /// The other end.
    pub to: RecordId,
    /// How the two relate.
    pub relation: Relation,
    /// When the link was made.
    pub created_at: OffsetDateTime,
}

impl Link {
    /// Links two records, refusing to link a record to itself.
    pub fn new(
        from: RecordId,
        to: RecordId,
        relation: Relation,
        created_at: OffsetDateTime,
    ) -> Result<Self, LinkError> {
        if from == to {
            return Err(LinkError::SameRecord);
        }
        Ok(Self {
            from,
            to,
            relation,
            created_at,
        })
    }

    /// Whether this link joins the same two records with the same relation as `other`,
    /// in either direction. Two such links may not both exist.
    pub fn is_same_as(&self, other: &Link) -> bool {
        self.relation == other.relation && self.joins(other.from, other.to)
    }

    /// Whether the link joins these two records, whichever end each is at.
    pub fn joins(&self, one: RecordId, other: RecordId) -> bool {
        (self.from == one && self.to == other) || (self.from == other && self.to == one)
    }

    /// The record at the other end from `record`, when `record` is one of the ends.
    pub fn other_end(&self, record: RecordId) -> Option<RecordId> {
        if self.from == record {
            Some(self.to)
        } else if self.to == record {
            Some(self.from)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    //! Unit tests for the rules of links (T062).

    use time::OffsetDateTime;

    use super::{Link, LinkError};
    use crate::shared::record::RecordId;
    use crate::shared::text::Relation;

    /// A link between two numbered records with the named relation.
    fn link(from: u128, to: u128, relation: &str) -> Result<Link, LinkError> {
        let relation = Relation::new(relation).expect("valid relation");
        Link::new(
            RecordId::from_u128(from),
            RecordId::from_u128(to),
            relation,
            OffsetDateTime::UNIX_EPOCH,
        )
    }

    #[test]
    fn a_record_cannot_be_linked_to_itself() {
        assert_eq!(link(1, 1, "related"), Err(LinkError::SameRecord));
    }

    #[test]
    fn a_link_and_its_reverse_are_the_same_link() {
        let forward = link(1, 2, "related").expect("valid");
        let reverse = link(2, 1, "related").expect("valid");
        assert!(forward.is_same_as(&reverse));
        assert!(forward.is_same_as(&forward.clone()));
    }

    #[test]
    fn a_different_relation_or_record_makes_a_different_link() {
        let forward = link(1, 2, "related").expect("valid");
        assert!(!forward.is_same_as(&link(1, 2, "cites").expect("valid")));
        assert!(!forward.is_same_as(&link(1, 3, "related").expect("valid")));
    }

    #[test]
    fn a_link_is_seen_from_both_ends() {
        let link = link(1, 2, "related").expect("valid");
        assert_eq!(
            link.other_end(RecordId::from_u128(1)),
            Some(RecordId::from_u128(2))
        );
        assert_eq!(
            link.other_end(RecordId::from_u128(2)),
            Some(RecordId::from_u128(1))
        );
        assert_eq!(link.other_end(RecordId::from_u128(3)), None);
    }
}

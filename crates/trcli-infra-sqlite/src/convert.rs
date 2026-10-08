//! Conversions between the domain's values and what the tables hold, and between the
//! database's errors and the application's.
//!
//! Identifiers are stored as hyphenated text and instants as whole milliseconds since
//! 1970 in UTC: both sort correctly as stored and come back exactly as written, which the
//! audit trail's hashes depend on.

use sea_orm::DbErr;
use time::OffsetDateTime;
use trcli_application::ports::unit_of_work::StoreError;
use trcli_domain::shared::record::RecordId;

/// A moment as the tables hold it.
pub(crate) fn to_millis(moment: OffsetDateTime) -> i64 {
    (moment.unix_timestamp_nanos() / 1_000_000) as i64
}

/// A moment read from a table.
pub(crate) fn from_millis(milliseconds: i64) -> Result<OffsetDateTime, StoreError> {
    OffsetDateTime::from_unix_timestamp_nanos(i128::from(milliseconds) * 1_000_000)
        .map_err(|_| corrupt("a stored moment is out of range"))
}

/// An identifier read from a table.
pub(crate) fn to_id(text: &str) -> Result<RecordId, StoreError> {
    RecordId::parse(text).ok_or_else(|| corrupt("a stored identifier is not an identifier"))
}

/// The error for stored data that no version of the tool could have written.
pub(crate) fn corrupt(what: &str) -> StoreError {
    StoreError::Damaged {
        what: what.to_owned(),
        location: "the workspace's database".to_owned(),
    }
}

/// Turns a database error into the application's vocabulary.
///
/// SQLite reports what went wrong in words; the cases a researcher can act on are told
/// apart here, once, so that no store has to.
pub(crate) fn store_error(error: DbErr) -> StoreError {
    let text = error.to_string();
    let lowered = text.to_lowercase();
    if lowered.contains("database is locked") || lowered.contains("database table is locked") {
        StoreError::Busy
    } else if lowered.contains("readonly database") || lowered.contains("read-only") {
        StoreError::ReadOnly(text)
    } else if lowered.contains("database or disk is full") {
        StoreError::ReadOnly("the disk is full".to_owned())
    } else if lowered.contains("constraint failed") {
        StoreError::Constraint(text)
    } else if lowered.contains("not a database") || lowered.contains("malformed") {
        corrupt("the database file is not a valid database")
    } else {
        StoreError::Failed(text)
    }
}

#[cfg(test)]
mod tests {
    //! Unit tests for the conversions.

    use sea_orm::DbErr;
    use time::OffsetDateTime;
    use trcli_application::ports::unit_of_work::StoreError;

    use super::{from_millis, store_error, to_id, to_millis};

    #[test]
    fn a_moment_round_trips_to_the_millisecond() {
        let moment =
            OffsetDateTime::from_unix_timestamp_nanos(1_791_468_000_123_000_000).expect("valid");
        assert_eq!(from_millis(to_millis(moment)), Ok(moment));
    }

    #[test]
    fn a_stored_identifier_must_be_one() {
        assert!(to_id("0192aaaa-0000-7000-8000-000000000001").is_ok());
        assert!(matches!(to_id("nonsense"), Err(StoreError::Damaged { .. })));
    }

    #[test]
    fn database_errors_a_researcher_can_act_on_are_told_apart() {
        let error = |text: &str| store_error(DbErr::Custom(text.to_owned()));
        assert_eq!(
            error("error returned from database: (code: 5) database is locked"),
            StoreError::Busy
        );
        assert!(matches!(
            error("attempt to write a readonly database"),
            StoreError::ReadOnly(_)
        ));
        assert!(matches!(
            error("FOREIGN KEY constraint failed"),
            StoreError::Constraint(_)
        ));
        assert!(matches!(
            error("file is not a database"),
            StoreError::Damaged { .. }
        ));
        assert!(matches!(error("something else"), StoreError::Failed(_)));
    }
}

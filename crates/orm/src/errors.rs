
use thiserror::Error;

/// Errors returned by Umbriel.
#[derive(Error, Debug)]
#[non_exhaustive]
pub enum Error {
    /// The query could not be built from its inputs.
    #[error("invalid query: {0}")]
    QueryBuilder(String),

    /// More records were returned than the operation allows.
    #[error("expected at most {expected} record(s), but more were returned")]
    TooManyItemsReturned {
        /// The maximum number of records the operation accepts.
        expected: usize,
    },

    /// The database rejected or failed to run the query.
    #[error("query failed: {0}")]
    Query(#[source] surrealdb::Error),

    /// A value returned by the database did not convert into the requested type.
    #[error("unexpected value from the database: {0}")]
    Conversion(#[source] surrealdb::Error),

    /// Fields requested for fetching are not record links.
    #[error(
        "these fields are not record links and cannot be fetched: {0}; only `link_self`, `link_one` and `link_many` fields can be"
    )]
    FieldsUnfetchableNotARecordLink(String),

    /// No record was returned where one was required.
    #[error("no record returned from {0}")]
    RecordNotFound(String),

    /// A migration failed.
    #[cfg(feature = "migrations")]
    #[error(transparent)]
    Migration(Box<crate::migrations::MigrationError>),
}

#[cfg(feature = "migrations")]
impl From<crate::migrations::MigrationError> for Error {
    fn from(error: crate::migrations::MigrationError) -> Self {
        Self::Migration(Box::new(error))
    }
}

/// The result type returned by Umbriel.
pub type Result<T, E = Error> = std::result::Result<T, E>;

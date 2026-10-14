use surrealdb::types::{Datetime, Table};

use crate::Renderer;

use super::{Statement, impl_statement};

/// Creates a SHOW CHANGES statement builder, reading a table's changes from `since` on.
///
/// Examples
/// ```rust
/// use umbriel::{*, statements::show_changes_for_table};
///
/// let statement = show_changes_for_table("reading", 5u64).limit(10);
/// assert_eq!(statement.to_query().unwrap().sql(), "SHOW CHANGES FOR TABLE reading SINCE 5 LIMIT 10;");
///
/// let timestamp = chrono::DateTime::from_timestamp(1_700_000_000, 0).unwrap();
/// let statement = show_changes_for_table("reading", timestamp);
/// assert_eq!(
///     statement.to_debug_sql(),
///     "SHOW CHANGES FOR TABLE reading SINCE d'2023-11-14T22:13:20Z';"
/// );
/// ```
pub fn show_changes_for_table(
    table: impl Into<Table>,
    since: impl Into<ChangesSince>,
) -> ShowChangesStatement {
    ShowChangesStatement {
        table: table.into(),
        since: since.into(),
        limit: None,
    }
}

/// Where `SHOW CHANGES` starts reading. SurrealQL takes it, and the limit, only as literals.
#[derive(Debug, Clone)]
pub enum ChangesSince {
    /// Changes at or after a point in time.
    Timestamp(Datetime),
    /// Changes at or after a versionstamp.
    Versionstamp(u64),
}

impl From<Datetime> for ChangesSince {
    fn from(timestamp: Datetime) -> Self {
        Self::Timestamp(timestamp)
    }
}

impl From<chrono::DateTime<chrono::Utc>> for ChangesSince {
    fn from(timestamp: chrono::DateTime<chrono::Utc>) -> Self {
        Self::Timestamp(timestamp.into())
    }
}

impl From<u64> for ChangesSince {
    fn from(versionstamp: u64) -> Self {
        Self::Versionstamp(versionstamp)
    }
}

/// Represents the initial state for building a SHOW CHANGES statement.
pub struct ShowChangesStatement {
    table: Table,
    since: ChangesSince,
    limit: Option<u32>,
}

impl ShowChangesStatement {
    /// Sets the LIMIT clause for the SHOW CHANGES statement.
    pub fn limit(mut self, limit: u32) -> ShowChangesStatement {
        self.limit = Some(limit);
        self
    }
}

impl Statement for ShowChangesStatement {
    fn render_body(&self, out: &mut Renderer) {
        out.push_str("SHOW CHANGES FOR TABLE ")
            .push(&self.table)
            .push_str(" SINCE ");
        match &self.since {
            ChangesSince::Timestamp(timestamp) => out.push_syntax(timestamp),
            ChangesSince::Versionstamp(versionstamp) => out.push_display(versionstamp),
        };
        if let Some(limit) = self.limit {
            out.push_str(" LIMIT ").push_display(&limit);
        }
    }
}

impl_statement!(ShowChangesStatement);

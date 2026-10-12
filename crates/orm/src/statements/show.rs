use surrealdb::types::Table;

use crate::{DatetimeLike, NumberLike, Renderer};

use super::{Statement, impl_statement};

/// Creates a SHOW CHANGES statement builder.
///
/// Examples
/// ```rust
/// use umbriel::{*, statements::show_changes_for_table};
///
/// let timestamp = chrono::DateTime::from_timestamp(4343434434, 232).unwrap();
/// let statement = show_changes_for_table("reading").since(timestamp).limit(10);
/// let query = statement.to_query().unwrap();
/// assert!(query.sql().starts_with("SHOW CHANGES FOR TABLE reading SINCE d'"));
/// assert!(query.sql().ends_with("' LIMIT $_p0;"));
/// ```
pub fn show_changes_for_table(table: impl Into<Table>) -> ShowChangesStatement {
    ShowChangesStatement {
        table: table.into(),
        since: None,
        limit: None,
    }
}

/// Represents the initial state for building a SHOW CHANGES statement.
pub struct ShowChangesStatement {
    table: Table,
    since: Option<DatetimeLike>,
    limit: Option<NumberLike>,
}

impl ShowChangesStatement {
    /// Sets the SINCE clause for the SHOW CHANGES statement. It is written as a literal.
    pub fn since(mut self, timestamp: impl Into<DatetimeLike>) -> ShowChangesStatement {
        self.since = Some(timestamp.into());
        self
    }

    /// Sets the LIMIT clause for the SHOW CHANGES statement.
    pub fn limit(mut self, limit: impl Into<NumberLike>) -> ShowChangesStatement {
        self.limit = Some(limit.into());
        self
    }
}

impl Statement for ShowChangesStatement {
    fn render_body(&self, out: &mut Renderer) {
        out.push_str("SHOW CHANGES FOR TABLE ").push(&self.table);
        if let Some(timestamp) = &self.since {
            out.push_str(" SINCE ").push_inlined(timestamp);
        }
        if let Some(limit) = &self.limit {
            out.push_str(" LIMIT ").push(limit);
        }
    }
}

impl_statement!(ShowChangesStatement);

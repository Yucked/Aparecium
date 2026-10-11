use surrealdb::types::Table;

use crate::{Sql, TableIndex};

use super::{Statement, impl_definition};

/// Remove index statement
///
/// # Arguments
/// * `index` - The name of the index to be removed. Can be a string or a TableIndex type.
///
/// # Example
/// ```rust
/// use aparecium::{*, statements::remove_index};
/// # let user = Table::new("user");
/// # let party = TableIndex::new("party");
///
/// let statement = remove_index(party).on_table(user);
/// assert_eq!(statement.to_debug_sql(), "REMOVE INDEX party ON TABLE user;");
/// ```
pub fn remove_index(index: impl Into<TableIndex>) -> RemoveIndexStatement {
    RemoveIndexStatement {
        index: index.into(),
        table: None,
    }
}

/// Remove index statement
pub struct RemoveIndexStatement {
    index: TableIndex,
    table: Option<Table>,
}

impl RemoveIndexStatement {
    /// Set the table to remove the index from
    pub fn on_table(mut self, table: impl Into<Table>) -> Self {
        self.table = Some(table.into());
        self
    }
}

impl Statement for RemoveIndexStatement {
    fn render_body(&self, sql: &mut Sql) {
        sql.push_str("REMOVE INDEX ").push(&self.index);
        if let Some(table) = &self.table {
            sql.push_str(" ON TABLE ").push(table);
        }
    }
}

impl_definition!(RemoveIndexStatement);

use surrealdb::types::Table;

use crate::Sql;

use super::{Statement, impl_definition};

/// Remove table statement
///
/// # Arguments
///
/// * `table` - The name of the table to be removed. Can be a string or a Table type.
///
/// # Example
/// ```rust
/// use aparecium::{*, statements::remove_table};
/// # let table = Table::new("user");
/// let statement = remove_table(table);
/// assert_eq!(statement.to_debug_sql(), "REMOVE TABLE user;");
/// ```
pub fn remove_table(table: impl Into<Table>) -> RemoveTableStatement {
    RemoveTableStatement {
        table: table.into(),
    }
}

/// Remove table statement
pub struct RemoveTableStatement {
    table: Table,
}

impl Statement for RemoveTableStatement {
    fn render_body(&self, sql: &mut Sql) {
        sql.push_str("REMOVE TABLE ").push(&self.table);
    }
}

impl_definition!(RemoveTableStatement);

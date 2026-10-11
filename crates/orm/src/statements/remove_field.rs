use surrealdb::types::Table;

use crate::{Field, Sql};

use super::{Statement, impl_definition};

/// Remove field statement
///
/// # Arguments
/// * `field` - The name of the field to be removed. Can be a string or a Field type.
///
/// # Example
/// ```rust
/// use aparecium::{*, statements::remove_field};
/// # let user = Table::new("user");
/// # let name = Field::new("name");
///
/// let statement = remove_field(name).on_table(user);
/// assert_eq!(statement.to_debug_sql(), "REMOVE FIELD name ON TABLE user;");
/// ```
pub fn remove_field(field: impl Into<Field>) -> RemoveFieldStatement {
    RemoveFieldStatement {
        field: field.into(),
        table: None,
    }
}

/// Remove field statement
pub struct RemoveFieldStatement {
    field: Field,
    table: Option<Table>,
}

impl RemoveFieldStatement {
    /// Set the table to remove the field from
    pub fn on_table(mut self, table: impl Into<Table>) -> Self {
        self.table = Some(table.into());
        self
    }
}

impl Statement for RemoveFieldStatement {
    fn render_body(&self, sql: &mut Sql) {
        sql.push_str("REMOVE FIELD ").push(&self.field);
        if let Some(table) = &self.table {
            sql.push_str(" ON TABLE ").push(table);
        }
    }
}

impl_definition!(RemoveFieldStatement);

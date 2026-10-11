use crate::{Field, Sql, ValueLike};

use super::{Statement, impl_statement, select::Fetchables};

/// The `RETURN` statement.
///
/// # Arguemnt
/// * `return_value` - The value to return.
///
/// # Examples
/// ```
/// use aparecium::{*, statements::*, functions::*, types::RecordId};
///
/// let user = RecordId::new("user", "alice");
/// let account = Field::new("account");
/// let connection = Field::new("connection");
///
/// let query = return_(user).fetch(&[account, connection]);
/// assert_eq!(
///     query.to_debug_sql(),
///     "RETURN user:alice FETCH account, connection;"
/// );
/// ```
pub fn return_(return_value: impl Into<ValueLike>) -> ReturnStatement {
    ReturnStatement {
        return_value: return_value.into(),
        fetch: vec![],
    }
}

/// The `RETURN` statement.
#[derive(Debug, Clone)]
pub struct ReturnStatement {
    return_value: ValueLike,
    fetch: Vec<Field>,
}

impl ReturnStatement {
    /// Fetches the specified fields.
    pub fn fetch(mut self, fetchables: impl Into<Fetchables>) -> Self {
        self.fetch.extend(Vec::<Field>::from(fetchables.into()));
        self
    }
}

impl Statement for ReturnStatement {
    fn render_body(&self, sql: &mut Sql) {
        sql.push_str("RETURN ").push(&self.return_value);
        if !self.fetch.is_empty() {
            sql.push_str(" FETCH ").push(&self.fetch);
        }
    }
}

impl_statement!(ReturnStatement);

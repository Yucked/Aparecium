use crate::Sql;

use super::{Statement, impl_statement};

/// Creates a CONTINUE statement builder.
///
/// Examples
/// ```rust
/// use aparecium::{*, statements::continue_};
///
/// assert_eq!(continue_().to_debug_sql(), "CONTINUE;");
/// ```
pub fn continue_() -> ContinueStatement {
    ContinueStatement
}

/// Represents the CONTINUE statement
pub struct ContinueStatement;

impl Statement for ContinueStatement {
    fn render_body(&self, sql: &mut Sql) {
        sql.push_str("CONTINUE");
    }
}

impl_statement!(ContinueStatement);

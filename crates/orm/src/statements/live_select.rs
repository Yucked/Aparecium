use crate::{Renderer, statements::SelectStatement};

use super::{Statement, impl_statement};

/// Creates a LIVE SELECT statement builder.
///
/// Examples
/// ```rust
/// use umbriel::{*, statements::{live, select}};
/// let person = Table::new("person");
/// let statement = live(select(All).from(person));
/// assert_eq!(statement.to_debug_sql(), "LIVE SELECT * FROM person;");
/// ```
pub fn live(select_statement: SelectStatement) -> LiveSelectStatement {
    LiveSelectStatement {
        select: select_statement,
    }
}

/// Represents the LIVE SELECT statement.
pub struct LiveSelectStatement {
    select: SelectStatement,
}

impl Statement for LiveSelectStatement {
    fn render_body(&self, out: &mut Renderer) {
        out.push_str("LIVE ");
        self.select.render_body(out);
    }
}

impl_statement!(LiveSelectStatement);

use crate::Renderer;

use super::{Statement, impl_statement};

/// Creates a BREAK statement.
///
/// Examples
/// ```rust
/// use umbriel::{*, statements::break_};
/// assert_eq!(break_().to_debug_sql(), "BREAK;");
/// ```
pub fn break_() -> BreakStatement {
    BreakStatement
}

/// Represents the BREAK statement
pub struct BreakStatement;

impl Statement for BreakStatement {
    fn render_body(&self, out: &mut Renderer) {
        out.push_str("BREAK");
    }
}

impl_statement!(BreakStatement);

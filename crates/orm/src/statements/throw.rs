use crate::{Renderer, StrandLike};

use super::{Statement, impl_statement};

/// Creates a THROW statement builder.
///
/// Examples
/// ```rust
/// use umbriel::{*, statements::throw};
/// assert_eq!(throw("some error message").to_debug_sql(), "THROW 'some error message';");
/// ```
pub fn throw(message: impl Into<StrandLike>) -> ThrowStatement {
    ThrowStatement {
        message: message.into(),
    }
}

/// Represents the THROW statement
pub struct ThrowStatement {
    message: StrandLike,
}

impl Statement for ThrowStatement {
    fn render_body(&self, out: &mut Renderer) {
        out.push_str("THROW ").push(&self.message);
    }
}

impl_statement!(ThrowStatement);

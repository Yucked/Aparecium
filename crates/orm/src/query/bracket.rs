use crate::{Render, Renderer, ValueLike};

/// A code block. Surrounds the code with brackets.
#[derive(Debug, Clone, PartialEq)]
pub struct Bracket(ValueLike);

/// Wraps the code in brackets.
pub fn bracket(code: impl Into<ValueLike>) -> Bracket {
    Bracket(code.into())
}

impl Render for Bracket {
    fn render(&self, out: &mut Renderer) {
        out.push_str("(").push(&self.0).push_str(")");
    }
}

impl From<ValueLike> for Bracket {
    fn from(code: ValueLike) -> Self {
        Self(code)
    }
}

impl From<Bracket> for ValueLike {
    fn from(bracket: Bracket) -> Self {
        ValueLike::new(bracket)
    }
}

use crate::{Block, Expression, Param, Renderer};

use super::{Statement, impl_statement};

/// The loop variables of a `FOR` statement.
pub struct ForParam(Vec<Param>);

impl From<Vec<Param>> for ForParam {
    fn from(value: Vec<Param>) -> Self {
        Self(value)
    }
}

impl From<ForParam> for Vec<Param> {
    fn from(value: ForParam) -> Self {
        value.0
    }
}

impl From<&[Param]> for ForParam {
    fn from(value: &[Param]) -> Self {
        Self(value.to_vec())
    }
}

impl From<Param> for ForParam {
    fn from(value: Param) -> Self {
        Self(vec![value])
    }
}

impl From<&Param> for ForParam {
    fn from(value: &Param) -> Self {
        Self(vec![value.clone()])
    }
}

/// A helper function to create a for loop
/// You are encouraged to use for loop within
/// chain! macro or use for_! macro.
/// ```rust, ignore
/// use umbriel::{*, statements::for_};
///
/// let query = for_(vec![param!(name), param!(age)])
///    .in_(vec![1, 2, 3, 4, 5]);
///    .block(
///         chain!(select(All).from(SpaceShip::table()))
///     );
/// ```
pub fn for_(params: impl Into<ForParam>) -> ForIterable {
    ForIterable(params.into().0)
}

/// A `FOR` statement awaiting what to iterate over.
pub struct ForIterable(Vec<Param>);

impl ForIterable {
    /// Iterates over the given array.
    pub fn in_(self, iterable: impl Into<Expression>) -> ForStatementBlock {
        ForStatementBlock {
            params: self.0,
            iterable: iterable.into(),
        }
    }
}

/// A `FOR` statement awaiting its block.
pub struct ForStatementBlock {
    params: Vec<Param>,
    iterable: Expression,
}

impl ForStatementBlock {
    /// Sets the loop body.
    pub fn block(self, block: impl Into<Block>) -> ForLoopStatement {
        ForLoopStatement {
            params: self.params,
            iterable: self.iterable,
            block: block.into(),
        }
    }
}

/// A `FOR` statement.
#[derive(Debug, Clone)]
pub struct ForLoopStatement {
    params: Vec<Param>,
    iterable: Expression,
    block: Block,
}

impl Statement for ForLoopStatement {
    fn render_body(&self, out: &mut Renderer) {
        out.push_str("FOR ")
            .push(&self.params)
            .push_str(" IN ")
            .push(&self.iterable)
            .push_str(" ")
            .push(&self.block);
    }
}

impl_statement!(ForLoopStatement);

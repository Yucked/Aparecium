use crate::{Block, Expression, Param, Renderer};

use super::{Statement, impl_statement};

/// A helper function to create a for loop
/// You are encouraged to use for loop within
/// chain! macro or use for_! macro.
/// ```rust
/// use umbriel::{*, statements::{for_, select}};
///
/// let name = Param::new("name");
/// let query = for_(&name)
///     .in_(vec!["Alice", "Bob"])
///     .block(chain(select(All).from(Table::from("user")).where_(Field::new("name").equal(&name))));
/// assert_eq!(
///     query.to_debug_sql(),
///     "FOR $name IN ['Alice', 'Bob'] {\nSELECT * FROM user WHERE name = $name;\n};"
/// );
/// ```
pub fn for_(param: impl Into<Param>) -> ForIterable {
    ForIterable(param.into())
}

/// A `FOR` statement awaiting what to iterate over.
pub struct ForIterable(Param);

impl ForIterable {
    /// Iterates over the given array.
    pub fn in_(self, iterable: impl Into<Expression>) -> ForStatementBlock {
        ForStatementBlock {
            param: self.0,
            iterable: iterable.into(),
        }
    }
}

/// A `FOR` statement awaiting its block.
pub struct ForStatementBlock {
    param: Param,
    iterable: Expression,
}

impl ForStatementBlock {
    /// Sets the loop body.
    pub fn block(self, block: impl Into<Block>) -> ForLoopStatement {
        ForLoopStatement {
            param: self.param,
            iterable: self.iterable,
            block: block.into(),
        }
    }
}

/// A `FOR` statement. SurrealQL loops over one parameter.
#[derive(Debug, Clone)]
pub struct ForLoopStatement {
    param: Param,
    iterable: Expression,
    block: Block,
}

impl Statement for ForLoopStatement {
    fn render_body(&self, out: &mut Renderer) {
        out.push_str("FOR ")
            .push(&self.param)
            .push_str(" IN ")
            .push(&self.iterable)
            .push_str(" ")
            .push(&self.block);
    }
}

impl_statement!(ForLoopStatement);

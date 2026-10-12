
use crate::{Render, Renderer, ValueLike, statements::Subquery};

/// An expression is a value or statement that can be used within another query.
#[derive(Clone, Debug)]
pub enum Expression {
    /// A value or expression.
    Value(ValueLike),
    /// A nested statement.
    Subquery(Subquery),
}

impl Render for Expression {
    fn render(&self, out: &mut Renderer) {
        match self {
            Expression::Value(value) => value.render(out),
            Expression::Subquery(subquery) => subquery.render(out),
        }
    }
}

impl<T> From<T> for Expression
where
    T: Into<ValueLike>,
{
    fn from(value: T) -> Self {
        Expression::Value(value.into())
    }
}

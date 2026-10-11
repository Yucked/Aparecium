
use crate::{Render, Sql, ValueLike, statements::Subquery};

/// An expression is a value or statement that can be used within another query.
#[derive(Clone, Debug)]
pub enum Expression {
    /// A value or expression.
    Value(ValueLike),
    /// A nested statement.
    Subquery(Subquery),
}

impl Render for Expression {
    fn render(&self, sql: &mut Sql) {
        match self {
            Expression::Value(value) => value.render(sql),
            Expression::Subquery(subquery) => subquery.render(sql),
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

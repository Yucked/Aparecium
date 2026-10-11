
use std::ops::{Add, Div, Mul, Sub};

use crate::{Field, NumberLike, Operation, Render, Sql, ValueLike, statements::LetStatement};

/// `lhs <operator> rhs`, with an operation on the right parenthesized.
fn arithmetic(lhs: Sql, operator: &str, rhs: impl Into<NumberLike>) -> Operation {
    let rhs = ValueLike::from(rhs.into());
    let mut sql = lhs;
    sql.push_str(" ").push_str(operator).push_str(" ");
    rhs.render_operand(&mut sql);
    Operation::new(sql)
}

macro_rules! impl_arithmetic {
    ($lhs:ty, |$this:ident| $render:expr => $($trait:ident :: $method:ident ($operator:literal)),* $(,)?) => {
        $(
            impl<T: Into<NumberLike>> $trait<T> for $lhs {
                type Output = Operation;

                fn $method(self, rhs: T) -> Self::Output {
                    let $this = self;
                    arithmetic($render, $operator, rhs)
                }
            }
        )*
    };
}

impl_arithmetic!(LetStatement, |this| this.get_param().to_sql() =>
    Add::add("+"), Mul::mul("*"), Div::div("/"));
impl_arithmetic!(&LetStatement, |this| this.get_param().to_sql() =>
    Add::add("+"), Sub::sub("-"), Mul::mul("*"), Div::div("/"));
impl_arithmetic!(Field, |this| this.to_sql() =>
    Add::add("+"), Sub::sub("-"), Mul::mul("*"), Div::div("/"));
impl_arithmetic!(&Field, |this| this.to_sql() => Add::add("+"));
// An operation on the left is parenthesized so it binds before the new operator.
impl_arithmetic!(Operation, |this| {
    let mut sql = Sql::new();
    sql.push_str("(").push(&this).push_str(")");
    sql
} => Add::add("+"), Sub::sub("-"), Mul::mul("*"), Div::div("/"));

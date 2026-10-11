
use crate::{Block, Edge, Model, Node, Render, Sql};

use super::{
    CreateStatement, DeleteStatement, IfElseStatement, InsertStatement, RelateStatement,
    ReturnStatement, SelectStatement, Statement, UpdateStatement,
};

/// A statement nested inside another query, rendered in parentheses with its own values.
#[derive(Debug, Clone, PartialEq)]
pub struct Subquery(Sql);

impl Subquery {
    fn of(statement: &impl Statement) -> Self {
        let mut sql = Sql::new();
        sql.push_str("(");
        statement.render_body(&mut sql);
        sql.push_str(")");
        Self(sql)
    }
}

impl Render for Subquery {
    fn render(&self, sql: &mut Sql) {
        sql.push(&self.0);
    }
}

impl From<Block> for Subquery {
    fn from(block: Block) -> Self {
        Self(block.to_sql())
    }
}

// Separate arms: an optional `[..]` before a `ty` is ambiguous, since a type can start with `[`.
macro_rules! subquery_from {
    ([$($generics:tt)*] $statement:ty) => {
        impl<$($generics)*> From<$statement> for Subquery {
            fn from(statement: $statement) -> Self {
                Self::of(&statement)
            }
        }
    };
    ($statement:ty) => {
        impl From<$statement> for Subquery {
            fn from(statement: $statement) -> Self {
                Self::of(&statement)
            }
        }
    };
}

subquery_from!(SelectStatement);
subquery_from!(IfElseStatement);
subquery_from!(ReturnStatement);
subquery_from!([T: Node] CreateStatement<T>);
subquery_from!([T: Model] UpdateStatement<T>);
subquery_from!([T: Model] DeleteStatement<T>);
subquery_from!([T: Edge] RelateStatement<T>);
subquery_from!([T: Node] InsertStatement<T>);

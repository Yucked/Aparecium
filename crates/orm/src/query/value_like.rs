
use surrealdb::types::SurrealValue;

use crate::{
    Alias, All, E, Edge, Field, Filter, Function, Model, NONE, NULL, Node, Operation, Param,
    Render, Sql,
    statements::{
        CreateStatement, DeleteStatement, IfElseStatement, InsertStatement, LetStatement,
        RelateStatement, SelectStatement, Subquery, UpdateStatement,
    },
};

/// Any SurrealQL expression: a bound value, field, parameter, operation, function call or
/// subquery.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct ValueLike {
    sql: Sql,
    /// Built from an operation, so it needs parentheses as an arithmetic operand.
    operation: bool,
}

impl ValueLike {
    /// Wraps any query fragment.
    pub fn new(node: impl Render) -> Self {
        Self {
            sql: node.to_sql(),
            operation: false,
        }
    }

    /// A value, bound as a parameter when the query is rendered.
    pub fn value(value: impl SurrealValue) -> Self {
        let mut sql = Sql::new();
        sql.push_value(value.into_value());
        Self {
            sql,
            operation: false,
        }
    }

    /// Whether the expression renders to nothing.
    pub fn is_empty(&self) -> bool {
        self.sql.is_empty()
    }

    /// Renders the expression, parenthesized when it is an operation.
    pub(crate) fn render_operand(&self, sql: &mut Sql) {
        if self.operation {
            sql.push_str("(").push(&self.sql).push_str(")");
        } else {
            sql.push(&self.sql);
        }
    }
}

impl Render for ValueLike {
    fn render(&self, sql: &mut Sql) {
        sql.push(&self.sql);
    }
}

impl From<Operation> for ValueLike {
    fn from(value: Operation) -> Self {
        Self {
            sql: value.to_sql(),
            operation: true,
        }
    }
}

impl<T: SurrealValue> From<T> for ValueLike {
    fn from(value: T) -> Self {
        Self::value(value)
    }
}

/// Implements `From<$source>` by rendering the source as an expression.
macro_rules! value_like_from_render {
    ($target:ty: $($source:ty),* $(,)?) => {
        $(
            impl From<$source> for $target {
                fn from(value: $source) -> Self {
                    Self::new(value)
                }
            }
        )*
    };
}

value_like_from_render!(ValueLike: Field, &Field, Param, &Param, Alias, Filter, Function, Subquery);

impl From<LetStatement> for ValueLike {
    fn from(value: LetStatement) -> Self {
        Self::new(value.get_param())
    }
}

impl From<&LetStatement> for ValueLike {
    fn from(value: &LetStatement) -> Self {
        Self::new(value.get_param())
    }
}

impl From<All> for ValueLike {
    fn from(value: All) -> Self {
        Self::new(value)
    }
}

impl From<NULL> for ValueLike {
    fn from(value: NULL) -> Self {
        Self::new(value)
    }
}

impl From<NONE> for ValueLike {
    fn from(value: NONE) -> Self {
        Self::new(value)
    }
}

impl From<E> for ValueLike {
    fn from(value: E) -> Self {
        Self::new(value)
    }
}

impl From<SelectStatement> for ValueLike {
    fn from(statement: SelectStatement) -> Self {
        Self::new(Subquery::from(statement))
    }
}

impl From<IfElseStatement> for ValueLike {
    fn from(statement: IfElseStatement) -> Self {
        Self::new(Subquery::from(statement))
    }
}

impl<T: Node> From<CreateStatement<T>> for ValueLike {
    fn from(statement: CreateStatement<T>) -> Self {
        Self::new(Subquery::from(statement))
    }
}

impl<T: Model> From<UpdateStatement<T>> for ValueLike {
    fn from(statement: UpdateStatement<T>) -> Self {
        Self::new(Subquery::from(statement))
    }
}

impl<T: Model> From<DeleteStatement<T>> for ValueLike {
    fn from(statement: DeleteStatement<T>) -> Self {
        Self::new(Subquery::from(statement))
    }
}

impl<T: Edge> From<RelateStatement<T>> for ValueLike {
    fn from(statement: RelateStatement<T>) -> Self {
        Self::new(Subquery::from(statement))
    }
}

impl<T: Node> From<InsertStatement<T>> for ValueLike {
    fn from(statement: InsertStatement<T>) -> Self {
        Self::new(Subquery::from(statement))
    }
}

/// A macro to create a heterogenous list of anything
/// that can be converted into `ValueLike` including all
/// supported surrealdb types like numbers,
/// dates, field, param, geometry etc, and statements.
/// It creates Vec<ValueLike>` from a list of values.
///
/// # Arguments
/// * `$( $val:expr ),*` - A list of values that can be converted into `ValueLike`
/// # Example
/// ```rust
/// use aparecium::{*, functions::{math, count}, types::Duration};
/// let country = Field::new("country");
/// let age = Field::new("age");
/// let total = AliasName::new("total");
///
/// let values = arr![
///         1,
///         2,
///         3,
///         count!().__as__(total),
///         math::sum!(age),
///         country,
///         54,
///         Duration::from_secs(43)
///     ];
///
///    assert_eq!(
///         values.iter()
///             .map(|v| v.to_debug_sql())
///             .collect::<Vec<_>>()
///             .join(", "),
///         "1, 2, 3, count() AS total, math::sum(age), country, 54, 43s"
///    );
///    ```
#[macro_export]
macro_rules! arr {
    ($( $val:expr ),*) => {{
        vec![
            $( $crate::ValueLike::from($val) ),*
        ]
    }};
}

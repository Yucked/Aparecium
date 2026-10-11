use surrealdb::types::SurrealValue;

use crate::{
    Block, Conditional, Edge, Field, Node, Operator, Param, Render, Sql, ValueLike,
    render::sql,
    statements::{
        DeleteStatement, IfElseStatement, InsertStatement, LetStatement, RelateStatement,
        SelectStatement, Subquery, UpdateStatement,
    },
};

/// An assignment, e.g. `name = $_p0`, used in `SET` clauses.
#[derive(Debug, Clone, PartialEq)]
pub struct Setter(Sql);

impl Setter {
    pub(crate) fn new(sql: Sql) -> Self {
        Self(sql)
    }
}

impl Render for Setter {
    fn render(&self, sql: &mut Sql) {
        sql.push(&self.0);
    }
}

impl Conditional for Setter {}

impl From<Setter> for Vec<Setter> {
    fn from(value: Setter) -> Self {
        vec![value]
    }
}

/// The right-hand side of an assignment: a value of the field's type, or an expression.
pub enum SetterArg<T> {
    /// A value, bound as a parameter.
    Value(T),
    /// A field, parameter, block or subquery.
    Expression(ValueLike),
}

// `V: SurrealValue` keeps this disjoint from the expression impls below: those types are local and
// never values.
impl<T: SurrealValue, V: Into<T> + SurrealValue> From<V> for SetterArg<T> {
    fn from(value: V) -> Self {
        Self::Value(value.into())
    }
}

/// Implements `From<$source>` for `SetterArg` by rendering the source as an expression.
macro_rules! setter_arg_from_expression {
    ($($source:ty),* $(,)?) => {
        $(
            impl<T> From<$source> for SetterArg<T> {
                fn from(value: $source) -> Self {
                    Self::Expression(value.into())
                }
            }
        )*
    };
}

setter_arg_from_expression!(
    Field,
    &Field,
    Param,
    &Param,
    LetStatement,
    &LetStatement,
    Subquery,
    SelectStatement,
    IfElseStatement,
);

impl<T> From<Block> for SetterArg<T> {
    fn from(value: Block) -> Self {
        Self::Expression(Subquery::from(value).into())
    }
}

impl<T> From<&Block> for SetterArg<T> {
    fn from(value: &Block) -> Self {
        Self::Expression(Subquery::from(value.clone()).into())
    }
}

impl<T, V: Node> From<UpdateStatement<V>> for SetterArg<T> {
    fn from(value: UpdateStatement<V>) -> Self {
        Self::Expression(value.into())
    }
}

impl<T, V: Node> From<DeleteStatement<V>> for SetterArg<T> {
    fn from(value: DeleteStatement<V>) -> Self {
        Self::Expression(value.into())
    }
}

impl<T, V: Node> From<InsertStatement<V>> for SetterArg<T> {
    fn from(value: InsertStatement<V>) -> Self {
        Self::Expression(value.into())
    }
}

impl<T, V: Edge> From<RelateStatement<V>> for SetterArg<T> {
    fn from(value: RelateStatement<V>) -> Self {
        Self::Expression(value.into())
    }
}

fn assign<T: SurrealValue>(
    field: &Field,
    operator: Operator,
    value: impl Into<SetterArg<T>>,
) -> Setter {
    let mut sql = sql!(field, " ", operator, " ");
    match value.into() {
        SetterArg::Value(value) => sql.push_value(value.into_value()),
        SetterArg::Expression(expression) => sql.push(&expression),
    };
    Setter(sql)
}

/// A trait for assigning values to a field used in `SET`
/// function in create and update statements.
pub trait SetterAssignable<T: SurrealValue>
where
    Self: std::ops::Deref<Target = Field>,
{
    /// Assigns the given value to the field.
    fn equal_to(&self, value: impl Into<SetterArg<T>>) -> Setter {
        assign(self.deref(), Operator::Equal, value)
    }

    /// Derefs to field type.
    fn to_field(&self) -> Field {
        self.deref().clone()
    }
}

/// A trait for incrementing or decrementing values to a field used in `SET`
/// function in create and update statements.
pub trait SetterNumeric<T: SurrealValue>
where
    Self: std::ops::Deref<Target = Field>,
{
    /// Increments the value of the field by the given value.
    fn increment_by(&self, value: impl Into<SetterArg<T>>) -> Setter {
        assign(self.deref(), Operator::Inc, value)
    }

    /// Decrements the value of the field by the given value.
    fn decrement_by(&self, value: impl Into<SetterArg<T>>) -> Setter {
        assign(self.deref(), Operator::Dec, value)
    }

    /// Derefs to field type.
    fn to_field(&self) -> Field {
        self.deref().clone()
    }
}

/// Setter for array fields.
pub trait SetterArray<T: SurrealValue>
where
    Self: std::ops::Deref<Target = Field>,
{
    /// Appends the given value to the array.
    fn append(&self, value: impl Into<SetterArg<T>>) -> Setter {
        assign(self.deref(), Operator::Inc, value)
    }

    /// Removes the given value from the array.
    fn remove(&self, value: impl Into<SetterArg<T>>) -> Setter {
        assign(self.deref(), Operator::Dec, value)
    }

    /// Derefs to field type.
    fn to_field(&self) -> Field {
        self.deref().clone()
    }
}

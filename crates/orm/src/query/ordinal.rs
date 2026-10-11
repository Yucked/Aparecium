
use surrealdb::types::{self, SurrealValue, Value};

use super::Field;
use crate::{Render, Sql, ValueLike};

/// A value that can be ordered: a number, datetime or geometry, or a field holding one.
#[derive(Debug, Clone, PartialEq)]
pub enum Ordinal {
    /// A number, datetime or geometry value
    Value(Value),
    /// A field value
    Field(Field),
}

impl Render for Ordinal {
    fn render(&self, sql: &mut Sql) {
        match self {
            Ordinal::Value(value) => {
                sql.push_value(value.clone());
            }
            Ordinal::Field(field) => {
                sql.push(field);
            }
        }
    }
}

impl From<Ordinal> for ValueLike {
    fn from(value: Ordinal) -> Self {
        ValueLike::new(value)
    }
}

macro_rules! ordinal_values {
    ($($t:ty),* $(,)?) => {
        $(impl From<$t> for Ordinal {
            fn from(value: $t) -> Self {
                Self::Value(value.into_value())
            }
        })*
    };
}

ordinal_values!(
    i8,
    i16,
    i32,
    i64,
    isize,
    u8,
    u16,
    u32,
    u64,
    usize,
    f32,
    f64,
    types::Number,
    types::Decimal,
    types::Datetime,
    chrono::DateTime<chrono::Utc>,
    types::Geometry,
);

impl<T: Into<Field>> From<T> for Ordinal {
    fn from(val: T) -> Self {
        Ordinal::Field(val.into())
    }
}

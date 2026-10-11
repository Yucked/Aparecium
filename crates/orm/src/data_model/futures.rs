use crate::{Render, Sql, ValueLike};

/// Represents a future: a value computed when the record is read.
#[derive(Debug, Clone)]
pub struct Future(ValueLike);

impl std::ops::Deref for Future {
    type Target = ValueLike;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Render for Future {
    fn render(&self, sql: &mut Sql) {
        sql.push(&self.0);
    }
}

/// Wraps a value in a future: `<future> { value }`.
pub fn future(value: impl Into<ValueLike>) -> Future {
    let value: ValueLike = value.into();
    let mut sql = Sql::new();
    sql.push_str("<future> { ").push(&value).push_str(" }");
    Future(ValueLike::new(sql))
}

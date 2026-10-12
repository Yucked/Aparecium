use crate::{Render, Renderer, ValueLike};

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
    fn render(&self, out: &mut Renderer) {
        out.push(&self.0);
    }
}

/// Wraps a value in a future: `<future> { value }`.
pub fn future(value: impl Into<ValueLike>) -> Future {
    Future(ValueLike::new(FutureBody(value.into())))
}

struct FutureBody(ValueLike);

impl Render for FutureBody {
    fn render(&self, out: &mut Renderer) {
        out.push_str("<future> { ").push(&self.0).push_str(" }");
    }
}

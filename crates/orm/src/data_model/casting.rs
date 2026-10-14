use crate::{Render, Renderer, ValueLike};

/// Represents a cast function.
#[derive(Debug, Clone)]
pub struct Cast(ValueLike);

impl std::ops::Deref for Cast {
    type Target = ValueLike;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Render for Cast {
    fn render(&self, out: &mut Renderer) {
        out.push(&self.0);
    }
}

/// A cast is an expression, usable wherever a value is.
impl From<Cast> for ValueLike {
    fn from(cast: Cast) -> Self {
        cast.0
    }
}

/// `<kind> value`
struct CastExpression {
    prefix: &'static str,
    value: ValueLike,
}

impl Render for CastExpression {
    fn render(&self, out: &mut Renderer) {
        out.push_str(self.prefix).push(&self.value);
    }
}

macro_rules! create_cast_function {
    ($(#[$attr:meta])* => $function_name: expr) => {
        paste::paste! {
            $(#[$attr])*
            pub fn [<$function_name>](value: impl Into<ValueLike>) -> Cast {
                Cast(ValueLike::new(CastExpression {
                    prefix: concat!("<", $function_name, "> "),
                    value: value.into(),
                }))
            }
        }
    };
}

create_cast_function!(
    /// Casts a value to a boolean.
    => "bool"
);

create_cast_function!(
    /// Casts a value to a datetime.
    => "datetime"
);

create_cast_function!(
    /// Casts a value to a decimal.
    => "decimal"
);

create_cast_function!(
    /// Casts a value to a duration.
    => "duration"
);

create_cast_function!(
    /// Casts a value to a float.
    => "float"
);

create_cast_function!(
    /// Casts a value to an int.
    => "int"
);

create_cast_function!(
    /// Casts a value to a number.
    => "number"
);

create_cast_function!(
    /// Casts a value to a string.
    => "string"
);

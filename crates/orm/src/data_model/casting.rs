use crate::{Render, Sql, ValueLike};

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
    fn render(&self, sql: &mut Sql) {
        sql.push(&self.0);
    }
}

macro_rules! create_cast_function {
    ($(#[$attr:meta])* => $function_name: expr) => {
        paste::paste! {
            $(#[$attr])*
            pub fn [<$function_name>](value: impl Into<ValueLike>) -> Cast {
                let value: ValueLike = value.into();
                let mut sql = Sql::new();
                sql.push_str(concat!("<", $function_name, "> ")).push(&value);
                Cast(ValueLike::new(sql))
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

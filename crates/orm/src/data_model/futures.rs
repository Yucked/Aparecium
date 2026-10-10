/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 */

use std::fmt::Display;

use crate::{BindingsList, Buildable, Erroneous, Parametric, ValueLike};

/// Represents a cast function.
pub struct Future(ValueLike);

impl std::ops::Deref for Future {
    type Target = ValueLike;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Parametric for Future {
    fn get_bindings(&self) -> BindingsList {
        self.0.bindings.to_vec()
    }
}

impl Display for Future {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.build())
    }
}

impl Buildable for Future {
    fn build(&self) -> String {
        self.0.string.to_string()
    }
}

impl Erroneous for Future {}

macro_rules! create_cast_function {
    ($(#[$attr:meta])* => $function_name: expr) => {
        paste::paste! {
            $(#[$attr])*
            pub fn [<$function_name>](value: impl Into<ValueLike>) -> Future {
                let value: ValueLike = value.into();
                let string = format!("<{}> {{ {} }}", $function_name, value.build());

                Future(ValueLike {
                    string,
                    bindings: value.get_bindings(),
                    errors: value.get_errors(),
                })
            }

        }
    };
}

create_cast_function!(
    /// Casts a value to a future.
    => "future"
);

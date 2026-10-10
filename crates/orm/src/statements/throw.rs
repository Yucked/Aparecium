/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 */

use std::fmt;

use crate::{
    StrandLike,
    traits::{BindingsList, Buildable, Erroneous, Parametric, Queryable},
};

/// Creates a THROW statement builder.
///
/// Examples
/// ```rust
/// use aparecium::{*, statements::throw};
/// throw("some error message").build();
/// ```
pub fn throw(message: impl Into<StrandLike>) -> ThrowStatement {
    let message: StrandLike = message.into();
    ThrowStatement { message }
}

/// Represents the THROW statement
pub struct ThrowStatement {
    message: StrandLike,
}

impl Queryable for ThrowStatement {}

impl Erroneous for ThrowStatement {
    fn get_errors(&self) -> crate::ErrorList {
        self.message.get_errors()
    }
}

impl Parametric for ThrowStatement {
    fn get_bindings(&self) -> BindingsList {
        self.message.get_bindings()
    }
}

impl Buildable for ThrowStatement {
    fn build(&self) -> String {
        format!("THROW {};", self.message.build())
    }
}

impl fmt::Display for ThrowStatement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.build())
    }
}

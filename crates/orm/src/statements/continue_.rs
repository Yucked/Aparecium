/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 */

use std::fmt;

use crate::traits::{BindingsList, Buildable, Erroneous, Parametric, Queryable};

/// Creates a CONTINUE statement builder.
///
/// Examples
/// ```rust
/// use aparecium::{*, statements::continue_};
///
/// continue_();
/// ```
pub fn continue_() -> ContinueStatement {
    ContinueStatement
}

/// Represents the CONTINUE statement
pub struct ContinueStatement;

impl Queryable for ContinueStatement {}

impl Erroneous for ContinueStatement {}

impl Parametric for ContinueStatement {
    fn get_bindings(&self) -> BindingsList {
        vec![]
    }
}

impl Buildable for ContinueStatement {
    fn build(&self) -> String {
        "CONTINUE;".to_string()
    }
}

impl fmt::Display for ContinueStatement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.build())
    }
}

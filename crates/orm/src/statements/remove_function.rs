/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 */

use std::fmt::{self, Display};

use crate::{
    Binding,
    traits::{BindingsList, Buildable, Erroneous, Parametric, Queryable},
};

pub struct FunctionName(String);

impl From<&str> for FunctionName {
    fn from(name: &str) -> Self {
        Self(name.to_string())
    }
}

impl From<String> for FunctionName {
    fn from(name: String) -> Self {
        Self(name)
    }
}

impl Display for FunctionName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "fn::{}", self.0.trim_start_matches("fn::"))
    }
}

/// Creates a REMOVE FUNCTION statement builder.
///
/// Examples
/// ```rust
/// use aparecium::{*, statements::remove_function};
/// remove_function("fn::update_author").build();
/// ```
pub fn remove_function(name: impl Into<FunctionName>) -> RemoveFunctionStatement {
    let name: FunctionName = name.into();
    let name: String = name.to_string();
    let name = Binding::new(name).as_raw();
    RemoveFunctionStatement {
        name: name.get_param_dollarised(),
        bindings: vec![name],
        errors: vec![],
    }
}

/// Represents the REMOVE FUNCTION statement.
pub struct RemoveFunctionStatement {
    name: String,
    bindings: BindingsList,
    errors: crate::ErrorList,
}

impl Queryable for RemoveFunctionStatement {}

impl Erroneous for RemoveFunctionStatement {
    fn get_errors(&self) -> crate::ErrorList {
        self.errors.clone()
    }
}

impl Parametric for RemoveFunctionStatement {
    fn get_bindings(&self) -> BindingsList {
        self.bindings.clone()
    }
}

impl Buildable for RemoveFunctionStatement {
    fn build(&self) -> String {
        format!("REMOVE FUNCTION {};", self.name)
    }
}

impl fmt::Display for RemoveFunctionStatement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.build())
    }
}

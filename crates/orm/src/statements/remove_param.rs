/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 */
use std::fmt::{self};

use crate::{
    traits::{BindingsList, Buildable, Erroneous, Parametric, Queryable},
    Param,
};

/// Creates a REMOVE PARAM statement builder.
///
/// Examples
/// ```rust
/// use aparecium::{*, statements::remove_param};
/// remove_param("website_name");
/// ```
pub fn remove_param(name: impl Into<Param>) -> RemoveParamStatement {
    let name: Param = name.into();

    RemoveParamStatement {
        name: name.to_string(),
        bindings: vec![],
        errors: vec![],
    }
}

/// Represents the REMOVE PARAM statement.
pub struct RemoveParamStatement {
    name: String,
    bindings: BindingsList,
    errors: crate::ErrorList,
}

impl Queryable for RemoveParamStatement {}

impl Erroneous for RemoveParamStatement {
    fn get_errors(&self) -> crate::ErrorList {
        self.errors.clone()
    }
}

impl Parametric for RemoveParamStatement {
    fn get_bindings(&self) -> BindingsList {
        self.bindings.clone()
    }
}

impl Buildable for RemoveParamStatement {
    fn build(&self) -> String {
        format!("REMOVE PARAM {};", self.name)
    }
}

impl fmt::Display for RemoveParamStatement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.build())
    }
}


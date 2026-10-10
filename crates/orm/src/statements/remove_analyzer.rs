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

/// Creates a REMOVE ANALYZER statement builder.
///
/// Examples
/// ```rust
/// use aparecium::{*, statements::remove_analyzer};
/// remove_analyzer("analyzer::standard").build();
/// ```
pub fn remove_analyzer(name: impl Display) -> RemoveAnalyzerStatement {
    let name: String = name.to_string();
    let name = Binding::new(name).as_raw();
    RemoveAnalyzerStatement {
        name: name.get_param_dollarised(),
        bindings: vec![name],
        errors: vec![],
    }
}

/// Represents the REMOVE ANALYZER statement.
pub struct RemoveAnalyzerStatement {
    name: String,
    bindings: BindingsList,
    errors: crate::ErrorList,
}

impl Queryable for RemoveAnalyzerStatement {}

impl Erroneous for RemoveAnalyzerStatement {
    fn get_errors(&self) -> crate::ErrorList {
        self.errors.clone()
    }
}

impl Parametric for RemoveAnalyzerStatement {
    fn get_bindings(&self) -> BindingsList {
        self.bindings.clone()
    }
}

impl Buildable for RemoveAnalyzerStatement {
    fn build(&self) -> String {
        format!("REMOVE ANALYZER {};", self.name)
    }
}

impl fmt::Display for RemoveAnalyzerStatement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.build())
    }
}

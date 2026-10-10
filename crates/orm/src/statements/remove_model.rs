/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 */
use std::fmt;

use crate::{BindingsList, Buildable, Erroneous, Parametric, Queryable};

use super::define_model::{ModelName, ModelVersion};

/// Creates a REMOVE MODEL statement builder.
///
/// Examples
/// ```rust
/// use aparecium::{*, statements::remove_model};
///
/// remove_model("recommendation").version("1.0.0");
/// ```
pub fn remove_model(name: impl Into<ModelName>) -> RemoveModelStatement {
    let name: ModelName = name.into();

    RemoveModelStatement {
        name: name.build(),
        version: None,
        bindings: name.get_bindings(),
        errors: vec![],
    }
}

/// Represents the REMOVE MODEL statement.
pub struct RemoveModelStatement {
    name: String,
    version: Option<String>,
    bindings: BindingsList,
    errors: crate::ErrorList,
}

impl RemoveModelStatement {
    /// Adds a version to the REMOVE MODEL statement.
    pub fn version(mut self, version: impl Into<ModelVersion>) -> Self {
        let version: ModelVersion = version.into();
        self.version = Some(version.build());
        self.bindings.extend(version.get_bindings());
        self
    }
}

impl Queryable for RemoveModelStatement {}

impl Erroneous for RemoveModelStatement {
    fn get_errors(&self) -> crate::ErrorList {
        self.errors.to_vec()
    }
}

impl Parametric for RemoveModelStatement {
    fn get_bindings(&self) -> BindingsList {
        self.bindings.to_vec()
    }
}

impl Buildable for RemoveModelStatement {
    fn build(&self) -> String {
        let query = format!("REMOVE MODEL ml::{}", self.name);

        let query = if let Some(version) = &self.version {
            format!("{query}<{version}>")
        } else {
            query
        };

        let query = format!("{query};");

        query
    }
}

impl fmt::Display for RemoveModelStatement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.build())
    }
}

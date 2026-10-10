/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 */

use crate::{Buildable, Projections, ValueLike};
use std::fmt::{self, Display};

/// Return type
#[derive(Debug, Clone)]
pub enum ReturnType {
    /// Return nothing
    None,
    /// Return previous state
    Before,
    /// Return current state after change. This is the default
    After,
    /// Return the diff
    Diff,
    /// Return the listed fields/projection
    Projections(Projections),
}

impl From<Vec<ValueLike>> for ReturnType {
    fn from(value_like: Vec<ValueLike>) -> Self {
        ReturnType::Projections(Projections(value_like))
    }
}

impl Display for ReturnType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let return_type = match self {
            ReturnType::None => "NONE".to_string(),
            ReturnType::Before => "BEFORE".to_string(),
            ReturnType::After => "AFTER".to_string(),
            ReturnType::Diff => "DIFF".to_string(),
            ReturnType::Projections(projections) => projections.build(),
        };
        write!(f, "RETURN {return_type} ")
    }
}

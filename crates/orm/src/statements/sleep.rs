/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 */

use std::fmt::Display;

use crate::{
    traits::BindingsList,
    traits::{Buildable, Erroneous, Parametric, Queryable},
    types::DurationLike,
};

/// Creates a SLEEP statement.
///
/// Examples
///
/// ```rust
/// use std::time::Duration;
/// use aparecium::{*, statements::sleep};
///
/// sleep(Duration::from_secs(43));
/// ```
pub fn sleep(duration: impl Into<DurationLike>) -> SleepStatement {
    let duration: DurationLike = duration.into();

    SleepStatement {
        duration: duration.build(),
        bindings: duration.get_bindings(),
    }
}

/// Sleep statement initialization builder
pub struct SleepStatement {
    duration: String,
    bindings: BindingsList,
}

impl Buildable for SleepStatement {
    fn build(&self) -> String {
        format!("SLEEP {};", self.duration)
    }
}

impl Queryable for SleepStatement {}
impl Erroneous for SleepStatement {}

impl Parametric for SleepStatement {
    fn get_bindings(&self) -> BindingsList {
        self.bindings.to_vec()
    }
}

impl Display for SleepStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.build())
    }
}


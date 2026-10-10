/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 */

use std::fmt;

use crate::{
    statements::SelectStatement,
    traits::{BindingsList, Buildable, Erroneous, Parametric, Queryable},
};

/// Creates a LIVE SELECT statement builder.
///
/// Examples
/// ```rust
/// use aparecium::{*, statements::{live, select}};
/// let person = Table::new("person");
/// let statement = live(select(All).from(person));
/// ```
pub fn live(select_statement: SelectStatement) -> LiveSelectStatement {
    LiveSelectStatement {
        select: select_statement,
    }
}

/// Represents the LIVE SELECT statement.
pub struct LiveSelectStatement {
    select: SelectStatement,
}

impl Queryable for LiveSelectStatement {}

impl Erroneous for LiveSelectStatement {}

impl Parametric for LiveSelectStatement {
    fn get_bindings(&self) -> BindingsList {
        self.select.get_bindings()
    }
}

impl Buildable for LiveSelectStatement {
    fn build(&self) -> String {
        let query = format!("LIVE {}", &self.select.build());
        query
    }
}

impl fmt::Display for LiveSelectStatement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.build())
    }
}

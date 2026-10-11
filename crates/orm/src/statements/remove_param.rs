use crate::{Param, Sql};

use super::{Statement, impl_definition};

/// Creates a REMOVE PARAM statement builder.
///
/// Examples
/// ```rust
/// use aparecium::{*, statements::remove_param};
/// assert_eq!(remove_param("website_name").to_debug_sql(), "REMOVE PARAM $website_name;");
/// ```
pub fn remove_param(name: impl Into<Param>) -> RemoveParamStatement {
    RemoveParamStatement { name: name.into() }
}

/// Represents the REMOVE PARAM statement.
pub struct RemoveParamStatement {
    name: Param,
}

impl Statement for RemoveParamStatement {
    fn render_body(&self, sql: &mut Sql) {
        sql.push_str("REMOVE PARAM ").push(&self.name);
    }
}

impl_definition!(RemoveParamStatement);

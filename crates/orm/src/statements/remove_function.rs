use std::borrow::Cow;

use crate::Sql;

use super::{Statement, impl_definition};

/// A function's full name, e.g. `fn::update_author`, written verbatim.
pub struct FunctionName(Cow<'static, str>);

impl From<&str> for FunctionName {
    fn from(name: &str) -> Self {
        Self(Cow::Owned(name.to_owned()))
    }
}

impl From<String> for FunctionName {
    fn from(name: String) -> Self {
        Self(Cow::Owned(name))
    }
}

/// Creates a REMOVE FUNCTION statement builder.
///
/// Examples
/// ```rust
/// use aparecium::{*, statements::remove_function};
/// assert_eq!(
///     remove_function("fn::update_author").to_debug_sql(),
///     "REMOVE FUNCTION fn::update_author;"
/// );
/// ```
pub fn remove_function(name: impl Into<FunctionName>) -> RemoveFunctionStatement {
    RemoveFunctionStatement { name: name.into() }
}

/// Represents the REMOVE FUNCTION statement.
pub struct RemoveFunctionStatement {
    name: FunctionName,
}

impl Statement for RemoveFunctionStatement {
    fn render_body(&self, sql: &mut Sql) {
        sql.push_str("REMOVE FUNCTION ").push_str(&self.name.0);
    }
}

impl_definition!(RemoveFunctionStatement);

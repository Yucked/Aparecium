use crate::{Scope, Sql};

use super::{Statement, impl_definition};

/// Remove scope statement
///
/// # Arguments
///
/// * `scope` - The name of the scope to be removed. Can be a string or a Scope type.
///
/// # Example
/// ```rust
/// use aparecium::{*, statements::remove_scope};
/// # let scope = Scope::new("scope");
/// let statement = remove_scope(scope);
/// assert_eq!(statement.to_debug_sql(), "REMOVE SCOPE scope;");
/// ```
pub fn remove_scope(scope: impl Into<Scope>) -> RemoveScopeStatement {
    RemoveScopeStatement {
        scope: scope.into(),
    }
}

/// Remove scope statement
pub struct RemoveScopeStatement {
    scope: Scope,
}

impl Statement for RemoveScopeStatement {
    fn render_body(&self, sql: &mut Sql) {
        sql.push_str("REMOVE SCOPE ").push(&self.scope);
    }
}

impl_definition!(RemoveScopeStatement);

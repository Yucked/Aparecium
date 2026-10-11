use crate::{Namespace, Sql};

use super::{Statement, impl_definition};

/// Remove namespace statement
///
/// # Arguments
///
/// * `namespace` - The name of the namespace to be removed. Can be a string or a Namespace type.
///
/// # Example
/// ```rust
/// use aparecium::{*, statements::remove_namespace};
///
/// # let namespace = Namespace::new("acme");
/// let statement = remove_namespace(namespace);
/// assert_eq!(statement.to_debug_sql(), "REMOVE NAMESPACE acme;");
/// ```
pub fn remove_namespace(namespace: impl Into<Namespace>) -> RemoveNamespaceStatement {
    RemoveNamespaceStatement {
        namespace: namespace.into(),
    }
}

/// Remove namespace statement
pub struct RemoveNamespaceStatement {
    namespace: Namespace,
}

impl Statement for RemoveNamespaceStatement {
    fn render_body(&self, sql: &mut Sql) {
        sql.push_str("REMOVE NAMESPACE ").push(&self.namespace);
    }
}

impl_definition!(RemoveNamespaceStatement);

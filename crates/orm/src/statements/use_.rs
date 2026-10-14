use crate::{Database, Namespace, Renderer};

use super::{Statement, impl_statement};

/// Creates a USE statement.
/// The USE statement specifies a namespace and / or a database to use for the subsequent SurrealQL statements.
///
/// Examples
///
/// ```rust
/// use umbriel::{*, statements::use_};
///
/// // Switch to the test Namespace
/// use_().database(Database::from("test"));
///
/// // Switch to the test Namespace
/// use_().namespace(Namespace::from("test"));
///
/// // Switch to the test Namespace and test database
/// let statement = use_()
///     .database(Database::from("test"))
///     .namespace(Namespace::from("test"));
/// assert_eq!(statement.to_debug_sql(), "USE NS test DB test;");
/// ```
pub fn use_() -> UseStatement {
    UseStatement::default()
}

/// Use statement builder
#[derive(Default, Clone)]
pub struct UseStatement {
    namespace: Option<Namespace>,
    database: Option<Database>,
}

impl UseStatement {
    /// Switch to the Namespace
    pub fn namespace(mut self, namespace: impl Into<Namespace>) -> Self {
        self.namespace = Some(namespace.into());
        self
    }

    /// Switch to the Database
    pub fn database(mut self, database: impl Into<Database>) -> Self {
        self.database = Some(database.into());
        self
    }
}

impl Statement for UseStatement {
    fn render_body(&self, out: &mut Renderer) {
        // SurrealQL takes the namespace first: `USE NS … DB …`.
        out.push_str("USE");
        if let Some(namespace) = &self.namespace {
            out.push_str(" NS ").push(namespace);
        }
        if let Some(database) = &self.database {
            out.push_str(" DB ").push(database);
        }
        if self.namespace.is_none() && self.database.is_none() {
            out.push_error("USE needs a namespace, a database or both");
        }
    }
}

impl_statement!(UseStatement);

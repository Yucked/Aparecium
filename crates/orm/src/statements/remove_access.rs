use crate::{Access, AccessLevel, Renderer};

use super::{Statement, impl_definition};

/// Removes an access method: `REMOVE ACCESS @name ON [ ROOT | NAMESPACE | DATABASE ]`.
///
/// # Example
/// ```rust
/// use umbriel::{*, statements::remove_access};
///
/// let statement = remove_access("account").on_database();
/// assert_eq!(statement.to_debug_sql(), "REMOVE ACCESS account ON DATABASE;");
///
/// // The level is required.
/// assert!(remove_access("account").to_query().is_err());
/// ```
pub fn remove_access(name: impl Into<Access>) -> RemoveAccessStatement {
    RemoveAccessStatement {
        name: name.into(),
        level: None,
    }
}

/// A `REMOVE ACCESS` statement.
pub struct RemoveAccessStatement {
    name: Access,
    level: Option<AccessLevel>,
}

impl RemoveAccessStatement {
    /// Removes the access method defined on the root.
    pub fn on_root(mut self) -> Self {
        self.level = Some(AccessLevel::Root);
        self
    }

    /// Removes the access method defined on the current namespace.
    pub fn on_namespace(mut self) -> Self {
        self.level = Some(AccessLevel::Namespace);
        self
    }

    /// Removes the access method defined on the current database.
    pub fn on_database(mut self) -> Self {
        self.level = Some(AccessLevel::Database);
        self
    }
}

impl Statement for RemoveAccessStatement {
    fn render_body(&self, out: &mut Renderer) {
        out.push_str("REMOVE ACCESS ").push(&self.name);
        match self.level {
            Some(level) => {
                out.push_str(" ON ").push(&level);
            }
            None => {
                out.push_error(
                    "REMOVE ACCESS needs a level: `on_root`, `on_namespace` or `on_database`",
                );
            }
        }
    }
}

impl_definition!(RemoveAccessStatement);

use surrealdb::types::Table;

use crate::Renderer;

use super::{Statement, impl_statement};

/// Creates statement for INFO for ROOT, NAMESPACE, DATABASE or TABLE.
///
/// Examples
/// ```rust
/// use umbriel::{*, statements::info_for};
/// assert_eq!(info_for().root().to_debug_sql(), "INFO FOR ROOT;");
/// ```
pub fn info_for() -> InfoStatementInit {
    InfoStatementInit
}

/// Enum representing the different levels of the SurrealDB system
enum InfoLevel {
    Root,
    Namespace,
    Database,
    Table(Table),
}

/// Information statement initialization builder
pub struct InfoStatementInit;

impl InfoStatementInit {
    /// Creates statement for INFO for ROOT.
    /// The top-level ROOT command returns information regarding the namespaces, root users and
    /// root access methods of the SurrealDB system.
    pub fn root(self) -> InfoStatement {
        InfoStatement::new(InfoLevel::Root)
    }

    /// Creates statement for INFO for NAMESPACE.
    /// The NS or NAMESPACE command returns information regarding the users, access methods and databases under a specific Namespace.
    pub fn namespace(self) -> InfoStatement {
        InfoStatement::new(InfoLevel::Namespace)
    }

    /// Creates statement for INFO for DATABASE.
    /// The DB or DATABASE command returns information regarding the users, access methods and tables under a specific Database.
    pub fn database(self) -> InfoStatement {
        InfoStatement::new(InfoLevel::Database)
    }

    /// Creates statement for INFO for TABLE.
    /// The TABLE command returns information regarding the events, fields, indexes, and foreign table configurations on a specific Table.
    ///
    /// ```rust
    /// use umbriel::{*, statements::info_for};
    /// assert_eq!(info_for().table("test_table").to_debug_sql(), "INFO FOR TABLE test_table;");
    /// ```
    pub fn table(self, table: impl Into<Table>) -> InfoStatement {
        InfoStatement::new(InfoLevel::Table(table.into()))
    }
}

/// Information statement builder
pub struct InfoStatement {
    level: InfoLevel,
    structure: bool,
}

impl InfoStatement {
    fn new(level: InfoLevel) -> Self {
        Self {
            level,
            structure: false,
        }
    }

    /// Returns each definition as an object of its properties (`STRUCTURE`) instead of its
    /// `DEFINE` statement.
    ///
    /// ```rust
    /// use umbriel::{*, statements::info_for};
    /// assert_eq!(info_for().database().structure().to_debug_sql(), "INFO FOR DB STRUCTURE;");
    /// assert_eq!(
    ///     info_for().table("person").structure().to_debug_sql(),
    ///     "INFO FOR TABLE person STRUCTURE;"
    /// );
    /// ```
    pub fn structure(mut self) -> Self {
        self.structure = true;
        self
    }
}

impl Statement for InfoStatement {
    fn render_body(&self, out: &mut Renderer) {
        match &self.level {
            InfoLevel::Root => {
                out.push_str("INFO FOR ROOT");
            }
            InfoLevel::Namespace => {
                out.push_str("INFO FOR NS");
            }
            InfoLevel::Database => {
                out.push_str("INFO FOR DB");
            }
            InfoLevel::Table(table) => {
                out.push_str("INFO FOR TABLE ").push(table);
            }
        }
        if self.structure {
            out.push_str(" STRUCTURE");
        }
    }
}

impl_statement!(InfoStatement);

use surrealdb::types::Table;

use crate::{Scope, Sql};

use super::{Statement, impl_statement};

/// Creates statement for INFO for KV(i.e system), NAMESPACE, DATABASE, SCOPE, or TABLE.
///
/// Examples
/// ```rust
/// use aparecium::{*, statements::info_for};
/// assert_eq!(info_for().kv().to_debug_sql(), "INFO FOR KV;");
/// ```
pub fn info_for() -> InfoStatementInit {
    InfoStatementInit
}

/// Enum representing the different levels of the SurrealDB system
enum InfoLevel {
    Kv,
    Namespace,
    Database,
    Scope(Scope),
    Table(Table),
}

/// Information statement initialization builder
pub struct InfoStatementInit;

impl InfoStatementInit {
    /// Creates statement for INFO for the KV(i.e system).
    /// The top-level KV command returns information regarding the namespaces which
    /// exists within the SurrealDB system.
    pub fn kv(self) -> InfoStatement {
        InfoStatement(InfoLevel::Kv)
    }

    /// Creates statement for INFO for NAMESPACE.
    /// The NS or NAMESPACE command returns information regarding the logins, tokens, and databases under a specific Namespace.
    pub fn namespace(self) -> InfoStatement {
        InfoStatement(InfoLevel::Namespace)
    }

    /// Creates statement for INFO for DATABASE.
    /// The DB or DATABASE command returns information regarding the logins, tokens, and scopes, and tables under a specific Database.
    pub fn database(self) -> InfoStatement {
        InfoStatement(InfoLevel::Database)
    }

    /// Creates statement for INFO for SCOPE.
    /// The SCOPE command returns information regarding the tokens configured under a specific Scope.
    ///
    /// ```rust
    /// use aparecium::{*, statements::info_for};
    /// assert_eq!(info_for().scope("test_scope").to_debug_sql(), "INFO FOR SCOPE test_scope;");
    /// ```
    pub fn scope(self, scope: impl Into<Scope>) -> InfoStatement {
        InfoStatement(InfoLevel::Scope(scope.into()))
    }

    /// Creates statement for INFO for TABLE.
    /// The TABLE command returns information regarding the events, fields, indexes, and foreign table configurations on a specific Table.
    ///
    /// ```rust
    /// use aparecium::{*, statements::info_for};
    /// assert_eq!(info_for().table("test_table").to_debug_sql(), "INFO FOR TABLE test_table;");
    /// ```
    pub fn table(self, table: impl Into<Table>) -> InfoStatement {
        InfoStatement(InfoLevel::Table(table.into()))
    }
}

/// Information statement builder
pub struct InfoStatement(InfoLevel);

impl Statement for InfoStatement {
    fn render_body(&self, sql: &mut Sql) {
        match &self.0 {
            InfoLevel::Kv => {
                sql.push_str("INFO FOR KV");
            }
            InfoLevel::Namespace => {
                sql.push_str("INFO FOR NS");
            }
            InfoLevel::Database => {
                sql.push_str("INFO FOR DB");
            }
            InfoLevel::Scope(scope) => {
                sql.push_str("INFO FOR SCOPE ").push(scope);
            }
            InfoLevel::Table(table) => {
                sql.push_str("INFO FOR TABLE ").push(table);
            }
        }
    }
}

impl_statement!(InfoStatement);

use crate::{Login, Sql};

use super::{NamespaceOrDatabase, Statement, impl_definition};

/// Remove login statement
///
/// # Arguments
///
/// * `login` - The name of the login to be removed. Can be a string or a Login type.
///
/// # Example
/// ```rust
/// use aparecium::{*, statements::remove_login};
/// # let username = Login::new("alice");
/// let statement = remove_login(username).on_namespace();
/// assert_eq!(statement.to_debug_sql(), "REMOVE LOGIN alice ON NAMESPACE;");
/// ```
pub fn remove_login(login: impl Into<Login>) -> RemoveLoginStatementInit {
    RemoveLoginStatementInit {
        login: login.into(),
    }
}

/// A REMOVE LOGIN statement awaiting its level.
pub struct RemoveLoginStatementInit {
    login: Login,
}

impl RemoveLoginStatementInit {
    /// Remove login on namespace
    pub fn on_namespace(self) -> RemoveLoginStatement {
        RemoveLoginStatement {
            login: self.login,
            on: NamespaceOrDatabase::Namespace,
        }
    }

    /// Remove login on database
    pub fn on_database(self) -> RemoveLoginStatement {
        RemoveLoginStatement {
            login: self.login,
            on: NamespaceOrDatabase::Database,
        }
    }
}

/// Remove login statement
pub struct RemoveLoginStatement {
    login: Login,
    on: NamespaceOrDatabase,
}

impl Statement for RemoveLoginStatement {
    fn render_body(&self, sql: &mut Sql) {
        sql.push_str("REMOVE LOGIN ")
            .push(&self.login)
            .push_str(" ON ")
            .push(&self.on);
    }
}

impl_definition!(RemoveLoginStatement);

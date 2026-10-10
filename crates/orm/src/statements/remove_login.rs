/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 */

/*
 *
 *
REMOVE statement

Statement syntax
REMOVE [
    NAMESPACE @name
    | DATABASE @name
    | LOGIN @name ON [ NAMESPACE | DATABASE ]
    | TOKEN @name ON [ NAMESPACE | DATABASE ]
    | SCOPE @name
    | TABLE @name
    | EVENT @name ON [ TABLE ] @table
    | FIELD @name ON [ TABLE ] @table
    | INDEX @name ON [ TABLE ] @table
]
 * */

use std::fmt::{self, Display};

use crate::{BindingsList, Buildable, Erroneous, Login, Parametric, Queryable};

use super::NamespaceOrDatabase;

/// Remove login statement
///
/// # Arguments
///
/// * `login` - The name of the login to be removed. Can be a string or a Login type.
///
/// # Example
/// ```rust
/// use aparecium::{*, statements::remove_login};
/// # let username = Login::new("username");
/// let statement = remove_login(username).on_namespace();
/// assert_eq!(statement.build(), "REMOVE LOGIN username ON NAMESPACE;");
/// ```
pub fn remove_login(login: impl Into<Login>) -> RemoveLoginStatementInit {
    RemoveLoginStatementInit {
        login: login.into(),
        on: None,
    }
}

pub struct RemoveLoginStatementInit {
    login: Login,
    on: Option<NamespaceOrDatabase>,
}

impl RemoveLoginStatementInit {
    /// Remove login on namespace
    /// ```rust
    ///
    /// use aparecium::{*, statements::remove_login};
    ///
    /// let username = Login::new("username");
    /// let statement = remove_login(username).on_namespace();
    /// assert_eq!(statement.build(), "REMOVE LOGIN username ON NAMESPACE;");
    /// ```
    pub fn on_namespace(mut self) -> RemoveLoginStatement {
        self.on = Some(NamespaceOrDatabase::Namespace);
        self.into()
    }

    /// Remove login on database
    /// ```rust
    ///
    /// use aparecium::{*, statements::remove_login};
    ///
    /// let username = Login::new("username");
    /// let statement = remove_login(username).on_database();
    /// assert_eq!(statement.build(), "REMOVE LOGIN username ON DATABASE;");
    /// ```
    pub fn on_database(mut self) -> RemoveLoginStatement {
        self.on = Some(NamespaceOrDatabase::Database);
        self.into()
    }
}

/// Remove login statement
pub struct RemoveLoginStatement(RemoveLoginStatementInit);

impl From<RemoveLoginStatementInit> for RemoveLoginStatement {
    fn from(init: RemoveLoginStatementInit) -> Self {
        Self(init)
    }
}

impl std::ops::Deref for RemoveLoginStatement {
    type Target = RemoveLoginStatementInit;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Buildable for RemoveLoginStatement {
    fn build(&self) -> String {
        let mut query = format!("REMOVE LOGIN {}", self.login);

        if let Some(on) = &self.on {
            query = format!("{} ON {}", query, on);
        }

        format!("{};", query)
    }
}

impl Display for RemoveLoginStatement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.build())
    }
}

impl Parametric for RemoveLoginStatement {
    fn get_bindings(&self) -> BindingsList {
        vec![]
    }
}

impl Erroneous for RemoveLoginStatement {}

impl Queryable for RemoveLoginStatement {}

use std::fmt::{self, Display};

use crate::{Renderer, User};

use super::{Statement, impl_definition};

/// Remove user statement
///
/// # Arguments
///
/// * `user` - User to be removed. Can be a string or a User type.
///
/// # Example
/// ```rust
/// use umbriel::{*, statements::remove_user};
/// let statement = remove_user("alice").on_namespace();
/// assert_eq!(statement.to_debug_sql(), "REMOVE USER alice ON NAMESPACE;");
///
/// let statement = remove_user("alice").on_database();
/// assert_eq!(statement.to_query().unwrap().sql(), "REMOVE USER alice ON DATABASE;");
/// ```
pub fn remove_user(user: impl Into<User>) -> RemoveUserStatementInit {
    RemoveUserStatementInit { user: user.into() }
}

/// The level a user is removed from.
#[derive(Clone, Copy)]
pub enum UserPermissionScope {
    /// The current namespace
    Namespace,
    /// The current database
    Database,
    /// The root level
    Root,
}

impl Display for UserPermissionScope {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            UserPermissionScope::Namespace => "NAMESPACE",
            UserPermissionScope::Database => "DATABASE",
            UserPermissionScope::Root => "ROOT",
        })
    }
}

/// A REMOVE USER statement awaiting its level.
pub struct RemoveUserStatementInit {
    user: User,
}

impl RemoveUserStatementInit {
    /// Specify to remove the User from root
    pub fn on_root(self) -> RemoveUserStatement {
        self.on(UserPermissionScope::Root)
    }

    /// Specify to remove the User from namespace
    pub fn on_namespace(self) -> RemoveUserStatement {
        self.on(UserPermissionScope::Namespace)
    }

    /// Specify to remove the User from database
    pub fn on_database(self) -> RemoveUserStatement {
        self.on(UserPermissionScope::Database)
    }

    fn on(self, on: UserPermissionScope) -> RemoveUserStatement {
        RemoveUserStatement {
            user: self.user,
            on,
        }
    }
}

/// Remove User statement
pub struct RemoveUserStatement {
    user: User,
    on: UserPermissionScope,
}

impl Statement for RemoveUserStatement {
    fn render_body(&self, out: &mut Renderer) {
        out.push_str("REMOVE USER ")
            .push(&self.user)
            .push_str(" ON ")
            .push_display(&self.on);
    }
}

impl_definition!(RemoveUserStatement);

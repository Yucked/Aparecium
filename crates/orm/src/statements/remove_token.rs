use crate::{Scope, Renderer, Token, TokenTarget};

use super::{Statement, impl_definition};

/// Remove token statement
///
/// # Arguments
///
/// * `token` - The name of the token to be removed. Can be a string or a Token type.
///
/// # Example
/// ```rust
/// use umbriel::{*, statements::remove_token};
/// let token = Token::new("token");
/// let statement = remove_token(token).on_namespace();
/// assert_eq!(statement.to_debug_sql(), "REMOVE TOKEN token ON NAMESPACE;");
///
/// let token = Token::new("token");
/// let statement = remove_token(token).on_database();
/// assert_eq!(statement.to_debug_sql(), "REMOVE TOKEN token ON DATABASE;");
/// ```
pub fn remove_token(token: impl Into<Token>) -> RemoveTokenStatementInit {
    RemoveTokenStatementInit {
        token: token.into(),
    }
}

/// A REMOVE TOKEN statement awaiting its level.
pub struct RemoveTokenStatementInit {
    token: Token,
}

impl RemoveTokenStatementInit {
    /// Specify to remove the token from namespace
    pub fn on_namespace(self) -> RemoveTokenStatement {
        self.on(TokenTarget::Namespace)
    }

    /// Specify to remove the token from database
    pub fn on_database(self) -> RemoveTokenStatement {
        self.on(TokenTarget::Database)
    }

    /// Specify to remove the token from scope
    pub fn on_scope(self, scope_name: impl Into<Scope>) -> RemoveTokenStatement {
        self.on(TokenTarget::Scope(scope_name.into()))
    }

    fn on(self, on: TokenTarget) -> RemoveTokenStatement {
        RemoveTokenStatement {
            token: self.token,
            on,
        }
    }
}

/// Remove token statement
pub struct RemoveTokenStatement {
    token: Token,
    on: TokenTarget,
}

impl Statement for RemoveTokenStatement {
    fn render_body(&self, out: &mut Renderer) {
        out.push_str("REMOVE TOKEN ")
            .push(&self.token)
            .push_str(" ON ")
            .push(&self.on);
    }
}

impl_definition!(RemoveTokenStatement);

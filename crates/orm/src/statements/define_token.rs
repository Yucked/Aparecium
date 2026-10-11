use crate::{Ident, Scope, Sql, StrandLike, TokenTarget, TokenType};

use super::{Statement, impl_definition};

// DEFINE TOKEN statement
// SurrealDB can work with third-party OAuth providers. Let's say that your provider issues your service a JWT once it's authenticated. By using the DEFINE TOKEN statement, you can set the public key needed to verify a JWT's authenticity.
//
// Statement syntax
// DEFINE TOKEN @name ON [ NAMESPACE | DATABASE | SCOPE @scope ] TYPE @type VALUE @value

/// Define the API for the Token builder
pub struct DefineTokenStatement {
    name: Ident,
    token_type: Option<TokenType>,
    value: Option<StrandLike>,
    target: Option<TokenTarget>,
}

/// Define a new token.
///
/// SurrealDB can work with third-party OAuth providers. Let's say that your provider issues your service a JWT once it's authenticated. By using the DEFINE TOKEN statement, you can set the public key needed to verify a JWT's authenticity.
///
/// You can specify what TYPE of cryptographic signature algorithm your token uses. The following algorithms are supported:
/// EDDSA, ES256, ES384, ES512, HS256, HS384, HS512, PS256, PS384, PS512, RS256, RS384, RS512.
///
/// Examples:
///
/// ```rust
/// use aparecium::{*, statements::define_token};
/// let statement = define_token("acme_token")
///             .on_namespace()
///             .type_(TokenType::PS512)
///             .value("example-key");
/// assert_eq!(
///     statement.to_debug_sql(),
///     "DEFINE TOKEN acme_token ON NAMESPACE TYPE PS512 VALUE 'example-key';"
/// );
///
/// let statement = define_token("acme_token")
///             .on_database()
///             .type_(TokenType::EDDSA)
///             .value("example-key");
/// assert!(!statement.to_debug_sql().is_empty());
///
/// let statement = define_token("acme_token")
///             .on_scope("regional")
///             .type_(TokenType::HS256)
///             .value("example-key");
/// assert!(!statement.to_debug_sql().is_empty());
/// ```
pub fn define_token(token_name: impl Into<Ident>) -> DefineTokenStatement {
    DefineTokenStatement {
        name: token_name.into(),
        token_type: None,
        value: None,
        target: None,
    }
}

impl DefineTokenStatement {
    /// Define the token for the namespace
    pub fn on_namespace(mut self) -> Self {
        self.target = Some(TokenTarget::Namespace);
        self
    }

    /// Define the token for currentj database
    pub fn on_database(mut self) -> Self {
        self.target = Some(TokenTarget::Database);
        self
    }

    /// Use this OAuth provider for scope authorization
    pub fn on_scope(mut self, scope: impl Into<Scope>) -> Self {
        self.target = Some(TokenTarget::Scope(scope.into()));
        self
    }

    /// Specify the cryptographic signature algorithm used to sign the token
    pub fn type_(mut self, token_type: TokenType) -> Self {
        self.token_type = Some(token_type);
        self
    }

    /// Specify the public key so we can verify the authenticity of the token
    pub fn value(mut self, value: impl Into<StrandLike>) -> Self {
        self.value = Some(value.into());
        self
    }
}

impl Statement for DefineTokenStatement {
    fn render_body(&self, sql: &mut Sql) {
        sql.push_str("DEFINE TOKEN ").push(&self.name);

        if let Some(target) = &self.target {
            sql.push_str(" ON ").push(target);
        }

        if let Some(token_type) = &self.token_type {
            sql.push_str(" TYPE ").push(token_type);
        }

        if let Some(value) = &self.value {
            sql.push_str(" VALUE ").push(value);
        }
    }
}

impl_definition!(DefineTokenStatement);

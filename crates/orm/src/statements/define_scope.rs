// Statement syntax
// DEFINE SCOPE @name SESSION @duration SIGNUP @expression SIGNIN @expression
//
// -- Enable scope authentication directly in SurrealDB
// DEFINE SCOPE account SESSION 24h
// 	SIGNUP ( CREATE user SET email = $email, pass = crypto::argon2::generate($pass) )
// 	SIGNIN ( SELECT * FROM user WHERE email = $email AND crypto::argon2::compare(pass, $pass) )
// ;
use crate::{DurationLike, Raw, Render, Renderer, Scope};

use super::{Statement, Subquery, impl_definition};

/// A scope's SIGNUP or SIGNIN expression: a statement, rendered as a subquery, or raw SurrealQL
/// in parentheses.
pub enum ScopeExpression {
    /// A statement as a subquery.
    Subquery(Subquery),
    /// Raw SurrealQL.
    Raw(Raw),
}

impl<T: Into<Subquery>> From<T> for ScopeExpression {
    fn from(value: T) -> Self {
        Self::Subquery(value.into())
    }
}

impl From<Raw> for ScopeExpression {
    fn from(value: Raw) -> Self {
        Self::Raw(value)
    }
}

impl Render for ScopeExpression {
    fn render(&self, out: &mut Renderer) {
        match self {
            Self::Subquery(subquery) => {
                out.push(subquery);
            }
            Self::Raw(raw) => {
                out.push_str("(").push(raw).push_str(")");
            }
        }
    }
}

/// Define a new scope.
/// Setting scope access allows SurrealDB to operate as a web database.
/// With scopes you can set authentication and access rules which enable
/// fine-grained access to tables and fields.
///
/// Examples:
///
/// ```rust
/// use umbriel::{*, functions::crypto, statements::{define_scope, select}};
/// use std::time::Duration;
///
/// # let user= Table::new("user");
/// # let email = Field::new("email");
/// # let pass = Field::new("pass");
/// # let pass_param = Param::new("pass_param");
/// let statement = define_scope("acme_scope")
///     .session(Duration::from_secs(45))
///     .signup(Raw::new(
///         "CREATE user SET email = $email, pass = crypto::argon2::generate($pass)",
///     ))
///     .signin(
///         select(All).from(user).where_(
///             cond(email.equal("alice@example.com"))
///                 .and(crypto::argon2::compare!(pass, pass_param)),
///         ),
///     );
///
/// assert!(!statement.to_debug_sql().is_empty());
/// ```
pub fn define_scope(scope_name: impl Into<Scope>) -> DefineScopeStatement {
    DefineScopeStatement {
        name: scope_name.into(),
        duration: None,
        signup: None,
        signin: None,
    }
}

/// Define the API for the Scope builder
pub struct DefineScopeStatement {
    name: Scope,
    duration: Option<DurationLike>,
    signup: Option<ScopeExpression>,
    signin: Option<ScopeExpression>,
}

impl DefineScopeStatement {
    /// Set the session duration
    pub fn session(mut self, duration: impl Into<DurationLike>) -> Self {
        self.duration = Some(duration.into());
        self
    }

    /// Set the signup expression
    pub fn signup(mut self, expression: impl Into<ScopeExpression>) -> Self {
        self.signup = Some(expression.into());
        self
    }

    /// Set the signin expression
    pub fn signin(mut self, expression: impl Into<ScopeExpression>) -> Self {
        self.signin = Some(expression.into());
        self
    }
}

impl Statement for DefineScopeStatement {
    fn render_body(&self, out: &mut Renderer) {
        out.push_str("DEFINE SCOPE ").push(&self.name);

        if let Some(duration) = &self.duration {
            out.push_str(" SESSION ").push(duration);
        }

        if let Some(signup) = &self.signup {
            out.push_str(" \n\tSIGNUP ").push(signup);
        }

        if let Some(signin) = &self.signin {
            out.push_str(" \n\tSIGNIN ").push(signin);
        }
    }
}

impl_definition!(DefineScopeStatement);

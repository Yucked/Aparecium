use std::fmt::{self, Display};

use crate::{Render, Renderer, Scope};

/// A token's signature algorithm.
#[expect(missing_docs, reason = "each variant is the algorithm's own name")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TokenType {
    EDDSA,
    ES256,
    ES384,
    ES512,
    HS256,
    HS384,
    HS512,
    PS256,
    PS384,
    PS512,
    RS256,
    RS384,
    RS512,
}

impl TokenType {
    /// The algorithm's SurrealQL keyword.
    pub fn as_str(self) -> &'static str {
        match self {
            TokenType::EDDSA => "EDDSA",
            TokenType::ES256 => "ES256",
            TokenType::ES384 => "ES384",
            TokenType::ES512 => "ES512",
            TokenType::HS256 => "HS256",
            TokenType::HS384 => "HS384",
            TokenType::HS512 => "HS512",
            TokenType::PS256 => "PS256",
            TokenType::PS384 => "PS384",
            TokenType::PS512 => "PS512",
            TokenType::RS256 => "RS256",
            TokenType::RS384 => "RS384",
            TokenType::RS512 => "RS512",
        }
    }
}

impl Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl Render for TokenType {
    fn render(&self, out: &mut Renderer) {
        out.push_str(self.as_str());
    }
}

/// What a token authorizes.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TokenTarget {
    /// The namespace
    Namespace,
    /// The database
    Database,
    /// A scope
    Scope(Scope),
}

impl Render for TokenTarget {
    fn render(&self, out: &mut Renderer) {
        match self {
            TokenTarget::Namespace => {
                out.push_str("NAMESPACE");
            }
            TokenTarget::Database => {
                out.push_str("DATABASE");
            }
            TokenTarget::Scope(scope) => {
                out.push_str("SCOPE ").push(scope);
            }
        }
    }
}

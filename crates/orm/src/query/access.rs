use std::fmt::{self, Display};

use crate::{Render, Renderer};

/// A JWT signature algorithm.
#[expect(missing_docs, reason = "each variant is the algorithm's own name")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum JwtAlgorithm {
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

impl JwtAlgorithm {
    /// The algorithm's SurrealQL keyword.
    pub fn as_str(self) -> &'static str {
        match self {
            JwtAlgorithm::EDDSA => "EDDSA",
            JwtAlgorithm::ES256 => "ES256",
            JwtAlgorithm::ES384 => "ES384",
            JwtAlgorithm::ES512 => "ES512",
            JwtAlgorithm::HS256 => "HS256",
            JwtAlgorithm::HS384 => "HS384",
            JwtAlgorithm::HS512 => "HS512",
            JwtAlgorithm::PS256 => "PS256",
            JwtAlgorithm::PS384 => "PS384",
            JwtAlgorithm::PS512 => "PS512",
            JwtAlgorithm::RS256 => "RS256",
            JwtAlgorithm::RS384 => "RS384",
            JwtAlgorithm::RS512 => "RS512",
        }
    }
}

impl Display for JwtAlgorithm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl Render for JwtAlgorithm {
    fn render(&self, out: &mut Renderer) {
        out.push_str(self.as_str());
    }
}

/// The level an access method is defined on.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AccessLevel {
    /// `ROOT`
    Root,
    /// `NAMESPACE`
    Namespace,
    /// `DATABASE`
    Database,
}

impl Render for AccessLevel {
    fn render(&self, out: &mut Renderer) {
        out.push_str(match self {
            AccessLevel::Root => "ROOT",
            AccessLevel::Namespace => "NAMESPACE",
            AccessLevel::Database => "DATABASE",
        });
    }
}

use std::{borrow::Cow, fmt};

use surrealdb::types::{Kind, Table};

use crate::{Render, Renderer};

/// A SurrealQL identifier, rendered bare when it is a plain identifier and escaped otherwise.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Ident(Cow<'static, str>);

impl Ident {
    /// Creates an identifier. An empty name fails when the query is rendered.
    pub fn new(name: impl Into<Cow<'static, str>>) -> Self {
        Self(name.into())
    }

    /// The unescaped name.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Render for Ident {
    fn render(&self, out: &mut Renderer) {
        out.push_ident(&self.0);
    }
}

impl From<&str> for Ident {
    fn from(name: &str) -> Self {
        Self(Cow::Owned(name.to_owned()))
    }
}

impl From<String> for Ident {
    fn from(name: String) -> Self {
        Self(Cow::Owned(name))
    }
}

impl From<&Ident> for Ident {
    fn from(name: &Ident) -> Self {
        name.clone()
    }
}

impl fmt::Display for Ident {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl Render for Table {
    fn render(&self, out: &mut Renderer) {
        if self.as_str().is_empty() {
            out.push_error("table names cannot be empty");
        }
        out.push_syntax(self);
    }
}

/// A type in SurrealDB's canonical syntax, e.g. `none | string`, `record<user>`.
impl Render for Kind {
    fn render(&self, out: &mut Renderer) {
        out.push_syntax(self);
    }
}

macro_rules! define_names {
    ($($(#[$attr:meta])* $name:ident),* $(,)?) => {
        $(
            $(#[$attr])*
            #[derive(Debug, Clone, PartialEq, Eq, Hash)]
            pub struct $name(Ident);

            impl $name {
                /// Creates the name.
                pub fn new(name: impl Into<Cow<'static, str>>) -> Self {
                    Self(Ident::new(name))
                }

                /// The unescaped name.
                pub fn as_str(&self) -> &str {
                    self.0.as_str()
                }
            }

            impl Render for $name {
                fn render(&self, out: &mut Renderer) {
                    self.0.render(out);
                }
            }

            impl From<&str> for $name {
                fn from(name: &str) -> Self {
                    Self(name.into())
                }
            }

            impl From<String> for $name {
                fn from(name: String) -> Self {
                    Self(name.into())
                }
            }

            impl From<Ident> for $name {
                fn from(name: Ident) -> Self {
                    Self(name)
                }
            }

            impl fmt::Display for $name {
                fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    self.0.fmt(f)
                }
            }
        )*
    };
}

define_names!(
    /// A namespace name.
    Namespace,
    /// A database name.
    Database,
    /// An access method name.
    Access,
    /// A user name.
    User,
    /// An event name.
    Event,
    /// An index name.
    TableIndex,
);

/// A list of tables.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Tables(Vec<Table>);

impl From<Vec<Table>> for Tables {
    fn from(value: Vec<Table>) -> Self {
        Self(value)
    }
}

impl<const N: usize> From<[Table; N]> for Tables {
    fn from(value: [Table; N]) -> Self {
        Self(value.into())
    }
}

impl<const N: usize> From<&[Table; N]> for Tables {
    fn from(value: &[Table; N]) -> Self {
        Self(value.to_vec())
    }
}

impl<const N: usize> From<&[&Table; N]> for Tables {
    fn from(value: &[&Table; N]) -> Self {
        Self(value.iter().map(|&table| table.clone()).collect())
    }
}

impl From<Tables> for Vec<Table> {
    fn from(value: Tables) -> Self {
        value.0
    }
}

impl Render for Tables {
    fn render(&self, out: &mut Renderer) {
        out.push_joined(&self.0, ", ");
    }
}

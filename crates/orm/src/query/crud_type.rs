use std::fmt::{self, Display};

use crate::{Render, Renderer};

/// CRUD operation type
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CrudType {
    /// Creates a new record in the database
    Create,
    /// Retrieves existing records from the database
    Select,
    /// Updates existing records in the database
    Update,
    /// Deletes existing records from the database
    Delete,
}

impl CrudType {
    /// The operation's SurrealQL keyword.
    pub fn as_str(self) -> &'static str {
        match self {
            CrudType::Create => "create",
            CrudType::Select => "select",
            CrudType::Update => "update",
            CrudType::Delete => "delete",
        }
    }
}

impl Display for CrudType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl Render for CrudType {
    fn render(&self, out: &mut Renderer) {
        out.push_str(self.as_str());
    }
}

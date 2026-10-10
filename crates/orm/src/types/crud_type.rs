/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 */

use std::fmt::{self, Display};

/// CRUD operation type
#[derive(Clone, Copy, Debug)]
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

impl Display for CrudType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let crud_type = match self {
            CrudType::Create => "create",
            CrudType::Select => "select",
            CrudType::Update => "update",
            CrudType::Delete => "delete",
        };
        write!(f, "{}", crud_type)
    }
}

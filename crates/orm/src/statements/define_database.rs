use crate::{Database, Renderer};

use super::{Statement, impl_definition};

/// Define a new database statement.
///
/// # Arguments
///
/// * `database` - The name of the database to be defined.
///
/// # Example
/// ```rust
///  use umbriel::{*, statements::define_database};
///  assert_eq!(
///          define_database("acme").to_debug_sql(),
///          "DEFINE DATABASE acme;"
///      );
///
///  let acme = Database::new("acme");
///  assert_eq!(
///          define_database(acme).to_query().unwrap().sql(),
///          "DEFINE DATABASE acme;"
///      );
/// ```
pub fn define_database(database: impl Into<Database>) -> DefineDatabaseStatement {
    DefineDatabaseStatement {
        database: database.into(),
    }
}

/// A statement for defining a database.
pub struct DefineDatabaseStatement {
    database: Database,
}

impl Statement for DefineDatabaseStatement {
    fn render_body(&self, out: &mut Renderer) {
        out.push_str("DEFINE DATABASE ").push(&self.database);
    }
}

impl_definition!(DefineDatabaseStatement);

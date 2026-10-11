use crate::{Database, Sql};

use super::{Statement, impl_definition};

/// Remove database statement.
///
/// # Arguments
///
/// * `database` - The name of the database to be removed.
///
/// # Example
/// ```rust
///  use aparecium::{*, statements::remove_database};
///  assert_eq!(
///          remove_database("acme").to_debug_sql(),
///          "REMOVE DATABASE acme;"
///      );
///
///  let acme = Database::new("acme");
///  assert_eq!(
///          remove_database(acme).to_query().unwrap().sql(),
///          "REMOVE DATABASE acme;"
///      );
/// ```
pub fn remove_database(database: impl Into<Database>) -> RemoveDatabaseStatement {
    RemoveDatabaseStatement {
        database: database.into(),
    }
}

/// A statement for removing a database.
pub struct RemoveDatabaseStatement {
    database: Database,
}

impl Statement for RemoveDatabaseStatement {
    fn render_body(&self, sql: &mut Sql) {
        sql.push_str("REMOVE DATABASE ").push(&self.database);
    }
}

impl_definition!(RemoveDatabaseStatement);

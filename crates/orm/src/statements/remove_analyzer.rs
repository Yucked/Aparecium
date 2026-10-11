use crate::{Ident, Sql};

use super::{Statement, impl_definition};

/// Creates a REMOVE ANALYZER statement builder.
///
/// Examples
/// ```rust
/// use aparecium::{*, statements::remove_analyzer};
/// assert_eq!(remove_analyzer("standard").to_debug_sql(), "REMOVE ANALYZER standard;");
/// ```
pub fn remove_analyzer(name: impl Into<Ident>) -> RemoveAnalyzerStatement {
    RemoveAnalyzerStatement { name: name.into() }
}

/// Represents the REMOVE ANALYZER statement.
pub struct RemoveAnalyzerStatement {
    name: Ident,
}

impl Statement for RemoveAnalyzerStatement {
    fn render_body(&self, sql: &mut Sql) {
        sql.push_str("REMOVE ANALYZER ").push(&self.name);
    }
}

impl_definition!(RemoveAnalyzerStatement);

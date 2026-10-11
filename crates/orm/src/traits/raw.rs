
use std::fmt::{Display, Formatter};

use crate::{Queryable, Render, Sql};

/// SurrealQL written by the caller, rendered verbatim.
///
/// Raw text may name any parameter, including the `$_p…` names generated for bound values, so a
/// query that contains it may bind no values: rendering such a query fails. Raw text runs as a
/// query of its own or inside definitions, which write their values inline.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Raw {
    text: String,
    /// Errors of the node a definition was rendered from; they surface when the text is run.
    errors: Vec<String>,
}

impl Raw {
    /// Creates a new `Raw` query.
    pub fn new(query: impl Into<String>) -> Self {
        Self {
            text: query.into(),
            errors: vec![],
        }
    }

    /// A definition rendered with its values inline, keeping any errors found while building it so
    /// they fail the query the text later runs in.
    #[doc(hidden)]
    pub fn inline(definition: &(impl Render + ?Sized)) -> Self {
        let sql = definition.to_sql();
        Self {
            text: sql.to_debug_sql(),
            errors: sql.errors().to_vec(),
        }
    }

    /// Returns true if the query is empty.
    pub fn is_empty(&self) -> bool {
        self.text.is_empty()
    }

    /// The query text.
    pub fn as_str(&self) -> &str {
        &self.text
    }
}

impl Render for Raw {
    fn render(&self, sql: &mut Sql) {
        for error in &self.errors {
            sql.push_error(error.as_str());
        }
        sql.push_raw(&self.text);
    }
}

impl Queryable for Raw {}

impl Display for Raw {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.text)
    }
}

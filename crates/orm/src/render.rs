//! Structural query rendering.
//!
//! Fragments carry values, never parameter names. Names are allocated in render order only when a
//! complete query is produced, so nested and reused fragments cannot collide and a query renders
//! identically every time. Debug SQL runs the same pass with values written inline, so no text is
//! ever searched and replaced.

use std::fmt::Write as _;

use surrealdb::types::{SqlFormat, ToSql, Value, Variables};

use crate::{Error, Result};

/// Prefix of the parameter names allocated for bound values. Caller-named parameters may not
/// start with it.
pub const GENERATED_PARAM_PREFIX: &str = "_p";

/// A SurrealQL fragment: query text interleaved with the values it binds, plus any errors found
/// while building it.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Sql {
    parts: Vec<Part>,
    errors: Vec<String>,
    /// Whether caller-written SurrealQL is part of the fragment. Such text could name a generated
    /// parameter, so a query holding it may bind no values.
    raw: bool,
}

#[derive(Debug, Clone, PartialEq)]
enum Part {
    Text(String),
    Value(Value),
}

impl Sql {
    /// Creates an empty fragment.
    pub fn new() -> Self {
        Self::default()
    }

    /// Appends SurrealQL syntax. Never pass caller data here; values go through `push_value`.
    pub(crate) fn push_str(&mut self, text: &str) -> &mut Self {
        if !text.is_empty() {
            self.text_mut().push_str(text);
        }
        self
    }

    /// Appends SurrealQL written by the caller, verbatim.
    pub(crate) fn push_raw(&mut self, text: &str) -> &mut Self {
        self.raw = true;
        self.push_str(text)
    }

    /// Appends an identifier, escaped when it is not a plain SurrealQL identifier.
    pub(crate) fn push_ident(&mut self, name: &str) -> &mut Self {
        if name.is_empty() {
            self.push_error("identifiers cannot be empty");
        }
        write_ident(self.text_mut(), name);
        self
    }

    /// Appends a syntax element (a table name, a type) in SurrealDB's own formatting.
    pub(crate) fn push_syntax(&mut self, syntax: &impl ToSql) -> &mut Self {
        syntax.fmt_sql(self.text_mut(), SqlFormat::SingleLine);
        self
    }

    /// Appends SurrealQL syntax produced by a `Display` impl, such as a keyword enum.
    pub(crate) fn push_display(&mut self, syntax: &impl std::fmt::Display) -> &mut Self {
        // Writing to a `String` cannot fail.
        let _ = write!(self.text_mut(), "{syntax}");
        self
    }

    /// Appends a value, bound as a parameter when the query is rendered.
    pub fn push_value(&mut self, value: Value) -> &mut Self {
        self.parts.push(Part::Value(value));
        self
    }

    /// Appends another fragment.
    pub fn push(&mut self, node: &(impl Render + ?Sized)) -> &mut Self {
        node.render(self);
        self
    }

    /// Appends a fragment with its values written as literals. Used where SurrealDB stores the
    /// expression for later evaluation, such as definitions, where query parameters no longer exist.
    pub(crate) fn push_inlined(&mut self, node: &(impl Render + ?Sized)) -> &mut Self {
        let inner = node.to_sql();
        for part in &inner.parts {
            match part {
                Part::Text(text) => {
                    self.push_str(text);
                }
                Part::Value(value) => value.fmt_sql(self.text_mut(), SqlFormat::SingleLine),
            }
        }
        self.errors.extend(inner.errors);
        self.raw |= inner.raw;
        self
    }

    /// Appends `items` separated by `separator`.
    pub(crate) fn push_joined<I>(&mut self, items: I, separator: &str) -> &mut Self
    where
        I: IntoIterator,
        I::Item: Render,
    {
        for (index, item) in items.into_iter().enumerate() {
            if index > 0 {
                self.push_str(separator);
            }
            item.render(self);
        }
        self
    }

    /// Records an error that makes the query fail before it is executed.
    pub(crate) fn push_error(&mut self, error: impl Into<String>) -> &mut Self {
        self.errors.push(error.into());
        self
    }

    /// Whether the fragment renders to nothing.
    pub fn is_empty(&self) -> bool {
        self.parts.is_empty()
    }

    /// Errors recorded while building the fragment.
    pub fn errors(&self) -> &[String] {
        &self.errors
    }

    fn text_mut(&mut self) -> &mut String {
        if !matches!(self.parts.last(), Some(Part::Text(_))) {
            self.parts.push(Part::Text(String::new()));
        }
        match self.parts.last_mut() {
            Some(Part::Text(text)) => text,
            _ => unreachable!("a text part was just ensured"),
        }
    }

    fn append(&mut self, other: &Sql) {
        for part in &other.parts {
            match part {
                Part::Text(text) => {
                    self.push_str(text);
                }
                Part::Value(value) => self.parts.push(Part::Value(value.clone())),
            }
        }
        self.errors.extend_from_slice(&other.errors);
        self.raw |= other.raw;
    }

    /// Renders the parameterised query and its variables, or the errors recorded while building it.
    pub fn into_query(self) -> Result<Query> {
        if !self.errors.is_empty() {
            return Err(Error::QueryBuilder(self.errors.join("; ")));
        }
        if self.raw && self.parts.iter().any(|part| matches!(part, Part::Value(_))) {
            return Err(Error::QueryBuilder(format!(
                "raw SurrealQL cannot share a query with bound values: it could refer to their \
                 generated `${GENERATED_PARAM_PREFIX}…` parameters; run it as a query of its own"
            )));
        }

        let mut sql = String::new();
        let mut vars = Variables::new();
        let mut next = 0usize;
        for part in self.parts {
            match part {
                Part::Text(text) => sql.push_str(&text),
                Part::Value(value) => {
                    let name = format!("{GENERATED_PARAM_PREFIX}{next}");
                    // Writing to a `String` cannot fail.
                    let _ = write!(sql, "${name}");
                    vars.insert(name, value);
                    next += 1;
                }
            }
        }

        Ok(Query { sql, vars })
    }

    /// Renders the query with values written inline. For diagnostics only: execution always binds.
    pub fn to_debug_sql(&self) -> String {
        let mut sql = String::new();
        for part in &self.parts {
            match part {
                Part::Text(text) => sql.push_str(text),
                Part::Value(value) => value.fmt_sql(&mut sql, SqlFormat::SingleLine),
            }
        }
        sql
    }
}

impl Render for Sql {
    fn render(&self, sql: &mut Sql) {
        sql.append(self);
    }

    fn to_sql(&self) -> Sql {
        self.clone()
    }
}

/// A rendered query: SurrealQL text and the variables its parameters refer to.
#[derive(Debug, Clone, PartialEq)]
pub struct Query {
    sql: String,
    vars: Variables,
}

impl Query {
    /// The SurrealQL text.
    pub fn sql(&self) -> &str {
        &self.sql
    }

    /// The bound variables, keyed by parameter name without `$`.
    pub fn vars(&self) -> &Variables {
        &self.vars
    }

    /// Splits the query into its text and variables.
    pub fn into_parts(self) -> (String, Variables) {
        (self.sql, self.vars)
    }
}

/// Anything that renders into SurrealQL.
#[diagnostic::on_unimplemented(
    message = "`{Self}` is not a query fragment",
    label = "cannot be rendered into SurrealQL",
    note = "use an Aparecium statement, field, parameter or value here"
)]
pub trait Render {
    /// Writes this node into `sql`.
    fn render(&self, sql: &mut Sql);

    /// Renders this node into a new fragment.
    fn to_sql(&self) -> Sql {
        let mut sql = Sql::new();
        self.render(&mut sql);
        sql
    }

    /// Renders the parameterised query and its variables.
    fn to_query(&self) -> Result<Query> {
        self.to_sql().into_query()
    }

    /// Renders the query with values inline, for diagnostics.
    fn to_debug_sql(&self) -> String {
        self.to_sql().to_debug_sql()
    }
}

impl<T: Render + ?Sized> Render for &T {
    fn render(&self, sql: &mut Sql) {
        (**self).render(sql);
    }
}

/// Lists render comma-separated, as in projections, assignments and arguments.
impl<T: Render> Render for [T] {
    fn render(&self, sql: &mut Sql) {
        sql.push_joined(self, ", ");
    }
}

impl<T: Render> Render for Vec<T> {
    fn render(&self, sql: &mut Sql) {
        sql.push_joined(self, ", ");
    }
}

impl<T: Render> Render for Option<T> {
    fn render(&self, sql: &mut Sql) {
        if let Some(node) = self {
            node.render(sql);
        }
    }
}

/// Writes `name` as a SurrealQL identifier: bare when it is a plain identifier, otherwise
/// backtick-quoted with the same escapes SurrealDB uses for table names.
pub(crate) fn write_ident(out: &mut String, name: &str) {
    let plain = name
        .bytes()
        .next()
        .is_some_and(|first| first.is_ascii_alphabetic() || first == b'_')
        && name.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'_');
    if plain {
        out.push_str(name);
        return;
    }

    out.push('`');
    for c in name.chars() {
        match c {
            '`' => out.push_str("\\`"),
            '\\' => out.push_str("\\\\"),
            '\0' => out.push_str("\\0"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            '\n' => out.push_str("\\n"),
            '\x08' => out.push_str("\\u{8}"),
            '\x0C' => out.push_str("\\f"),
            c => out.push(c),
        }
    }
    out.push('`');
}

/// Builds a [`Sql`] fragment from string literals (SurrealQL syntax) and nodes implementing
/// [`Render`], in order.
macro_rules! sql {
    (@push $sql:ident;) => {};
    (@push $sql:ident; $text:literal $(, $($rest:tt)*)?) => {
        $sql.push_str($text);
        $crate::render::sql!(@push $sql; $($($rest)*)?);
    };
    (@push $sql:ident; $node:expr $(, $($rest:tt)*)?) => {
        $sql.push(&$node);
        $crate::render::sql!(@push $sql; $($($rest)*)?);
    };
    ($($parts:tt)+) => {{
        let mut sql = $crate::Sql::new();
        $crate::render::sql!(@push sql; $($parts)+);
        sql
    }};
}
pub(crate) use sql;

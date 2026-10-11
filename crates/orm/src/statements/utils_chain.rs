
use crate::{Block, Queryable, Render, Sql, ValueLike};

/// Chains together multiple queries into a single `QueryChain`.
///
/// # Arguments
///
/// * `query` - The first query in the chain. Subsequent queries can be added using the `chain` method on the returned `QueryChain`.
///
/// # Example Usage
///
/// ```rust
/// use aparecium::{*, statements::select, types::RecordId};
///
/// // Create a query chain with a single query
/// let user_lowo = RecordId::new("user", "oyelowo");
/// let user_dayo = RecordId::new("user", "oyedayo");
///
/// // Append a new query to the chain
/// let query1 = select(All).from(user_lowo);
/// let query2 = select(All).from(user_dayo).limit(10);
/// let chain = chain(query1).chain(query2);
///
/// assert_eq!(chain.to_query().unwrap().sql(), "SELECT * FROM $_p0;\n\nSELECT * FROM $_p1 LIMIT $_p2;");
/// assert_eq!(chain.to_debug_sql(), "SELECT * FROM user:oyelowo;\n\nSELECT * FROM user:oyedayo LIMIT 10;");
/// ```
pub fn chain(query: impl Queryable) -> QueryChain {
    QueryChain {
        queries: vec![query.to_sql()],
        parenthesized: false,
    }
}

/// Chains together multiple queries into a single `QueryChain`.
///
/// A `QueryChain` is created with an initial query, and additional queries can be added to the chain using the `chain` method.
#[derive(Debug, Clone, PartialEq)]
pub struct QueryChain {
    queries: Vec<Sql>,
    parenthesized: bool,
}

/// A chainable query.
#[derive(Debug, Clone)]
pub struct Chainable(Sql);

impl<T: Render> From<T> for Chainable {
    fn from(query: T) -> Self {
        Self(query.to_sql())
    }
}

impl QueryChain {
    /// Appends a new query to the end of the chain.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use aparecium::{*, statements::select, types::RecordId};
    ///
    /// // Create a query chain with a single query
    /// let user_lowo = RecordId::new("user", "oyelowo");
    /// let user_dayo = RecordId::new("user", "oyedayo");
    ///
    /// // Append a new query to the chain
    /// let query1 = select(All).from(user_lowo);
    /// let query2 = select(All).from(user_dayo).limit(10);
    /// let chain = chain(query1).chain(query2);
    ///
    /// assert_eq!(chain.to_query().unwrap().sql(), "SELECT * FROM $_p0;\n\nSELECT * FROM $_p1 LIMIT $_p2;");
    /// assert_eq!(chain.to_debug_sql(), "SELECT * FROM user:oyelowo;\n\nSELECT * FROM user:oyedayo LIMIT 10;");
    /// ```
    pub fn chain(mut self, query: impl Into<Chainable>) -> Self {
        self.queries.push(query.into().0);
        self
    }

    /// Surrounds the query chain with a parenthesis.
    pub fn parenthesized(mut self) -> Self {
        self.parenthesized = true;
        self
    }

    /// Surrounds the query chain with a curly brace.
    pub fn as_block(self) -> Block {
        self.into()
    }
}

impl Queryable for QueryChain {}

impl Render for QueryChain {
    fn render(&self, sql: &mut Sql) {
        if self.parenthesized {
            sql.push_str("(");
        }
        sql.push_joined(&self.queries, "\n\n");
        if self.parenthesized {
            sql.push_str(")");
        }
    }
}

impl From<Vec<ValueLike>> for QueryChain {
    fn from(values: Vec<ValueLike>) -> Self {
        Self {
            queries: values.iter().map(Render::to_sql).collect(),
            parenthesized: false,
        }
    }
}

impl From<Vec<Chainable>> for QueryChain {
    fn from(values: Vec<Chainable>) -> Self {
        Self {
            queries: values.into_iter().map(|query| query.0).collect(),
            parenthesized: false,
        }
    }
}

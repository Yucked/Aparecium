use crate::{Block, Fragment, Queryable, Render, Renderer, ValueLike};

/// Chains together multiple queries into a single `QueryChain`.
///
/// # Arguments
///
/// * `query` - The first query in the chain. Subsequent queries can be added using the `chain` method on the returned `QueryChain`.
///
/// # Example Usage
///
/// ```rust
/// use umbriel::{*, statements::select, types::RecordId};
///
/// // Create a query chain with a single query
/// let user_rlie = RecordId::new("user", "charlie");
/// let user_nett = RecordId::new("user", "bennett");
///
/// // Append a new query to the chain
/// let query1 = select(All).from(user_rlie);
/// let query2 = select(All).from(user_nett).limit(10);
/// let chain = chain(query1).chain(query2);
///
/// assert_eq!(chain.to_query().unwrap().sql(), "SELECT * FROM $_p0;\n\nSELECT * FROM $_p1 LIMIT $_p2;");
/// assert_eq!(chain.to_debug_sql(), "SELECT * FROM user:charlie;\n\nSELECT * FROM user:bennett LIMIT 10;");
/// ```
pub fn chain(query: impl Queryable) -> QueryChain {
    QueryChain {
        queries: vec![query.into_fragment()],
        parenthesized: false,
    }
}

/// Chains together multiple queries into a single `QueryChain`.
///
/// A `QueryChain` is created with an initial query, and additional queries can be added to the chain using the `chain` method.
#[derive(Debug, Clone, PartialEq)]
pub struct QueryChain {
    queries: Vec<Fragment>,
    parenthesized: bool,
}

/// A chainable query.
#[derive(Debug, Clone)]
pub struct Chainable(Fragment);

impl<T: Queryable> From<T> for Chainable {
    fn from(query: T) -> Self {
        Self(query.into_fragment())
    }
}

impl QueryChain {
    /// Appends a new query to the end of the chain.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use umbriel::{*, statements::select, types::RecordId};
    ///
    /// // Create a query chain with a single query
    /// let user_rlie = RecordId::new("user", "charlie");
    /// let user_nett = RecordId::new("user", "bennett");
    ///
    /// // Append a new query to the chain
    /// let query1 = select(All).from(user_rlie);
    /// let query2 = select(All).from(user_nett).limit(10);
    /// let chain = chain(query1).chain(query2);
    ///
    /// assert_eq!(chain.to_query().unwrap().sql(), "SELECT * FROM $_p0;\n\nSELECT * FROM $_p1 LIMIT $_p2;");
    /// assert_eq!(chain.to_debug_sql(), "SELECT * FROM user:charlie;\n\nSELECT * FROM user:bennett LIMIT 10;");
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

impl Queryable for QueryChain {
    fn into_fragment(self) -> Fragment {
        Fragment::new(self)
    }
}

impl Render for QueryChain {
    fn render(&self, out: &mut Renderer) {
        if self.parenthesized {
            out.push_str("(");
        }
        out.push_joined(&self.queries, "\n\n");
        if self.parenthesized {
            out.push_str(")");
        }
    }
}

impl From<Vec<ValueLike>> for QueryChain {
    fn from(values: Vec<ValueLike>) -> Self {
        Self {
            queries: values.into_iter().map(Fragment::new).collect(),
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

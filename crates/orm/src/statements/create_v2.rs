use crate::{
    DurationLike, Node, Renderer, ReturnType, ReturnableDefault, ReturnableStandard, Setter,
};

use super::{CreateStatement, Statement, create, impl_statement};

/// The data of a record to create: a whole model (`CONTENT`) or assignments (`SET`).
#[derive(Debug, Clone)]
pub enum ContentOrSets<T> {
    /// serializable surrealdb node struct used with CONTENT in create statement.
    Content(T),
    /// assignments used with SET in create statement.
    Sets(Vec<Setter>),
}

impl<T: Node> From<T> for ContentOrSets<T> {
    fn from(content: T) -> Self {
        ContentOrSets::Content(content)
    }
}

impl<T: Node> From<Vec<Setter>> for ContentOrSets<T> {
    fn from(sets: Vec<Setter>) -> Self {
        ContentOrSets::Sets(sets)
    }
}

impl<T: Node> From<Setter> for ContentOrSets<T> {
    fn from(set: Setter) -> Self {
        ContentOrSets::Sets(vec![set])
    }
}

impl<const N: usize, T: Node> From<[Setter; N]> for ContentOrSets<T> {
    fn from(sets: [Setter; N]) -> Self {
        ContentOrSets::Sets(sets.into())
    }
}

impl<T: Node> From<&[Setter]> for ContentOrSets<T> {
    fn from(sets: &[Setter]) -> Self {
        ContentOrSets::Sets(sets.to_vec())
    }
}

/// Creates a new CREATE SQL statement for a given type from its content or assignments.
///
/// # Examples
///
/// ```rust, ignore
/// create_v2(User {
///     id: User::id("alice"),
///     name: "Alice".to_string(),
/// });
/// ```
pub fn create_v2<T: Node>(content: impl Into<ContentOrSets<T>>) -> CreateStatementV2<T> {
    CreateStatementV2(match content.into() {
        ContentOrSets::Content(content) => create().content(content),
        ContentOrSets::Sets(sets) => create().set(sets),
    })
}

/// A CREATE statement built by [`create_v2`].
pub struct CreateStatementV2<T: Node>(CreateStatement<T>);

impl<T: Node> CreateStatementV2<T> {
    /// Sets the return type for the query.
    pub fn return_type(self, return_type: impl Into<ReturnType>) -> Self {
        Self(self.0.return_type(return_type))
    }

    /// Sets the timeout duration for the query. It is written as a literal.
    pub fn timeout(self, duration: impl Into<DurationLike>) -> Self {
        Self(self.0.timeout(duration))
    }

    /// Indicates that the query should be executed in parallel.
    pub fn parallel(self) -> Self {
        Self(self.0.parallel())
    }
}

impl<T: Node> Statement for CreateStatementV2<T> {
    fn render_body(&self, out: &mut Renderer) {
        self.0.render_body(out);
    }
}

impl_statement!([T: Node] CreateStatementV2<T>);

impl<T: Node> ReturnableDefault<T> for CreateStatementV2<T> {}

impl<T> ReturnableStandard<T> for CreateStatementV2<T>
where
    T: Node + Send + Sync,
{
    fn set_return_type(self, return_type: ReturnType) -> Self {
        Self(self.0.set_return_type(return_type))
    }

    fn get_return_type(&self) -> ReturnType {
        self.0.get_return_type()
    }
}

use std::marker::PhantomData;

use crate::{
    DurationLike, Node, Renderer, ReturnType, ReturnableDefault, ReturnableStandard, Setter,
    ValueLike,
};

use super::{Statement, impl_statement};

/// Creates a new CREATE SQL statement for a given type.
///
/// # Examples
///
/// ```rust, ignore
/// create().content(User {
///     id: User::id("alice"),
///     name: "Alice".to_string(),
/// });
/// ```
pub fn create<T: Node>() -> CreateStatementInit<T> {
    CreateStatementInit::<T> {
        is_only: false,
        data: Data::None,
        return_type: None,
        timeout: None,
        parallel: false,
        model: PhantomData,
    }
}

/// Creates a new CREATE ONLY SQL statement for a given type, returning a single object.
pub fn create_only<T: Node>() -> CreateStatementInit<T> {
    let mut create_statement = create::<T>();
    create_statement.is_only = true;
    create_statement
}

#[derive(Debug, Clone)]
enum Data {
    None,
    Content(ValueLike),
    Set(Vec<Setter>),
}

/// Initializes the create statement
#[derive(Debug, Clone)]
pub struct CreateStatementInit<T: Node> {
    is_only: bool,
    data: Data,
    return_type: Option<ReturnType>,
    timeout: Option<DurationLike>,
    parallel: bool,
    model: PhantomData<fn() -> T>,
}

impl<T: Node> CreateStatementInit<T> {
    /// Sets the content of the record to be created.
    /// When using this, the type can be automatically inferred unlike the `set` method.
    pub fn content(mut self, content: T) -> CreateStatement<T> {
        self.data = Data::Content(ValueLike::value(content));
        CreateStatement(self)
    }

    /// Sets the values of the fields of the record to be created.
    ///
    /// # Arguments
    ///
    /// * `settables` - one assignment from `equal_to`, or a list of them, e.g. from the
    ///   `object!` macro.
    pub fn set(mut self, settables: impl Into<Vec<Setter>>) -> CreateStatement<T> {
        let settables: Vec<Setter> = settables.into();
        match &mut self.data {
            Data::Set(setters) => setters.extend(settables),
            data => *data = Data::Set(settables),
        }
        CreateStatement(self)
    }
}

/// A CREATE statement.
pub struct CreateStatement<T: Node>(CreateStatementInit<T>);

impl<T: Node> CreateStatement<T> {
    /// Sets the return type for the query.
    pub fn return_type(mut self, return_type: impl Into<ReturnType>) -> Self {
        self.0.return_type = Some(return_type.into());
        self
    }

    /// Sets the timeout duration for the query. It is written as a literal.
    pub fn timeout(mut self, duration: impl Into<DurationLike>) -> Self {
        self.0.timeout = Some(duration.into());
        self
    }

    /// Indicates that the query should be executed in parallel.
    pub fn parallel(mut self) -> Self {
        self.0.parallel = true;
        self
    }
}

impl<T: Node> Statement for CreateStatement<T> {
    fn render_body(&self, out: &mut Renderer) {
        let statement = &self.0;
        out.push_str("CREATE ");
        if statement.is_only {
            out.push_str("ONLY ");
        }
        out.push(&T::table());

        match &statement.data {
            Data::None => {}
            Data::Content(content) => {
                out.push_str(" CONTENT ").push(content);
            }
            Data::Set(setters) => {
                out.push_str(" SET ").push(setters);
            }
        }

        if let Some(return_type) = &statement.return_type {
            out.push_str(" ").push(return_type);
        }

        if let Some(timeout) = &statement.timeout {
            out.push_str(" TIMEOUT ").push_inlined(timeout);
        }

        if statement.parallel {
            out.push_str(" PARALLEL");
        }
    }
}

impl_statement!([T: Node] CreateStatement<T>);

impl<T: Node> ReturnableDefault<T> for CreateStatement<T> {}

impl<T> ReturnableStandard<T> for CreateStatement<T>
where
    T: Node + Send + Sync,
{
    fn set_return_type(mut self, return_type: ReturnType) -> Self {
        self.0.return_type = Some(return_type);
        self
    }

    fn get_return_type(&self) -> ReturnType {
        self.0.return_type.clone().unwrap_or(ReturnType::None)
    }
}

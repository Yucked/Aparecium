// Statement syntax
// RELATE [ ONLY ] @from -> @table -> @with
// 	[ CONTENT @value
// 	  | SET @field = @value ...
// 	]
// 	[ RETURN [ NONE | BEFORE | AFTER | DIFF | @projections ... ]
// 	[ TIMEOUT @duration ]
// ;
use std::marker::PhantomData;

use crate::{
    DurationLike, Edge, Field, Renderer, ReturnType, ReturnableDefault, ReturnableStandard, Setter,
    ValueLike,
};

use super::{Statement, impl_statement};

/// Creates a new RELATE statement.
///
/// # Arguments
///
/// * `connection` - built using `with` method on a node. e.g `Student::with(..).writes(..).book(..)`
/// # Examples
///
/// ```rust, ignore
/// // Add a graph edge between two specific records setting a field on the edge
/// relate(Student::with(student_id).writes__(Empty).book(book_id))
///     .set(updater(score).equals(5));
///
/// // Instead of specifying record data using the SET clause,
/// // it is also possible to use the CONTENT keyword to specify the record data using a SurrealQL object.
/// relate(Student::with(student_id).writes__(Empty).book(book_id))
///     .content(write);
/// // Generates e.g RELATE student:1->writes->book:2 CONTENT {...}
///
/// // Add a graph edge between multiple specific students and books
/// relate(
///     Student::with(select(All).from(Student::table()))
///         .writes__(Empty)
///         .book(select(All).from(Book::table()))
/// ).content(write)
/// // Generates e.g RELATE (select * from student)->writes->(select * from book) CONTENT {...}
/// ```
pub fn relate<T: Edge>(connection: impl Into<Field>) -> RelateStatement<T> {
    RelateStatement {
        relation: connection.into(),
        is_only: false,
        data: Data::None,
        return_type: None,
        timeout: None,
        model: PhantomData,
    }
}

/// Creates a new RELATE ONLY statement, which returns a single edge.
pub fn relate_only<T: Edge>(connection: impl Into<Field>) -> RelateStatement<T> {
    let mut relate_statement = relate(connection);
    relate_statement.is_only = true;
    relate_statement
}

#[derive(Debug, Clone)]
enum Data {
    None,
    Content(ValueLike),
    Set(Vec<Setter>),
}

/// Relate statement initialization builder
#[derive(Debug, Clone)]
pub struct RelateStatement<T: Edge> {
    /// The graph path `from->edge->to`.
    relation: Field,
    is_only: bool,
    data: Data,
    return_type: Option<ReturnType>,
    timeout: Option<DurationLike>,
    model: PhantomData<fn() -> T>,
}

impl<T: Edge> RelateStatement<T> {
    /// Sets the edge's content.
    pub fn content(mut self, content: T) -> Self {
        self.data = Data::Content(ValueLike::value(content));
        self
    }

    /// This updates records on the edge field.
    /// This clause also allows setting, incrementing and decrementing numeric values, and adding or removing values from arrays.
    ///
    /// # Examples
    ///
    /// ```rust, ignore
    /// // Set fields using a helper macro function:
    /// .set(object_partial!(Weapon {
    ///     id: weapon_id.clone(),
    ///     name: "Laser".to_string()
    /// }))
    ///
    /// // Set multiple fields as an array or vector:
    /// .set([name.equal_to("Laser"), damage.increment_by(100)]);
    ///
    /// // set a single field number. Generates  =
    /// .set(score.equal_to(5))
    ///
    /// // increment a field number. Generates  +=
    /// .set(score.increment_by(5))
    ///
    /// // decrement a field number. Generates  -=
    /// .set(score.decrement_by(5))
    ///
    /// // add to an array. Generates  +=
    /// .set(friends_names.append("Alice"))
    ///
    /// // remove value from an array. Generates  -=
    /// .set(friends_names.remove("Bob"))
    /// ```
    pub fn set(mut self, settables: impl Into<Vec<Setter>>) -> Self {
        let settables: Vec<Setter> = settables.into();
        match &mut self.data {
            Data::Set(setters) => setters.extend(settables),
            data => *data = Data::Set(settables),
        }
        self
    }

    /// Sets the return type for the query.
    pub fn return_type(mut self, return_type: impl Into<ReturnType>) -> Self {
        self.return_type = Some(return_type.into());
        self
    }

    /// Sets the timeout duration for the query. It is written as a literal.
    pub fn timeout(mut self, duration: impl Into<DurationLike>) -> Self {
        self.timeout = Some(duration.into());
        self
    }
}

impl<T: Edge> Statement for RelateStatement<T> {
    fn render_body(&self, out: &mut Renderer) {
        out.push_str("RELATE ");
        if self.is_only {
            out.push_str("ONLY ");
        }
        out.push(&self.relation);

        match &self.data {
            Data::None => {}
            Data::Content(content) => {
                out.push_str(" CONTENT ").push(content);
            }
            Data::Set(setters) => {
                out.push_str(" SET ").push(setters);
            }
        }

        if let Some(return_type) = &self.return_type {
            out.push_str(" ").push(return_type);
        }

        if let Some(timeout) = &self.timeout {
            out.push_str(" TIMEOUT ").push_inlined(timeout);
        }
    }
}

impl_statement!([T: Edge] RelateStatement<T>);

impl<T> ReturnableDefault<T> for RelateStatement<T> where T: Edge + Send + Sync {}

impl<T> ReturnableStandard<T> for RelateStatement<T>
where
    T: Edge + Send + Sync,
{
    fn set_return_type(mut self, return_type: ReturnType) -> Self {
        self.return_type = Some(return_type);
        self
    }

    fn get_return_type(&self) -> ReturnType {
        self.return_type.clone().unwrap_or(ReturnType::None)
    }
}

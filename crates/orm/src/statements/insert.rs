// Statement syntax
// INSERT [ IGNORE ] INTO @what
// 	[ @value
// 	  | (@fields) VALUES (@values)
// 		[ ON DUPLICATE KEY UPDATE @field = @value ... ]
// 	]
// ;
use std::{
    collections::{BTreeMap, BTreeSet},
    marker::PhantomData,
};

use surrealdb::types::Value;

use crate::{
    Node, Renderer, ReturnType, ReturnableDefault, ReturnableStandard, Updateables, Updater,
};

use super::{SelectStatement, Statement, Subquery, impl_statement};

/// Insert statement initialization builder
#[derive(Debug, Clone)]
pub struct InsertStatement<T: Node> {
    data: Data,
    on_duplicate_key_update: Vec<Updater>,
    return_type: Option<ReturnType>,
    model: PhantomData<fn() -> T>,
}

#[derive(Debug, Clone)]
enum Data {
    /// Column names and one row of values per record. A record without a column inserts `NONE`.
    Rows {
        fields: Vec<String>,
        rows: Vec<Vec<Value>>,
        errors: Vec<String>,
    },
    /// Copies the result of a subquery.
    Select(Subquery),
}

/// Creates a new INSERT SQL statement for a given type.
///
/// The INSERT statement can be used to insert or update data into the database, using the same statement syntax as the traditional SQL Insert statement.
///
/// # Arguments
///
/// * `insertables` - a single or list of surrealdb nodes or a select statement to
///   copy from another table.
/// # Examples
///
/// ```rust, ignore
/// // You can insert a single object
/// insert(User { id: User::id("alice"), name: "Alice".to_string(), age: 30 });
///
/// // You can also insert a list of object
/// insert(vec![
///     User { id: User::id("alice"), name: "Alice".to_string(), age: 30 },
///     User { id: User::id("bob"), name: "Bob".to_string(), age: 31 },
/// ]);
///
/// // You can also insert from another table. This is good for copying into a new table:
/// insert(select(All)
///         .from(Company::table())
///         .where_(age.greater(18))
/// );
/// ```
pub fn insert<T: Node>(insertables: impl Into<Insertables<T>>) -> InsertStatement<T> {
    let data = match insertables.into() {
        Insertables::Node(node) => rows([node]),
        Insertables::Nodes(nodes) => rows(nodes),
        Insertables::FromSubQuery(query) => Data::Select(query),
    };

    InsertStatement {
        data,
        on_duplicate_key_update: vec![],
        return_type: None,
        model: PhantomData,
    }
}

/// Converts the nodes into rows over the union of their fields, in sorted order.
fn rows<T: Node>(nodes: impl IntoIterator<Item = T>) -> Data {
    let mut errors = vec![];
    let objects: Vec<BTreeMap<String, Value>> = nodes
        .into_iter()
        .map(|node| match node.into_value() {
            Value::Object(object) => object.into_inner(),
            other => {
                errors.push(format!(
                    "a node must convert into an object to be inserted, not {:?}",
                    other.kind()
                ));
                BTreeMap::new()
            }
        })
        .collect();

    let fields: Vec<String> = objects
        .iter()
        .flat_map(BTreeMap::keys)
        .cloned()
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect();

    let rows = objects
        .into_iter()
        .map(|mut object| {
            fields
                .iter()
                .map(|field| object.remove(field).unwrap_or_default())
                .collect()
        })
        .collect();

    Data::Rows {
        fields,
        rows,
        errors,
    }
}

impl<T: Node> ReturnableDefault<T> for InsertStatement<T> {}

impl<T> ReturnableStandard<T> for InsertStatement<T>
where
    T: Node + Send + Sync,
{
    fn set_return_type(mut self, return_type: ReturnType) -> Self {
        self.return_type = Some(return_type);
        self
    }

    fn get_return_type(&self) -> ReturnType {
        self.return_type.clone().unwrap_or(ReturnType::None)
    }
}

/// Things that can be inserted including a single or list of surrealdb nodes or a select statement
/// when copying from a table to another table.
#[derive(Debug)]
pub enum Insertables<T: Node> {
    /// A single surrealdb node
    Node(T),
    /// A list of surrealdb node
    Nodes(Vec<T>),
    /// A select statement
    FromSubQuery(Subquery),
}

impl<T: Node> From<Vec<T>> for Insertables<T> {
    fn from(value: Vec<T>) -> Self {
        Self::Nodes(value)
    }
}

impl<T: Node> From<T> for Insertables<T> {
    fn from(value: T) -> Self {
        Self::Node(value)
    }
}

impl<T: Node> From<SelectStatement> for Insertables<T> {
    fn from(value: SelectStatement) -> Self {
        Self::FromSubQuery(value.into())
    }
}

impl<T: Node> From<&SelectStatement> for Insertables<T> {
    fn from(value: &SelectStatement) -> Self {
        Self::FromSubQuery(value.clone().into())
    }
}

impl<T: Node> InsertStatement<T> {
    /// Generates ON DUPLICATE KEY UPDATE clause.
    /// This updates records which already exist by specifying an ON DUPLICATE KEY UPDATE clause.
    /// This clause also allows incrementing and decrementing numeric values, and adding or removing values from arrays.
    ///
    /// # Examples
    ///
    /// ```rust, ignore
    /// // increment a field number. Generates  +=
    /// updater(score).increment_by(5)
    /// // or alias
    /// updater(score).plus_equal(5)
    ///
    /// // decrement a field number. Generates  -=
    /// updater(score).decrement_by(5)
    /// // or alias
    /// updater(score).minus_equal(5)
    ///
    /// // add to an array. Generates  +=
    /// updater(friends_names).append("Alice")
    /// // or alias
    /// updater(friends_names).plus_equal("Alice")
    ///
    /// // remove value from an array. Generates  -=
    /// updater(friends_names).remove("Bob")
    /// // or alias
    /// updater(friends_names).minus_equal("Bob")
    /// ```
    pub fn on_duplicate_key_update(mut self, updateables: impl Into<Updateables>) -> Self {
        match updateables.into() {
            Updateables::Updater(updater) => self.on_duplicate_key_update.push(updater),
            Updateables::Updaters(updaters) => self.on_duplicate_key_update.extend(updaters),
        }
        self
    }

    /// Sets the return type for the query.
    pub fn return_type(mut self, return_type: impl Into<ReturnType>) -> Self {
        self.return_type = Some(return_type.into());
        self
    }
}

impl<T: Node> Statement for InsertStatement<T> {
    fn render_body(&self, out: &mut Renderer) {
        out.push_str("INSERT INTO ").push(&T::table()).push_str(" ");

        match &self.data {
            Data::Select(query) => {
                out.push(query);
            }
            Data::Rows {
                fields,
                rows,
                errors,
            } => {
                for error in errors {
                    out.push_error(error.as_str());
                }
                out.push_str("(");
                for (index, field) in fields.iter().enumerate() {
                    if index > 0 {
                        out.push_str(", ");
                    }
                    out.push_ident(field);
                }
                out.push_str(") VALUES ");
                for (index, row) in rows.iter().enumerate() {
                    if index > 0 {
                        out.push_str(", ");
                    }
                    out.push_str("(");
                    for (index, value) in row.iter().enumerate() {
                        if index > 0 {
                            out.push_str(", ");
                        }
                        out.push_value(value);
                    }
                    out.push_str(")");
                }
            }
        }

        if !self.on_duplicate_key_update.is_empty() {
            out.push_str(" ON DUPLICATE KEY UPDATE ")
                .push(&self.on_duplicate_key_update);
        }

        if let Some(return_type) = &self.return_type {
            out.push_str(" ").push(return_type);
        }
    }
}

impl_statement!([T: Node] InsertStatement<T>);

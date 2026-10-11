// Statement syntax
// UPDATE [ ONLY ] @targets
// 	[ CONTENT @value
// 	  | MERGE @value
// 	  | PATCH @value
// 	  | SET @field = @value ...
// 	]
// 	[ WHERE @condition ]
// 	[ RETURN [ NONE | BEFORE | AFTER | DIFF | @projections ... ]
// 	[ TIMEOUT @duration ]
// 	[ PARALLEL ]
// ;
use std::marker::PhantomData;

use surrealdb::types::{self, SurrealValue, Table, ToSql};

use crate::{
    Conditional, DurationLike, Filter, Model, PatchOp, RecordId, ReturnType,
    ReturnableDefault, ReturnableStandard, Setter, Sql, ValueLike,
};

use super::{Statement, impl_statement};

/// Creates a new UPDATE statement.
/// The UPDATE statement can be used to update or modify records in the database.
///
/// # Examples
///
/// ```rust, ignore
/// use aparecium::{*, statements::update};
///
/// // Update using set method
/// update::<User>(user)
///     .set(object_partial!(User { score: 5 }))
///     .where_(age.greater_than(18));
///
/// // Replace a specific record using content method
/// update::<User>(user1).content(User { name: "Alice".into(), age: 30, ..Default::default() });
///
/// // Apply a generated patch
/// update::<User>(user2).set(UserPatch { score: Patch::Set(5), ..Default::default() }.into_setters());
/// ```
pub fn update<T: Model>(targettables: impl Into<TargettablesForUpdate>) -> UpdateStatementInit<T> {
    UpdateStatementInit {
        target: targettables.into().checked::<T>(),
        is_only: false,
        data: Data::None,
        where_: None,
        return_type: None,
        timeout: None,
        parallel: false,
        model: PhantomData,
    }
}

/// Creates a new UPDATE ONLY statement, returning a single object.
pub fn update_only<T: Model>(
    targettables: impl Into<TargettablesForUpdate>,
) -> UpdateStatementInit<T> {
    let mut statement = update(targettables);
    statement.is_only = true;
    statement
}

/// The target of an `UPDATE` or `DELETE`: the model's table or one of its records.
pub enum TargettablesForUpdate {
    /// A table, rendered as its name.
    Table(Table),
    /// A record, bound as a value.
    Record(types::RecordId),
}

impl TargettablesForUpdate {
    /// Renders the target, recording an error when it does not belong to `T`'s table.
    pub(crate) fn checked<T: Model>(self) -> Sql {
        let table = T::table();
        let mut sql = Sql::new();
        match self {
            Self::Table(target) => {
                if target != table {
                    sql.push_error(format!(
                        "table `{}` is not the table `{}` of the model",
                        target.as_str(),
                        table.as_str()
                    ));
                }
                sql.push(&target);
            }
            Self::Record(id) => {
                if id.table != table {
                    sql.push_error(format!(
                        "record `{}` does not belong to the model's table `{}`",
                        id.to_sql(),
                        table.as_str()
                    ));
                }
                sql.push_value(id.into_value());
            }
        }
        sql
    }
}

impl<T: Model> From<T> for TargettablesForUpdate {
    fn from(value: T) -> Self {
        Self::Record(value.record_id().into())
    }
}

impl From<Table> for TargettablesForUpdate {
    fn from(value: Table) -> Self {
        Self::Table(value)
    }
}

impl From<&Table> for TargettablesForUpdate {
    fn from(value: &Table) -> Self {
        Self::Table(value.clone())
    }
}

impl From<types::RecordId> for TargettablesForUpdate {
    fn from(value: types::RecordId) -> Self {
        Self::Record(value)
    }
}

impl From<&types::RecordId> for TargettablesForUpdate {
    fn from(value: &types::RecordId) -> Self {
        Self::Record(value.clone())
    }
}

impl<T: Model> From<RecordId<T>> for TargettablesForUpdate {
    fn from(value: RecordId<T>) -> Self {
        Self::Record(value.into())
    }
}

impl<T: Model> From<&RecordId<T>> for TargettablesForUpdate {
    fn from(value: &RecordId<T>) -> Self {
        Self::Record(value.into())
    }
}

#[derive(Debug, Clone)]
enum Data {
    None,
    Content(ValueLike),
    Merge(ValueLike),
    Replace(ValueLike),
    Set(Vec<Setter>),
    Patch(Vec<PatchOp>),
}

/// Update statement initializer
#[derive(Debug, Clone)]
pub struct UpdateStatementInit<T: Model> {
    target: Sql,
    is_only: bool,
    data: Data,
    where_: Option<Filter>,
    return_type: Option<ReturnType>,
    timeout: Option<DurationLike>,
    parallel: bool,
    model: PhantomData<T>,
}

impl<T: Model> UpdateStatementInit<T> {
    /// Caution! Replaces the whole record. Specify the full record data using the CONTENT keyword.
    pub fn content(mut self, content: T) -> UpdateStatement<T> {
        self.data = Data::Content(ValueLike::value(content));
        UpdateStatement(self)
    }

    /// Merges the given object into the record with the MERGE keyword. Nested objects are merged
    /// recursively; use `set` to replace a field's value instead.
    pub fn merge(mut self, merge: impl SurrealValue) -> UpdateStatement<T> {
        self.data = Data::Merge(ValueLike::value(merge));
        UpdateStatement(self)
    }

    /// Caution! Replaces the whole record with the given object, removing every field it does
    /// not contain.
    pub fn replace(mut self, replacement: impl SurrealValue) -> UpdateStatement<T> {
        self.data = Data::Replace(ValueLike::value(replacement));
        UpdateStatement(self)
    }

    /// When specifying fields to update using the SET clause,
    /// it is possible to increment and decrement numeric values,
    /// and add or remove values from arrays.
    /// To increment a numeric value, or to add an item to an array,
    /// use the `append` or incremenet (i.e +=) operator. To decrement a numeric value,
    /// or to remove an value from an array, use the `decrement` or `remove` (i.e -=) operator.
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
    pub fn set(mut self, settables: impl Into<Vec<Setter>>) -> UpdateStatement<T> {
        let settables: Vec<Setter> = settables.into();
        match &mut self.data {
            Data::Set(setters) => setters.extend(settables),
            data => *data = Data::Set(settables),
        }
        UpdateStatement(self)
    }

    /// Specify the patch operations to be applied to the record using the PATCH keyword.
    ///
    /// # Example
    /// ```rust, ignore
    /// name.patch_replace("Alice");
    /// name.patch_add("Alice");
    /// name.patch_remove();
    /// // regex search and replace
    /// name.patch_change("@@ -1,4 +1,4 @@\n te\n-s\n+x\n t\n");
    /// ```
    pub fn patch(mut self, patch_op: impl Into<Vec<PatchOp>>) -> UpdateStatement<T> {
        let patch_ops: Vec<PatchOp> = patch_op.into();
        match &mut self.data {
            Data::Patch(ops) => ops.extend(patch_ops),
            data => *data = Data::Patch(patch_ops),
        }
        UpdateStatement(self)
    }
}

/// A builder for update statements.
pub struct UpdateStatement<T: Model>(UpdateStatementInit<T>);

impl<T: Model> From<UpdateStatementInit<T>> for UpdateStatement<T> {
    fn from(value: UpdateStatementInit<T>) -> Self {
        Self(value)
    }
}

impl<T: Model> UpdateStatement<T> {
    /// Adds a condition to the `WHERE` clause of the query.
    ///
    /// # Example
    ///
    /// ```rust, ignore
    /// // You can use a simple filter without the `cond` helper function
    /// .where_(age.greater_than_or_equal(18)
    ///
    /// // or with the `cond` helper function for multiple `AND` or `OR` conditions
    /// .where_(cond(age.greater_than_or_equal(18)).and(age.less_than_or_equal(90)))
    /// ```
    pub fn where_(mut self, condition: impl Conditional) -> Self {
        self.0.where_ = Some(Filter::new(condition));
        self
    }

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

impl<T: Model> Statement for UpdateStatement<T> {
    fn render_body(&self, sql: &mut Sql) {
        let statement = &self.0;
        sql.push_str("UPDATE ");
        if statement.is_only {
            sql.push_str("ONLY ");
        }
        sql.push(&statement.target);

        match &statement.data {
            Data::None => {}
            Data::Content(content) => {
                sql.push_str(" CONTENT ").push(content);
            }
            Data::Merge(merge) => {
                sql.push_str(" MERGE ").push(merge);
            }
            Data::Replace(replacement) => {
                sql.push_str(" REPLACE ").push(replacement);
            }
            Data::Set(setters) => {
                sql.push_str(" SET ").push(setters);
            }
            Data::Patch(ops) => {
                sql.push_str(" PATCH [").push(ops).push_str("]");
            }
        }

        if let Some(condition) = &statement.where_ {
            sql.push_str(" WHERE ").push(condition);
        }

        if let Some(return_type) = &statement.return_type {
            sql.push_str(" ").push(return_type);
        }

        if let Some(timeout) = &statement.timeout {
            sql.push_str(" TIMEOUT ").push_inlined(timeout);
        }

        if statement.parallel {
            sql.push_str(" PARALLEL");
        }
    }
}

impl_statement!([T: Model] UpdateStatement<T>);

impl<T: Model> ReturnableDefault<T> for UpdateStatement<T> {}

impl<T> ReturnableStandard<T> for UpdateStatement<T>
where
    T: Model + Send + Sync,
{
    fn set_return_type(mut self, return_type: ReturnType) -> Self {
        self.0.return_type = Some(return_type);
        self
    }

    fn get_return_type(&self) -> ReturnType {
        self.0.return_type.clone().unwrap_or(ReturnType::None)
    }
}

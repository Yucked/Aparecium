// Statement syntax
// DELETE [ ONLY ] @targets
//     [ WHERE @condition ]
//     [ RETURN [ NONE | BEFORE | AFTER | DIFF | @projections ... ]
//     [ TIMEOUT @duration ]
//     [ PARALLEL ]
// ;
use std::marker::PhantomData;

use crate::{
    Conditional, DurationLike, Filter, Model, ReturnType, ReturnableDefault, ReturnableStandard,
    Sql,
};

use super::{Statement, impl_statement, update::TargettablesForUpdate};

/// Creates a new DELETE statement.
/// The DELETE statement can be used to delete records from the database.
///
/// # Examples
///
/// ```rust, ignore
/// delete::<User>(user)
///     .where_(age.less_than(18)); // simple filtering on a field
///
/// delete::<User>(user)
///     .where_(cond(age.greater_than(18)) // Or more complex filtering with `cond` helper
///         .and(age.less_than(80)));
/// ```
pub fn delete<T: Model>(targettables: impl Into<TargettablesForUpdate>) -> DeleteStatement<T> {
    DeleteStatement {
        target: targettables.into().checked::<T>(),
        is_only: false,
        where_: None,
        return_type: None,
        timeout: None,
        parallel: false,
        model: PhantomData,
    }
}

/// Creates a new DELETE ONLY statement, which returns a single record.
pub fn delete_only<T: Model>(targettables: impl Into<TargettablesForUpdate>) -> DeleteStatement<T> {
    let mut delete_statement = delete(targettables);
    delete_statement.is_only = true;
    delete_statement
}

/// Define the API for delete Statement
#[derive(Debug, Clone)]
pub struct DeleteStatement<T: Model> {
    target: Sql,
    is_only: bool,
    where_: Option<Filter>,
    return_type: Option<ReturnType>,
    timeout: Option<DurationLike>,
    parallel: bool,
    model: PhantomData<T>,
}

impl<T: Model> DeleteStatement<T> {
    /// Adds a condition to the delete statement.
    ///
    /// Examples:
    ///
    /// ```rust, ignore
    /// // simple filtering on a field
    /// .where_(age.less_than(18));
    ///
    /// // Or more complex filtering with `cond` helper
    /// .where_(cond(age.greater_than(18))
    ///         .and(age.less_than(80)));
    /// ```
    pub fn where_(mut self, condition: impl Conditional + Clone) -> Self {
        self.where_ = Some(Filter::new(condition));
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

    /// Indicates that the query should be executed in parallel.
    pub fn parallel(mut self) -> Self {
        self.parallel = true;
        self
    }
}

impl<T: Model> Statement for DeleteStatement<T> {
    fn render_body(&self, sql: &mut Sql) {
        sql.push_str("DELETE ");
        if self.is_only {
            sql.push_str("ONLY ");
        }
        sql.push(&self.target);

        if let Some(condition) = &self.where_ {
            sql.push_str(" WHERE ").push(condition);
        }

        if let Some(return_type) = &self.return_type {
            sql.push_str(" ").push(return_type);
        }

        if let Some(timeout) = &self.timeout {
            sql.push_str(" TIMEOUT ").push_inlined(timeout);
        }

        if self.parallel {
            sql.push_str(" PARALLEL");
        }
    }
}

impl_statement!([T: Model] DeleteStatement<T>);

impl<T: Model> ReturnableDefault<T> for DeleteStatement<T> {}

impl<T> ReturnableStandard<T> for DeleteStatement<T>
where
    T: Model + Send + Sync,
{
    fn set_return_type(mut self, return_type: ReturnType) -> Self {
        self.return_type = Some(return_type);
        self
    }

    fn get_return_type(&self) -> ReturnType {
        self.return_type.clone().unwrap_or(ReturnType::None)
    }
}

/// Used in model helper methods for deleting records
#[derive(Debug, Clone)]
pub struct DeleteStatementMini<T: Model>(DeleteStatement<T>);

impl<T: Model> From<DeleteStatement<T>> for DeleteStatementMini<T> {
    fn from(statement: DeleteStatement<T>) -> Self {
        Self(statement)
    }
}

impl<T: Model> Statement for DeleteStatementMini<T> {
    fn render_body(&self, sql: &mut Sql) {
        self.0.render_body(sql);
    }
}

impl_statement!([T: Model] DeleteStatementMini<T>);

impl<T: Model> ReturnableDefault<T> for DeleteStatementMini<T> {}

impl<T> ReturnableStandard<T> for DeleteStatementMini<T>
where
    T: Model + Send + Sync,
{
    fn set_return_type(self, return_type: ReturnType) -> Self {
        Self(self.0.set_return_type(return_type))
    }

    fn get_return_type(&self) -> ReturnType {
        self.0.get_return_type()
    }
}

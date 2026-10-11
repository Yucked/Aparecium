use crate::{Clause, Conditional, Expression, Operatable, Param, SchemaGetter, Sql};

use super::{Statement, impl_statement};

/// Builds LET statement.
///
/// The LET statement sets and stores a value which can then be used in a subsequent query.
/// A parameter can store any value, including the result of a query.
///
/// Examples
/// ```rust
/// use aparecium::{*, statements::{let_, select}};
/// # let user = Table::new("user");
/// // You can assign a value surrealdb value
/// let_("name").equal_to(5);
///
/// // and even a select statement
/// let_("users").equal_to(select(All).from(user));
/// ```
pub fn let_(parameter: impl Into<Param>) -> LetStatement {
    LetStatement {
        parameter: parameter.into(),
        value: None,
    }
}

#[macro_export]
/// Macro for creating a LET statement
///
/// Examples
/// ```rust
/// use aparecium::{*, statements::{let_, select}};
/// # let ref user = Table::new("user");
/// // You can assign a value surrealdb value
/// let_!(name = 5);
/// // or
/// let_statement!(name = 5);
///
/// // and even a select statement
/// let_!(users = select(All).from(user));
/// // or
/// let_statement!(users = select(All).from(user));
/// ```
macro_rules! let_statement {
    ($param: ident =  $expr: expr) => {
        let $param = $crate::statements::let_(stringify!($param)).equal_to($expr);
    };
}

pub use let_statement as let_;

/// Let statement builder
#[derive(Debug, Clone)]
pub struct LetStatement {
    parameter: Param,
    value: Option<Expression>,
}

/// As an operand or condition, a `LET` statement stands for the parameter it defines.
impl Operatable for LetStatement {
    fn render_operand(&self, sql: &mut Sql) {
        sql.push(&self.parameter);
    }
}

impl Conditional for LetStatement {
    fn render_condition(&self, sql: &mut Sql) {
        sql.push(&self.parameter);
    }
}

impl LetStatement {
    /// Assigned expression
    /// Examples
    /// ```rust
    /// use aparecium::{*, statements::{let_, select}};
    /// # let user = Table::new("user");
    /// // You can assign a value surrealdb value
    /// # let_("name")
    /// .equal_to(5);
    ///
    /// // and even a select statement
    /// # let_("users")
    /// .equal_to(select(All).from(user));
    pub fn equal_to(mut self, value: impl Into<Expression>) -> Self {
        self.value = Some(value.into());
        self
    }

    /// helper function for getting assigned param from the LET statement
    pub fn get_param(&self) -> Param {
        self.parameter.clone()
    }

    /// For traversing from the param
    pub fn with_path<T: SchemaGetter>(&self, clause: impl Into<Clause>) -> T::Schema {
        self.parameter.with_path::<T>(clause)
    }
}

impl Statement for LetStatement {
    fn render_body(&self, sql: &mut Sql) {
        sql.push_str("LET ").push(&self.parameter);
        if let Some(value) = &self.value {
            sql.push_str(" = ").push(value);
        }
    }
}

impl_statement!(LetStatement);

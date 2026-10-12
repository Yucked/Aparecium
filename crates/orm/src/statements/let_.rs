use std::borrow::Cow;

use crate::{Clause, Conditional, Expression, Operatable, Param, Renderer, SchemaGetter};

use super::{Statement, impl_statement};

/// Builds LET statement.
///
/// The LET statement sets and stores a value which can then be used in a subsequent query.
/// A parameter can store any value, including the result of a query.
///
/// Examples
/// ```rust
/// use umbriel::{*, statements::{let_, select}};
/// # let user = Table::new("user");
/// // You can assign a value surrealdb value
/// let_("name").equal_to(5);
///
/// // and even a select statement
/// let_("users").equal_to(select(All).from(user));
/// ```
///
/// An invalid name (see [`Param::check`]) fails when the query is rendered.
pub fn let_(name: impl Into<Cow<'static, str>>) -> LetStatement {
    LetStatement {
        parameter: Param::new(name),
        value: None,
    }
}

#[macro_export]
/// Macro for creating a LET statement
///
/// Examples
/// ```rust
/// use umbriel::{*, statements::{let_, select}};
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
impl Operatable for LetStatement {}

impl Conditional for LetStatement {}

impl LetStatement {
    /// Assigned expression
    /// Examples
    /// ```rust
    /// use umbriel::{*, statements::{let_, select}};
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
    fn render_body(&self, out: &mut Renderer) {
        out.push_str("LET ").push(&self.parameter);
        if let Some(value) = &self.value {
            out.push_str(" = ").push(value);
        }
    }
}

impl_statement!(LetStatement);

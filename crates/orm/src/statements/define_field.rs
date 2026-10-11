use surrealdb::types::{Kind, Table};

use crate::{Conditional, Field, Filter, Sql, ValueLike};

use super::{
    Statement, impl_definition,
    for_permission::{Permissions, PermissionsClause},
};

// DEFINE FIELD statement
// The DEFINE FIELD statement allows you to instantiate a named field on a table, enabling you to set the field's data type, set a default value, apply assertions to protect data consistency, and set permissions specifying what operations can be performed on the field.
//
// Statement syntax
// DEFINE FIELD @name ON [ TABLE ] @table
// 	[ TYPE @type ]
// 	[ VALUE @expression ]
// 	[ ASSERT @expression ]
// 	[ PERMISSIONS [ NONE | FULL
// 		| FOR select @expression
// 		| FOR create @expression
// 		| FOR update @expression
// 		| FOR delete @expression
// 	] ]

/// A statement for defining a Field.
#[derive(Clone, Debug)]
pub struct DefineFieldStatement {
    field: Field,
    table: Option<Table>,
    type_: Option<Kind>,
    value: Option<ValueLike>,
    assert: Option<Filter>,
    permissions: PermissionsClause,
}

/// Define a new field.
/// The DEFINE FIELD statement allows you to instantiate a named field on a table, enabling you to set the field's data type, set a default value, apply assertions to protect data consistency, and set permissions specifying what operations can be performed on the field.
///
/// ```rust
/// use aparecium::{*, CrudType::*, statements::{define_field, for_permission}, types::Kind};
///
/// # let name = Field::new("name");
/// # let user_table = Table::from("user");
/// # let age = Field::new("age");
/// # let email = Field::new("email");
///
/// let statement = define_field(email)
///     .on_table(user_table)
///     .type_(Kind::String)
///     .value("alice@example.com")
///     .assert(cond(value().is_not(NONE)).and(value().like("is_email")))
///     // Additional permission chaining accumulates
///     .permissions(for_permission(Select).where_(age.greater_than_or_equal(18))) // Single works
///     .permissions(for_permission([Create, Update]).where_(name.is("Alice"))) // Multiple
///     // Multiples multples
///     .permissions([
///         for_permission([Create, Delete]).where_(name.is("Alice")),
///         for_permission(Update).where_(age.less_than_or_equal(130)),
///     ]);
///
/// assert!(!statement.to_debug_sql().is_empty());
/// ```
pub fn define_field(fieldable: impl Into<Field>) -> DefineFieldStatement {
    DefineFieldStatement {
        field: fieldable.into(),
        table: None,
        type_: None,
        value: None,
        assert: None,
        permissions: PermissionsClause::default(),
    }
}

impl DefineFieldStatement {
    /// Set the table where the field is defined.
    pub fn on_table(mut self, table: impl Into<Table>) -> Self {
        self.table = Some(table.into());
        self
    }

    /// Set the data type of the field.
    pub fn type_(mut self, field_type: impl Into<Kind>) -> Self {
        self.type_ = Some(field_type.into());
        self
    }

    /// Set the default value for the field.
    pub fn value(mut self, default_value: impl Into<ValueLike>) -> Self {
        self.value = Some(default_value.into());
        self
    }

    /// assert constraint on the field.
    ///
    ///  Examples:
    ///
    /// ```rust
    ///     use aparecium::{*, CrudType::*, statements::{define_field}, types::Kind};
    ///
    ///     # let user_table = Table::from("user");
    ///     # let email = Field::new("email");
    ///
    ///     # let statement = define_field(email)
    ///     #    .on_table(user_table)
    ///     #    .type_(Kind::String)
    ///     #    .value("alice@example.com");
    ///
    /// // For simple single condition
    /// let statement = statement.assert(value().is_not(NONE));
    ///
    /// // For multiple conditions
    /// let statement = statement.assert(cond(value().is_not(NONE))
    ///                                      .and(value().like("is_email"))
    ///                                 );
    /// ```
    pub fn assert(mut self, assertion: impl Conditional) -> Self {
        self.assert = Some(Filter::new(assertion));
        self
    }

    /// set no permission.
    pub fn permissions_none(mut self) -> Self {
        self.permissions.none();
        self
    }

    /// set full permission.
    pub fn permissions_full(mut self) -> Self {
        self.permissions.full();
        self
    }

    /// set specific permissions for specific event type inluding CREATE, UPDATE, SELECT and DELETE.
    /// Additional permission chaining accumulates
    pub fn permissions(mut self, fors: impl Into<Permissions>) -> Self {
        self.permissions.push(fors);
        self
    }
}

impl Statement for DefineFieldStatement {
    fn render_body(&self, sql: &mut Sql) {
        sql.push_str("DEFINE FIELD ").push(&self.field);

        if let Some(table) = &self.table {
            sql.push_str(" ON TABLE ").push(table);
        }

        if let Some(field_type) = &self.type_ {
            sql.push_str(" TYPE ").push(field_type);
        }

        if let Some(value) = &self.value {
            sql.push_str(" VALUE $value OR ").push(value);
        }

        if let Some(assertion) = &self.assert {
            sql.push_str(" ASSERT ").push(assertion);
        }

        sql.push(&self.permissions);
    }
}

impl_definition!(DefineFieldStatement);

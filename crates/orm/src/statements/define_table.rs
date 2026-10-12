use surrealdb::types::Table;

use crate::Renderer;

use super::{
    Statement,
    for_permission::{Permissions, PermissionsClause},
    impl_definition,
    select::SelectStatement,
};

// DEFINE TABLE statement
// The DEFINE TABLE statement allows you to declare your table by name, enabling you to apply strict controls to a table's schema by making it SCHEMAFULL, create a foreign table view, and set permissions specifying what operations can be performed on the field.
//
// Statement syntax
// DEFINE TABLE @name
// 	[ DROP ]
// 	[ SCHEMAFULL | SCHEMALESS ]
// 	[ AS SELECT @projections
// 		FROM @tables
// 		[ WHERE @condition ]
// 		[ GROUP [ BY ] @groups ]
// 	]
// 	[ PERMISSIONS [ NONE | FULL
// 		| FOR select @expression
// 		| FOR create @expression
// 		| FOR update @expression
// 		| FOR delete @expression
// 	] ]

/// Define the API for the Table builder
pub struct DefineTableStatement {
    table: Table,
    drop: bool,
    schema_type: Option<SchemaType>,
    as_: Option<SelectStatement>,
    permissions: PermissionsClause,
}

/// Define a new table.
///
/// The DEFINE TABLE statement allows you to declare your table by name,
/// enabling you to apply strict controls to a table's schema by making it SCHEMAFULL,
/// create a foreign table view, and set permissions specifying what operations can be performed on the field.
///
/// Examples:
///
/// ```rust
/// use umbriel::{*, statements::{define_table, for_permission, order, select}, types::RecordId};
/// use CrudType::*;
///
/// let name = Field::new("name");
/// let user = Table::from("user");
/// let age = Field::new("age");
/// let country = Field::new("country");
/// let fake_id = RecordId::new("user", "alice");
/// let statement = define_table(user)
///     .drop()
///     .as_(
///         select(All)
///             .from(fake_id)
///             .where_(country.is("INDONESIA"))
///             .order_by(order(&age).numeric().desc())
///             .limit(20)
///             .start(5),
///     )
///     .schemafull()
///     .permissions(for_permission(Select).where_(age.greater_than_or_equal(18))) // Single works
///     .permissions(for_permission(&[Create, Delete]).where_(name.is("Alice"))) //Multiple
///     .permissions(&[
///         for_permission(&[Create, Delete]).where_(name.is("Alice")),
///         for_permission(Update).where_(age.less_than_or_equal(130)),
///     ]);
///
/// assert!(!statement.to_debug_sql().is_empty());
/// ```
pub fn define_table(table: impl Into<Table>) -> DefineTableStatement {
    DefineTableStatement {
        table: table.into(),
        drop: false,
        schema_type: None,
        as_: None,
        permissions: PermissionsClause::default(),
    }
}

impl DefineTableStatement {
    /// Drop the table if it exists and create a new one with the same name.
    pub fn drop(mut self) -> Self {
        self.drop = true;
        self
    }

    /// Make table scehmafull
    pub fn schemafull(mut self) -> Self {
        self.schema_type = Some(SchemaType::Schemafull);
        self
    }

    /// Make table scehmaless.
    pub fn schemaless(mut self) -> Self {
        self.schema_type = Some(SchemaType::Schemaless);
        self
    }

    /// Defines the table as a view over a select statement.
    pub fn as_(mut self, select_statement: impl Into<SelectStatement>) -> Self {
        self.as_ = Some(select_statement.into());
        self
    }

    /// Set permission as NONE
    pub fn permissions_none(mut self) -> Self {
        self.permissions.none();
        self
    }

    /// Set permission as FULL
    pub fn permissions_full(mut self) -> Self {
        self.permissions.full();
        self
    }

    /// set specific permissions for the table with constraint for certain table access types.
    /// Events include type inluding CREATE, UPDATE, SELECT and DELETE.
    /// Additional permission chaining accumulates
    pub fn permissions(mut self, fors: impl Into<Permissions>) -> Self {
        self.permissions.push(fors);
        self
    }
}

impl Statement for DefineTableStatement {
    fn render_body(&self, out: &mut Renderer) {
        out.push_str("DEFINE TABLE ").push(&self.table);

        if self.drop {
            out.push_str(" DROP");
        }

        match self.schema_type {
            Some(SchemaType::Schemafull) => {
                out.push_str(" SCHEMAFULL");
            }
            Some(SchemaType::Schemaless) => {
                out.push_str(" SCHEMALESS");
            }
            None => {}
        }

        if let Some(select_statement) = &self.as_ {
            out.push_str(" AS \n\t");
            select_statement.render_body(out);
        }

        out.push(&self.permissions);
    }
}

impl_definition!(DefineTableStatement);

#[derive(Clone, Copy)]
enum SchemaType {
    Schemafull,
    Schemaless,
}

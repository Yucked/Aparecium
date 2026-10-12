use std::borrow::Cow;

use crate::{Ident, Render, Renderer, StrandLike, statements::Permissions};

use super::{Statement, for_permission::PermissionsClause, impl_definition};

/// DEFINE MODEL statement
/// The DEFINE MODEL statement allows you to define a machine learning model in the database, specifying its name, version, and permissions.
///
/// Statement syntax
/// DEFINE MODEL ml::@name<@version>
/// [ PERMISSIONS @permissions ]
/// A statement for defining a ML model.
#[derive(Clone, Debug)]
pub struct DefineModelStatement {
    model_name: ModelName,
    version: Option<ModelVersion>,
    comment: Option<StrandLike>,
    permissions: PermissionsClause,
}

/// A model name.
pub type ModelName = Ident;

/// A model version, e.g. `1.2.3`, written verbatim inside `<…>`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ModelVersion(Cow<'static, str>);

impl<T: Into<Cow<'static, str>>> From<T> for ModelVersion {
    fn from(value: T) -> Self {
        Self(value.into())
    }
}

impl Render for ModelVersion {
    fn render(&self, out: &mut Renderer) {
        out.push_str(&self.0);
    }
}

/// Define a new ML model.
/// The DEFINE MODEL statement allows you to define a machine learning model in the database.
///
/// ```rust
/// use umbriel::{*, statements::{define_model, for_permission}};
/// use CrudType::*;
///
/// # let name = Field::new("name");
/// # let age = Field::new("age");
///
/// let statement = define_model("recommendation")
///     .version("1.2.3")
///     // Additional permission chaining accumulates
///     .permissions(for_permission(Select).where_(age.greater_than_or_equal(18))) // Single works
///     .permissions(for_permission([Create, Update]).where_(name.is("Alice"))) // Multiple
///     // Multiples multples
///     .permissions([
///         for_permission([Create, Delete]).where_(name.is("Alice")),
///         for_permission(Update).where_(age.less_than_or_equal(130)),
///     ]);
///
/// assert!(statement.to_debug_sql().starts_with("DEFINE MODEL ml::recommendation<1.2.3>"));
/// ```
pub fn define_model(name: impl Into<ModelName>) -> DefineModelStatement {
    DefineModelStatement {
        model_name: name.into(),
        version: None,
        comment: None,
        permissions: PermissionsClause::default(),
    }
}

impl DefineModelStatement {
    /// Set the version of the model.
    pub fn version(mut self, version: impl Into<ModelVersion>) -> Self {
        self.version = Some(version.into());
        self
    }

    /// Set a comment for the model.
    pub fn comment(mut self, comment: impl Into<StrandLike>) -> Self {
        self.comment = Some(comment.into());
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

impl Statement for DefineModelStatement {
    fn render_body(&self, out: &mut Renderer) {
        out.push_str("DEFINE MODEL ml::").push(&self.model_name);

        if let Some(version) = &self.version {
            out.push_str("<").push(version).push_str(">");
        }

        if let Some(comment) = &self.comment {
            out.push_str("\nCOMMENT ").push(comment);
        }

        out.push(&self.permissions);
    }
}

impl_definition!(DefineModelStatement);

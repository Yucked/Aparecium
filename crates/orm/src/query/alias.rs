
use std::{borrow::Cow, fmt::Display};

use crate::{Ident, Render, Sql};

/// An alias for a table or a field/column or even a statement. You usually do not instantiate this
/// yourself.
#[derive(Debug, Clone, PartialEq)]
pub struct Alias {
    name: AliasName,
    sql: Sql,
}

impl Render for Alias {
    fn render(&self, sql: &mut Sql) {
        sql.push(&self.sql);
    }
}

impl Alias {
    /// Get the alias name.
    pub fn get_alias_name(self) -> AliasName {
        self.name
    }
}

/// The name of an alias.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AliasName(Ident);

impl From<&str> for AliasName {
    fn from(value: &str) -> Self {
        Self(value.into())
    }
}

impl From<String> for AliasName {
    fn from(value: String) -> Self {
        Self(value.into())
    }
}

impl AliasName {
    /// Create a new alias name.
    pub fn new(name: impl Into<Cow<'static, str>>) -> Self {
        Self(Ident::new(name))
    }

    /// The unescaped name.
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl Render for AliasName {
    fn render(&self, sql: &mut Sql) {
        self.0.render(sql);
    }
}

impl Display for AliasName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

/// A trait for aliasing a statement or a field/column.
pub trait Aliasable: Render {
    /// Alias the current statement or field/column.
    ///
    /// # Arguments
    /// * `alias` - The alias name for the field/column or statement.
    ///
    /// # Example
    ///
    /// ```rust
    /// use aparecium::{*, statements::*};
    /// # let name =  Field::new("name");
    /// let moniker =  AliasName::new("moniker");
    /// assert_eq!(name.__as__(moniker).to_debug_sql(), "name AS moniker");
    ///
    /// # let name =  Field::new("name");
    /// assert_eq!(name.__as__("moniker").to_debug_sql(), "name AS moniker");
    ///
    /// # let users =  Table::new("users");
    /// # let selected_users =  AliasName::new("selected_users");
    /// let select = select(All).from(users).__as__(selected_users);
    /// assert_eq!(
    ///    select.to_debug_sql(),
    ///    "(SELECT * FROM users) AS selected_users"
    ///    );
    /// ```
    fn __as__(&self, alias: impl Into<AliasName>) -> Alias {
        let name: AliasName = alias.into();
        let mut sql = Sql::new();
        self.render_aliasable(&mut sql);
        sql.push_str(" AS ").push(&name);
        Alias { name, sql }
    }

    /// Renders the aliased node. Statements override this to render as a subquery.
    fn render_aliasable(&self, sql: &mut Sql) {
        self.render(sql);
    }
}

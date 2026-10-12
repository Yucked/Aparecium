
use surrealdb::types::SurrealValue;

use crate::{NumberLike, Operator, Render, Renderer, Setter, ValueLike};

use super::Field;

/// A helper struct for generating SQL update statements.
#[derive(Debug, Clone, PartialEq)]
pub struct Updater {
    field: Field,
    /// The operator and value, once one is chosen; the bare field renders until then.
    update: Option<(Operator, ValueLike)>,
}

/// A helper struct for generating setters used in various statements
pub fn updater(field: impl Into<Field>) -> Updater {
    Updater {
        field: field.into(),
        update: None,
    }
}

impl Render for Updater {
    fn render(&self, out: &mut Renderer) {
        out.push(&self.field);
        if let Some((operator, value)) = &self.update {
            out.push_str(" ")
                .push(operator)
                .push_str(" ")
                .push(value);
        }
    }
}

/// Things that can be updated
pub enum Updateables {
    /// Single updater
    Updater(Updater),
    /// Multiple updaters
    Updaters(Vec<Updater>),
}

impl From<Updater> for Updateables {
    fn from(value: Updater) -> Self {
        Self::Updater(value)
    }
}

impl From<Vec<Updater>> for Updateables {
    fn from(value: Vec<Updater>) -> Self {
        Self::Updaters(value)
    }
}

impl From<Setter> for Updateables {
    fn from(value: Setter) -> Self {
        let (field, operator, value) = value.into_parts();
        Self::Updater(Updater {
            field,
            update: Some((operator, value)),
        })
    }
}

impl Render for Updateables {
    fn render(&self, out: &mut Renderer) {
        match self {
            Updateables::Updater(updater) => updater.render(out),
            Updateables::Updaters(updaters) => updaters.render(out),
        }
    }
}

impl Updater {
    /// Sets a field name
    ///
    /// # Arguments
    ///
    /// * `value` - The value to set.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use umbriel::*;
    /// # let score = Field::new("score");
    /// let updated_updater = updater(score).equal(2);
    /// assert_eq!(updated_updater.to_debug_sql(), "score = 2");
    /// # assert_eq!(updated_updater.to_query().unwrap().sql(), "score = $_p0");
    /// ```
    pub fn equal(&self, value: impl SurrealValue) -> Self {
        self.update_field(Operator::Equal, ValueLike::value(value))
    }

    /// Returns a new `Updater` instance with the string to increment the column by the given value.
    /// Alias for plus_equal but idiomatically for numbers
    ///
    /// # Arguments
    ///
    /// * `value` - The value to increment the column by.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use umbriel::*;
    /// # let score = Field::new("score");
    /// let updated_updater = updater(score).increment_by(2);
    /// assert_eq!(updated_updater.to_debug_sql(), "score += 2");
    ///
    /// # assert_eq!(updated_updater.to_query().unwrap().sql(), "score += $_p0");
    /// ```
    pub fn increment_by(&self, value: impl Into<NumberLike>) -> Self {
        self.update_field(Operator::Inc, value.into())
    }

    /// Returns a new `Updater` instance with the string to append the given value to a column that stores an array.
    /// Alias for plus_equal but idiomatically for an array
    ///
    /// # Arguments
    ///
    /// * `value` - The value to append to the column's array.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use umbriel::*;
    /// # let tags = Field::new("tags");
    /// let updated_updater = updater(tags).append("rust");
    /// assert_eq!(updated_updater.to_debug_sql(), "tags += 'rust'");
    /// # assert_eq!(updated_updater.to_query().unwrap().sql(), "tags += $_p0");
    /// ```
    pub fn append(&self, value: impl SurrealValue) -> Self {
        self.update_field(Operator::Inc, ValueLike::value(value))
    }

    /// Returns a new `Updater` instance with the string to decrement the column by the given value.
    /// Alias for minus_equal but idiomatically for an number
    ///
    /// # Arguments
    ///
    /// * `value` - The value to decrement the column by.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use umbriel::*;
    /// # let score = Field::new("score");
    /// let updated_updater = updater(score).decrement_by(2);
    /// assert_eq!(updated_updater.to_debug_sql(), "score -= 2");
    /// # assert_eq!(updated_updater.to_query().unwrap().sql(), "score -= $_p0");
    /// ```
    pub fn decrement_by(&self, value: impl Into<NumberLike>) -> Self {
        self.update_field(Operator::Dec, value.into())
    }

    /// Returns a new `Updater` instance with the string to remove the given value from a column that stores an array.
    /// Alias for minus_equal but idiomatically for an array
    ///
    /// # Arguments
    ///
    /// * `value` - The value to remove from the column's array.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use umbriel::*;
    /// # let tags = Field::new("tags");
    /// let updated_updater = updater(tags).remove("rust");
    /// assert_eq!(updated_updater.to_debug_sql(), "tags -= 'rust'");
    /// # assert_eq!(updated_updater.to_query().unwrap().sql(), "tags -= $_p0");
    /// ```
    pub fn remove(&self, value: impl SurrealValue) -> Self {
        self.update_field(Operator::Dec, ValueLike::value(value))
    }

    /// Returns a new `Updater` instance with the string to add the given value to the column.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to add to the field or push to the field if it is an array.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use umbriel::*;
    /// # let tags = Field::new("tags");
    /// let updated_updater = updater(tags).plus_equal("rust");
    /// assert_eq!(updated_updater.to_debug_sql(), "tags += 'rust'");
    /// # assert_eq!(updated_updater.to_query().unwrap().sql(), "tags += $_p0");
    /// ```
    pub fn plus_equal(&self, value: impl SurrealValue) -> Self {
        self.update_field(Operator::Inc, ValueLike::value(value))
    }

    /// Returns a new `Updater` instance with the string to remove the given value from the column.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to subtract from the field or remove from the field if it is an array.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use umbriel::*;
    /// # let tags = Field::new("tags");
    /// let updated_updater = updater(tags).minus_equal("rust");
    /// assert_eq!(updated_updater.to_debug_sql(), "tags -= 'rust'");
    /// # assert_eq!(updated_updater.to_query().unwrap().sql(), "tags -= $_p0");
    /// ```
    pub fn minus_equal(&self, value: impl SurrealValue) -> Self {
        self.update_field(Operator::Dec, ValueLike::value(value))
    }

    fn update_field(&self, operator: Operator, value: impl Into<ValueLike>) -> Updater {
        Self {
            field: self.field.clone(),
            update: Some((operator, value.into())),
        }
    }
}

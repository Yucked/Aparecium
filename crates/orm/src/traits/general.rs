
use crate::{Field, Render, Sql};

/// Denoted by `.*`. Used for accessing all nested fields and arrays and links
pub trait AllGetter {
    /// Appends `.*` to the current path. Get all nested fields and arrays and links
    fn all(&self) -> Field;
}

impl<B: Render> AllGetter for B {
    fn all(&self) -> Field {
        Field::from_expr(self).all()
    }
}

/// Used for statements
pub trait Queryable: Render {}

impl<T: Queryable + ?Sized> Queryable for &T {}

/// Used for filters
pub trait Conditional: Render {
    /// Writes the node as a condition. A `LET` statement stands for its parameter here.
    fn render_condition(&self, sql: &mut Sql) {
        self.render(sql);
    }

    /// Whether the condition is already a single parenthesized group.
    fn is_grouped(&self) -> bool {
        false
    }
}

impl<T: Conditional + ?Sized> Conditional for &T {
    fn render_condition(&self, sql: &mut Sql) {
        (**self).render_condition(sql);
    }

    fn is_grouped(&self) -> bool {
        (**self).is_grouped()
    }
}

/// Used for marking a struct used in UPDATE MERGE statement.
pub trait DataUpdater {}

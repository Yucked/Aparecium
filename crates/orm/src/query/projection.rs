
use crate::{Alias, Field, Function, Param, Render, Sql, ValueLike};

/// Used to represent a list of projections to access fields of a table or
/// those of foreign tables and can even include filters.
#[derive(Debug, Clone, PartialEq)]
pub struct Projections(pub Vec<ValueLike>);

impl std::ops::Deref for Projections {
    type Target = Vec<ValueLike>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Render for Projections {
    fn render(&self, sql: &mut Sql) {
        self.0.render(sql);
    }
}

impl From<Vec<ValueLike>> for Projections {
    fn from(value: Vec<ValueLike>) -> Self {
        Self(value)
    }
}

impl From<Vec<&ValueLike>> for Projections {
    fn from(value: Vec<&ValueLike>) -> Self {
        Self(value.into_iter().cloned().collect())
    }
}

impl From<&[ValueLike]> for Projections {
    fn from(value: &[ValueLike]) -> Self {
        Self(value.to_vec())
    }
}

/// Implements `From<$single>`, `From<Vec<$single>>` and `From<&[$single]>` for `Projections`.
macro_rules! projections_from {
    ($($single:ty),*) => {
        $(
            impl From<$single> for Projections {
                fn from(value: $single) -> Self {
                    Self(vec![value.into()])
                }
            }

            impl From<Vec<$single>> for Projections {
                fn from(value: Vec<$single>) -> Self {
                    Self(value.into_iter().map(Into::into).collect())
                }
            }

            impl From<&[$single]> for Projections {
                fn from(value: &[$single]) -> Self {
                    Self(value.iter().cloned().map(Into::into).collect())
                }
            }
        )*
    };
}

projections_from!(Field, Alias, Function, Param);

impl From<&Field> for Projections {
    fn from(value: &Field) -> Self {
        Self(vec![value.into()])
    }
}

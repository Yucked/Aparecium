
use crate::{Projections, Render, Sql, ValueLike};

/// Return type
#[derive(Debug, Clone, PartialEq)]
pub enum ReturnType {
    /// Return nothing
    None,
    /// Return previous state
    Before,
    /// Return current state after change. This is the default
    After,
    /// Return the diff
    Diff,
    /// Return the listed fields/projection
    Projections(Projections),
}

impl From<Vec<ValueLike>> for ReturnType {
    fn from(value_like: Vec<ValueLike>) -> Self {
        ReturnType::Projections(Projections(value_like))
    }
}

impl Render for ReturnType {
    fn render(&self, sql: &mut Sql) {
        sql.push_str("RETURN ");
        match self {
            ReturnType::None => {
                sql.push_str("NONE");
            }
            ReturnType::Before => {
                sql.push_str("BEFORE");
            }
            ReturnType::After => {
                sql.push_str("AFTER");
            }
            ReturnType::Diff => {
                sql.push_str("DIFF");
            }
            ReturnType::Projections(projections) => {
                sql.push(projections);
            }
        }
    }
}

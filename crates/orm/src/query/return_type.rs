
use crate::{Projections, Render, Renderer, ValueLike};

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
    fn render(&self, out: &mut Renderer) {
        out.push_str("RETURN ");
        match self {
            ReturnType::None => {
                out.push_str("NONE");
            }
            ReturnType::Before => {
                out.push_str("BEFORE");
            }
            ReturnType::After => {
                out.push_str("AFTER");
            }
            ReturnType::Diff => {
                out.push_str("DIFF");
            }
            ReturnType::Projections(projections) => {
                out.push(projections);
            }
        }
    }
}

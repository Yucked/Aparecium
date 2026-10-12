use std::borrow::Cow;

use crate::{Param, Renderer};

use super::{Statement, impl_definition};

/// Creates a REMOVE PARAM statement builder. An invalid name (see [`Param::check`]) fails when
/// the query is rendered.
///
/// Examples
/// ```rust
/// use umbriel::{*, statements::remove_param};
/// assert_eq!(remove_param("website_name").to_debug_sql(), "REMOVE PARAM $website_name;");
/// ```
pub fn remove_param(name: impl Into<Cow<'static, str>>) -> RemoveParamStatement {
    RemoveParamStatement {
        name: Param::new(name),
    }
}

/// Represents the REMOVE PARAM statement.
pub struct RemoveParamStatement {
    name: Param,
}

impl Statement for RemoveParamStatement {
    fn render_body(&self, out: &mut Renderer) {
        out.push_str("REMOVE PARAM ").push(&self.name);
    }
}

impl_definition!(RemoveParamStatement);

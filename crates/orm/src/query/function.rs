
use std::borrow::Cow;

use crate::{Aliasable, Render, Sql};

/// A function call, e.g. `math::sum(age)`.
#[derive(Debug, Clone, PartialEq)]
pub struct Function {
    name: Cow<'static, str>,
    args: Vec<Sql>,
    /// The JavaScript body of an embedded `function(…) { … }`, written verbatim.
    body: Option<String>,
}

impl Function {
    /// A call to the function `name`, e.g. `math::sum`, with no arguments yet.
    pub(crate) fn new(name: impl Into<Cow<'static, str>>) -> Self {
        Self {
            name: name.into(),
            args: vec![],
            body: None,
        }
    }

    /// Appends an argument.
    pub(crate) fn arg(mut self, arg: impl Render) -> Self {
        self.args.push(arg.to_sql());
        self
    }

    /// Sets the body of an embedded JavaScript function.
    pub(crate) fn with_body(mut self, body: String) -> Self {
        self.body = Some(body);
        self
    }

    /// A call to the custom function `fn::<name>`; used by the helpers `define_function!`
    /// generates.
    #[doc(hidden)]
    pub fn __custom(name: &str, args: Vec<Sql>) -> Self {
        Self {
            name: Cow::Owned(format!("fn::{name}")),
            args,
            body: None,
        }
    }
}

impl Render for Function {
    fn render(&self, sql: &mut Sql) {
        sql.push_str(&self.name)
            .push_str("(")
            .push_joined(&self.args, ", ")
            .push_str(")");
        if let Some(body) = &self.body {
            sql.push_str(" ").push_str(body);
        }
    }
}

impl Aliasable for Function {}

use surrealdb::types::Kind;

use crate::{Block, Param, Sql};

use super::{Statement, impl_definition};

/// Represents a surrealdb define function statement argument
#[derive(Debug, Clone)]
pub struct FunctionArgument {
    /// The name of the argument
    pub name: Param,
    /// The type of the argument
    pub type_: Kind,
}

/// A function definition statement
#[derive(Debug, Clone)]
pub struct DefineFunctionStatement {
    name: String,
    args: Vec<FunctionArgument>,
    body: Option<Block>,
}

impl DefineFunctionStatement {
    /// Create a new function definition statement
    ///
    /// # Arguments
    /// * `name` - The name of the function, without the `fn::` prefix
    pub fn new(name: String) -> Self {
        define_function(name)
    }

    /// Sets the arguments for the function
    pub fn arguments(mut self, args: Vec<FunctionArgument>) -> Self {
        self.args = args;
        self
    }

    /// Sets the body of the function
    pub fn body(mut self, body: impl Into<Block>) -> Self {
        self.body = Some(body.into());
        self
    }
}

/// Create a new function definition statement
pub fn define_function(name: impl Into<String>) -> DefineFunctionStatement {
    DefineFunctionStatement {
        name: name.into(),
        args: vec![],
        body: None,
    }
}

impl Statement for DefineFunctionStatement {
    fn render_body(&self, sql: &mut Sql) {
        // The name is a `::`-separated path, written as given.
        sql.push_str("DEFINE FUNCTION fn::")
            .push_str(&self.name)
            .push_str("(");
        for (index, FunctionArgument { name, type_ }) in self.args.iter().enumerate() {
            if index > 0 {
                sql.push_str(", ");
            }
            sql.push(name).push_str(": ").push(type_);
        }
        sql.push_str(") ");
        if let Some(body) = &self.body {
            sql.push(body);
        }
    }
}

impl_definition!(DefineFunctionStatement);

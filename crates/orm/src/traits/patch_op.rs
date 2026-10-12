use std::ops::Deref;

use surrealdb::types::{SurrealValue, Value};

use crate::{Field, Render, Renderer};

const INVALID_PATH: &str = "The path you have provided is invalid. Make sure that there are no clauses or conditions included. Valid path include e.g name, name(E).first, name(E).first(E).second, etc.";

/// A JSON patch operation, e.g. `{ op: 'add', path: '/name', value: 'Alice' }`.
#[derive(Debug, Clone, PartialEq)]
pub struct PatchOp {
    op: &'static str,
    /// The JSON pointer to the field, or `None` when the field path has no pointer.
    path: Option<Value>,
    value: Option<Value>,
}

impl PatchOp {
    fn new(path: &Field, op: &'static str, value: Option<Value>) -> Self {
        Self {
            op,
            path: path.json_pointer().map(Value::String),
            value,
        }
    }
}

impl Render for PatchOp {
    fn render(&self, out: &mut Renderer) {
        out.push_str("{ op: '")
            .push_str(self.op)
            .push_str("', path: ");
        match &self.path {
            Some(pointer) => out.push_value(pointer),
            None => out.push_error(INVALID_PATH).push_value(&Value::None),
        };
        if let Some(value) = &self.value {
            out.push_str(", value: ").push_value(value);
        }
        out.push_str(" }");
    }
}

impl From<PatchOp> for Vec<PatchOp> {
    fn from(patch_op: PatchOp) -> Self {
        vec![patch_op]
    }
}

/// Patchable trait
pub trait Patchable<T: SurrealValue>
where
    Self: Deref<Target = Field>,
{
    /// Json Add patch operation
    /// The field acts as the path to the field to be patched. It automatically
    /// converts a field path to a valid json path to the field.
    /// So, `name` will become `/name` and `name.first` will become `/name/first`
    ///
    /// # Arguments
    ///
    /// value: The value to be patched
    ///
    /// # Examples
    /// ```rust, ignore
    /// name.patch_add("Charlie");
    /// ```
    fn patch_add(&self, value: impl Into<T>) -> PatchOp {
        PatchOp::new(self.deref(), "add", Some(value.into().into_value()))
    }

    /// Json Remove patch operation
    /// The field acts as the path to the field to be patched. It automatically
    /// converts a field path to a valid json path to the field.
    /// So, `name` will become `/name` and `name.first` will become `/name/first`
    ///
    /// # Examples
    /// ```rust, ignore
    /// name.patch_remove();
    /// ```
    fn patch_remove(&self) -> PatchOp {
        PatchOp::new(self.deref(), "remove", None)
    }

    /// Json Replace patch operation
    /// The field acts as the path to the field to be patched. It automatically
    /// converts a field path to a valid json path to the field.
    /// So, `name` will become `/name` and `name.first` will become `/name/first`
    ///
    /// # Arguments
    /// value: The value to be patched
    ///
    /// # Examples
    /// ```rust, ignore
    /// name.patch_replace("Charlie");
    /// ```
    fn patch_replace(&self, value: impl Into<T>) -> PatchOp {
        PatchOp::new(self.deref(), "replace", Some(value.into().into_value()))
    }

    /// Json Change patch operation
    /// The field acts as the path to the field to be patched. It automatically
    /// converts a field path to a valid json path to the field.
    /// So, `name` will become `/name` and `name.first` will become `/name/first`
    ///
    /// # Arguments
    /// regex_match_and_replace_string: A string that matches a regex pattern and replaces it with the string
    ///
    /// # Examples
    /// ```rust, ignore
    /// // The below changes "test" to "text
    /// name.patch_change("@@ -1,4 +1,4 @@\n te\n-s\n+x\n t\n");
    /// ```
    fn patch_change(&self, regex_match_and_replace_string: &str) -> PatchOp {
        PatchOp::new(
            self.deref(),
            "change",
            Some(Value::String(regex_match_and_replace_string.to_owned())),
        )
    }

    /// Derefs to field
    fn to_field(&self) -> Field {
        self.deref().clone()
    }
}

use crate::Sql;

use super::{
    Statement,
    define_model::{ModelName, ModelVersion},
    impl_definition,
};

/// Creates a REMOVE MODEL statement builder.
///
/// Examples
/// ```rust
/// use aparecium::{*, statements::remove_model};
///
/// assert_eq!(
///     remove_model("recommendation").version("1.0.0").to_debug_sql(),
///     "REMOVE MODEL ml::recommendation<1.0.0>;"
/// );
/// ```
pub fn remove_model(name: impl Into<ModelName>) -> RemoveModelStatement {
    RemoveModelStatement {
        name: name.into(),
        version: None,
    }
}

/// Represents the REMOVE MODEL statement.
pub struct RemoveModelStatement {
    name: ModelName,
    version: Option<ModelVersion>,
}

impl RemoveModelStatement {
    /// Adds a version to the REMOVE MODEL statement.
    pub fn version(mut self, version: impl Into<ModelVersion>) -> Self {
        self.version = Some(version.into());
        self
    }
}

impl Statement for RemoveModelStatement {
    fn render_body(&self, sql: &mut Sql) {
        sql.push_str("REMOVE MODEL ml::").push(&self.name);
        if let Some(version) = &self.version {
            sql.push_str("<").push(version).push_str(">");
        }
    }
}

impl_definition!(RemoveModelStatement);

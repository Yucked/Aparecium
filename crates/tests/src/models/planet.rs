use chrono::{DateTime, Utc};
use umbriel::{RecordId, model, types::SurrealValue, validators};

/// A generic node: `data`'s type is the caller's, stored as an integer.
#[model(node, table = "planet")]
#[surreal(rename_all = "camelCase")]
#[derive(Debug, Clone, Default)]
pub struct Planet<T: SurrealValue + validators::Int + 'static> {
    pub id: RecordId<Self>,
    pub name: String,
    pub population: u64,
    pub created: DateTime<Utc>,
    pub tags: Vec<u64>,
    #[orm(ty = "int")]
    pub data: T,
}

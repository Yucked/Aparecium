use umbriel::{RecordId, model};

// `ty` must be a SurrealQL type.
#[model(node, table = "user")]
pub struct User {
    pub id: RecordId<Self>,
    #[orm(ty = "array<")]
    pub tags: Vec<String>,
}

fn main() {}

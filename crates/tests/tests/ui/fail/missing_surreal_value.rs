use umbriel::{RecordId, model};

// With `value = manual`, the model must implement `SurrealValue` itself.
#[model(node, table = "user", value = manual)]
pub struct User {
    pub id: RecordId<Self>,
    pub name: String,
}

fn main() {}

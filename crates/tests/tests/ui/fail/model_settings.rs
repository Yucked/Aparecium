use umbriel::{RecordId, model};

// A node needs a table.
#[model(node)]
pub struct User {
    pub id: RecordId<Self>,
}

// An embedded object has no table.
#[model(object, table = "address")]
pub struct Address {
    pub city: String,
}

// A model is exactly one kind.
#[model(node, edge, table = "team")]
pub struct Team {
    pub id: RecordId<Self>,
}

fn main() {}

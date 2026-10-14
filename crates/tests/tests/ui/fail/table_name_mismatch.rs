use umbriel::{RecordId, model};

// The table must be the struct name in snake case unless `relax_table` is set.
#[model(node, table = "person")]
pub struct User {
    pub id: RecordId<Self>,
}

fn main() {}

use umbriel::{RecordId, model};

// Two fields stored under one name would overwrite each other.
#[model(node, table = "user")]
pub struct User {
    pub id: RecordId<Self>,
    pub name: String,
    #[surreal(rename = "name")]
    pub nickname: String,
}

fn main() {}

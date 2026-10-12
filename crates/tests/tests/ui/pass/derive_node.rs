use umbriel::{Node, SurrealSimpleId};
use serde::{Deserialize, Serialize};

#[derive(Node, Serialize, Deserialize, Debug, Clone)]
#[orm(table = "user")]
pub struct User {
    id: SurrealSimpleId<Self>,
    name: String,
}

fn main() {}

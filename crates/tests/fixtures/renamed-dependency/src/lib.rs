//! Derive output must resolve `apa::…` paths, not `aparecium::…`.

use apa::serde::{Deserialize, Serialize};
use apa::{Node, SurrealSimpleId};

#[derive(Node, Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "apa::serde")]
#[orm(table = "user")]
pub struct User {
    id: SurrealSimpleId<Self>,
    name: String,
}

pub fn query() -> &'static str {
    apa::query_raw!("SELECT * FROM user")
}

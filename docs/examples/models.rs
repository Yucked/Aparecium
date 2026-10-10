//! Define nodes, an edge and an embedded object.
//!
//! Draft for milestone 2 (see `docs/api.md`); not compiled yet.

use aparecium::{Edge, Node, Object, RecordId};

#[derive(Node, Debug, Clone)]
#[orm(table = "user")]
pub struct User {
    pub id: RecordId<User>,
    pub name: String,
    pub email: String,
    /// Absent (`NONE`) when `None`.
    pub nickname: Option<String>,
    pub address: Address,
}

#[derive(Object, Debug, Clone)]
pub struct Address {
    pub city: String,
    pub country: String,
}

#[derive(Node, Debug, Clone)]
#[orm(table = "company")]
pub struct Company {
    pub id: RecordId<Company>,
    pub name: String,
}

/// `user -> works_at -> company`
#[derive(Edge, Debug, Clone)]
#[orm(table = "works_at")]
pub struct WorksAt {
    pub id: RecordId<WorksAt>,
    pub role: String,
}

fn main() {
    let alice = User {
        id: User::id("alice"),
        name: "Alice".into(),
        email: "alice@example.com".into(),
        nickname: None,
        address: Address {
            city: "Springfield".into(),
            country: "Exampleland".into(),
        },
    };
    let acme = Company {
        id: Company::id("acme"),
        name: "Acme Corp".into(),
    };
    println!("{alice:?} {acme:?}");
}

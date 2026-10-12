use umbriel::{Node, SurrealSimpleId, statements::DefineFieldStatement};
use serde::{Deserialize, Serialize};

fn define_age() -> DefineFieldStatement {
    unimplemented!()
}

// `define` and `define_fn` both define the field; using both must be rejected at the attribute.
#[derive(Node, Serialize, Deserialize, Debug, Clone)]
#[orm(table = "user")]
pub struct User {
    id: SurrealSimpleId<Self>,
    #[orm(define = define_age(), define_fn = define_age)]
    age: u8,
}

fn main() {}

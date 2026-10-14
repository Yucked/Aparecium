use umbriel::{RecordId, model, statements::DefineFieldStatement};

fn define_age() -> DefineFieldStatement {
    unimplemented!()
}

// `define` gives the whole definition, so it cannot be combined with its clauses.
#[model(node, table = "user")]
pub struct User {
    pub id: RecordId<Self>,
    #[orm(ty = "int", define = define_age(), assert = umbriel::value().is_not(umbriel::NONE))]
    pub age: u8,
}

fn main() {}

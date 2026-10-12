// mod edge_field_attributes;
// mod edge_table_attributes;
// mod node_field_attributes;
// mod node_table_attributes;
//
// pub use crate::{AlienVisitsPlanet, Planet, SpaceShip, Visits, Weapon};
pub use umbriel::{
    serde::{Deserialize, Serialize},
    statements::{DefineFieldStatement, DefineTableStatement, Permissions, SelectStatement},
    *,
};

pub fn define_field_fn() -> DefineFieldStatement {
    unimplemented!()
}

pub fn define_table_fn() -> DefineTableStatement {
    unimplemented!()
}

pub fn permissions_fn() -> Permissions {
    unimplemented!()
}

pub fn permissions_fn2(_name: &'static str) -> Permissions {
    unimplemented!()
}

pub fn assert_fn() -> Filter {
    unimplemented!()
}

pub fn value_fn() -> u8 {
    unimplemented!()
}

pub fn as_fn() -> SelectStatement {
    unimplemented!()
}

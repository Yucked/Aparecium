
// pub mod aparecium {
//     pub use surreal_derive::*;
//     pub use surreal_query_builder::*;
// }

/// Items shared by the `macros_attribute_combinations` compile-pass checks.
pub mod compile_support;
pub mod migrations;
/// Helpers shared by the `orm_migrations_*` tests.
pub mod migrator_support;
mod models;
// pub use migrations::*;
pub use models::alien::*;
pub use models::attributes::*;
pub use models::configuration::*;
pub use models::planet::*;
pub use models::spaceship::*;
pub use models::user::*;
pub use models::visits::*;
pub use models::weapon::*;
pub use models::with_generics::*;
// pub use models::*;

// use aparecium::{
//     serde::{Deserialize, Serialize},
//     Node, SurrealId,
// };

// #[derive(Node, Serialize, Deserialize, Debug, Clone)]
// #[serde(rename_all = "camelCase")]
// #[orm(table = balance)]
// pub struct Balance {
//     pub id: SurrealId<Self, String>,
//     #[orm(ty = "string")]
//     pub amount: &'static str,
//     // pub amount: Option<f64>,
// }

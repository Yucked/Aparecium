/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 */
// Shared lints live in the workspace manifest; only the public crate requires docs.
#![warn(missing_docs)]
#![recursion_limit = "256"]
#![cfg_attr(docsrs, feature(doc_cfg))]

//! This library includes augmented surrealdb types, custom types, statements, functions, operators, castings, and other utilities to
//! to make working with surrealdb a joy.

mod errors;

/// Contains math constants, all the casting functions and future.
mod data_model;
pub mod functions;
mod helpers;
mod operators_macros;
/// This module contains the different types of statements that can be used to query the
/// database.
pub mod statements;
mod traits;
mod types;
/// For compile time validations
pub mod validators;

pub use data_model::*;
pub use errors::*;
pub use helpers::*;
pub use statements::select::CanOrder;
pub use statements::utils::*;
pub use traits::*;
pub use types::*;

pub use serde;
pub use surrealdb::sql;

/// Embedded and file-based schema migrations, generated from model definitions.
#[cfg(feature = "migrations")]
pub mod migrations;

#[doc = include_str!("docs/edge_description.md")]
#[doc = include_str!("docs/edge_struct_attributes.md")]
#[doc = include_str!("docs/edge_field_attributes.md")]
pub use aparecium_macros::Edge;

#[doc = include_str!("docs/node_description.md")]
#[doc = include_str!("docs/node_struct_attributes.md")]
#[doc = include_str!("docs/node_field_attributes.md")]
pub use aparecium_macros::Node;

#[doc = include_str!("docs/object_description.md")]
#[doc = include_str!("docs/object_struct_attributes.md")]
#[doc = include_str!("docs/object_field_attributes.md")]
pub use aparecium_macros::Object;

pub use aparecium_macros::{
    Pickable, TableResources, block, chain, define_function, for_, pick, query, query_raw,
    transaction,
};

#[doc(hidden)]
pub mod internal_tools {
    pub use paste::paste;
    pub use typed_builder;
}

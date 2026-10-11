// Meta functions
// These functions can be used to retrieve specific metadata from a SurrealDB Record ID.

use crate::{Function, RecordIdLike};

/// Extracts and returns the table id from a SurrealDB Record ID
pub fn id_fn(record_id: impl Into<RecordIdLike>) -> Function {
    Function::new("meta::id").arg(record_id.into())
}

/**
Extracts and returns the table id from a SurrealDB Record ID
Also aliased as `meta_id!`

# Arguments
* `record_id` - The record id to extract the table name from. Can also be a field or a parameter
    representing the record id.

# Example
```rust
use aparecium::{*, functions::meta, statements::let_, types::RecordId};

let record_id = RecordId::new("person", "alice");
let result = meta::id!(record_id);
assert_eq!(result.to_debug_sql(), "meta::id(person:alice)");

let id_field = Field::new("id_field");
let result = meta::id!(id_field);
assert_eq!(result.to_debug_sql(), "meta::id(id_field)");

let id_param = let_("id_param").equal_to("person:alice").get_param();
let result = meta::id!(id_param);
assert_eq!(result.to_debug_sql(), "meta::id($id_param)");
```
*/
#[macro_export]
macro_rules! meta_id {
    ($record_id: expr) => {
        $crate::functions::meta::id_fn($record_id)
    };
}
pub use meta_id as id;

/// Extracts and returns the table name from a SurrealDB Record ID
pub fn tb_fn(record_id: impl Into<RecordIdLike>) -> Function {
    Function::new("meta::tb").arg(record_id.into())
}

/**
Extracts and returns the table name from a SurrelDB Record ID
Also aliased as `meta_tb!`

# Arguments
* `record_id` - The record id to extract the table name from. Can also be a field or a parameter
    representing the record id.

# Example
```rust
use aparecium::{*, functions::meta, statements::let_, types::RecordId};

let record_id = RecordId::new("person", "alice");
let result = meta::tb!(record_id);
assert_eq!(result.to_debug_sql(), "meta::tb(person:alice)");

let id_field = Field::new("id_field");
let result = meta::tb!(id_field);
assert_eq!(result.to_debug_sql(), "meta::tb(id_field)");

let id_param = let_("id_param").equal_to("person:alice").get_param();
let result = meta::tb!(id_param);
assert_eq!(result.to_debug_sql(), "meta::tb($id_param)");
```
*/
#[macro_export]
macro_rules! meta_tb {
    ($record_id: expr) => {
        $crate::functions::meta::tb_fn($record_id)
    };
}
pub use meta_tb as tb;

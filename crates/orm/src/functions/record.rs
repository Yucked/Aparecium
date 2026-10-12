// Record functions
// These functions retrieve specific metadata from a SurrealDB record ID.

use crate::{Function, RecordIdLike};

/// Extracts and returns the key of a SurrealDB record ID
pub fn id_fn(record_id: impl Into<RecordIdLike>) -> Function {
    Function::new("record::id").arg(record_id.into())
}

/**
Extracts and returns the key of a SurrealDB record ID
Also aliased as `record_id!`

# Arguments
* `record_id` - The record id to extract the key from. Can also be a field or a parameter
  representing the record id.

# Example
```rust
use umbriel::{*, functions::record, statements::let_, types::RecordId};

let record_id = RecordId::new("person", "alice");
let result = record::id!(record_id);
assert_eq!(result.to_debug_sql(), "record::id(person:alice)");

let id_field = Field::new("id_field");
let result = record::id!(id_field);
assert_eq!(result.to_debug_sql(), "record::id(id_field)");

let id_param = let_("id_param").equal_to("person:alice").get_param();
let result = record::id!(id_param);
assert_eq!(result.to_debug_sql(), "record::id($id_param)");
```
*/
#[macro_export]
macro_rules! record_id {
    ($record_id: expr) => {
        $crate::functions::record::id_fn($record_id)
    };
}
pub use record_id as id;

/// Extracts and returns the table name from a SurrealDB record ID
pub fn tb_fn(record_id: impl Into<RecordIdLike>) -> Function {
    Function::new("record::tb").arg(record_id.into())
}

/**
Extracts and returns the table name from a SurrealDB record ID
Also aliased as `record_tb!`

# Arguments
* `record_id` - The record id to extract the table name from. Can also be a field or a parameter
  representing the record id.

# Example
```rust
use umbriel::{*, functions::record, statements::let_, types::RecordId};

let record_id = RecordId::new("person", "alice");
let result = record::tb!(record_id);
assert_eq!(result.to_debug_sql(), "record::tb(person:alice)");

let id_field = Field::new("id_field");
let result = record::tb!(id_field);
assert_eq!(result.to_debug_sql(), "record::tb(id_field)");

let id_param = let_("id_param").equal_to("person:alice").get_param();
let result = record::tb!(id_param);
assert_eq!(result.to_debug_sql(), "record::tb($id_param)");
```
*/
#[macro_export]
macro_rules! record_tb {
    ($record_id: expr) => {
        $crate::functions::record::tb_fn($record_id)
    };
}
pub use record_tb as tb;

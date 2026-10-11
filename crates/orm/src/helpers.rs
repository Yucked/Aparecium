use surrealdb::types::{SurrealValue, Value};

/// Converts a Rust value into a SurrealDB value.
pub fn val(val: impl SurrealValue) -> Value {
    val.into_value()
}

/// A macro to create a vector of heterogeous/diverse surrealdb value
#[macro_export]
macro_rules! array {
    ($( $val:expr ),*) => {{
        vec![
            $( $crate::types::SurrealValue::into_value($val) ),*
        ]
    }};
}

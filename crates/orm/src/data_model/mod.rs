/// Contains all the casting functions
mod casting;

/// Mathematic constants supported by surrealdb
pub mod math_constants;

pub use math_constants as MATH;

/// Contains all the casting functions
pub mod cast {
    pub use super::casting::*;
}


// Sleep function
// The SLEEP function is used to introduce a delay or pause in the execution of a query or a batch of queries for a specific amount of time.
//
// Function	Description
// sleep(@duration)	Delays or pauses in the execution of a query or a batch of queries.

use crate::{DurationLike, Function};

/// The SLEEP function is used to introduce a delay or pause in the execution of a query or a batch
/// of queries for a specific amount of time.
pub fn sleep_fn(duration: impl Into<DurationLike>) -> Function {
    Function::new("sleep").arg(duration.into())
}

/// Creates a new sleep function
/// The SLEEP function is used to introduce a delay or pause in the execution of a query or a batch
/// of queries for a specific amount of time.
///
/// # Arguments
///
/// * `duration` - The duration to sleep for. This can be a `Duration` or a `Field` or a `Param`
///     that represents a duration.
///
/// # Example
/// ```rust
/// use aparecium::{*, functions::sleep, statements::let_};
/// use std::time;
/// let sleep = sleep!(time::Duration::from_secs(55));
/// assert_eq!(sleep.to_debug_sql(), "sleep(55s)");
///
/// let waiting_time_field = Field::new("waiting_time_field");
/// let sleep = sleep!(waiting_time_field);
/// assert_eq!(sleep.to_debug_sql(), "sleep(waiting_time_field)");
///
/// let waiting_time_param = Param::new("waiting_time_param");
/// let sleep = sleep!(waiting_time_param);
/// assert_eq!(sleep.to_debug_sql(), "sleep($waiting_time_param)");
/// ```
#[macro_export]
macro_rules! sleep {
    ( $duration:expr ) => {
        $crate::functions::sleep::sleep_fn($duration)
    };
}

pub use sleep;

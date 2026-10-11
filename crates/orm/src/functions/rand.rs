// These functions can be used when generating random data values.
//
// Function	Description
// rand()	Generates and returns a random floating point number
// rand::bool()	Generates and returns a random boolean
// rand::enum()	Randomly picks a value from the specified values
// rand::float()	Generates and returns a random floating point number
// rand::guid()	Generates and returns a random guid
// rand::int()	Generates and returns a random integer
// rand::string()	Generates and returns a random string
// rand::time()	Generates and returns a random datetime
// rand::uuid()	Generates and returns a random UUID
// rand::uuid::v4() Generates and returns a random Version 4 UUID
// rand::uuid::v7() Generates and returns a random Version 7 UUID

use crate::{Function, NumberLike, ValueLike};

/// `name()`, or `name(from, to)` when both bounds are given.
fn range_fn(
    function_name: &'static str,
    from: Option<impl Into<NumberLike>>,
    to: Option<impl Into<NumberLike>>,
) -> Function {
    let function = Function::new(function_name);
    match (from, to) {
        (Some(from), Some(to)) => function.arg(from.into()).arg(to.into()),
        _ => function,
    }
}

/// The rand function generates a random float, between 0 and 1.
///
/// rand() -> number
/// The following example shows this function, and its output, when used in a select statement:
///
/// SELECT * FROM rand();
/// 0.7062321084863658
/// The following example shows this function being used in a SELECT statement with an ORDER BY clause:
///
/// SELECT * FROM [{ age: 33 }, { age: 45 }, { age: 39 }] ORDER BY rand();
/// [
///     {
///         age: 45
///     },
///    {
///        age: 39
///    },
///    {
///        age: 33
///    }
/// ]
pub fn rand_fn() -> Function {
    Function::new("rand")
}

/// The `rand` function generates a random float, between 0 and 1.
///
/// # Example
/// ```rust
/// use aparecium::{*, functions::rand};
///
/// assert_eq!(rand!().to_debug_sql(), "rand()");
/// ```
#[macro_export]
macro_rules! rand_rand {
    () => {
        $crate::functions::rand::rand_fn()
    };
}

pub use rand_rand as rand;

/// The rand::bool function generates a random boolean value.
pub fn bool_fn() -> Function {
    Function::new("rand::bool")
}

/// The `rand::bool!` function generates a random boolean value.
/// The function is also aliased as `rand_bool!()`.
///
/// # Example
/// ```rust
/// use aparecium::{*, functions::rand};
/// assert_eq!(rand::bool!().to_debug_sql(), "rand::bool()");
/// ```
#[macro_export]
macro_rules! rand_bool {
    () => {
        $crate::functions::rand::bool_fn()
    };
}

pub use rand_bool as bool;

/// The rand::uuid function generates a random UUID.
pub fn uuid_fn() -> Function {
    Function::new("rand::uuid")
}

/// The `rand::uuid!` function generates a random UUID.
/// The function is also aliased as `rand_uuid!()`.
///
/// # Example
/// ```rust
/// use aparecium::{*, functions::rand};
/// assert_eq!(rand::uuid!().to_debug_sql(), "rand::uuid()");
/// ```
#[macro_export]
macro_rules! rand_uuid {
    () => {
        $crate::functions::rand::uuid_fn()
    };
}

pub use rand_uuid as uuid;

/// The rand::enum function generates a random value, from a multitude of values.
pub fn enum_fn<T: Into<ValueLike>>(values: Vec<T>) -> Function {
    values
        .into_iter()
        .fold(Function::new("rand::enum"), |function, value| {
            function.arg(value.into())
        })
}

/// The `rand::enum_!` function generates a random value, from a multitude of values.
/// The function is also aliased as `rand_enum_!()`.
///
/// # Example
/// ```rust
/// use aparecium::{*, functions::rand};
/// assert_eq!(rand::enum_!(1, 2, 3).to_debug_sql(), "rand::enum(1, 2, 3)");
/// assert_eq!(rand::enum_!(arr![1, 2, 3]).to_debug_sql(), "rand::enum(1, 2, 3)");
/// ```
#[macro_export]
macro_rules! rand_enum {
    ( $val:expr ) => {
        $crate::functions::rand::enum_fn( $val )
    };
    ($( $val:expr ),*) => {
        $crate::functions::rand::enum_fn($crate::array![ $( $val ), * ])
    };
}

pub use rand_enum as enum_;

/// The rand::float function generates a random float, between 0 and 1.
///
/// rand::float() -> float
/// If two numbers are provided, then the function generates a random float, between two numbers.
///
/// rand::float(number, number) -> float
pub fn float_fn(
    from: Option<impl Into<NumberLike>>,
    to: Option<impl Into<NumberLike>>,
) -> Function {
    range_fn("rand::float", from, to)
}

/// The `rand::float!` function generates a random float, between 0 and 1 if no arguments are
/// provided, otherwise it generates a random float between the two numbers provided.
///
/// The function is also aliased as `rand_float!()`.
/// If you want to specify a range, you can pass the range as arguments.
///
/// # Arguments
/// * `from` - The minimum value of the range. Can be a number, field or parameter reprsenting
///    the number.
/// * `to` - The maximum value of the range. Can be a number, field or parameter reprsenting
///    the number.
///
/// # Example
/// ```rust
/// use aparecium::{*, functions::rand};
/// assert_eq!(rand::float!().to_debug_sql(), "rand::float()");
/// assert_eq!(rand::float!(1, 2).to_debug_sql(), "rand::float(1, 2)");
///
/// # let minimum_field = Field::new("minimum_field");
/// # let maximum_field = Field::new("maximum_field");
/// assert_eq!(rand::float!(minimum_field, maximum_field).to_debug_sql(), "rand::float(minimum_field, maximum_field)");
///
/// # let minimum_param = Param::new("minimum_param");
/// # let maximum_param = Param::new("maximum_param");
/// assert_eq!(rand::float!(minimum_param, maximum_param).to_debug_sql(), "rand::float($minimum_param, $maximum_param)");
/// ```
#[macro_export]
macro_rules! rand_float {
    () => {
        $crate::functions::rand::float_fn(
            None as Option<$crate::NumberLike>,
            None as Option<$crate::NumberLike>,
        )
    };
    ( $from:expr, $to:expr ) => {
        $crate::functions::rand::float_fn(Some($from), Some($to))
    };
}

pub use rand_float as float;

/// The rand::int function generates a random int.
///
/// rand::int() -> int
/// If two numbers are provided, then the function generates a random int, between two numbers.
///
/// rand::int(number, number) -> int
pub fn int_fn(from: Option<impl Into<NumberLike>>, to: Option<impl Into<NumberLike>>) -> Function {
    range_fn("rand::int", from, to)
}

/// Generates a random integer if no arguments are provided, otherwise it generates a random
/// integer between the two numbers provided.
/// The function is also aliased as `rand_int!()`.
/// If you want to specify a range, you can pass the range as arguments.
///
/// # Arguments
/// * `from` - The minimum value of the range. Can be a number, field or parameter reprsenting
///     the number.
/// * `to` - The maximum value of the range. Can be a number, field or parameter reprsenting
///     the number.
///
/// # Example
/// ```rust
/// use aparecium::{*, functions::rand};
/// assert_eq!(rand::int!().to_debug_sql(), "rand::int()");
/// assert_eq!(rand::int!(1, 2).to_debug_sql(), "rand::int(1, 2)");
/// # let minimum_field = Field::new("minimum_field");
/// # let maximum_field = Field::new("maximum_field");
/// assert_eq!(rand::int!(minimum_field, maximum_field).to_debug_sql(), "rand::int(minimum_field, maximum_field)");
/// # let minimum_param = Param::new("minimum_param");
/// # let maximum_param = Param::new("maximum_param");
/// assert_eq!(rand::int!(minimum_param, maximum_param).to_debug_sql(), "rand::int($minimum_param, $maximum_param)");
/// ```
#[macro_export]
macro_rules! rand_int {
    () => {
        $crate::functions::rand::int_fn(
            None as Option<$crate::NumberLike>,
            None as Option<$crate::NumberLike>,
        )
    };
    ( $from:expr, $to:expr ) => {
        $crate::functions::rand::int_fn(Some($from), Some($to))
    };
}
pub use rand_int as int;

/// The rand::time function generates a random datetime.
///
/// rand::time() -> datetime
/// The rand::time function generates a random datetime, between two unix timestamps.
///
/// rand::time(number, number) -> datetime
pub fn time_fn(from: Option<impl Into<NumberLike>>, to: Option<impl Into<NumberLike>>) -> Function {
    range_fn("rand::time", from, to)
}

/// Generates a random datetime if no arguments are provided, otherwise it generates a random
/// datetime between the two unix timestamps provided.
/// The function is also aliased as `rand_time!()`.
/// If you want to specify a range, you can pass the range as arguments.
///
/// # Arguments
/// * `from` - The minimum value of the range. Can be a number, field or parameter reprsenting
///     the number.
/// * `to` - The maximum value of the range. Can be a number, field or parameter reprsenting
///     the number.
///
/// # Example
/// ```rust
/// use aparecium::{*, functions::rand};
/// assert_eq!(rand::time!().to_debug_sql(), "rand::time()");
/// assert_eq!(rand::time!(1, 2).to_debug_sql(), "rand::time(1, 2)");
/// # let minimum_field = Field::new("minimum_field");
/// # let maximum_field = Field::new("maximum_field");
/// assert_eq!(rand::time!(minimum_field, maximum_field).to_debug_sql(), "rand::time(minimum_field, maximum_field)");
/// # let minimum_param = Param::new("minimum_param");
/// # let maximum_param = Param::new("maximum_param");
/// assert_eq!(rand::time!(minimum_param, maximum_param).to_debug_sql(), "rand::time($minimum_param, $maximum_param)");
/// ```
#[macro_export]
macro_rules! rand_time {
    () => {
        $crate::functions::rand::time_fn(
            None as Option<$crate::NumberLike>,
            None as Option<$crate::NumberLike>,
        )
    };
    ( $from:expr, $to:expr ) => {
        $crate::functions::rand::time_fn(Some($from), Some($to))
    };
}
pub use rand_time as time;

/// The rand::string function generates a random string, with 32 characters.
///
/// rand::string() -> string
/// The rand::string function generates a random string, with a specific length.
///
/// rand::string(number) -> string
/// If two numbers are provided, then the function generates a random string, with a length between two numbers.
pub fn string_fn(
    from: Option<impl Into<NumberLike>>,
    to: Option<impl Into<NumberLike>>,
) -> Function {
    match (from, to) {
        (Some(length), None) => Function::new("rand::string").arg(length.into()),
        (from, to) => range_fn("rand::string", from, to),
    }
}

/// Generates a random string if no arguments are provided. If one argument is provided, then it
/// generates a random string with the length provided. If two arguments are provided, then it
/// generates a random string with a length between the two numbers provided.
/// The function is also aliased as `rand_string!()`.
///
/// # Arguments
/// * `from` - The length of the string if one argument is provided. If two arguments are
///     provided, then it is the minimum length of the string. Can be a number, field or parameter
///     reprsenting the number.
/// * `to` - The maximum length of the string. Can be a number, field or parameter reprsenting
///     the number.
///
/// # Example
/// ```rust
/// use aparecium::{*, functions::rand};
///
/// assert_eq!(rand::string!().to_debug_sql(), "rand::string()");
/// assert_eq!(rand::string!(1).to_debug_sql(), "rand::string(1)");
/// assert_eq!(rand::string!(1, 2).to_debug_sql(), "rand::string(1, 2)");
/// # let minimum_field = Field::new("minimum_field");
/// # let maximum_field = Field::new("maximum_field");
/// assert_eq!(rand::string!(minimum_field, maximum_field).to_debug_sql(), "rand::string(minimum_field, maximum_field)");
/// # let minimum_param = Param::new("minimum_param");
/// # let maximum_param = Param::new("maximum_param");
/// assert_eq!(rand::string!(minimum_param, maximum_param).to_debug_sql(), "rand::string($minimum_param, $maximum_param)");
/// ```
#[macro_export]
macro_rules! rand_string {
    () => {
        $crate::functions::rand::string_fn(
            None as Option<$crate::NumberLike>,
            None as Option<$crate::NumberLike>,
        )
    };
    ( $length:expr) => {
        $crate::functions::rand::string_fn(Some($length), None as Option<$crate::NumberLike>)
    };
    ( $from:expr, $to:expr ) => {
        $crate::functions::rand::string_fn(Some($from), Some($to))
    };
}
pub use rand_string as string;

/// The rand::guid function generates a 20-character random guid.
///
/// rand::guid() -> string
/// If a number is provided, then the function generates a random guid, with a specific length.
pub fn guid_fn(length: Option<impl Into<NumberLike>>) -> Function {
    let function = Function::new("rand::guid");
    match length {
        Some(length) => function.arg(length.into()),
        None => function,
    }
}

/// Generates a random guid if no arguments are provided. If one argument is provided, then it
/// generates a random guid with the length provided.
/// The function is also aliased as `rand_guid!()`.
///
/// # Arguments
///
/// * `length` - The length of the guid. Can be a number, field or parameter reprsenting the
///     number.
///
/// # Example
/// ```rust
/// use aparecium::{*, functions::rand};
///
/// assert_eq!(rand::guid!().to_debug_sql(), "rand::guid()");
/// assert_eq!(rand::guid!(1).to_debug_sql(), "rand::guid(1)");
/// # let length_field = Field::new("length_field");
/// assert_eq!(rand::guid!(length_field).to_debug_sql(), "rand::guid(length_field)");
/// # let length_param = Param::new("length_param");
/// assert_eq!(rand::guid!(length_param).to_debug_sql(), "rand::guid($length_param)");
/// ```
#[macro_export]
macro_rules! rand_guid {
    () => {
        $crate::functions::rand::guid_fn(None as Option<$crate::NumberLike>)
    };
    ( $length:expr ) => {
        $crate::functions::rand::guid_fn(Some($length))
    };
}

pub use rand_guid as guid;

/// This module contains functions for generating versioned random uuids.
pub mod uuid {
    use crate::Function;

    /// The rand::uuid::v4 function generates a random Version 4 UUID.
    ///
    /// rand::uuid::v4() -> uuid
    pub fn v4_fn() -> Function {
        Function::new("rand::uuid::v4")
    }

    /// Generates a random Version 4 UUID.
    /// The function is also aliased as `rand_uuid_v4!()`.
    ///
    /// # Example
    /// ```rust
    /// use aparecium::{*, functions::rand};
    ///
    /// assert_eq!(rand::uuid::v4!().to_debug_sql(), "rand::uuid::v4()");
    /// ```
    #[macro_export]
    macro_rules! rand_uuid_v4 {
        () => {
            $crate::functions::rand::uuid::v4_fn()
        };
    }

    pub use rand_uuid_v4 as v4;

    /// The rand::uuid::v7 function generates a random Version 7 UUID.
    ///
    /// rand::uuid::v7() -> uuid
    pub fn v7_fn() -> Function {
        Function::new("rand::uuid::v7")
    }

    /// Generates a random Version 7 UUID.
    /// The function is also aliased as `rand_uuid_v7!()`.
    ///
    /// # Example
    /// ```rust
    /// use aparecium::{*, functions::rand};
    ///
    /// assert_eq!(rand::uuid::v7!().to_debug_sql(), "rand::uuid::v7()");
    /// ```
    #[macro_export]
    macro_rules! rand_uuid_v7 {
        () => {
            $crate::functions::rand::uuid::v7_fn()
        };
    }

    pub use rand_uuid_v7 as v7;
}

// Type functions
// These functions can be used for generating and coercing data to specific data types. These functions are useful when accepting input values in client libraries, and ensuring that they are the desired type within SQL statements.
//
// Function	Description
// type::bool()	Converts a value into a boolean
// type::datetime()	Converts a value into a datetime
// type::decimal()	Converts a value into a decimal
// type::duration()	Converts a value into a duration
// type::field()	Projects a single field within a SELECT statement
// type::fields()	Projects a multiple fields within a SELECT statement
// type::float()	Converts a value into a floating point number
// type::int()	Converts a value into an integer
// type::number()	Converts a value into a number
// type::point()	Converts a value into a geometry point
// type::record()	Converts a value into a record ID
// type::string()	Converts a value into a string
// type::table()	Converts a value into a table
// type::is_array()	Checks if given value is of type array
// type::is_bool()	Checks if given value is of type bool
// type::is_bytes()	Checks if given value is of type bytes
// type::is_collection()	Checks if given value is of type collection
// type::is_datetime()	Checks if given value is of type datetime
// type::is_decimal()	Checks if given value is of type decimal
// type::is_duration()	Checks if given value is of type duration
// type::is_float()	Checks if given value is of type float
// type::is_geometry()	Checks if given value is of type geometry
// type::is_int()	Checks if given value is of type int
// type::is_line()	Checks if given value is of type line
// type::is_null()	Checks if given value is of type null
// type::is_multiline()	Checks if given value is of type multiline
// type::is_multipoint()	Checks if given value is of type multipoint
// type::is_multipolygon()	Checks if given value is of type multipolygon
// type::is_number()	Checks if given value is of type number
// type::is_object()	Checks if given value is of type object
// type::is_point()	Checks if given value is of type point
// type::is_polygon()	Checks if given value is of type polygon
// type::is_record()	Checks if given value is of type record
// type::is_string()	Checks if given value is of type string
// type::is_uuid()	Checks if given value is of type uuid

// Conversions and type checks take any value: converting or checking it is their purpose.
use crate::{ArrayLike, Function, ValueLike};

macro_rules! create_type {
    ($(#[$attr:meta])* => $function_name:expr, $value_type: ty) => {
        paste::paste! {

            $(#[$attr])*
            pub fn [<$function_name _fn>](value: impl Into<$value_type>) -> $crate::Function {
                $crate::Function::new(concat!("type::", $function_name)).arg(value.into())
            }

            $(#[$attr])*
            #[macro_export]
            macro_rules! [<type_ $function_name>] {
                ( $string:expr ) => {
                    $crate::functions::type_::[<$function_name _fn>]($string)
                };
            }
            pub use [<type_ $function_name>] as [<$function_name>];

        }
    };
}

// The type::bool function converts a value into a bool, if the value is truthy.
create_type!(
    /// The type::bool function converts a value into a bool, if the value is truthy.
    /// Also aliased as `type_bool!`
    ///
    /// # Arguments
    /// * `value` - The value to be converted to a bool. Could also be a field or a parameter representing the value.
    ///
    /// # Example
    /// ```
    /// use umbriel::{*, functions::type_};
    /// let result = type_::bool!(1234);
    /// assert_eq!(result.to_debug_sql(), "type::bool(1234)");
    ///
    /// let bool_field = Field::new("bool_field");
    /// let result = type_::bool!(bool_field);
    /// assert_eq!(result.to_debug_sql(), "type::bool(bool_field)");
    ///
    /// let bool_param = Param::new("bool_param");
    /// let result = type_::bool!(bool_param);
    /// assert_eq!(result.to_debug_sql(), "type::bool($bool_param)");
    /// ```
    =>
    "bool", ValueLike
);

create_type!(
    /// The type::datetime function converts a value into a datetime.
    /// Also aliased as `type_datetime!`
    ///
    /// # Arguments
    /// * `value` - The value to be converted to a datetime. Could also be a field or a parameter
    /// representing the value.
    ///
    /// # Example
    /// ```rust
    /// use umbriel::{*, functions::type_};
    ///
    /// let dt = chrono::DateTime::from_timestamp(61, 0).unwrap();
    /// let result = type_::datetime!(dt);
    /// assert_eq!(result.to_debug_sql(), "type::datetime(d'1970-01-01T00:01:01Z')");
    ///
    /// let datetime_field = Field::new("datetime_field");
    /// let result = type_::datetime!(datetime_field);
    /// assert_eq!(result.to_debug_sql(), "type::datetime(datetime_field)");
    ///
    /// let datetime_param = Param::new("datetime_param");
    /// let result = type_::datetime!(datetime_param);
    /// assert_eq!(result.to_debug_sql(), "type::datetime($datetime_param)");
    ///
    /// let result = type_::datetime!("2021-11-01T08:30:17Z");
    /// assert_eq!(result.to_debug_sql(), "type::datetime('2021-11-01T08:30:17Z')");
    /// ```
    =>
    "datetime", ValueLike
);

// The type::duration function converts a value into a duration.
create_type!(
    /// The type::duration function converts a value into a duration.
    /// Also aliased as `type_duration!`
    ///
    /// # Arguments
    /// * `value` - The value to be converted to a duration. Could also be a field or a parameter
    /// representing the value.
    ///
    /// # Example
    /// ```rust
    /// use umbriel::{*, functions::type_};
    ///
    /// let duration = std::time::Duration::from_secs(60 * 60);
    /// let result = type_::duration!(duration);
    /// assert_eq!(result.to_debug_sql(), "type::duration(1h)");
    ///
    /// let duration_field = Field::new("duration_field");
    /// let result = type_::duration!(duration_field);
    /// assert_eq!(result.to_debug_sql(), "type::duration(duration_field)");
    ///
    /// let duration_param = Param::new("duration_param");
    /// let result = type_::duration!(duration_param);
    /// assert_eq!(result.to_debug_sql(), "type::duration($duration_param)");
    ///
    /// let result = type_::duration!("1h");
    /// assert_eq!(result.to_debug_sql(), "type::duration('1h')");
    /// ```
    =>
    "duration", ValueLike
);

create_type!(
    /// The type::field function projects a single field within a SELECT statement.
    /// Also aliased as `type_field!`
    ///
    /// # Arguments
    /// * `value` - The value to be converted to a field. Could also be a field or a parameter
    /// representing the value.
    ///
    /// # Example
    /// ```rust
    /// use umbriel::{*, functions::type_};
    /// let result = type_::field!("name");
    /// assert_eq!(result.to_debug_sql(), "type::field('name')");
    ///
    /// let field_field = Field::new("field_field");
    /// let result = type_::field!(field_field);
    /// assert_eq!(result.to_debug_sql(), "type::field(field_field)");
    ///
    /// let field_param = Param::new("field_param");
    /// let result = type_::field!(field_param);
    /// assert_eq!(result.to_debug_sql(), "type::field($field_param)");
    /// ```
    =>
    "field", ValueLike
);

create_type!(
    /// The type::fields function projects multiple fields within a SELECT statement.
    /// Also aliased as `type_fields!`
    ///
    /// # Arguments
    /// * `value` - The value to be converted to a field. Could also be a field or a parameter
    /// representing the value.
    ///
    /// # Example
    /// ```rust
    /// use umbriel::{
    ///     *,
    ///     functions::type_,
    ///     statements::let_,
    /// };
    /// let result = type_::fields!(["name", "age"]);
    /// assert_eq!(result.to_debug_sql(), "type::fields(['name', 'age'])");
    ///
    /// let field_field = Field::new("field_field");
    /// let result = type_::fields!(field_field);
    /// assert_eq!(result.to_debug_sql(), "type::fields(field_field)");
    ///
    /// let field_param = Param::new("field_param");
    /// let result = type_::fields!(field_param);
    /// assert_eq!(result.to_debug_sql(), "type::fields($field_param)");
    /// ```
    =>
    "fields", ArrayLike
);

create_type!(
    /// The type::float function converts a value into a float.
    /// Also aliased as `type_float!`
    ///
    /// # Arguments
    /// * `value` - The value to be converted to a float. Could also be a field or a parameter
    /// representing the value.
    ///
    /// # Example
    /// ```rust
    /// use umbriel::{*, functions::type_};
    /// let result = type_::float!(1234);
    /// assert_eq!(result.to_debug_sql(), "type::float(1234)");
    ///
    /// let float_field = Field::new("float_field");
    /// let result = type_::float!(float_field);
    /// assert_eq!(result.to_debug_sql(), "type::float(float_field)");
    ///
    /// let float_param = Param::new("float_param");
    /// let result = type_::float!(float_param);
    /// assert_eq!(result.to_debug_sql(), "type::float($float_param)");
    ///
    /// let result = type_::float!("1.5");
    /// assert_eq!(result.to_debug_sql(), "type::float('1.5')");
    /// ```
    =>
    "float", ValueLike
);

create_type!(
    /// The type::int function converts a value into an int.
    /// Also aliased as `type_int!`
    ///
    /// # Arguments
    /// * `value` - The value to be converted to an int. Could also be a field or a parameter
    /// representing the value.
    ///
    /// # Example
    /// ```rust
    /// use umbriel::{*, functions::type_};
    /// let result = type_::int!(1234);
    /// assert_eq!(result.to_debug_sql(), "type::int(1234)");
    ///
    /// let int_field = Field::new("int_field");
    /// let result = type_::int!(int_field);
    /// assert_eq!(result.to_debug_sql(), "type::int(int_field)");
    ///
    /// let int_param = Param::new("int_param");
    /// let result = type_::int!(int_param);
    /// assert_eq!(result.to_debug_sql(), "type::int($int_param)");
    ///
    /// let result = type_::int!("5");
    /// assert_eq!(result.to_debug_sql(), "type::int('5')");
    /// ```
    =>
    "int", ValueLike
);

create_type!(
    /// The type::number function converts a value into a number.
    /// Also aliased as `type_number!`
    ///
    /// # Arguments
    /// * `value` - The value to be converted to a number. Could also be a field or a parameter
    /// representing the value.
    ///
    /// # Example
    /// ```rust
    /// use umbriel::{*, functions::type_};
    /// let result = type_::number!(1234);
    /// assert_eq!(result.to_debug_sql(), "type::number(1234)");
    ///
    /// let number_field = Field::new("number_field");
    /// let result = type_::number!(number_field);
    /// assert_eq!(result.to_debug_sql(), "type::number(number_field)");
    ///
    /// let result = type_::number!("5");
    /// assert_eq!(result.to_debug_sql(), "type::number('5')");
    /// ```
    =>
    "number", ValueLike
);

create_type!(
    /// The type::string function converts a value into a string.
    /// Also aliased as `type_string!`
    ///
    /// # Arguments
    /// * `value` - The value to be converted to a string. Could also be a field or a parameter
    /// representing the value.
    ///
    /// # Example
    /// ```rust
    /// use umbriel::{*, functions::type_};
    /// let result = type_::string!(1234);
    /// assert_eq!(result.to_debug_sql(), "type::string(1234)");
    ///
    /// let string_field = Field::new("string_field");
    /// let result = type_::string!(string_field);
    /// assert_eq!(result.to_debug_sql(), "type::string(string_field)");
    ///
    /// let string_param = Param::new("string_param");
    /// let result = type_::string!(string_param);
    /// assert_eq!(result.to_debug_sql(), "type::string($string_param)");
    /// ```
    =>
    "string", ValueLike
);

create_type!(
    /// The type::table function converts a value into a table definition.
    /// Also aliased as `type_table!`
    ///
    /// # Arguments
    /// * `value` - The value to be converted to a table definition. Could also be a field or a parameter
    /// representing the value.
    ///
    /// # Example
    /// ```rust
    /// use umbriel::{*, functions::type_, statements::let_};
    /// let result = type_::table!(Table::new("user"));
    /// assert_eq!(result.to_debug_sql(), "type::table(user)");
    ///
    /// let table_field = Field::new("table_field");
    /// let result = type_::table!(table_field);
    /// assert_eq!(result.to_debug_sql(), "type::table(table_field)");
    ///
    /// let table_param = let_("table_param").equal_to("user").get_param();
    /// let result = type_::table!(table_param);
    /// assert_eq!(result.to_debug_sql(), "type::table($table_param)");
    ///
    /// let result = type_::table!("user");
    /// assert_eq!(result.to_debug_sql(), "type::table('user')");
    /// ```
    =>
    "table", ValueLike
);

create_type!(
    /// The type::point function converts a value, such as a `[longitude, latitude]` array, into a
    /// geometry point.
    /// Also aliased as `type_point!`
    ///
    /// # Arguments
    /// * `value` - The value to be converted to a point. Could also be a field or a parameter
    /// representing the value.
    ///
    /// # Example
    /// ```rust
    /// use umbriel::{*, functions::type_, statements::let_};
    /// let result = type_::point!(vec![51.509865, -0.118092]);
    /// assert_eq!(result.to_debug_sql(), "type::point([51.509865f, -0.118092f])");
    ///
    /// let point_field = Field::new("point_field");
    /// let result = type_::point!(point_field);
    /// assert_eq!(result.to_debug_sql(), "type::point(point_field)");
    ///
    /// let point_param = let_("point_param").equal_to(vec![1, 2]).get_param();
    /// let result = type_::point!(point_param);
    /// assert_eq!(result.to_debug_sql(), "type::point($point_param)");
    /// ```
    =>
    "point", ValueLike
);

/// The type::record function converts a table and a key into a record ID.
pub fn record_fn(table: impl Into<ValueLike>, value: impl Into<ValueLike>) -> Function {
    Function::new("type::record")
        .arg(table.into())
        .arg(value.into())
}

/// The type::record function converts a table and a key into a record ID.
/// Also aliased as `type_record!`
///
/// # Arguments
/// * `table` - The table of the record ID. Could also be a field or a parameter
///   representing the value.
/// * `value` - The key of the record ID. Could also be a field or a parameter
///   representing the value.
///
/// # Example
/// ```rust
/// use umbriel::{*, functions::type_, statements::let_};
/// let result = type_::record!("user", 1234);
/// assert_eq!(result.to_debug_sql(), "type::record('user', 1234)");
///
/// let table_field = Field::new("table_field");
/// let value_field = Field::new("value_field");
/// let result = type_::record!(table_field, value_field);
/// assert_eq!(result.to_debug_sql(), "type::record(table_field, value_field)");
///
/// let table_param = let_("table_param").equal_to("user").get_param();
/// let value_param = let_("value_param").equal_to(1234).get_param();
/// let result = type_::record!(table_param, value_param);
/// assert_eq!(result.to_debug_sql(), "type::record($table_param, $value_param)");
/// ```
#[macro_export]
macro_rules! type_record {
    ( $table:expr, $value:expr ) => {
        $crate::functions::type_::record_fn($table, $value)
    };
}

pub use type_record as record;

// create_is_function macro
macro_rules! create_is_function {
    ($(#[$attr:meta])* => $function_name:expr, $value_type: ty) => {
        paste::paste! {

            $(#[$attr])*
            pub fn [<is_ $function_name _fn>](value: impl Into<$value_type>) -> $crate::Function {
                $crate::Function::new(concat!("type::is_", $function_name)).arg(value.into())
            }

            $(#[$attr])*
            #[macro_export]
            macro_rules! [<type_is_ $function_name>] {
                ( $value:expr ) => {
                    $crate::functions::type_::[<is_ $function_name _fn>]($value)
                };
            }
            pub use [<type_is_ $function_name>] as [<is_ $function_name>];

        }
    };
}

// The `type::is_*` functions check if given value is of a specific type.
create_is_function!(
    /// The type::is_array function checks if given value is of type array.
    /// Also aliased as `type_is_array!`
    ///
    /// # Arguments
    /// * `value` - The value to be checked if it is of type array. Could also be a field or a parameter
    /// representing the value.
    ///
    /// # Example
    /// ```rust
    /// use umbriel::{*, functions::type_};
    /// let result = type_::is_array!(1234);
    /// assert_eq!(result.to_debug_sql(), "type::is_array(1234)");
    ///
    /// let array_field = Field::new("array_field");
    /// let result = type_::is_array!(array_field);
    /// assert_eq!(result.to_debug_sql(), "type::is_array(array_field)");
    ///
    /// let array_param = Param::new("array_param");
    /// let result = type_::is_array!(array_param);
    /// assert_eq!(result.to_debug_sql(), "type::is_array($array_param)");
    /// ```
    =>
    "array", ValueLike
);

create_is_function!(
    /// The type::is_bool function checks if given value is of type bool.
    /// Also aliased as `type_is_bool!`
    ///
    /// # Arguments
    /// * `value` - The value to be checked if it is of type bool. Could also be a field or a parameter
    /// representing the value.
    ///
    /// # Example
    /// ```rust
    /// use umbriel::{*, functions::type_};
    /// let result = type_::is_bool!(1234);
    /// assert_eq!(result.to_debug_sql(), "type::is_bool(1234)");
    ///
    /// let bool_field = Field::new("bool_field");
    /// let result = type_::is_bool!(bool_field);
    /// assert_eq!(result.to_debug_sql(), "type::is_bool(bool_field)");
    ///
    /// let bool_param = Param::new("bool_param");
    /// let result = type_::is_bool!(bool_param);
    /// assert_eq!(result.to_debug_sql(), "type::is_bool($bool_param)");
    /// ```
    =>
    "bool", ValueLike
);

create_is_function!(
    /// The type::is_bytes function checks if given value is of type bytes.
    /// Also aliased as `type_is_bytes!`
    ///
    /// # Arguments
    /// * `value` - The value to be checked if it is of type bytes. Could also be a field or a parameter
    /// representing the value.
    ///
    /// # Example
    /// ```rust
    /// use umbriel::{*, functions::type_};
    /// let result = type_::is_bytes!(1234);
    /// assert_eq!(result.to_debug_sql(), "type::is_bytes(1234)");
    ///
    /// let bytes_field = Field::new("bytes_field");
    /// let result = type_::is_bytes!(bytes_field);
    /// assert_eq!(result.to_debug_sql(), "type::is_bytes(bytes_field)");
    ///
    /// let bytes_param = Param::new("bytes_param");
    /// let result = type_::is_bytes!(bytes_param);
    /// assert_eq!(result.to_debug_sql(), "type::is_bytes($bytes_param)");
    /// ```
    =>
    "bytes", ValueLike
);

create_is_function!(
    /// The type::is_datetime function checks if given value is of type datetime.
    /// Also aliased as `type_is_datetime!`
    ///
    /// # Arguments
    /// * `value` - The value to be checked if it is of type datetime. Could also be a field or a parameter
    /// representing the value.
    ///
    /// # Example
    /// ```rust
    /// use umbriel::{*, functions::type_};
    ///
    /// let dt = chrono::DateTime::from_timestamp(61, 0).unwrap();
    /// let result = type_::is_datetime!(dt);
    /// assert_eq!(result.to_debug_sql(), "type::is_datetime(d'1970-01-01T00:01:01Z')");
    ///
    /// let datetime_field = Field::new("datetime_field");
    /// let result = type_::is_datetime!(datetime_field);
    /// assert_eq!(result.to_debug_sql(), "type::is_datetime(datetime_field)");
    ///
    /// let datetime_param = Param::new("datetime_param");
    /// let result = type_::is_datetime!(datetime_param);
    /// assert_eq!(result.to_debug_sql(), "type::is_datetime($datetime_param)");
    /// ```
    =>
    "datetime", ValueLike
);

create_is_function!(
    /// The type::is_collection function checks if given value is of type collection.
    /// Also aliased as `type_is_collection!`
    ///
    /// # Arguments
    /// * `value` - The value to be checked if it is of type collection. Could also be a field or a parameter
    /// representing the value.
    ///
    /// # Example
    /// ```rust
    /// use umbriel::{*, functions::type_};
    ///
    /// let result = type_::is_collection!(1234);
    /// assert_eq!(result.to_debug_sql(), "type::is_collection(1234)");
    ///
    /// let collection_field = Field::new("collection_field");
    /// let result = type_::is_collection!(collection_field);
    /// assert_eq!(result.to_debug_sql(), "type::is_collection(collection_field)");
    ///
    /// let collection_param = Param::new("collection_param");
    /// let result = type_::is_collection!(collection_param);
    /// assert_eq!(result.to_debug_sql(), "type::is_collection($collection_param)");
    /// ```
    =>
    "collection", ValueLike
);

create_is_function!(
    /// The type::is_decimal function checks if given value is of type decimal.
    /// Also aliased as `type_is_decimal!`
    ///
    /// # Arguments
    /// * `value` - The value to be checked if it is of type decimal. Could also be a field or a parameter
    /// representing the value.
    ///
    /// # Example
    /// ```rust
    /// use umbriel::{*, functions::type_};
    /// let result = type_::is_decimal!(1234);
    /// assert_eq!(result.to_debug_sql(), "type::is_decimal(1234)");
    ///
    /// let decimal_field = Field::new("decimal_field");
    /// let result = type_::is_decimal!(decimal_field);
    /// assert_eq!(result.to_debug_sql(), "type::is_decimal(decimal_field)");
    ///
    /// let decimal_param = Param::new("decimal_param");
    /// let result = type_::is_decimal!(decimal_param);
    /// assert_eq!(result.to_debug_sql(), "type::is_decimal($decimal_param)");
    /// ```
    =>
    "decimal", ValueLike
);

create_is_function!(
    /// The type::is_duration function checks if given value is of type duration.
    /// Also aliased as `type_is_duration!`
    ///
    /// # Arguments
    /// * `value` - The value to be checked if it is of type duration. Could also be a field or a parameter
    /// representing the value.
    ///
    /// # Example
    /// ```rust
    /// use umbriel::{*, functions::type_};
    ///
    /// let duration = std::time::Duration::from_secs(60 * 60);
    /// let result = type_::is_duration!(duration);
    /// assert_eq!(result.to_debug_sql(), "type::is_duration(1h)");
    ///
    /// let duration_field = Field::new("duration_field");
    /// let result = type_::is_duration!(duration_field);
    /// assert_eq!(result.to_debug_sql(), "type::is_duration(duration_field)");
    ///
    /// let duration_param = Param::new("duration_param");
    /// let result = type_::is_duration!(duration_param);
    /// assert_eq!(result.to_debug_sql(), "type::is_duration($duration_param)");
    /// ```
    =>
    "duration", ValueLike
);

create_is_function!(
    /// The type::is_float function checks if given value is of type float.
    /// Also aliased as `type_is_float!`
    ///
    /// # Arguments
    /// * `value` - The value to be checked if it is of type float. Could also be a field or a parameter
    /// representing the value.
    ///
    /// # Example
    /// ```rust
    /// use umbriel::{*, functions::type_};
    /// let result = type_::is_float!(1234);
    /// assert_eq!(result.to_debug_sql(), "type::is_float(1234)");
    ///
    /// let float_field = Field::new("float_field");
    /// let result = type_::is_float!(float_field);
    /// assert_eq!(result.to_debug_sql(), "type::is_float(float_field)");
    ///
    /// let float_param = Param::new("float_param");
    /// let result = type_::is_float!(float_param);
    /// assert_eq!(result.to_debug_sql(), "type::is_float($float_param)");
    /// ```
    =>
    "float", ValueLike
);

create_is_function!(
    /// The type::is_int function checks if given value is of type int.
    /// Also aliased as `type_is_int!`
    ///
    /// # Arguments
    /// * `value` - The value to be checked if it is of type int. Could also be a field or a parameter
    /// representing the value.
    ///
    /// # Example
    /// ```rust
    /// use umbriel::{*, functions::type_};
    /// let result = type_::is_int!(1234);
    /// assert_eq!(result.to_debug_sql(), "type::is_int(1234)");
    ///
    /// let int_field = Field::new("int_field");
    /// let result = type_::is_int!(int_field);
    /// assert_eq!(result.to_debug_sql(), "type::is_int(int_field)");
    ///
    /// let int_param = Param::new("int_param");
    /// let result = type_::is_int!(int_param);
    /// assert_eq!(result.to_debug_sql(), "type::is_int($int_param)");
    /// ```
    =>
    "int", ValueLike
);

create_is_function!(
    /// The type::is_null function checks if given value is of type null.
    /// Also aliased as `type_is_null!`
    ///
    /// # Arguments
    /// * `value` - The value to be checked if it is of type null. Could also be a field or a parameter
    /// representing the value.
    ///
    /// # Example
    /// ```rust
    /// use umbriel::{*, functions::type_};
    ///
    /// let result = type_::is_null!(1234);
    /// assert_eq!(result.to_debug_sql(), "type::is_null(1234)");
    ///
    /// let null_field = Field::new("null_field");
    /// let result = type_::is_null!(null_field);
    /// assert_eq!(result.to_debug_sql(), "type::is_null(null_field)");
    ///
    /// let null_param = Param::new("null_param");
    /// let result = type_::is_null!(null_param);
    /// assert_eq!(result.to_debug_sql(), "type::is_null($null_param)");
    /// ```
    =>
    "null", ValueLike
);

create_is_function!(
    /// The type::is_number function checks if given value is of type number.
    /// Also aliased as `type_is_number!`
    ///
    /// # Arguments
    /// * `value` - The value to be checked if it is of type number. Could also be a field or a parameter
    /// representing the value.
    ///
    /// # Example
    /// ```rust
    /// use umbriel::{*, functions::type_};
    /// let result = type_::is_number!(1234);
    /// assert_eq!(result.to_debug_sql(), "type::is_number(1234)");
    ///
    /// let number_field = Field::new("number_field");
    /// let result = type_::is_number!(number_field);
    /// assert_eq!(result.to_debug_sql(), "type::is_number(number_field)");
    ///
    /// let number_param = Param::new("number_param");
    /// let result = type_::is_number!(number_param);
    /// assert_eq!(result.to_debug_sql(), "type::is_number($number_param)");
    /// ```
    =>
    "number", ValueLike
);

create_is_function!(
    /// The type::is_string function checks if given value is of type string.
    /// Also aliased as `type_is_string!`
    ///
    /// # Arguments
    /// * `value` - The value to be checked if it is of type string. Could also be a field or a parameter
    /// representing the value.
    ///
    /// # Example
    /// ```rust
    ///
    /// use umbriel::{*, functions::type_};
    /// let result = type_::is_string!("Alice");
    /// assert_eq!(result.to_debug_sql(), "type::is_string('Alice')");
    ///
    /// let string_field = Field::new("string_field");
    /// let result = type_::is_string!(string_field);
    /// assert_eq!(result.to_debug_sql(), "type::is_string(string_field)");
    ///
    /// let string_param = Param::new("string_param");
    /// let result = type_::is_string!(string_param);
    /// assert_eq!(result.to_debug_sql(), "type::is_string($string_param)");
    /// ```
    =>
    "string", ValueLike
);

create_is_function!(
    /// The type::is_geometry function checks if given value is of type geometry.
    /// Also aliased as `type_is_geometry!`
    ///
    /// # Arguments
    /// * `value` - The value to be checked if it is of type geometry. Could also be a field or a parameter
    /// representing the value.
    ///
    /// # Example
    /// ```rust
    /// use umbriel::{*, functions::type_};
    ///
    /// let result = type_::is_geometry!(geo::point!(x: 51.509865, y: -0.118092));
    /// assert_eq!(result.to_debug_sql(), "type::is_geometry((51.509865f, -0.118092f))");
    ///
    /// let geometry_field = Field::new("geometry_field");
    /// let result = type_::is_geometry!(geometry_field);
    /// assert_eq!(result.to_debug_sql(), "type::is_geometry(geometry_field)");
    ///
    /// let geometry_param = Param::new("geometry_param");
    /// let result = type_::is_geometry!(geometry_param);
    /// assert_eq!(result.to_debug_sql(), "type::is_geometry($geometry_param)");
    /// ```
    =>
    "geometry", ValueLike
);

create_is_function!(
    /// The type::is_line function checks if given value is of type line.
    /// Also aliased as `type_is_line!`
    ///
    /// # Arguments
    /// * `value` - The value to be checked if it is of type line. Could also be a field or a parameter
    /// representing the value.
    ///
    /// # Example
    /// ```rust
    /// use umbriel::{*, functions::type_};
    ///
    /// let result = type_::is_line!(geo::LineString(vec![
    ///    geo::Coord {
    ///    x: -122.33583,
    ///    y: 47.60621,
    ///    },
    ///    geo::Coord {
    ///    x: -122.33583,
    ///    y: 47.60622,
    ///    },
    /// ]));
    /// assert_eq!(result.to_debug_sql(),
    /// "type::is_line({ type: 'LineString', coordinates: [[-122.33583f, 47.60621f], [-122.33583f, 47.60622f]] })");
    ///
    /// let line_field = Field::new("line_field");
    /// let result = type_::is_line!(line_field);
    /// assert_eq!(result.to_debug_sql(), "type::is_line(line_field)");
    ///
    /// let line_param = Param::new("line_param");
    /// let result = type_::is_line!(line_param);
    /// assert_eq!(result.to_debug_sql(), "type::is_line($line_param)");
    /// ```
    =>
    "line", ValueLike
);

create_is_function!(
    /// The type::is_multiline function checks if given value is of type multiline.
    /// Also aliased as `type_is_multiline!`
    ///
    /// # Arguments
    /// * `value` - The value to be checked if it is of type multiline. Could also be a field or a parameter
    /// representing the value.
    ///
    /// # Example
    /// ```rust
    /// use umbriel::{*, functions::type_};
    ///
    /// let result = type_::is_multiline!(geo::MultiLineString(vec![]));
    /// assert_eq!(result.to_debug_sql(),
    /// "type::is_multiline({ type: 'MultiLineString', coordinates: [] })");
    ///
    /// let multiline_field = Field::new("multiline_field");
    /// let result = type_::is_multiline!(multiline_field);
    /// assert_eq!(result.to_debug_sql(), "type::is_multiline(multiline_field)");
    ///
    /// let multiline_param = Param::new("multiline_param");
    /// let result = type_::is_multiline!(multiline_param);
    /// assert_eq!(result.to_debug_sql(), "type::is_multiline($multiline_param)");
    /// ```
    =>
    "multiline", ValueLike
);

create_is_function!(
    /// The type::is_multipoint function checks if given value is of type multipoint.
    /// Also aliased as `type_is_multipoint!`
    ///
    /// # Arguments
    /// * `value` - The value to be checked if it is of type multipoint. Could also be a field or a parameter
    /// representing the value.
    ///
    /// # Example
    /// ```rust
    /// use umbriel::{*, functions::type_};
    ///
    /// let result = type_::is_multipoint!(geo::MultiPoint(vec![(0.0, 0.0).into(), (1.0, 1.0).into(), (2.0, 35.0).into()]));
    /// assert_eq!(result.to_debug_sql(),
    /// "type::is_multipoint({ type: 'MultiPoint', coordinates: [[0f, 0f], [1f, 1f], [2f, 35f]] })");
    ///
    /// let multipoint_field = Field::new("multipoint_field");
    /// let result = type_::is_multipoint!(multipoint_field);
    /// assert_eq!(result.to_debug_sql(), "type::is_multipoint(multipoint_field)");
    ///
    /// let multipoint_param = Param::new("multipoint_param");
    /// let result = type_::is_multipoint!(multipoint_param);
    /// assert_eq!(result.to_debug_sql(), "type::is_multipoint($multipoint_param)");
    /// ```
    =>
    "multipoint", ValueLike
);

create_is_function!(
    /// The type::is_multipolygon function checks if given value is of type multipolygon.
    /// Also aliased as `type_is_multipolygon!`
    ///
    /// # Arguments
    /// * `value` - The value to be checked if it is of type multipolygon. Could also be a field or a parameter
    /// representing the value.
    ///
    /// # Example
    /// ```rust
    /// use umbriel::{*, functions::type_};
    ///
    /// let result = type_::is_multipolygon!(geo::MultiPolygon(vec![]));
    ///
    /// assert_eq!(result.to_debug_sql(),
    /// "type::is_multipolygon({ type: 'MultiPolygon', coordinates: [] })");
    /// let multipolygon_field = Field::new("multipolygon_field");
    /// let result = type_::is_multipolygon!(multipolygon_field);
    /// assert_eq!(result.to_debug_sql(), "type::is_multipolygon(multipolygon_field)");
    /// let multipolygon_param = Param::new("multipolygon_param");
    /// let result = type_::is_multipolygon!(multipolygon_param);
    /// assert_eq!(result.to_debug_sql(), "type::is_multipolygon($multipolygon_param)");
    /// ```
    =>
    "multipolygon", ValueLike
);

create_is_function!(
    /// The type::is_point function checks if given value is of type point.
    /// Also aliased as `type_is_point!`
    ///
    /// # Arguments
    /// * `value` - The value to be checked if it is of type point. Could also be a field or a parameter
    /// representing the value.
    ///
    /// # Example
    /// ```rust
    /// use umbriel::{*, functions::type_};
    ///
    /// let result = type_::is_point!(geo::Point::new(0.0, 0.0));
    /// assert_eq!(result.to_debug_sql(), "type::is_point((0f, 0f))");
    ///
    /// let point_field = Field::new("point_field");
    /// let result = type_::is_point!(point_field);
    /// assert_eq!(result.to_debug_sql(), "type::is_point(point_field)");
    ///
    /// let point_param = Param::new("point_param");
    /// let result = type_::is_point!(point_param);
    /// assert_eq!(result.to_debug_sql(), "type::is_point($point_param)");
    /// ```
    =>
    "point", ValueLike
);

create_is_function!(
    /// The type::is_polygon function checks if given value is of type polygon.
    /// Also aliased as `type_is_polygon!`
    ///
    /// # Arguments
    /// * `value` - The value to be checked if it is of type polygon. Could also be a field or a parameter
    /// representing the value.
    ///
    /// # Example
    /// ```rust
    /// use umbriel::{*, functions::type_};
    ///
    /// let result = type_::is_polygon!(geo::Polygon::new(
    ///    geo::LineString(vec![
    ///         geo::Coord {
    ///             x: -122.33583,
    ///             y: 47.60621,
    ///         },
    ///         geo::Coord {
    ///             x: -122.33583,
    ///             y: 47.60622,
    ///         },
    ///    ]),
    ///    vec![geo::LineString(vec![
    ///             geo::Coord {
    ///                 x: -122.33583,
    ///                 y: 47.60621,
    ///             },
    ///             geo::Coord {
    ///                 x: -122.33583,
    ///                 y: 47.60622,
    ///             },
    ///         ])],
    ///    ));
    ///    println!("{}", result.to_debug_sql());
    ///
    ///    let polygon_field = Field::new("polygon_field");
    ///    let result = type_::is_polygon!(polygon_field);
    ///    assert_eq!(result.to_debug_sql(), "type::is_polygon(polygon_field)");
    ///
    ///    let polygon_param = Param::new("polygon_param");
    ///    let result = type_::is_polygon!(polygon_param);
    ///    assert_eq!(result.to_debug_sql(), "type::is_polygon($polygon_param)");
    ///    ```
    =>
    "polygon", ValueLike
);

create_is_function!(
    /// The type::is_object function checks if given value is of type object.
    /// Also aliased as `type_is_object!`
    ///
    /// # Arguments
    /// * `value` - The value to be checked if it is of type object. Could also be a field or a parameter
    /// representing the value.
    ///
    /// # Example
    /// ```rust
    /// use umbriel::{*, functions::type_};
    ///
    /// let result = type_::is_object!(types::Object::default());
    /// assert_eq!(result.to_debug_sql(), "type::is_object({  })");
    ///
    /// let object_field = Field::new("object_field");
    /// let result = type_::is_object!(object_field);
    /// assert_eq!(result.to_debug_sql(), "type::is_object(object_field)");
    ///
    /// let object_param = Param::new("object_param");
    /// let result = type_::is_object!(object_param);
    /// assert_eq!(result.to_debug_sql(), "type::is_object($object_param)");
    /// ```
    =>
    "object", ValueLike
);

create_is_function!(
    /// The type::is_record function checks if given value is of type record.
    /// Also aliased as `type_is_record!`
    ///
    /// # Arguments
    /// * `value` - The value to be checked if it is of type record. Could also be a field or a parameter
    /// representing the value.
    ///
    /// # Example
    /// ```rust
    /// use umbriel::{*, functions::type_};
    ///
    /// let result = type_::is_record!(types::RecordId::new("user", "alice"));
    /// assert_eq!(result.to_debug_sql(), "type::is_record(user:alice)");
    ///
    /// let record_field = Field::new("record_field");
    /// let result = type_::is_record!(record_field);
    /// assert_eq!(result.to_debug_sql(), "type::is_record(record_field)");
    ///
    /// let record_param = Param::new("record_param");
    /// let result = type_::is_record!(record_param);
    /// assert_eq!(result.to_debug_sql(), "type::is_record($record_param)");
    /// ```
    =>
    "record", ValueLike
);

create_is_function!(
    /// The type::is_uuid function checks if given value is of type uuid.
    /// Also aliased as `type_is_uuid!`
    ///
    /// # Arguments
    /// * `value` - The value to be checked if it is of type uuid. Could also be a field or a parameter
    /// representing the value.
    ///
    /// # Example
    /// ```rust
    /// use umbriel::{*, functions::type_};
    ///
    /// let result = type_::is_uuid!(uuid::Uuid::parse_str("936DA01F9ABD4d9d80C702AF85C822A8").unwrap());
    /// assert_eq!(result.to_debug_sql(), "type::is_uuid(u'936da01f-9abd-4d9d-80c7-02af85c822a8')");
    ///
    /// let uuid_field = Field::new("uuid_field");
    /// let result = type_::is_uuid!(uuid_field);
    /// assert_eq!(result.to_debug_sql(), "type::is_uuid(uuid_field)");
    ///
    /// let uuid_param = Param::new("uuid_param");
    /// let result = type_::is_uuid!(uuid_param);
    /// assert_eq!(result.to_debug_sql(), "type::is_uuid($uuid_param)");
    /// ```
    =>
    "uuid", ValueLike
);


// String functions
// These functions can be used when working with and manipulating text and string values.
//
// Function	Description
// string::concat()	Concatenates strings together
// string::contains()	Check wether a string contains another string
// string::endsWith()	Checks whether a string ends with another string
// string::join()	Joins strings together with a delimiter
// string::len()	Returns the length of a string
// string::lowercase()	Converts a string to lowercase
// string::repeat()	Repeats a string a number of times
// string::replace()	Replaces an occurence of a string with another string
// string::reverse()	Reverses a string
// string::slice()	Extracts and returns a section of a string
// string::slug()	Converts a string into human and URL-friendly string
// string::split()	Divides a string into an ordered list of substrings
// string::startsWith()	Checks whether a string starts with another string
// string::trim()	Removes whitespace from the start and end of a string
// string::uppercase()	Converts a string to uppercase
// string::words()	Splits a string into an array of separate words
// string::distance::hamming
// string::distance::levenshtein,
// string::similarity::fuzzy
// string::similarity::jaro
// string::similarity::smithwaterman
// string::is::alphanum()	Checks whether a value has only alphanumeric characters
// string::is::alpha()	Checks whether a value has only alpha characters
// string::is::ascii()	Checks whether a value has only ascii characters
// string::is::datetime() Checks whether a value matches a datetime format
// string::is::domain()	Checks whether a value is a domain
// string::is::email()	Checks whether a value is an email
// string::is::hexadecimal()	Checks whether a value is hexadecimal
// string::is::latitude()	Checks whether a value is a latitude value
// string::is::longitude()	Checks whether a value is a longitude value
// string::is::numeric()	Checks whether a value has only numeric characters
// string::is::semver()	Checks whether a value matches a semver version
// string::is::url() Checks whether a value is a valid URL
// string::is::uuid()	Checks whether a value is a UUID

use crate::{ArgsList, Function, NumberLike, StrandLike, ValueLike};

fn create_fn_with_single_string_arg(
    value: impl Into<StrandLike>,
    function_name: &'static str,
) -> Function {
    Function::new(function_name).arg(value.into())
}

/// Returns length of a string
pub fn len_fn(string: impl Into<StrandLike>) -> Function {
    create_fn_with_single_string_arg(string, "string::length")
}

/// Returns length of a string.
/// The macro function is also aliases as `string_len!`
///
/// # Arguments
/// * `string` - The string to get the length of. Can be a string literal, a field or a parameter representing a string.
///
/// # Example
/// ```rust
/// use aparecium::{*, functions::string, statements::let_};
/// let len = string::len!("Alice Smith");
/// assert_eq!(len.to_debug_sql(), "string::length('Alice Smith')");
///
/// let string_field = Field::new("string_field");
/// let len = string::len!(string_field);
/// assert_eq!(len.to_debug_sql(), "string::length(string_field)");
///
/// let string_param = let_("string_param").equal_to("Alice Smith").get_param();
/// let len = string::len!(string_param);
/// assert_eq!(len.to_debug_sql(), "string::length($string_param)");
/// ```
#[macro_export]
macro_rules! string_len {
    ( $string:expr ) => {
        $crate::functions::string::len_fn($string)
    };
}
pub use string_len as len;

/// The string::lowercase function converts a string to lowercase.
pub fn lowercase_fn(string: impl Into<StrandLike>) -> Function {
    create_fn_with_single_string_arg(string, "string::lowercase")
}

/// The string::lowercase function converts a string to lowercase.
/// The macro function is also aliases as `string_lowercase!`
/// # Arguments
/// * `string` - The string to convert to lowercase. Can be a string literal, a field or a parameter representing a string.
/// # Example
/// ```rust
/// use aparecium::{*, functions::string, statements::let_};
/// let lowercase = string::lowercase!("Alice Smith");
/// assert_eq!(lowercase.to_debug_sql(), "string::lowercase('Alice Smith')");
///
/// let string_field = Field::new("string_field");
/// let lowercase = string::lowercase!(string_field);
/// assert_eq!(lowercase.to_debug_sql(), "string::lowercase(string_field)");
///
/// let string_param = let_("string_param").equal_to("Alice Smith").get_param();
/// let lowercase = string::lowercase!(string_param);
/// assert_eq!(lowercase.to_debug_sql(), "string::lowercase($string_param)");
/// ```
#[macro_export]
macro_rules! string_lowercase {
    ( $string:expr ) => {
        $crate::functions::string::lowercase_fn($string)
    };
}

pub use string_lowercase as lowercase;

/// The string::uppercase function converts a string to uppercase.
pub fn uppercase_fn(string: impl Into<StrandLike>) -> Function {
    create_fn_with_single_string_arg(string, "string::uppercase")
}

/// The string::uppercase function converts a string to uppercase.
/// The macro function is also aliases as `string_uppercase!`
///
/// # Arguments
///
/// * `string` - The string to convert to uppercase. Can be a string literal, a field or a parameter representing a string.
///
/// # Example
/// ```rust
/// use aparecium::{*, functions::string, statements::let_};
/// let uppercase = string::uppercase!("Alice Smith");
/// assert_eq!(uppercase.to_debug_sql(), "string::uppercase('Alice Smith')");
///
/// let string_field = Field::new("string_field");
/// let uppercase = string::uppercase!(string_field);
/// assert_eq!(uppercase.to_debug_sql(), "string::uppercase(string_field)");
///
/// let string_param = let_("string_param").equal_to("Alice Smith").get_param();
/// let uppercase = string::uppercase!(string_param);
/// assert_eq!(uppercase.to_debug_sql(), "string::uppercase($string_param)");
/// ```
#[macro_export]
macro_rules! string_uppercase {
    ( $string:expr ) => {
        $crate::functions::string::uppercase_fn($string)
    };
}
pub use string_uppercase as uppercase;

/// The string::words function splits a string into an array of separate words.
pub fn words_fn(string: impl Into<StrandLike>) -> Function {
    create_fn_with_single_string_arg(string, "string::words")
}

/// The string::words function splits a string into an array of separate words.
/// The macro function is also aliases as `string_words!`
///
/// # Arguments
/// * `string` - The string to split into words. Can be a string literal, a field or a parameter representing a string.
///
/// # Example
/// ```rust
/// use aparecium::{*, functions::string, statements::let_};
/// let words = string::words!("Alice Smith");
/// assert_eq!(words.to_debug_sql(), "string::words('Alice Smith')");
///
/// let string_field = Field::new("string_field");
/// let words = string::words!(string_field);
/// assert_eq!(words.to_debug_sql(), "string::words(string_field)");
///
/// let string_param = let_("string_param").equal_to("Alice Smith").get_param();
/// let words = string::words!(string_param);
/// assert_eq!(words.to_debug_sql(), "string::words($string_param)");
/// ```
#[macro_export]
macro_rules! string_words {
    ( $string:expr ) => {
        $crate::functions::string::words_fn($string)
    };
}
pub use string_words as words;

/// The string::reverse function reverses a string.
pub fn reverse_fn(string: impl Into<StrandLike>) -> Function {
    create_fn_with_single_string_arg(string, "string::reverse")
}

/// The string::reverse function reverses a string.
/// The macro function is also aliases as `string_reverse!`
///
/// # Arguments
/// * `string` - The string to reverse. Can be a string literal, a field or a parameter representing a string.
///
/// # Example
/// ```rust
/// use aparecium::{*, functions::string, statements::let_};
/// let reverse = string::reverse!("Alice Smith");
/// assert_eq!(reverse.to_debug_sql(), "string::reverse('Alice Smith')");
///
/// let string_field = Field::new("string_field");
/// let reverse = string::reverse!(string_field);
/// assert_eq!(reverse.to_debug_sql(), "string::reverse(string_field)");
///
/// let string_param = let_("string_param").equal_to("Alice Smith").get_param();
/// let reverse = string::reverse!(string_param);
/// assert_eq!(reverse.to_debug_sql(), "string::reverse($string_param)");
/// ```
#[macro_export]
macro_rules! string_reverse {
    ( $string:expr ) => {
        $crate::functions::string::reverse_fn($string)
    };
}

pub use string_reverse as reverse;

/// The string::trim function removes leading and trailing whitespace from a string.
pub fn trim_fn(string: impl Into<StrandLike>) -> Function {
    create_fn_with_single_string_arg(string, "string::trim")
}

/// The string::trim function removes leading and trailing whitespace from a string.
/// The macro function is also aliases as `string_trim!`
///
/// # Arguments
/// * `string` - The string to trim. Can be a string literal, a field or a parameter representing a string.
///
/// # Example
/// ```rust
/// use aparecium::{*, functions::string, statements::let_};
/// let trim = string::trim!("Alice Smith");
/// assert_eq!(trim.to_debug_sql(), "string::trim('Alice Smith')");
///
/// let string_field = Field::new("string_field");
/// let trim = string::trim!(string_field);
/// assert_eq!(trim.to_debug_sql(), "string::trim(string_field)");
///
/// let string_param = let_("string_param").equal_to("Alice Smith").get_param();
/// let trim = string::trim!(string_param);
/// assert_eq!(trim.to_debug_sql(), "string::trim($string_param)");
/// ```
#[macro_export]
macro_rules! string_trim {
    ( $string:expr ) => {
        $crate::functions::string::trim_fn($string)
    };
}
pub use string_trim as trim;

/// The string::slug function converts a string into a human and URL-friendly string.
pub fn slug_fn(string: impl Into<StrandLike>) -> Function {
    create_fn_with_single_string_arg(string, "string::slug")
}

/// The string::slug function converts a string into a human and URL-friendly string.
/// The macro function is also aliases as `string_slug!`
///
/// # Arguments
///
/// * `string` - The string to convert into a slug. Can be a string literal, a field or a parameter representing a string.
///
/// # Example
/// ```rust
/// use aparecium::{*, functions::string, statements::let_};
/// let slug = string::slug!("Alice Smith");
/// assert_eq!(slug.to_debug_sql(), "string::slug('Alice Smith')");
/// let string_field = Field::new("string_field");
/// let slug = string::slug!(string_field);
/// assert_eq!(slug.to_debug_sql(), "string::slug(string_field)");
///
/// let string_param = let_("string_param").equal_to("Alice Smith").get_param();
/// let slug = string::slug!(string_param);
/// assert_eq!(slug.to_debug_sql(), "string::slug($string_param)");
/// ```
#[macro_export]
macro_rules! string_slug {
    ( $string:expr ) => {
        $crate::functions::string::slug_fn($string)
    };
}

pub use string_slug as slug;

/// The string::concat function concatenates strings together.
pub fn concat_fn<T: Into<ValueLike>>(values: Vec<T>) -> Function {
    values
        .into_iter()
        .fold(Function::new("string::concat"), |function, value| {
            function.arg(value.into())
        })
}

/// The string::concat function concatenates strings together.
/// The macro function is also aliases as `string_concat!`
///
/// # Arguments
///
/// * `values` - The strings to concatenate. Can be string literals, fields or parameters representing strings.
///
/// # Example
/// ```rust
/// use aparecium::{*, functions::string, statements::let_};
/// let concat = string::concat!("Alice", "Smith");
/// assert_eq!(concat.to_debug_sql(), "string::concat('Alice', 'Smith')");
///
/// let concat = string::concat!(vec!["Alice", "Smith"]);
/// assert_eq!(concat.to_debug_sql(), "string::concat('Alice', 'Smith')");
///
/// let string_field = Field::new("string_field");
/// let concat = string::concat!(string_field, "Smith");
/// assert_eq!(concat.to_debug_sql(), "string::concat(string_field, 'Smith')");
///
/// let string_param = let_("string_param").equal_to("Alice Smith").get_param();
/// let concat = string::concat!(string_param, "Smith");
/// assert_eq!(concat.to_debug_sql(), "string::concat($string_param, 'Smith')");
/// ```
#[macro_export]
macro_rules! string_concat {
        ( $val:expr ) => {
            $crate::functions::string::concat_fn( $val )
        };
        ($( $val:expr ),*) => {
            $crate::functions::string::concat_fn($crate::arr![ $( $val ), * ])
        };
    }

pub use string_concat as concat;

// contains
/// The string::contains function checks whether a string contains another string.
pub fn contains_fn(string: impl Into<StrandLike>, contains: impl Into<StrandLike>) -> Function {
    Function::new("string::contains")
        .arg(string.into())
        .arg(contains.into())
}

/// The string::contains function checks whether a string contains another string.
/// The macro function is also aliases as `string_contains!`
/// # Arguments
/// * `string` - The string to check. Can be a string literal, field or parameter representing a string.
/// * `contains` - The string to check for. Can be a string literal, field or parameter representing a string.
/// # Example
/// ```rust
/// use aparecium::{*, functions::string, statements::let_};
///
/// let contains = string::contains!("Alice Smith", "Smith");
/// assert_eq!(contains.to_debug_sql(), "string::contains('Alice Smith', 'Smith')");
///
/// let string_field = Field::new("string_field");
/// let contains = string::contains!(string_field, "Smith");
/// assert_eq!(contains.to_debug_sql(), "string::contains(string_field, 'Smith')");
///
/// let string_param = let_("string_param").equal_to("Alice Smith").get_param();
/// let contains = string::contains!(string_param, "Smith");
/// assert_eq!(contains.to_debug_sql(), "string::contains($string_param, 'Smith')");
/// ```
#[macro_export]
macro_rules! string_contains {
    ( $string:expr, $contains: expr ) => {
        $crate::functions::string::contains_fn($string, $contains)
    };
}
pub use string_contains as contains;

/// The string::join function joins strings together with a delimiter.
pub fn join_fn(values: impl Into<ArgsList>) -> Function {
    Function::new("string::join").arg(values.into())
}

/// The string::join function joins strings together with a delimiter.
/// The macro function is also aliases as `string_join!`
///
/// # Arguments
///
/// * `values` - The strings to join. Can be string literals, fields or parameters representing strings.
///
/// # Example
/// ```rust
/// use aparecium::{*, functions::string, statements::let_};
/// let join = string::join!("Alice", "Smith");
/// assert_eq!(join.to_debug_sql(), "string::join('Alice', 'Smith')");
///
/// let join = string::join!(vec!["Alice", "Smith"]);
/// assert_eq!(join.to_debug_sql(), "string::join('Alice', 'Smith')");
/// let string_field = Field::new("string_field");
///
/// let join = string::join!(string_field);
/// assert_eq!(join.to_debug_sql(), "string::join(string_field)");
///
/// let string_param = let_("string_param").equal_to("Alice Smith").get_param();
/// let join = string::join!(string_param);
/// assert_eq!(join.to_debug_sql(), "string::join($string_param)");
/// ```
#[macro_export]
macro_rules! string_join {
        ( $val:expr ) => {
            $crate::functions::string::join_fn( $val )
        };
        ($( $val:expr ),*) => {
            $crate::functions::string::join_fn($crate::arr![ $( $val ), * ])
        };
    }

pub use string_join as join;

/// The string::endsWith function checks whether a string ends with another string.
pub fn ends_with_fn(string: impl Into<StrandLike>, ending: impl Into<StrandLike>) -> Function {
    Function::new("string::ends_with")
        .arg(string.into())
        .arg(ending.into())
}

/// The string::endsWith function checks whether a string ends with another string.
/// The macro function is also aliases as `string_ends_with!`
///
/// # Arguments
///
/// * `string` - The string to check. Can be a string literal, field or parameter representing a string.
/// * `ending` - The string to check for. Can be a string literal, field or parameter representing a string.
///
/// # Example
/// ```rust
/// use aparecium::{*, functions::string, statements::let_};
/// let ends_with = string::ends_with!("Alice", "Smith");
/// assert_eq!(ends_with.to_debug_sql(), "string::ends_with('Alice', 'Smith')");
/// let string_field = Field::new("string_field");
/// let ends_with = string::ends_with!(string_field, "Smith");
/// assert_eq!(ends_with.to_debug_sql(), "string::ends_with(string_field, 'Smith')");
/// let string_param = let_("string_param").equal_to("Alice Smith").get_param();
/// let ends_with = string::ends_with!(string_param, "Smith");
/// assert_eq!(ends_with.to_debug_sql(), "string::ends_with($string_param, 'Smith')");
/// ```
#[macro_export]
macro_rules! string_ends_with {
    ( $string:expr, $ending: expr ) => {
        $crate::functions::string::ends_with_fn($string, $ending)
    };
}

pub use string_ends_with as ends_with;

/// The string::startsWith function checks whether a string starts with another string.
pub fn starts_with_fn(string: impl Into<StrandLike>, starting: impl Into<StrandLike>) -> Function {
    Function::new("string::starts_with")
        .arg(string.into())
        .arg(starting.into())
}

/// The string::startsWith function checks whether a string starts with another string.
/// The macro function is also aliases as `string_starts_with!`
///
/// # Arguments
///
/// * `string` - The string to check. Can be a string literal, field or parameter representing a string.
/// * `starting` - The string to check for. Can be a string literal, field or parameter representing a string.
///
/// # Example
/// ```rust
/// use aparecium::{*, functions::string, statements::let_};
/// let starts_with = string::starts_with!("Alice", "Smith");
/// assert_eq!(starts_with.to_debug_sql(), "string::starts_with('Alice', 'Smith')");
///
/// let string_field = Field::new("string_field");
/// let starts_with = string::starts_with!(string_field, "Smith");
/// assert_eq!(starts_with.to_debug_sql(), "string::starts_with(string_field, 'Smith')");
///
/// let string_param = let_("string_param").equal_to("Alice Smith").get_param();
/// let starts_with = string::starts_with!(string_param, "Smith");
/// assert_eq!(starts_with.to_debug_sql(), "string::starts_with($string_param, 'Smith')");
/// ```
#[macro_export]
macro_rules! string_starts_with {
    ( $string:expr, $ending: expr ) => {
        $crate::functions::string::starts_with_fn($string, $ending)
    };
}
pub use string_starts_with as starts_with;

/// The string::split function splits a string by a given delimiter.
pub fn split_fn(string: impl Into<StrandLike>, by: impl Into<StrandLike>) -> Function {
    Function::new("string::split")
        .arg(string.into())
        .arg(by.into())
}

/// The string::split function splits a string by a given delimiter.
/// The macro function is also aliases as `string_split!`
///
/// # Arguments
///
/// * `string` - The string to check. Can be a string literal, field or parameter representing a string.
/// * `by` - The string to check for. Can be a string literal, field or parameter representing a string.
///
/// # Example
/// ```rust
/// use aparecium::{*, functions::string, statements::let_};
/// let split = string::split!("Alice Smith", " ");
/// assert_eq!(split.to_debug_sql(), "string::split('Alice Smith', ' ')");
///
/// let string_field = Field::new("string_field");
/// let split = string::split!(string_field, " ");
/// assert_eq!(split.to_debug_sql(), "string::split(string_field, ' ')");
///
/// let string_param = let_("string_param").equal_to("Alice Smith").get_param();
/// let split = string::split!(string_param, " ");
/// assert_eq!(split.to_debug_sql(), "string::split($string_param, ' ')");
/// ```
#[macro_export]
macro_rules! string_split {
    ( $string:expr, $by: expr ) => {
        $crate::functions::string::split_fn($string, $by)
    };
}

pub use string_split as split;

/// The string::repeat function repeats a string a number of times.
pub fn repeat_fn(string: impl Into<StrandLike>, ending: impl Into<NumberLike>) -> Function {
    Function::new("string::repeat")
        .arg(string.into())
        .arg(ending.into())
}

/// The string::repeat function repeats a string a number of times.
/// The macro function is also aliases as `string_repeat!`
///
/// # Arguments
///
/// * `string` - The string to check. Can be a string literal, field or parameter representing a string.
/// * `count` - The number of times to repeat the string. Can be a number, field or parameter representing a number.
///
/// # Example
/// ```rust
/// use aparecium::{*, functions::string, statements::let_};
/// let repeat = string::repeat!("Alice", 3);
/// assert_eq!(repeat.to_debug_sql(), "string::repeat('Alice', 3)");
/// let string_field = Field::new("string_field");
/// let repeat = string::repeat!(string_field, 3);
///
/// assert_eq!(repeat.to_debug_sql(), "string::repeat(string_field, 3)");
/// let string_param = let_("string_param").equal_to("Alice").get_param();
/// let repeat = string::repeat!(string_param, 3);
/// assert_eq!(repeat.to_debug_sql(), "string::repeat($string_param, 3)");
/// ```
#[macro_export]
macro_rules! string_repeat {
    ( $string:expr, $count: expr ) => {
        $crate::functions::string::repeat_fn($string, $count)
    };
}

pub use string_repeat as repeat;

/// The string::slice function extracts and returns a section of a string.
pub fn slice_fn(
    string: impl Into<StrandLike>,
    from: impl Into<NumberLike>,
    to: impl Into<NumberLike>,
) -> Function {
    Function::new("string::slice")
        .arg(string.into())
        .arg(from.into())
        .arg(to.into())
}

/// The string::slice function extracts and returns a section of a string.
/// The macro function is also aliases as `string_slice!`
///
/// # Arguments
/// * `string` - The string to check. Can be a string literal, field or parameter representing a string.
/// * `from` - The index to start the slice from. Can be a number, field or parameter representing a number.
/// * `to` - The index to end the slice at. Can be a number, field or parameter representing a number.
///
/// # Example
/// ```rust
/// use aparecium::{*, functions::string, statements::let_};
/// let slice = string::slice!("Alice Smith", 0, 7);
/// assert_eq!(slice.to_debug_sql(), "string::slice('Alice Smith', 0, 7)");
///
/// let string_field = Field::new("string_field");
/// let slice = string::slice!(string_field, 0, 7);
/// assert_eq!(slice.to_debug_sql(), "string::slice(string_field, 0, 7)");
///
/// let string_param = let_("string_param").equal_to("Alice Smith").get_param();
/// let slice = string::slice!(string_param, 0, 7);
/// assert_eq!(slice.to_debug_sql(), "string::slice($string_param, 0, 7)");
/// ```
#[macro_export]
macro_rules! string_slice {
    ( $string:expr, $from: expr, $to: expr ) => {
        $crate::functions::string::slice_fn($string, $from, $to)
    };
}

pub use string_slice as slice;

/// The string::replace function replaces an occurence of a string with another string.
pub fn replace_fn(
    string: impl Into<StrandLike>,
    to_match: impl Into<StrandLike>,
    replacement: impl Into<StrandLike>,
) -> Function {
    Function::new("string::replace")
        .arg(string.into())
        .arg(to_match.into())
        .arg(replacement.into())
}

/// The string::replace function replaces an occurence of a string with another string.
/// The macro function is also aliases as `string_replace!`
///
/// # Arguments
///
/// * `string` - The string to check. Can be a string literal, field or parameter representing a string.
/// * `to_match` - The string to match. Can be a string literal, field or parameter representing a string.
/// * `replacement` - The string to replace the matched string with. Can be a string literal, field or parameter representing a string.
///
/// # Example
/// ```rust
/// use aparecium::{*, functions::string, statements::let_};
/// string::replace!("Alice Smith", "Smith", "Alice");
///
/// let string_field = Field::new("string_field");
/// let replace = string::replace!(string_field, "Smith", "Alice");
/// assert_eq!(replace.to_debug_sql(), "string::replace(string_field, 'Smith', 'Alice')");
///
/// let string_param = let_("string_param").equal_to("Alice Smith").get_param();
/// let replace = string::replace!(string_param, "Smith", "Alice");
/// assert_eq!(replace.to_debug_sql(), "string::replace($string_param, 'Smith', 'Alice')");
/// ```
#[macro_export]
macro_rules! string_replace {
    ( $string:expr, $match: expr, $replacement: expr ) => {
        $crate::functions::string::replace_fn($string, $match, $replacement)
    };
}

pub use string_replace as replace;

fn create_string_validation_function(
    value: impl Into<ValueLike>,
    function_name: &'static str,
) -> Function {
    Function::new(function_name).arg(value.into())
}

macro_rules! create_validation_with_tests {
    ($(#[$attr:meta])* => $function_name: expr) => {
        paste::paste! {
            $(#[$attr])*
            pub fn [<$function_name _fn>](value: impl Into<$crate::ValueLike>) -> $crate::Function {
                // Pathed: this module exports its own `concat` macro (`string::concat`).
                super::create_string_validation_function(value, ::core::concat!("string::is::", $function_name))
            }

            $(#[$attr])*
            #[macro_export]
            macro_rules!  [<string_validation_is_ $function_name>]{
                ( $geometry:expr ) => {
                    $crate::functions::string::is::[<$function_name _fn>]($geometry)
                };
            }
            pub use [<string_validation_is_ $function_name>] as [<$function_name>];


        }
    };
}

fn create_two_strings_args_helper(
    str1: impl Into<StrandLike>,
    str2: impl Into<StrandLike>,
    function_name: &'static str,
) -> Function {
    Function::new(function_name)
        .arg(str1.into())
        .arg(str2.into())
}

macro_rules! create_fn_with_two_strings_args {
    ($(#[$attr:meta])* => $function_name:expr, $function_path:expr) => {
        paste::paste! {
            $(#[$attr])*
            pub fn [<$function_name _fn>](str1: impl Into<$crate::StrandLike>, str2: impl Into<$crate::StrandLike>) -> $crate::Function {
                create_two_strings_args_helper(str1, str2, ::core::concat!("string::", $function_path))
            }

            $(#[$attr])*
            #[macro_export]
            macro_rules! [<string_ $function_name>] {
                ( $str1:expr, $str2:expr ) => {
                    $crate::functions::string::[<$function_name _fn>]($str1, $str2)
                };
            }
            pub use [<string_ $function_name>];
            // pub(self) use [<string_ $function_name>] as [<$function_name>];

        }
    };
}

create_fn_with_two_strings_args!(
    /// The string::is::format function checks whether a value matches a format.
    /// Also aliased as `string_is_format!`
    ///
    /// # Arguments
    ///
    /// * `value` - The value to check. Could be a field or a parameter that represents the
    /// value.
    /// * `format` - The format to check against. Could be a field or a parameter that represents the
    /// value.
    ///
    /// # Example
    /// ```rust
    /// use aparecium::{*, functions::string, statements::let_};
    ///
    /// let name = Field::new("name");
    /// let result = string::is::format!(name, "Alice");
    /// assert_eq!(result.to_debug_sql(), "string::is::format(name, 'Alice')");
    ///
    /// let result = string::is::format!("Alice", "Smith");
    /// assert_eq!(result.to_debug_sql(), "string::is::format('Alice', 'Smith')");
    /// ```
    =>
    "is_format",
    "is::format"
);

/// This module contains functions that validate values
pub mod is {
    pub use super::string_is_format as format;

    // The is::alphanum function checks whether a value has only alphanumeric characters.
    create_validation_with_tests!(
        /// The string::is::alphanum function checks whether a value has only alphanumeric characters.
        /// Also aliased as `string::is_alphanum!`
        ///
        /// # Arguments
        ///
        /// * `value` - The value to check. Could be a field or a parameter that represents the
        /// value.
        ///
        /// # Example
        /// ```rust
        /// use aparecium::{*, functions::string, statements::let_};
        ///
        /// let result = string::is::alphanum!("alice1234");
        /// assert_eq!(result.to_debug_sql(), "string::is::alphanum('alice1234')");
        ///
        /// let alphanum_field = Field::new("alphanum_field");
        /// let result = string::is::alphanum!(alphanum_field);
        /// assert_eq!(result.to_debug_sql(), "string::is::alphanum(alphanum_field)");
        ///
        /// let alphanum_param = let_("alphanum_param").equal_to("alice1234").get_param();
        /// let result = string::is::alphanum!(alphanum_param);
        /// assert_eq!(result.to_query().unwrap().sql(), "string::is::alphanum($alphanum_param)");
        /// ```
        =>
        "alphanum"
    );

    create_validation_with_tests!(
        /// The string::is::alpha function checks whether a value has only alpha characters.
        /// Also aliased as `string::is_alpha!`
        ///
        /// # Arguments
        ///
        /// * `value` - The value to check. Could be a field or a parameter that represents the
        /// value.
        ///
        /// # Example
        /// ```rust
        /// use aparecium::{*, functions::string, statements::let_};
        ///
        /// let result = string::is::alpha!("alice");
        /// assert_eq!(result.to_debug_sql(), "string::is::alpha('alice')");
        ///
        /// let alpha_field = Field::new("alpha_field");
        /// let result = string::is::alpha!(alpha_field);
        /// assert_eq!(result.to_debug_sql(), "string::is::alpha(alpha_field)");
        ///
        /// let alpha_param = let_("alpha_param").equal_to("alice").get_param();
        /// let result = string::is::alpha!(alpha_param);
        /// assert_eq!(result.to_query().unwrap().sql(), "string::is::alpha($alpha_param)");
        /// ```
        =>
        "alpha"
    );

    create_validation_with_tests!(
        /// The string::is::ascii function checks whether a value has only ascii characters.
        /// Also aliased as `string_is_ascii!`
        ///
        /// # Arguments
        ///
        /// * `value` - The value to check. Could be a field or a parameter that represents the
        /// value.
        ///
        /// # Example
        /// ```rust
        /// use aparecium::{*, functions::string, statements::let_};
        ///
        /// let result = string::is::ascii!("alice");
        /// assert_eq!(result.to_debug_sql(), "string::is::ascii('alice')");
        ///
        /// let ascii_field = Field::new("ascii_field");
        /// let result = string::is::ascii!(ascii_field);
        /// assert_eq!(result.to_debug_sql(), "string::is::ascii(ascii_field)");
        ///
        /// let ascii_param = let_("ascii_param").equal_to("alice").get_param();
        /// let result = string::is::ascii!(ascii_param);
        /// assert_eq!(result.to_query().unwrap().sql(), "string::is::ascii($ascii_param)");
        /// ```
        =>
        "ascii"
    );

    create_validation_with_tests!(
        /// The string::is::domain function checks whether a value is a domain.
        /// Also aliased as `string_is_domain!`
        ///
        /// # Arguments
        ///
        /// * `value` - The value to check. Could be a field or a parameter that represents the
        /// value.
        ///
        /// # Example
        /// ```rust
        /// use aparecium::{*, functions::string, statements::let_};
        ///
        /// let result = string::is::domain!("example.com");
        /// assert_eq!(result.to_debug_sql(), "string::is::domain('example.com')");
        ///
        /// let domain_field = Field::new("domain_field");
        /// let result = string::is::domain!(domain_field);
        /// assert_eq!(result.to_debug_sql(), "string::is::domain(domain_field)");
        ///
        /// let domain_param = let_("domain_param").equal_to("example.com").get_param();
        /// let result = string::is::domain!(domain_param);
        /// assert_eq!(result.to_query().unwrap().sql(), "string::is::domain($domain_param)");
        /// ```
        =>
        "domain"
    );

    create_validation_with_tests!(
        /// The string::is::email function checks whether a value is an email.
        /// Also aliased as `string_is_email!`
        ///
        /// # Arguments
        ///
        /// * `value` - The value to check. Could be a field or a parameter that represents the
        /// value.
        ///
        /// # Example
        /// ```rust
        /// use aparecium::{*, functions::string, statements::let_};
        ///
        /// let result = string::is::email!("alice@example.com");
        /// assert_eq!(result.to_debug_sql(), "string::is::email('alice@example.com')");
        ///
        /// let email_field = Field::new("email_field");
        /// let result = string::is::email!(email_field);
        /// assert_eq!(result.to_debug_sql(), "string::is::email(email_field)");
        ///
        /// let email_param = let_("email_param").equal_to("alice@codebreather").get_param();
        ///
        /// let result = string::is::email!(email_param);
        /// assert_eq!(result.to_query().unwrap().sql(), "string::is::email($email_param)");
        /// ```
        =>
        "email"
    );

    create_validation_with_tests!(
            /// The string::is::hexadecimal function checks whether a value is hexadecimal.
            /// Also aliased as `string_is_hexadecimal!`
            ///
            /// # Arguments
            ///
            /// * `value` - The value to check. Could be a field or a parameter that represents the
            /// value.
            ///
            /// # Example
            /// ```rust
            /// use aparecium::{*, functions::string, statements::let_};
            ///
            /// let result = string::is::hexadecimal!("alice");
            /// assert_eq!(result.to_debug_sql(), "string::is::hexadecimal('alice')");
            ///
            /// let hexadecimal_field = Field::new("hexadecimal_field");
            /// let result = string::is::hexadecimal!(hexadecimal_field);
            /// assert_eq!(result.to_debug_sql(), "string::is::hexadecimal(hexadecimal_field)");
            ///
            /// let hexadecimal_param = let_("hexadecimal_param").equal_to("alice").get_param();
            /// let result = string::is::hexadecimal!(hexadecimal_param);
            /// assert_eq!(result.to_query().unwrap().sql(), "string::is::hexadecimal($hexadecimal_param)");
            /// ```
            =>
            "hexadecimal"
    );

    create_validation_with_tests!(
        /// The string::is::latitude function checks whether a value is a latitude value.
        /// Also aliased as `string_is_latitude!`
        ///
        /// # Arguments
        ///
        /// * `value` - The value to check. Could be a field or a parameter that represents the
        /// value.
        ///
        /// # Example
        /// ```rust
        /// use aparecium::{*, functions::string, statements::let_};
        ///
        /// let result = string::is::latitude!("-0.118092");
        /// assert_eq!(result.to_debug_sql(), "string::is::latitude('-0.118092')");
        ///
        /// let latitude_field = Field::new("latitude_field");
        /// let result = string::is::latitude!(latitude_field);
        /// assert_eq!(result.to_debug_sql(), "string::is::latitude(latitude_field)");
        ///
        /// let latitude_param = let_("latitude_param").equal_to("alice").get_param();
        /// let result = string::is::latitude!(latitude_param);
        /// assert_eq!(result.to_query().unwrap().sql(), "string::is::latitude($latitude_param)");
        /// ```
        =>
        "latitude"
    );

    create_validation_with_tests!(
        /// The string::is::longitude function checks whether a value is a longitude value.
        /// Also aliased as `string_is_longitude!`
        ///
        /// # Arguments
        ///
        /// * `value` - The value to check. Could be a field or a parameter that represents the
        /// value.
        ///
        /// # Example
        /// ```rust
        /// use aparecium::{*, functions::string, statements::let_};
        ///
        /// let result = string::is::longitude!("51.509865");
        /// assert_eq!(result.to_debug_sql(), "string::is::longitude('51.509865')");
        ///
        /// let longitude_field = Field::new("longitude_field");
        /// let result = string::is::longitude!(longitude_field);
        /// assert_eq!(result.to_debug_sql(), "string::is::longitude(longitude_field)");
        ///
        /// let longitude_param = let_("longitude_param").equal_to("alice").get_param();
        /// let result = string::is::longitude!(longitude_param);
        /// assert_eq!(result.to_query().unwrap().sql(), "string::is::longitude($longitude_param)");
        /// ```
        =>
        "longitude"
    );

    create_validation_with_tests!(
        /// The string::is::numeric function checks whether a value has only numeric characters.
        /// Also aliased as `string_is_numeric!`
        ///
        /// # Arguments
        ///
        /// * `value` - The value to check. Could be a field or a parameter that represents the
        /// value.
        ///
        /// # Example
        /// ```rust
        /// use aparecium::{*, functions::string, statements::let_};
        ///
        /// let result = string::is::numeric!("alice");
        /// assert_eq!(result.to_debug_sql(), "string::is::numeric('alice')");
        ///
        /// let numeric_field = Field::new("numeric_field");
        /// let result = string::is::numeric!(numeric_field);
        /// assert_eq!(result.to_debug_sql(), "string::is::numeric(numeric_field)");
        ///
        /// let numeric_param = let_("numeric_param").equal_to("alice").get_param();
        /// let result = string::is::numeric!(numeric_param);
        /// assert_eq!(result.to_query().unwrap().sql(), "string::is::numeric($numeric_param)");
        /// ```
        =>
        "numeric"
    );

    create_validation_with_tests!(
        /// The string::is::semver function checks whether a value matches a semver version.
        /// Also aliased as `string_is_semver!`
        ///
        /// # Arguments
        ///
        /// * `value` - The value to check. Could be a field or a parameter that represents the
        /// value.
        ///
        /// # Example
        /// ```rust
        /// use aparecium::{*, functions::string, statements::let_};
        ///
        /// let result = string::is::semver!("alice");
        /// assert_eq!(result.to_debug_sql(), "string::is::semver('alice')");
        ///
        /// let semver_field = Field::new("semver_field");
        /// let result = string::is::semver!(semver_field);
        /// assert_eq!(result.to_debug_sql(), "string::is::semver(semver_field)");
        ///
        /// let semver_param = let_("semver_param").equal_to("alice").get_param();
        /// let result = string::is::semver!(semver_param);
        /// assert_eq!(result.to_query().unwrap().sql(), "string::is::semver($semver_param)");
        /// ```
        =>
        "semver"
    );

    create_validation_with_tests!(
        /// The string::is::uuid function checks whether a value is a UUID.
        /// Also aliased as `string_is_uuid!`
        ///
        /// # Arguments
        ///
        /// * `value` - The value to check. Could be a field or a parameter that represents the
        /// value.
        ///
        /// # Example
        /// ```rust
        /// use aparecium::{*, functions::string, statements::let_};
        ///
        /// let result = string::is::uuid!("alice");
        /// assert_eq!(result.to_debug_sql(), "string::is::uuid('alice')");
        ///
        /// let uuid_field = Field::new("uuid_field");
        /// let result = string::is::uuid!(uuid_field);
        /// assert_eq!(result.to_debug_sql(), "string::is::uuid(uuid_field)");
        ///
        /// let uuid_param = let_("uuid_param").equal_to("alice").get_param();
        /// let result = string::is::uuid!(uuid_param);
        /// assert_eq!(result.to_query().unwrap().sql(), "string::is::uuid($uuid_param)");
        /// ```
        =>
        "uuid"
    );

    create_validation_with_tests!(
        /// The string::is::datetime function checks whether a value matches a datetime format.
        /// Also aliased as `string_is_datetime!`
        ///
        /// # Arguments
        ///
        /// * `value` - The value to check. Could be a field or a parameter that represents the
        /// value.
        ///
        /// # Example
        /// ```rust
        /// use aparecium::{*, functions::string, statements::let_};
        ///
        /// let result = string::is::datetime!("alice");
        /// assert_eq!(result.to_debug_sql(), "string::is::datetime('alice')");
        ///
        /// let datetime_field = Field::new("datetime_field");
        /// let result = string::is::datetime!(datetime_field);
        /// assert_eq!(result.to_debug_sql(), "string::is::datetime(datetime_field)");
        ///
        /// let datetime_param = let_("datetime_param").equal_to("alice").get_param();
        /// let result = string::is::datetime!(datetime_param);
        /// assert_eq!(result.to_query().unwrap().sql(), "string::is::datetime($datetime_param)");
        /// ```
        =>
        "datetime"
    );
}

create_fn_with_two_strings_args!(
    /// The string::distance::hamming function calculates the hamming distance between two strings.
    /// Also aliased as `string_distance_hamming!`
    ///
    /// # Arguments
    ///
    /// * `str1` - The first string to compare. Could be a field or a parameter that represents the
    /// value.
    /// * `str2` - The second string to compare. Could be a field or a parameter that represents the
    /// value.
    ///
    /// # Example
    /// ```rust
    /// use aparecium::{*, functions::string, statements::let_};
    ///
    /// let name = Field::new("name");
    /// let result = string::distance::hamming!(name, "Alice");
    /// assert_eq!(result.to_debug_sql(), "string::distance::hamming(name, 'Alice')");
    ///
    /// let result = string::distance::hamming!("Alice", "Smith");
    /// assert_eq!(result.to_debug_sql(), "string::distance::hamming('Alice', 'Smith')");
    /// ```
    =>
    "distance_hamming",
    "distance::hamming"
);

create_fn_with_two_strings_args!(
    /// The string::distance::levenshtein function calculates the levenshtein distance between two strings.
    /// Also aliased as `string_distance_levenshtein!`
    ///
    /// # Arguments
    ///
    /// * `str1` - The first string to compare. Could be a field or a parameter that represents the
    /// value.
    /// * `str2` - The second string to compare. Could be a field or a parameter that represents the
    /// value.
    ///
    /// # Example
    /// ```rust
    /// use aparecium::{*, functions::string, statements::let_};
    ///
    /// let name = Field::new("name");
    /// let result = string::distance::levenshtein!(name, "Alice");
    /// assert_eq!(result.to_debug_sql(), "string::distance::levenshtein(name, 'Alice')");
    ///
    /// let result = string::distance::levenshtein!("Alice", "Smith");
    /// assert_eq!(result.to_debug_sql(), "string::distance::levenshtein('Alice', 'Smith')");
    /// ```
    =>
    "distance_levenshtein",
    "distance::levenshtein"
);

/// The string::distance::levenshtein function calculates the damerau levenshtein distance between two strings.
pub mod distance {
    pub use super::string_distance_hamming as hamming;
    pub use super::string_distance_levenshtein as levenshtein;
}

create_fn_with_two_strings_args!(
    /// The string::similarity::fuzzy function calculates the fuzzy similarity between two strings.
    /// Also aliased as `string_similarity_fuzzy!`
    ///
    /// # Arguments
    ///
    /// * `str1` - The first string to compare. Could be a field or a parameter that represents the
    /// value.
    /// * `str2` - The second string to compare. Could be a field or a parameter that represents the
    /// value.
    ///
    /// # Example
    /// ```rust
    /// use aparecium::{*, functions::string, statements::let_};
    ///
    /// let name = Field::new("name");
    /// let result = string::similarity::fuzzy!(name, "Alice");
    /// assert_eq!(result.to_debug_sql(), "string::similarity::fuzzy(name, 'Alice')");
    ///
    /// let result = string::similarity::fuzzy!("Alice", "Smith");
    /// assert_eq!(result.to_debug_sql(), "string::similarity::fuzzy('Alice', 'Smith')");
    /// ```
    =>
    "similarity_fuzzy",
    "similarity::fuzzy"
);

create_fn_with_two_strings_args!(
    /// The string::similarity::jaro function calculates the jaro similarity between two strings.
    /// Also aliased as `string_similarity_jaro!`
    ///
    /// # Arguments
    ///
    /// * `str1` - The first string to compare. Could be a field or a parameter that represents the
    /// value.
    /// * `str2` - The second string to compare. Could be a field or a parameter that represents the
    /// value.
    ///
    /// # Example
    /// ```rust
    /// use aparecium::{*, functions::string, statements::let_};
    ///
    /// let name = Field::new("name");
    /// let result = string::similarity::jaro!(name, "Alice");
    /// assert_eq!(result.to_debug_sql(), "string::similarity::jaro(name, 'Alice')");
    ///
    /// let result = string::similarity::jaro!("Alice", "Smith");
    /// assert_eq!(result.to_debug_sql(), "string::similarity::jaro('Alice', 'Smith')");
    /// ```
    =>
    "similarity_jaro",
    "similarity::jaro"
);

create_fn_with_two_strings_args!(
    /// The string::similarity::smithwaterman function calculates the smithwaterman similarity between two strings.
    /// Also aliased as `string_similarity_smithwaterman!`
    ///
    /// # Arguments
    ///
    /// * `str1` - The first string to compare. Could be a field or a parameter that represents the
    /// value.
    /// * `str2` - The second string to compare. Could be a field or a parameter that represents the
    /// value.
    ///
    /// # Example
    /// ```rust
    /// use aparecium::{*, functions::string, statements::let_};
    ///
    /// let name = Field::new("name");
    /// let result = string::similarity::smithwaterman!(name, "Alice");
    /// assert_eq!(result.to_debug_sql(), "string::similarity::smithwaterman(name, 'Alice')");
    ///
    /// let result = string::similarity::smithwaterman!("Alice", "Smith");
    /// assert_eq!(result.to_debug_sql(), "string::similarity::smithwaterman('Alice', 'Smith')");
    /// ```
    =>
    "similarity_smithwaterman",
    "similarity::smithwaterman"
);

/// The string::similarity module contains functions that calculate the similarity between two strings.
pub mod similarity {
    pub use super::string_similarity_fuzzy as fuzzy;
    pub use super::string_similarity_jaro as jaro;
    pub use super::string_similarity_smithwaterman as smithwaterman;
}

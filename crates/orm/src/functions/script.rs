// Embedded scripting functions
// SurrealDB allows for advanced functions with complicated logic, by allowing embedded functions to be written in JavaScript. These functions support the ES2020 JavaScript specification.
//
// Simple functions
// Embedded JavaScript functions within SurrealDB support all functionality in the ES2020 specification including async / await functions, and generator functions. Any value from SurrealDB is converted into a JavaScript type automatically, and the return value from the JavaScript function is converted to a SurrealDB value.
//
// CREATE person SET scores = function() {
// 	return [1,2,3].map(v => v * 10);
// };
//

use crate::{Function, Param};

/// SurrealDB allows for advanced functions with complicated logic, by allowing embedded functions to be written in JavaScript. These functions support the ES2020 JavaScript specification.
///
/// [See more](https://surrealdb.com/docs/functions/script)
pub fn function_fn<T: Into<Param>>(args: Vec<T>, jscode_body: impl Into<String>) -> Function {
    args.into_iter()
        .fold(Function::new("function"), |function, arg| {
            function.arg(arg.into())
        })
        .with_body(jscode_body.into())
}

/// Creates a new function
///
/// # Arguments
/// * `args` - A list of arguments to pass into the function
/// * `jscode_body` - The body of the function
///
/// # Example
/// ```rust
/// use umbriel::{*, functions::function, statements::let_};
/// // Simple functions
/// // Embedded JavaScript functions within SurrealDB support all functionality in the ES2020 specification including async / await functions, and generator functions. Any value from SurrealDB is converted into a JavaScript type automatically, and the return value from the JavaScript function is converted to a SurrealDB value.
/// // A `{ … }` body is written as the compiler stringifies its tokens, so its spacing may differ
/// // from the source; pass a string literal (below) to keep the text exactly.
/// let f2 = function!((), {
///    return [1, 2, 3].map(v => v * 10);
/// });
/// let sql = f2.to_debug_sql();
/// assert!(sql.starts_with("function() {") && sql.contains("[1, 2, 3].map(v => v * 10)"));
///
/// // Function arguments
/// // Additional arguments can be passed in to the function from SurrealDB, and these are accessible using the arguments object within the JavaScript function.
/// // Create a new parameter
/// let value = let_("value").equal_to("SurrealDB").get_param();
/// let words = let_("words").equal_to(vec!["awesome", "advanced", "cool"]).get_param();
/// // Pass the parameter values into the function
/// let f3 = function!((value, words), "{
///   return `${arguments[0]} is ${arguments[1]}`;
/// }");
/// assert_eq!(f3.to_debug_sql(), "function($value, $words) {
///   return `${arguments[0]} is ${arguments[1]}`;
/// }");
///
/// let name = Param::new("name");
/// let id = Param::new("id");
/// let f5 = function!(
///     (name, id),
///     "{ return [1, 2, 3].map(v => v * 10) ; }"
/// );
/// assert_eq!(f5.to_debug_sql(), "function($name, $id) { return [1, 2, 3].map(v => v * 10) ; }");
/// ```
/// [See more](https://surrealdb.com/docs/functions/script)
#[macro_export]
macro_rules! function {
    (($($arg:expr),*), {$($code:tt)*}) => {
        $crate::functions::function_fn(vec![$($arg),*] as ::std::vec::Vec<$crate::Param>, stringify!({$($code)*}))
    };
    (($($arg:expr),*), $($code:tt)*) => {
        $crate::functions::function_fn(vec![$($arg),*] as ::std::vec::Vec<$crate::Param>, concat!($($code)*))
    };
}
pub use function;

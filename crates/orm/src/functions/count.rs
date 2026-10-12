
use crate::{Function, ValueLike};
// Count functions
// These functions can be used when counting field values and expressions.

/// count()
/// Counts a row, or whether a given value is truthy
/// count
/// The count function counts the number of times that the function is called. This is useful for returning the total number of rows in a SELECT statement with a GROUP BY clause.
///
/// count() -> 1
/// If a value is given as the first argument, then this function checks whether a given value is truthy. This is useful for returning the total number of rows, which match a certain condition, in a SELECT statement, with a GROUP BY clause.
/// count(value) -> number
/// If an array is given, this function counts the number of items in the array which are truthy. If, instead, you want to count the total number of items in the given array, then use the array::len() function.
///
/// count(array) -> number
/// The following examples show this function, and its output, when used in a select statement:
pub fn count_fn(countable: impl Into<ValueLike>) -> Function {
    let countable: ValueLike = countable.into();
    let function = Function::new("count");
    // `count()` takes no argument when counting rows.
    if countable.is_empty() {
        function
    } else {
        function.arg(countable)
    }
}

/// Counts a row, or whether a given value is truthy
/// `count() -> 1`
/// If a value is given as the first argument, then this function checks whether a given value is truthy. This is useful for returning the total number of rows, which match a certain condition, in a SELECT statement, with a GROUP BY clause.
/// `count(value) -> number`
///
/// # Arguments
/// * `countable` - The value to count. Can be a field, a param, an operation, a filter, an array or a mixed array
///
/// # Examples
/// ```rust
/// # use umbriel::{*, functions::count, statements::let_};
/// count!();
///
/// # let email = Field::new("email");
/// count!(email.greater_than(15));
///
/// # let email = Field::new("email");
/// count!(cond(email.greater_than(15)).and(email.less_than(20)));
///
/// # let email = Field::new("email");
/// # let age = Field::new("age");
/// # let student_count = AliasName::new("student_count");
/// count!(cond(age.greater_than(15)).and(email.like("charlie@example.com")))
///     .__as__(student_count);
///
/// count!(1);
///
/// # let head_count = AliasName::new("head_count");
/// count!().__as__(head_count);
///
/// # let email_field = Field::new("email_field");
/// count!(email_field);
///
/// let email_param = let_("email_param").equal_to("charlie@example.com").get_param();
/// count!(email_param);
#[macro_export]
macro_rules! count {
    ( $countable:expr ) => {
        $crate::functions::count_fn($countable)
    };
    () => {
        $crate::functions::count_fn($crate::Empty)
    };
}
pub use count;

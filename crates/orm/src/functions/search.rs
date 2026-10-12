// These functions are used in conjunction with the 'matches' operator to either collect the relevance score or highlight the searched keywords within the content.
//
// Function	Description
// search::score()	Returns the relevance score
// search::highlight()	Highlights the matching keywords
// search::offsets()	Returns the position of the matching keywords

use crate::{Function, NumberLike, StrandLike};

fn create_single_search_arg_helper(
    search_arg: impl Into<StrandLike>,
    function_name: &'static str,
) -> Function {
    Function::new(function_name).arg(search_arg.into())
}

macro_rules! create_fn_with_single_search_arg {
    ($(#[$attr:meta])* => $function_name:expr, $function_path:expr) => {
        paste::paste! {
            $(#[$attr])*
            pub fn [<$function_name _fn>](search_arg: impl ::std::convert::Into<$crate::StrandLike>) -> $crate::Function {
                create_single_search_arg_helper(search_arg, concat!("search::", $function_path))
            }

            $(#[$attr])*
            #[macro_export]
            macro_rules! [<search_ $function_name>] {
                ( $search_arg:expr ) => {
                    $crate::functions::search::[<$function_name _fn>]($search_arg)
                };
            }
            pub use [<search_ $function_name>] as [<$function_name>];

        }
    };
}

create_fn_with_single_search_arg!(
    /// The search::score function returns the relevance score corresponding to the given 'matches' predicate reference numbers.
    ///
    /// # Arguments
    /// * `search_arg` - The search argument to compute the score of. Could be a field or a parameter that represents the value.
    ///
    /// # Example
    /// ```rust
    /// use umbriel::{*, functions::search};
    ///
    /// let title = Field::new("title");
    /// let result = search::score!(title);
    /// assert_eq!(result.to_debug_sql(), "search::score(title)");
    /// ```
    =>
    "score",
    "score"
);

create_fn_with_single_search_arg!(
    /// The search::offsets function returns the position of the matching keywords for the predicate reference number.
    ///
    /// # Arguments
    /// * `search_arg` - The search argument to find offsets for. Could be a field or a parameter that represents the value.
    ///
    /// # Example
    /// ```rust
    /// use umbriel::{*, functions::search};
    ///
    /// let title = Field::new("title");
    /// let result = search::offsets!(title);
    /// assert_eq!(result.to_debug_sql(), "search::offsets(title)");
    /// ```
    =>
    "offsets",
    "offsets"
);

/// The search_highlight_fn function highlights the matching keywords for the predicate reference number.
pub fn highlight_fn(
    str1: impl Into<StrandLike>,
    str2: impl Into<StrandLike>,
    predicate_ref_number: impl Into<NumberLike>,
) -> Function {
    Function::new("search::highlight")
        .arg(str1.into())
        .arg(str2.into())
        .arg(predicate_ref_number.into())
}

/// The search::highlight function highlights the matching keywords for the predicate reference number.
///
/// # Arguments
///
/// * `str1` - The string to use as the start of the highlight.
/// * `str2` - The string to use as the end of the highlight.
/// * `predicate_ref_number` - The predicate reference number to highlight.
///   All arguments could be a field or a parameter that represents the value.
///
/// # Example
/// ```rust
/// use umbriel::{*, functions::search};
/// let result = search::highlight!("<b>", "</b>", 1);
/// assert_eq!(result.to_debug_sql(), "search::highlight('<b>', '</b>', 1)");
/// ```
#[macro_export]
macro_rules! search_highlight {
    ( $str1:expr, $str2:expr, $predicate_ref_number:expr ) => {
        $crate::functions::search::highlight_fn($str1, $str2, $predicate_ref_number)
    };
}
pub use search_highlight as highlight;

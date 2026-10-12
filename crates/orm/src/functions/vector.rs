
// Vector functions
// A collection of essential vector operations that provide foundational functionality for numerical computation, machine learning, and data analysis. These operations include distance measurements, similarity coefficients, and other basic and complex operations related to vectors. Through understanding and implementing these functions, we can perform a wide variety of tasks ranging from data processing to advanced statistical analyses.
//
// Function	Description
// vector::add()	Performs element-wise addition of two vectors
// vector::angle()	Computes the angle between two vectors
// vector::cross()	Computes the cross product of two vectors
// vector::divide()	Performs element-wise division between two vectors
// vector::dot()	Computes the dot product of two vectors
// vector::magnitude()	Computes the magnitude (or length) of a vector
// vector::multiply()	Performs element-wise multiplication of two vectors
// vector::normalize()	Computes the normalization of a vector
// vector::project()	Computes the projection of one vector onto another
// vector::subtract()	Performs element-wise subtraction between two vectors
// vector::distance::chebyshev()	Computes the Chebyshev distance
// vector::distance::euclidean()	Computes the Euclidean distance between two vectors
// vector::distance::hamming()	Computes the Hamming distance between two vectors
// vector::distance::manhattan()	Computes the Manhattan distance between two vectors
// vector::distance::minkowski()	Computes the Minkowski distance between two vectors
// vector::similarity::cosine()	Computes the Cosine similarity between two vectors
// vector::similarity::jaccard()	Computes the Jaccard similarity between two vectors
// vector::similarity::pearson()	Computes the Pearson correlation coefficient between two vectors
// vector::add
// The vector::add function performs element-wise addition of two vectors, where each element in the first vector is added to the corresponding element in the second vector.

use crate::{ArrayLike, Function, NumberLike};

fn create_single_vector_arg_helper(
    vector: impl Into<ArrayLike>,
    function_name: &'static str,
) -> Function {
    Function::new(function_name).arg(vector.into())
}

macro_rules! create_fn_with_single_vector_arg {
    ($(#[$attr:meta])* => $function_name:expr, $function_path:expr) => {
        paste::paste! {
            $(#[$attr])*
            pub fn [<$function_name _fn>](vector: impl Into<$crate::ArrayLike>) -> $crate::Function {
                create_single_vector_arg_helper(vector, concat!("vector::", $function_path))
            }

            $(#[$attr])*
            #[macro_export]
            macro_rules! [<vector_ $function_name>] {
                ( $vector:expr ) => {
                    $crate::functions::vector::[<$function_name _fn>]($vector)
                };
            }
            pub use [<vector_ $function_name>] as [<$function_name>];

        }
    };
}

fn create_two_vectors_args_helper(
    vector1: impl Into<ArrayLike>,
    vector2: impl Into<ArrayLike>,
    function_name: &'static str,
) -> Function {
    Function::new(function_name)
        .arg(vector1.into())
        .arg(vector2.into())
}

macro_rules! create_fn_with_two_vectors_args {
    ($(#[$attr:meta])* => $function_name:expr, $function_path:expr) => {
        paste::paste! {
            $(#[$attr])*
            pub fn [<$function_name _fn>](vector1: impl Into<$crate::ArrayLike>, vector2: impl Into<$crate::ArrayLike>) -> $crate::Function {
                create_two_vectors_args_helper(vector1, vector2, concat!("vector::", $function_path))
            }

            $(#[$attr])*
            #[macro_export]
            macro_rules! [<vector_ $function_name>] {
                ( $vector1:expr, $vector2:expr ) => {
                    $crate::functions::vector::[<$function_name _fn>]($vector1, $vector2)
                };
            }
            // pub use [<vector_ $function_name>] as [<$function_name>];
            pub use [<vector_ $function_name>];

        }
    };
}

create_fn_with_single_vector_arg!(
    /// The vector::magnitude function computes the magnitude (or length) of a vector, providing a measure of the size of the vector in multi-dimensional space.
    ///
    /// # Arguments
    ///
    /// * `vector` - The vector to compute the magnitude of. Could be a field or a parameter that represents the
    /// value.
    ///
    /// # Example
    /// ```rust
    /// use umbriel::{*, functions::vector, statements::let_};
    ///
    /// let name = Field::new("name");
    /// let result = vector::magnitude!(name);
    /// assert_eq!(result.to_debug_sql(), "vector::magnitude(name)");
    ///
    /// let result = vector::magnitude!([1, 2, 3]);
    /// assert_eq!(result.to_debug_sql(), "vector::magnitude([1, 2, 3])");
    /// ```
    =>
    "magnitude",
    "magnitude"
);

create_fn_with_single_vector_arg!(
    /// The vector::normalize function computes the normalization of a vector, transforming it to a unit vector (a vector of length 1) that maintains the original direction.
    ///
    /// # Arguments
    ///
    /// * `vector` - The vector to compute the normalization of. Could be a field or a parameter that represents the
    /// value.
    ///
    /// # Example
    /// ```rust
    /// use umbriel::{*, functions::vector, statements::let_};
    ///
    /// let name = Field::new("name");
    /// let result = vector::normalize!(name);
    /// assert_eq!(result.to_debug_sql(), "vector::normalize(name)");
    ///
    /// let result = vector::normalize!([1, 2, 3]);
    /// assert_eq!(result.to_debug_sql(), "vector::normalize([1, 2, 3])");
    /// ```
    =>
    "normalize",
    "normalize"
);

create_fn_with_single_vector_arg!(
    /// The vector::project function computes the projection of one vector onto another.
    ///
    /// # Arguments
    ///
    /// * `vector` - The vector to compute the projection of. Could be a field or a parameter that represents the
    /// value.
    ///
    /// # Example
    /// ```rust
    /// use umbriel::{*, functions::vector, statements::let_};
    ///
    /// let name = Field::new("name");
    /// let result = vector::project!(name);
    /// assert_eq!(result.to_debug_sql(), "vector::project(name)");
    ///
    /// let result = vector::project!([1, 2, 3]);
    /// assert_eq!(result.to_debug_sql(), "vector::project([1, 2, 3])");
    /// ```
    =>
    "project",
    "project"
);

create_fn_with_two_vectors_args!(
    /// The vector::cross function computes the cross product of two vectors, which results in a vector that is orthogonal (perpendicular) to the plane containing the original vectors.
    ///
    /// # Arguments
    ///
    /// * `vector1` - The first vector to compute the cross product of. Could be a field or a parameter that represents the
    /// value.
    /// * `vector2` - The second vector to compute the cross product of. Could be a field or a parameter that represents the
    /// value.
    ///
    /// # Example
    /// ```rust
    /// use umbriel::{*, functions::vector, statements::let_};
    ///
    /// let arr1 = Field::new("arr1");
    /// let arr2 = Field::new("arr2");
    /// let result = vector::cross!(arr1, arr2);
    /// assert_eq!(result.to_debug_sql(), "vector::cross(arr1, arr2)");
    ///
    /// let result = vector::cross!([1, 2, 3], [1, 2, 3]);
    /// assert_eq!(result.to_debug_sql(), "vector::cross([1, 2, 3], [1, 2, 3])");
    /// ```
    =>
    "cross",
    "cross"
);
pub use vector_cross as cross;

create_fn_with_two_vectors_args!(
    /// The vector::dot function computes the dot product of two vectors, which is the sum of the products of the corresponding entries of the two sequences of numbers.
    ///
    /// # Arguments
    ///
    /// * `vector1` - The first vector to compute the dot product of. Could be a field or a parameter that represents the
    /// value.
    /// * `vector2` - The second vector to compute the dot product of. Could be a field or a parameter that represents the
    /// value.
    ///
    /// # Example
    /// ```rust
    /// use umbriel::{*, functions::vector, statements::let_};
    ///
    /// let arr1 = Field::new("arr1");
    /// let arr2 = Field::new("arr2");
    /// let result = vector::dot!(arr1, arr2);
    /// assert_eq!(result.to_debug_sql(), "vector::dot(arr1, arr2)");
    ///
    /// let result = vector::dot!([1, 2, 3], [1, 2, 3]);
    /// assert_eq!(result.to_debug_sql(), "vector::dot([1, 2, 3], [1, 2, 3])");
    /// ```
    =>
    "dot",
    "dot"
);
pub use vector_dot as dot;

create_fn_with_two_vectors_args!(
    /// The vector::add function performs element-wise addition of two vectors, where each element in the first vector is added to the corresponding element in the second vector.
    ///
    /// # Arguments
    ///
    /// * `vector1` - The first vector to compute the element-wise addition of. Could be a field or a parameter that represents the
    /// value.
    /// * `vector2` - The second vector to compute the element-wise addition of. Could be a field or a parameter that represents the
    /// value.
    ///
    /// # Example
    /// ```rust
    /// use umbriel::{*, functions::vector, statements::let_};
    ///
    /// let arr1 = Field::new("arr1");
    /// let arr2 = Field::new("arr2");
    /// let result = vector::add!(arr1, arr2);
    /// assert_eq!(result.to_debug_sql(), "vector::add(arr1, arr2)");
    ///
    /// let result = vector::add!([1, 2, 3], [1, 2, 3]);
    /// assert_eq!(result.to_debug_sql(), "vector::add([1, 2, 3], [1, 2, 3])");
    /// ```
    =>
    "add",
    "add"
);
pub use vector_add as add;

create_fn_with_two_vectors_args!(
    /// The vector::subtract function performs element-wise subtraction between two vectors, where each element in the second vector is subtracted from the corresponding element in the first vector.
    ///
    /// # Arguments
    ///
    /// * `vector1` - The first vector to compute the element-wise subtraction of. Could be a field or a parameter that represents the
    /// value.
    /// * `vector2` - The second vector to compute the element-wise subtraction of. Could be a field or a parameter that represents the
    /// value.
    ///
    /// # Example
    /// ```rust
    /// use umbriel::{*, functions::vector, statements::let_};
    ///
    /// let arr1 = Field::new("arr1");
    /// let arr2 = Field::new("arr2");
    /// let result = vector::subtract!(arr1, arr2);
    /// assert_eq!(result.to_debug_sql(), "vector::subtract(arr1, arr2)");
    ///
    /// let result = vector::subtract!([1, 2, 3], [1, 2, 3]);
    /// assert_eq!(result.to_debug_sql(), "vector::subtract([1, 2, 3], [1, 2, 3])");
    /// ```
    =>
    "subtract",
    "subtract"
);
pub use vector_subtract as subtract;

create_fn_with_two_vectors_args!(
    /// The vector::multiply function performs element-wise multiplication of two vectors, where each element in the first vector is multiplied by the corresponding element in the second vector.
    ///
    /// # Arguments
    ///
    /// * `vector1` - The first vector to compute the element-wise multiplication of. Could be a field or a parameter that represents the
    /// value.
    /// * `vector2` - The second vector to compute the element-wise multiplication of. Could be a field or a parameter that represents the
    /// value.
    ///
    /// # Example
    /// ```rust
    /// use umbriel::{*, functions::vector, statements::let_};
    ///
    /// let arr1 = Field::new("arr1");
    /// let arr2 = Field::new("arr2");
    /// let result = vector::multiply!(arr1, arr2);
    /// assert_eq!(result.to_debug_sql(), "vector::multiply(arr1, arr2)");
    ///
    /// let result = vector::multiply!([1, 2, 3], [1, 2, 3]);
    /// assert_eq!(result.to_debug_sql(), "vector::multiply([1, 2, 3], [1, 2, 3])");
    /// ```
    =>
    "multiply",
    "multiply"
);
pub use vector_multiply as multiply;

create_fn_with_two_vectors_args!(
    /// The vector::divide function performs element-wise division between two vectors, where each element in the first vector is divided by the corresponding element in the second vector.
    ///
    /// # Arguments
    ///
    /// * `vector1` - The first vector to compute the element-wise division of. Could be a field or a parameter that represents the
    /// value.
    /// * `vector2` - The second vector to compute the element-wise division of. Could be a field or a parameter that represents the
    /// value.
    ///
    /// # Example
    /// ```rust
    /// use umbriel::{*, functions::vector, statements::let_};
    ///
    /// let arr1 = Field::new("arr1");
    /// let arr2 = Field::new("arr2");
    /// let result = vector::divide!(arr1, arr2);
    /// assert_eq!(result.to_debug_sql(), "vector::divide(arr1, arr2)");
    ///
    /// let result = vector::divide!([1, 2, 3], [1, 2, 3]);
    /// assert_eq!(result.to_debug_sql(), "vector::divide([1, 2, 3], [1, 2, 3])");
    /// ```
    =>
    "divide",
    "divide"
);
pub use vector_divide as divide;

create_fn_with_two_vectors_args!(
    /// The vector::angle function computes the angle between two vectors, providing a measure of the orientation difference between them.
    ///
    /// # Arguments
    ///
    /// * `vector1` - The first vector to compute the angle of. Could be a field or a parameter that represents the
    /// value.
    /// * `vector2` - The second vector to compute the angle of. Could be a field or a parameter that represents the
    /// value.
    ///
    /// # Example
    /// ```rust
    /// use umbriel::{*, functions::{vector, math}, statements::let_};
    ///
    /// let arr1 = Field::new("arr1");
    /// let arr2 = Field::new("arr2");
    /// let result = vector::angle!(arr1, arr2);
    /// assert_eq!(result.to_debug_sql(), "vector::angle(arr1, arr2)");
    ///
    /// let result = vector::angle!([1, 2, 3], [1, 2, 3]);
    /// assert_eq!(result.to_debug_sql(), "vector::angle([1, 2, 3], [1, 2, 3])");
    /// ```
    =>
    "angle",
    "angle"
);
pub use vector_angle as angle;

create_fn_with_two_vectors_args!(
    /// The vector::distance::chebyshev function computes the Chebyshev distance (also known as maximum value distance) between two vectors, which is the greatest of their differences along any coordinate dimension.
    ///
    /// # Arguments
    ///
    /// * `vector1` - The first vector to compute the Chebyshev distance of. Could be a field or a parameter that represents the
    /// value.
    /// * `vector2` - The second vector to compute the Chebyshev distance of. Could be a field or a parameter that represents the
    /// value.
    ///
    /// # Example
    /// ```rust
    /// use umbriel::{
    ///     *,
    ///     functions::{vector, math},
    ///     statements::let_,
    /// };
    ///
    /// let arr1 = Field::new("arr1");
    /// let arr2 = Field::new("arr2");
    /// let result = vector::distance::chebyshev!(arr1, arr2);
    /// assert_eq!(result.to_debug_sql(), "vector::distance::chebyshev(arr1, arr2)");
    ///
    /// let result = vector::distance::chebyshev!([1, 2, 3], [1, 2, 3]);
    /// assert_eq!(result.to_debug_sql(), "vector::distance::chebyshev([1, 2, 3], [1, 2, 3])");
    /// ```
    =>
    "distance_chebyshev",
    "distance::chebyshev"
);

create_fn_with_two_vectors_args!(
    /// The vector::distance::euclidean function computes the Euclidean distance between two vectors, providing a measure of the straight-line distance between two points in a multi-dimensional space.
    ///
    /// # Arguments
    ///
    /// * `vector1` - The first vector to compute the Euclidean distance of. Could be a field or a parameter that represents the
    /// value.
    /// * `vector2` - The second vector to compute the Euclidean distance of. Could be a field or a parameter that represents the
    /// value.
    ///
    /// # Example
    /// ```rust
    /// use umbriel::{
    ///     *,
    ///     functions::{vector, math},
    ///     statements::let_,
    /// };
    ///
    /// let arr1 = Field::new("arr1");
    /// let arr2 = Field::new("arr2");
    /// let result = vector::distance::euclidean!(arr1, arr2);
    /// assert_eq!(result.to_debug_sql(), "vector::distance::euclidean(arr1, arr2)");
    ///
    /// let result = vector::distance::euclidean!([1, 2, 3], [1, 2, 3]);
    /// assert_eq!(result.to_debug_sql(), "vector::distance::euclidean([1, 2, 3], [1, 2, 3])");
    /// ```
    =>
    "distance_euclidean",
    "distance::euclidean"
);

create_fn_with_two_vectors_args!(
    /// The vector::distance::hamming function computes the Hamming distance between two vectors, measuring the minimum number of substitutions required to change one vector into the other, useful for comparing strings or codes.
    ///
    /// # Arguments
    ///
    /// * `vector1` - The first vector to compute the Hamming distance of. Could be a field or a parameter that represents the
    /// value.
    /// * `vector2` - The second vector to compute the Hamming distance of. Could be a field or a parameter that represents the
    /// value.
    ///
    /// # Example
    /// ```rust
    /// use umbriel::{
    ///     *,
    ///     functions::{vector, math},
    ///     statements::let_,
    /// };
    ///
    /// let arr1 = Field::new("arr1");
    /// let arr2 = Field::new("arr2");
    /// let result = vector::distance::hamming!(arr1, arr2);
    /// assert_eq!(result.to_debug_sql(), "vector::distance::hamming(arr1, arr2)");
    ///
    /// let result = vector::distance::hamming!([1, 2, 3], [1, 2, 3]);
    /// assert_eq!(result.to_debug_sql(), "vector::distance::hamming([1, 2, 3], [1, 2, 3])");
    /// ```
    =>
    "distance_hamming",
    "distance::hamming"
);

create_fn_with_two_vectors_args!(
    /// The vector::distance::manhattan function computes the Manhattan distance (also known as the L1 norm or Taxicab geometry) between two vectors, which is the sum of the absolute differences of their corresponding elements.
    ///
    /// # Arguments
    ///
    /// * `vector1` - The first vector to compute the Manhattan distance of. Could be a field or a parameter that represents the
    /// value.
    /// * `vector2` - The second vector to compute the Manhattan distance of. Could be a field or a parameter that represents the
    /// value.
    ///
    /// # Example
    /// ```rust
    /// use umbriel::{
    ///     *,
    ///     functions::{vector, math},
    ///     statements::let_,
    /// };
    ///
    /// let arr1 = Field::new("arr1");
    /// let arr2 = Field::new("arr2");
    /// let result = vector::distance::manhattan!(arr1, arr2);
    /// assert_eq!(result.to_debug_sql(), "vector::distance::manhattan(arr1, arr2)");
    ///
    /// let result = vector::distance::manhattan!([1, 2, 3], [1, 2, 3]);
    /// assert_eq!(result.to_debug_sql(), "vector::distance::manhattan([1, 2, 3], [1, 2, 3])");
    /// ```
    =>
    "distance_manhattan",
    "distance::manhattan"
);

/// The vector distance functions compute the distance between two vectors, providing a measure of the difference between them. The distance functions are useful for comparing vectors of different lengths, such as strings or codes.
pub mod distance {
    pub use super::vector_distance_chebyshev as chebyshev;
    pub use super::vector_distance_euclidean as euclidean;
    pub use super::vector_distance_hamming as hamming;
    pub use super::vector_distance_manhattan as manhattan;
    pub use super::vector_distance_minkowski as minkowski;
}

create_fn_with_two_vectors_args!(
    /// The vector::similarity::cosine function computes the Cosine similarity between two vectors, indicating the cosine of the angle between them, which is a measure of how closely two vectors are oriented to each other.
    ///
    /// # Arguments
    ///
    /// * `vector1` - The first vector to compute the Cosine similarity of. Could be a field or a parameter that represents the
    /// value.
    /// * `vector2` - The second vector to compute the Cosine similarity of. Could be a field or a parameter that represents the
    /// value.
    /// * `p` - The p parameter to use for the Cosine similarity. Could be a field or a parameter that represents the
    /// value.
    ///
    /// # Example
    /// ```rust
    /// use umbriel::{
    ///     *,
    ///     functions::{vector, math},
    ///     statements::let_,
    /// };
    ///
    /// let arr1 = Field::new("arr1");
    /// let arr2 = Field::new("arr2");
    /// let result = vector::similarity::cosine!(arr1, arr2);
    /// assert_eq!(result.to_debug_sql(), "vector::similarity::cosine(arr1, arr2)");
    ///
    /// let result = vector::similarity::cosine!([1, 2, 3], [1, 2, 3]);
    /// assert_eq!(result.to_debug_sql(), "vector::similarity::cosine([1, 2, 3], [1, 2, 3])");
    /// ```
    =>
    "similarity_cosine",
    "similarity::cosine"
);

create_fn_with_two_vectors_args!(
    /// The vector::similarity::jaccard function computes the Jaccard similarity between two vectors, which is a measure of how similar two vectors are to each other, where 0 indicates orthogonality and 1 indicates that the vectors are identical.
    ///
    /// # Arguments
    ///
    /// * `vector1` - The first vector to compute the Jaccard similarity of. Could be a field or a parameter that represents the
    /// value.
    /// * `vector2` - The second vector to compute the Jaccard similarity of. Could be a field or a parameter that represents the
    /// value.
    /// * `p` - The p parameter to use for the Jaccard similarity. Could be a field or a parameter that represents the
    /// value.
    ///
    /// # Example
    /// ```rust
    /// use umbriel::{
    ///     *,
    ///     functions::vector,
    ///     statements::let_,
    /// };
    ///
    /// let arr1 = Field::new("arr1");
    /// let arr2 = Field::new("arr2");
    /// let result = vector::similarity::jaccard!(arr1, arr2);
    /// assert_eq!(result.to_debug_sql(), "vector::similarity::jaccard(arr1, arr2)");
    ///
    /// let result = vector::similarity::jaccard!(vec![1, 2, 3], vec![1, 2, 3]);
    /// assert_eq!(result.to_debug_sql(), "vector::similarity::jaccard([1, 2, 3], [1, 2, 3])");
    /// ```
    =>
    "similarity_jaccard",
    "similarity::jaccard"
);

/// The vector similarity functions compute the similarity between two vectors, providing a measure of how similar they are to each other. The similarity functions are useful for comparing vectors of different lengths, such as strings or codes.
pub mod similarity {
    pub use super::vector_similarity_cosine as cosine;
    pub use super::vector_similarity_jaccard as jaccard;
}

/// The vector::distance::minkowski function computes the Minkowski distance between two vectors, a generalization of other distance metrics such as Euclidean and Manhattan when parameterized with different values of p.
pub fn distance_minkowski_fn(
    vector1: impl Into<ArrayLike>,
    vector2: impl Into<ArrayLike>,
    p: impl Into<NumberLike>,
) -> Function {
    Function::new("vector::distance::minkowski")
        .arg(vector1.into())
        .arg(vector2.into())
        .arg(p.into())
}

/// The vector::distance::minkowski function computes the Minkowski distance between two vectors, a generalization of other distance metrics such as Euclidean and Manhattan when parameterized with different values of p.
/// ```rust
/// use umbriel::{*, functions::vector};
///
/// let arr1 = Field::new("arr1");
/// let arr2 = Field::new("arr2");
/// let result = vector::distance::minkowski!(arr1, arr2, 1);
/// assert_eq!(result.to_debug_sql(), "vector::distance::minkowski(arr1, arr2, 1)");
/// ```
#[macro_export]
macro_rules! vector_distance_minkowski {
    ( $vector1:expr, $vector2:expr, $p:expr ) => {
        $crate::functions::vector::distance_minkowski_fn($vector1, $vector2, $p)
    };
}
pub use vector_distance_minkowski;

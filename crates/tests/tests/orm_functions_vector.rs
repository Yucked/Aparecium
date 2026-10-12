#[cfg(test)]
mod vector_distance_minkowski_tests {
    use umbriel::{functions::vector, *};

    #[test]
    fn test_vector_distance_minkowski_fn_on_vector_macro_on_diverse_vectors() {
        let arr1 = Field::new("arr1");
        let arr2 = Field::new("arr2");
        let result = vector::distance_minkowski_fn(arr1, arr2, 1);
        assert_eq!(
            result.fine_tune_params(),
            "vector::distance::minkowski(arr1, arr2, $_param_00000001)"
        );
        assert_eq!(
            result.to_raw().build(),
            "vector::distance::minkowski(arr1, arr2, 1)"
        );
    }

    #[test]
    fn test_vector_distance_minkowski_fn_on_same_element_types() {
        let result = vector::distance_minkowski_fn(vec![1, 2, 3], vec![1, 2, 3], 1);
        assert_eq!(
            result.fine_tune_params(),
            "vector::distance::minkowski($_param_00000001, $_param_00000002, $_param_00000003)"
        );

        assert_eq!(
            result.to_raw().build(),
            "vector::distance::minkowski([1, 2, 3], [1, 2, 3], 1)"
        );
    }

    #[test]
    fn test_vector_distance_minkowski_macro_on_strand_macro_on_diverse_vectors() {
        let field = Field::new("field");
        let param = Param::new("param");
        let result = vector::distance::minkowski!(field, param, 1);
        assert_eq!(
            result.fine_tune_params(),
            "vector::distance::minkowski(field, $param, $_param_00000001)"
        );
        assert_eq!(
            result.to_raw().build(),
            "vector::distance::minkowski(field, $param, 1)"
        );
    }

    #[test]
    fn test_vector_distance_minkowski_macro_on_same_element_types() {
        let result = vector::distance::minkowski!(vec![1, 2, 3], vec![1, 2, 3], 1);
        assert_eq!(
            result.fine_tune_params(),
            "vector::distance::minkowski($_param_00000001, $_param_00000002, $_param_00000003)"
        );

        assert_eq!(
            result.to_raw().build(),
            "vector::distance::minkowski([1, 2, 3], [1, 2, 3], 1)"
        );
    }
}

macro_rules! create_fn_with_single_vector_arg_tests {
    ($(#[$attr:meta])* => $function_name:expr, $function_path:expr) => {
        paste::paste! {
            mod [<test_ $function_name>] {
                use umbriel::{functions::vector, *};

                #[test]
                fn [<test $function_name fn_on_vector_macro_on_diverse_vectors>]() {
                    let name = Field::new("name");
                    let result = functions::vector::[<$function_name _fn>](name);
                    assert_eq!(
                        result.fine_tune_params(),
                        format!("vector::{}(name)", $function_path)
                    );
                    assert_eq!(
                        result.to_raw().build(),
                        format!("vector::{}(name)", $function_path)
                    );
                }

                #[test]
                fn [<test $function_name _fn_on_same_element_types>]() {
                    let result = vector::[<$function_name _fn>](vec![1, 2, 3]);
                    assert_eq!(
                        result.fine_tune_params(),
                        format!("vector::{}($_param_00000001)", $function_path)
                    );

                    assert_eq!(
                        result.to_raw().build(),
                        format!("vector::{}([1, 2, 3])", $function_path)
                    );
                }

                #[test]
                fn [<test $function_name _macro_on_vector_macro_on_diverse_vectors>]() {
                    let name = Field::new("name");
                    let result = vector::[<$function_name>]!(name);
                    assert_eq!(
                        result.fine_tune_params(),
                        format!("vector::{}(name)", $function_path)
                    );
                    assert_eq!(
                        result.to_raw
                        ().build(),
                        format!("vector::{}(name)", $function_path)
                    );
                }

                #[test]
                fn [<test $function_name _macro_on_same_element_types_with_arr_vec>]() {
                    let result = vector::[<$function_name>]!(arr![1, 2, 3]);
                    assert_eq!(
                        result.fine_tune_params(),
                        format!("vector::{}([$_param_00000001, $_param_00000002, $_param_00000003])", $function_path)
                    );

                    assert_eq!(
                        result.to_raw().build(),
                        format!("vector::{}([1, 2, 3])", $function_path)
                    );
                }
                #[test]
                fn [<test $function_name _macro_on_same_element_types>]() {
                    let result = vector::[<$function_name>]!(vec![1, 2, 3]);
                    assert_eq!(
                        result.fine_tune_params(),
                        format!("vector::{}($_param_00000001)", $function_path)
                    );

                    assert_eq!(
                        result.to_raw().build(),
                        format!("vector::{}([1, 2, 3])", $function_path)
                    );
                }
            }
        }
    };
}

create_fn_with_single_vector_arg_tests!(
    =>
    "magnitude",
    "magnitude"
);
create_fn_with_single_vector_arg_tests!(
    =>
    "normalize",
    "normalize"
);
create_fn_with_single_vector_arg_tests!(
    =>
    "project",
    "project"
);

macro_rules! create_fn_with_two_vectors_args_tests {
    ($(#[$attr:meta])* => $function_name:expr, $function_path:expr) => {
        paste::paste! {
            mod [<test_ $function_name>] {
                use umbriel::[<vector_ $function_name>] as [<$function_name>];

                use umbriel::{functions::vector, *};

                #[test]
                fn [<test $function_name fn_on_vector_macro_on_diverse_vectors>]() {
                    let arr1 = Field::new("arr1");
                    let arr2 = Field::new("arr2");
                    let result = functions::vector::[<$function_name _fn>](arr1, arr2);
                    assert_eq!(
                        result.fine_tune_params(),
                        format!("vector::{}(arr1, arr2)", $function_path)
                    );
                    assert_eq!(
                        result.to_raw().build(),
                        format!("vector::{}(arr1, arr2)", $function_path)
                    );
                }

                #[test]
                fn [<test $function_name _fn_on_same_element_types>]() {
                    let result = vector::[<$function_name _fn>](vec![1, 2, 3], vec![1, 2, 3]);
                    assert_eq!(
                        result.fine_tune_params(),
                        format!("vector::{}($_param_00000001, $_param_00000002)", $function_path)
                    );

                    assert_eq!(
                        result.to_raw().build(),
                        format!("vector::{}([1, 2, 3], [1, 2, 3])", $function_path)
                    );
                }

                #[test]
                fn [<test $function_name _
                macro_on_strand_macro_on_diverse_vectors>]() {
                    let field = Field::new("field");
                    let param = Param::new("param");
                    let result = self::[<$function_name>]!(field, param);
                    assert_eq!(
                        result.fine_tune_params(),
                        format!("vector::{}(field, $param)", $function_path)
                    );
                    assert_eq!(
                        result.to_raw().build(),
                        format!("vector::{}(field, $param)", $function_path)
                    );
                }

                #[test]
                fn [<test $function_name _macro_on_same_element_types>]() {
                    let result = self::[<$function_name>]!(vec![1, 2, 3], vec![1, 2, 3]);
                    assert_eq!(
                        result.fine_tune_params(),
                        format!("vector::{}($_param_00000001, $_param_00000002)", $function_path)
                    );

                    assert_eq!(
                        result.to_raw().build(),
                        format!("vector::{}([1, 2, 3], [1, 2, 3])", $function_path)
                    );
                }
            }
        }
    };
}

create_fn_with_two_vectors_args_tests!(
    =>
    "cross",
    "cross"
);
create_fn_with_two_vectors_args_tests!(
    =>
    "dot",
    "dot"
);
create_fn_with_two_vectors_args_tests!(
    =>
    "add",
    "add"
);
create_fn_with_two_vectors_args_tests!(
    =>
    "subtract",
    "subtract"
);
create_fn_with_two_vectors_args_tests!(
    =>
    "multiply",
    "multiply"
);
create_fn_with_two_vectors_args_tests!(
    =>
    "divide",
    "divide"
);
create_fn_with_two_vectors_args_tests!(
    =>
    "angle",
    "angle"
);
create_fn_with_two_vectors_args_tests!(
    =>
    "distance_chebyshev",
    "distance::chebyshev"
);
create_fn_with_two_vectors_args_tests!(
    =>
    "distance_euclidean",
    "distance::euclidean"
);
create_fn_with_two_vectors_args_tests!(
    =>
    "distance_hamming",
    "distance::hamming"
);
create_fn_with_two_vectors_args_tests!(
    =>
    "distance_manhattan",
    "distance::manhattan"
);
create_fn_with_two_vectors_args_tests!(
    =>
    "similarity_cosine",
    "similarity::cosine"
);
create_fn_with_two_vectors_args_tests!(
    =>
    "similarity_jaccard",
    "similarity::jaccard"
);

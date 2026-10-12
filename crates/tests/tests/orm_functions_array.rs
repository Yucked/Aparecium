#[cfg(test)]
mod add_tests {
    use umbriel::{functions::array, *};

    #[test]
    fn test_add_with_values() {
        let result = array::add!(vec![1, 2, 3], 4);

        assert_eq!(result.get_bindings().len(), 2);
        assert_eq!(
            result.fine_tune_params(),
            "array::add($_param_00000001, $_param_00000002)"
        );
        assert_eq!(result.to_raw().build(), "array::add([1, 2, 3], 4)");
    }

    #[test]
    fn test_add_with_field_and_param() {
        let numbers = Field::new("numbers");
        let value = Param::new("value");
        let result = array::add!(numbers, value);

        assert_eq!(result.get_bindings().len(), 0);
        assert_eq!(result.fine_tune_params(), "array::add(numbers, $value)");
        assert_eq!(result.to_raw().build(), "array::add(numbers, $value)");
    }
}

#[cfg(test)]
mod all_tests {
    use umbriel::{functions::array, *};

    #[test]
    fn test_all_with_values() {
        let result = array::all!(vec![1, 2, 3, 4, 5]);

        assert_eq!(result.get_bindings().len(), 1);
        assert_eq!(result.fine_tune_params(), "array::all($_param_00000001)");
        assert_eq!(result.to_raw().build(), "array::all([1, 2, 3, 4, 5])");
    }

    #[test]
    fn test_all_with_field() {
        let numbers = Field::new("numbers");
        let result = array::all!(numbers);

        assert_eq!(result.get_bindings().len(), 0);
        assert_eq!(result.fine_tune_params(), "array::all(numbers)");
        assert_eq!(result.to_raw().build(), "array::all(numbers)");
    }
}

#[cfg(test)]
mod any_tests {
    use umbriel::{functions::array, *};

    #[test]
    fn test_any_with_values() {
        let result = array::any!(vec![1, 2, 3, 4, 5]);

        assert_eq!(result.get_bindings().len(), 1);
        assert_eq!(result.fine_tune_params(), "array::any($_param_00000001)");
        assert_eq!(result.to_raw().build(), "array::any([1, 2, 3, 4, 5])");
    }

    #[test]
    fn test_any_with_field() {
        let numbers = Field::new("numbers");
        let result = array::any!(numbers);

        assert_eq!(result.get_bindings().len(), 0);
        assert_eq!(result.fine_tune_params(), "array::any(numbers)");
        assert_eq!(result.to_raw().build(), "array::any(numbers)");
    }
}

#[cfg(test)]
mod append_tests {
    use umbriel::{functions::array, *};

    #[test]
    fn test_append_with_values() {
        let result = array::append!(vec![1, 2, 3, 4, 5], 6);

        assert_eq!(result.get_bindings().len(), 2);
        assert_eq!(
            result.fine_tune_params(),
            "array::append($_param_00000001, $_param_00000002)"
        );
        assert_eq!(result.to_raw().build(), "array::append([1, 2, 3, 4, 5], 6)");
    }

    #[test]
    fn test_append_with_field() {
        let numbers = Field::new("numbers");
        let value = Field::new("value");
        let result = array::append!(numbers, value);

        assert_eq!(result.get_bindings().len(), 0);
        assert_eq!(result.fine_tune_params(), "array::append(numbers, value)");
        assert_eq!(result.to_raw().build(), "array::append(numbers, value)");
    }

    #[test]
    fn test_append_with_param() {
        let numbers = Field::new("numbers");
        let value = Param::new("value");
        let result = array::append!(numbers, value);

        assert_eq!(result.get_bindings().len(), 0);
        assert_eq!(result.fine_tune_params(), "array::append(numbers, $value)");
        assert_eq!(result.to_raw().build(), "array::append(numbers, $value)");
    }
}

#[cfg(test)]
mod array_transpose_test {
    use umbriel::{functions::array, *};

    #[test]
    fn test_array_transpose() {
        let result = array::transpose!(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]);
        assert_eq!(result.get_bindings().len(), 1);
        assert_eq!(
            result.fine_tune_params(),
            "array::transpose($_param_00000001)"
        );
        assert_eq!(
            result.to_raw().build(),
            "array::transpose([[1, 2, 3], [4, 5, 6], [7, 8, 9]])"
        );
    }

    #[test]
    fn test_array_transpose_array() {
        let arr = arr![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];

        let result = array::transpose!(arr);
        assert_eq!(result.get_bindings().len(), 3);
        assert_eq!(
            result.fine_tune_params(),
            "array::transpose([$_param_00000001, $_param_00000002, $_param_00000003])"
        );
        assert_eq!(
            result.to_raw().build(),
            "array::transpose([[1, 2, 3], [4, 5, 6], [7, 8, 9]])"
        );
    }

    #[test]
    fn test_array_transpose_multiple_fields_and_params() {
        let own_goals = Field::new("own_goals");
        let goals = Param::new("goals");

        let result = array::transpose!(arr![own_goals, goals]);
        assert_eq!(result.get_bindings().len(), 0);
        assert_eq!(
            result.fine_tune_params(),
            "array::transpose([own_goals, $goals])"
        );
        assert_eq!(
            result.to_raw().build(),
            "array::transpose([own_goals, $goals])"
        );
    }

    #[test]
    fn test_array_transpose_field() {
        let own_goals = Field::new("own_goals");

        let result = array::transpose!(own_goals);
        assert_eq!(result.get_bindings().len(), 0);
        assert_eq!(result.fine_tune_params(), "array::transpose(own_goals)");
        assert_eq!(result.to_raw().build(), "array::transpose(own_goals)");
    }

    #[test]
    fn test_array_transpose_param() {
        let goals = Param::new("goals");

        let result = array::transpose!(goals);
        assert_eq!(result.get_bindings().len(), 0);
        assert_eq!(result.fine_tune_params(), "array::transpose($goals)");
        assert_eq!(result.to_raw().build(), "array::transpose($goals)");
    }
}

#[cfg(test)]
mod array_at_tests {
    use umbriel::{functions::array, *};

    #[test]
    fn test_array_at() {
        let result = array::at!(vec![1, 2, 3, 4, 5], 2);
        assert_eq!(result.get_bindings().len(), 2);
        assert_eq!(
            result.fine_tune_params(),
            "array::at($_param_00000001, $_param_00000002)"
        );
        assert_eq!(result.to_raw().build(), "array::at([1, 2, 3, 4, 5], 2)");
    }

    #[test]
    fn test_array_at_field() {
        let own_goals = Field::new("own_goals");

        let result = array::at!(own_goals, 2);
        assert_eq!(result.get_bindings().len(), 1);
        assert_eq!(
            result.fine_tune_params(),
            "array::at(own_goals, $_param_00000001)"
        );
        assert_eq!(result.to_raw().build(), "array::at(own_goals, 2)");
    }

    #[test]
    fn test_array_at_param() {
        let goals = Param::new("goals");

        let result = array::at!(goals, 2);
        assert_eq!(result.get_bindings().len(), 1);
        assert_eq!(
            result.fine_tune_params(),
            "array::at($goals, $_param_00000001)"
        );
        assert_eq!(result.to_raw().build(), "array::at($goals, 2)");
    }
}

#[cfg(test)]
mod array_distinct_tests {
    use umbriel::{functions::array, *};

    #[test]
    fn test_array_distinct() {
        let result = array::distinct!(vec![1, 2, 3]);
        assert_eq!(result.get_bindings().len(), 1);
        assert_eq!(
            result.fine_tune_params(),
            "array::distinct($_param_00000001)"
        );
        assert_eq!(result.to_raw().build(), "array::distinct([1, 2, 3])");
    }

    #[test]
    fn test_array_distinct_field() {
        let own_goals = Field::new("own_goals");

        let result = array::distinct!(own_goals);
        assert_eq!(result.get_bindings().len(), 0);
        assert_eq!(result.fine_tune_params(), "array::distinct(own_goals)");
        assert_eq!(result.to_raw().build(), "array::distinct(own_goals)");
    }

    #[test]
    fn test_array_distinct_param() {
        let goals = Param::new("goals");

        let result = array::distinct!(goals);
        assert_eq!(result.get_bindings().len(), 0);
        assert_eq!(result.fine_tune_params(), "array::distinct($goals)");
        assert_eq!(result.to_raw().build(), "array::distinct($goals)");
    }
}

#[cfg(test)]
mod array_find_tests {
    use umbriel::{functions::array, *};

    #[test]
    fn test_array_find() {
        let result = array::find!(vec![1, 2, 3, 4], 2);
        assert_eq!(result.get_bindings().len(), 2);
        assert_eq!(
            result.fine_tune_params(),
            "array::find($_param_00000001, $_param_00000002)"
        );
        assert_eq!(result.to_raw().build(), "array::find([1, 2, 3, 4], 2)");
    }

    #[test]
    fn test_array_find_field() {
        let own_goals = Field::new("own_goals");

        let result = array::find!(own_goals, 2);
        assert_eq!(result.get_bindings().len(), 1);
        assert_eq!(
            result.fine_tune_params(),
            "array::find(own_goals, $_param_00000001)"
        );
        assert_eq!(result.to_raw().build(), "array::find(own_goals, 2)");
    }

    #[test]
    fn test_array_find_param() {
        let goals = Param::new("goals");

        let result = array::find!(goals, 2);
        assert_eq!(result.get_bindings().len(), 1);
        assert_eq!(
            result.fine_tune_params(),
            "array::find($goals, $_param_00000001)"
        );
        assert_eq!(result.to_raw().build(), "array::find($goals, 2)");
    }
}

#[cfg(test)]
mod array_find_index_tests {
    use umbriel::{functions::array, *};

    #[test]
    fn test_array_find_index() {
        let result = array::find_index!(vec![1, 2, 3, 4], 2);
        assert_eq!(result.get_bindings().len(), 2);
        assert_eq!(
            result.fine_tune_params(),
            "array::find_index($_param_00000001, $_param_00000002)"
        );
        assert_eq!(
            result.to_raw().build(),
            "array::find_index([1, 2, 3, 4], 2)"
        );
    }

    #[test]
    fn test_array_find_index_field() {
        let own_goals = Field::new("own_goals");

        let result = array::find_index!(own_goals, 2);
        assert_eq!(result.get_bindings().len(), 1);
        assert_eq!(
            result.fine_tune_params(),
            "array::find_index(own_goals, $_param_00000001)"
        );
        assert_eq!(result.to_raw().build(), "array::find_index(own_goals, 2)");
    }

    #[test]
    fn test_array_find_index_param() {
        let goals = Param::new("goals");

        let result = array::find_index!(goals, 2);
        assert_eq!(result.get_bindings().len(), 1);
        assert_eq!(
            result.fine_tune_params(),
            "array::find_index($goals, $_param_00000001)"
        );
        assert_eq!(result.to_raw().build(), "array::find_index($goals, 2)");
    }
}

#[cfg(test)]
mod array_flatten_tests {
    use umbriel::{functions::array, *};

    #[test]
    fn test_array_flatten() {
        let result = array::flatten!(array![vec![1, 2], vec![3, 4], "SurrealDB", vec![5, 6]]);
        assert_eq!(result.get_bindings().len(), 1);
        assert_eq!(
            result.fine_tune_params(),
            "array::flatten($_param_00000001)"
        );
        assert_eq!(
            result.to_raw().build(),
            "array::flatten([[1, 2], [3, 4], 'SurrealDB', [5, 6]])"
        );
    }

    #[test]
    fn test_array_flatten_field() {
        let own_goals = Field::new("own_goals");

        let result = array::flatten!(own_goals);
        assert_eq!(result.get_bindings().len(), 0);
        assert_eq!(result.fine_tune_params(), "array::flatten(own_goals)");
        assert_eq!(result.to_raw().build(), "array::flatten(own_goals)");
    }

    #[test]
    fn test_array_flatten_param() {
        let goals = Param::new("goals");

        let result = array::flatten!(goals);
        assert_eq!(result.get_bindings().len(), 0);
        assert_eq!(result.fine_tune_params(), "array::flatten($goals)");
        assert_eq!(result.to_raw().build(), "array::flatten($goals)");
    }
}

#[cfg(test)]
mod array_group_tests {
    use umbriel::{functions::array, *};

    #[test]
    fn test_array_group() {
        let result = array::group!(array![
            1,
            2,
            3,
            4,
            array![3, 5, 6],
            vec![2, 4, 5, 6],
            7,
            8,
            8,
            9
        ]);
        assert_eq!(result.get_bindings().len(), 1);
        assert_eq!(result.fine_tune_params(), "array::group($_param_00000001)");
        assert_eq!(
            result.to_raw().build(),
            "array::group([1, 2, 3, 4, [3, 5, 6], [2, 4, 5, 6], 7, 8, 8, 9])"
        );
    }

    #[test]
    fn test_array_group_field() {
        let own_goals = Field::new("own_goals");

        let result = array::group!(own_goals);
        assert_eq!(result.get_bindings().len(), 0);
        assert_eq!(result.fine_tune_params(), "array::group(own_goals)");
        assert_eq!(result.to_raw().build(), "array::group(own_goals)");
    }

    #[test]
    fn test_array_group_param() {
        let goals = Param::new("goals");

        let result = array::group!(goals);
        assert_eq!(result.get_bindings().len(), 0);
        assert_eq!(result.fine_tune_params(), "array::group($goals)");
        assert_eq!(result.to_raw().build(), "array::group($goals)");
    }
}

#[cfg(test)]
mod array_insert_tests {
    use umbriel::{functions::array, *};

    #[test]
    fn test_array_insert() {
        let result = array::insert!(array![1, 2, 3, 4], 5, 2);
        assert_eq!(result.get_bindings().len(), 3);
        assert_eq!(
            result.fine_tune_params(),
            "array::insert($_param_00000001, $_param_00000002, $_param_00000003)"
        );
        assert_eq!(result.to_raw().build(), "array::insert([1, 2, 3, 4], 5, 2)");
    }

    #[test]
    fn test_array_insert_field() {
        let own_goals = Field::new("own_goals");

        let result = array::insert!(own_goals, 5, 2);
        assert_eq!(result.get_bindings().len(), 2);
        assert_eq!(
            result.fine_tune_params(),
            "array::insert(own_goals, $_param_00000001, $_param_00000002)"
        );
        assert_eq!(result.to_raw().build(), "array::insert(own_goals, 5, 2)");
    }

    #[test]
    fn test_array_insert_param() {
        let goals = Param::new("goals");

        let result = array::insert!(goals, 5, 2);
        assert_eq!(result.get_bindings().len(), 2);
        assert_eq!(
            result.fine_tune_params(),
            "array::insert($goals, $_param_00000001, $_param_00000002)"
        );
        assert_eq!(result.to_raw().build(), "array::insert($goals, 5, 2)");
    }
}

#[cfg(test)]
mod array_len_tests {
    use umbriel::{functions::array, *};

    #[test]
    fn test_array_len() {
        let result = array::len!(vec![1, 2, 3]);
        assert_eq!(result.get_bindings().len(), 1);
        assert_eq!(result.fine_tune_params(), "array::len($_param_00000001)");
        assert_eq!(result.to_raw().build(), "array::len([1, 2, 3])");
    }

    #[test]
    fn test_array_len_field() {
        let own_goals = Field::new("own_goals");

        let result = array::len!(own_goals);
        assert_eq!(result.get_bindings().len(), 0);
        assert_eq!(result.fine_tune_params(), "array::len(own_goals)");
        assert_eq!(result.to_raw().build(), "array::len(own_goals)");
    }

    #[test]
    fn test_array_len_param() {
        let goals = Param::new("goals");

        let result = array::len!(goals);
        assert_eq!(result.get_bindings().len(), 0);
        assert_eq!(result.fine_tune_params(), "array::len($goals)");
        assert_eq!(result.to_raw().build(), "array::len($goals)");
    }
}

#[cfg(test)]
mod array_pop_tests {
    use umbriel::{functions::array, *};

    #[test]
    fn test_array_pop() {
        let result = array::pop!(vec![1, 2, 3]);
        assert_eq!(result.get_bindings().len(), 1);
        assert_eq!(result.fine_tune_params(), "array::pop($_param_00000001)");
        assert_eq!(result.to_raw().build(), "array::pop([1, 2, 3])");
    }

    #[test]
    fn test_array_pop_field() {
        let own_goals = Field::new("own_goals");

        let result = array::pop!(own_goals);
        assert_eq!(result.get_bindings().len(), 0);
        assert_eq!(result.fine_tune_params(), "array::pop(own_goals)");
        assert_eq!(result.to_raw().build(), "array::pop(own_goals)");
    }

    #[test]
    fn test_array_pop_param() {
        let goals = Param::new("goals");

        let result = array::pop!(goals);
        assert_eq!(result.get_bindings().len(), 0);
        assert_eq!(result.fine_tune_params(), "array::pop($goals)");
        assert_eq!(result.to_raw().build(), "array::pop($goals)");
    }
}

#[cfg(test)]
mod array_prepend_tests {
    use umbriel::{functions::array, *};

    #[test]
    fn test_array_prepend() {
        let result = array::prepend!(vec![1, 2, 3], 4);
        assert_eq!(result.get_bindings().len(), 2);
        assert_eq!(
            result.fine_tune_params(),
            "array::prepend($_param_00000001, $_param_00000002)"
        );
        assert_eq!(result.to_raw().build(), "array::prepend([1, 2, 3], 4)");
    }

    #[test]
    fn test_array_prepend_field() {
        let own_goals = Field::new("own_goals");

        let result = array::prepend!(own_goals, 4);
        assert_eq!(result.get_bindings().len(), 1);
        assert_eq!(
            result.fine_tune_params(),
            "array::prepend(own_goals, $_param_00000001)"
        );
        assert_eq!(result.to_raw().build(), "array::prepend(own_goals, 4)");
    }

    #[test]
    fn test_array_prepend_param() {
        let goals = Param::new("goals");

        let result = array::prepend!(goals, 4);
        assert_eq!(result.get_bindings().len(), 1);
        assert_eq!(
            result.fine_tune_params(),
            "array::prepend($goals, $_param_00000001)"
        );
        assert_eq!(result.to_raw().build(), "array::prepend($goals, 4)");
    }
}

#[cfg(test)]
mod array_push_tests {
    use umbriel::{functions::array, *};

    #[test]
    fn test_array_push() {
        let result = array::push!(vec![1, 2, 3], 4);
        assert_eq!(result.get_bindings().len(), 2);
        assert_eq!(
            result.fine_tune_params(),
            "array::push($_param_00000001, $_param_00000002)"
        );
        assert_eq!(result.to_raw().build(), "array::push([1, 2, 3], 4)");
    }

    #[test]
    fn test_array_push_field() {
        let own_goals = Field::new("own_goals");

        let result = array::push!(own_goals, 4);
        assert_eq!(result.get_bindings().len(), 1);
        assert_eq!(
            result.fine_tune_params(),
            "array::push(own_goals, $_param_00000001)"
        );
        assert_eq!(result.to_raw().build(), "array::push(own_goals, 4)");
    }

    #[test]
    fn test_array_push_param() {
        let goals = Param::new("goals");

        let result = array::push!(goals, 4);
        assert_eq!(result.get_bindings().len(), 1);
        assert_eq!(
            result.fine_tune_params(),
            "array::push($goals, $_param_00000001)"
        );
        assert_eq!(result.to_raw().build(), "array::push($goals, 4)");
    }
}

#[cfg(test)]
mod array_remove_tests {
    use umbriel::{functions::array, *};

    #[test]
    fn test_array_remove() {
        let result = array::remove!(vec![1, 2, 3, 4, 5], 2);
        assert_eq!(result.get_bindings().len(), 2);
        assert_eq!(
            result.fine_tune_params(),
            "array::remove($_param_00000001, $_param_00000002)"
        );
        assert_eq!(result.to_raw().build(), "array::remove([1, 2, 3, 4, 5], 2)");
    }

    #[test]
    fn test_array_remove_field() {
        let own_goals = Field::new("own_goals");

        let result = array::remove!(own_goals, 2);
        assert_eq!(result.get_bindings().len(), 1);
        assert_eq!(
            result.fine_tune_params(),
            "array::remove(own_goals, $_param_00000001)"
        );
        assert_eq!(result.to_raw().build(), "array::remove(own_goals, 2)");
    }

    #[test]
    fn test_array_remove_param() {
        let goals = Param::new("goals");

        let result = array::remove!(goals, 2);
        assert_eq!(result.get_bindings().len(), 1);
        assert_eq!(
            result.fine_tune_params(),
            "array::remove($goals, $_param_00000001)"
        );
        assert_eq!(result.to_raw().build(), "array::remove($goals, 2)");
    }
}

#[cfg(test)]
mod array_reverse_tests {
    use umbriel::{functions::array, *};

    #[test]
    fn test_array_reverse() {
        let result = array::reverse!(vec![1, 2, 3, 4, 5]);
        assert_eq!(result.get_bindings().len(), 1);
        assert_eq!(
            result.fine_tune_params(),
            "array::reverse($_param_00000001)"
        );
        assert_eq!(result.to_raw().build(), "array::reverse([1, 2, 3, 4, 5])");
    }

    #[test]
    fn test_array_reverse_field() {
        let own_goals = Field::new("own_goals");

        let result = array::reverse!(own_goals);
        assert_eq!(result.get_bindings().len(), 0);
        assert_eq!(result.fine_tune_params(), "array::reverse(own_goals)");
        assert_eq!(result.to_raw().build(), "array::reverse(own_goals)");
    }

    #[test]
    fn test_array_reverse_param() {
        let goals = Param::new("goals");

        let result = array::reverse!(goals);
        assert_eq!(result.get_bindings().len(), 0);
        assert_eq!(result.fine_tune_params(), "array::reverse($goals)");
        assert_eq!(result.to_raw().build(), "array::reverse($goals)");
    }
}

#[cfg(test)]
mod array_join_tests {
    use umbriel::{functions::array, *};

    #[test]
    fn test_array_join() {
        let result = array::join!(vec![1, 2, 3, 4, 5], ",");
        assert_eq!(result.get_bindings().len(), 2);
        assert_eq!(
            result.fine_tune_params(),
            "array::join($_param_00000001, $_param_00000002)"
        );
        assert_eq!(result.to_raw().build(), "array::join([1, 2, 3, 4, 5], ',')");
    }

    #[test]
    fn test_array_join_field() {
        let own_goals = Field::new("own_goals");

        let result = array::join!(own_goals, ",");
        assert_eq!(result.get_bindings().len(), 1);
        assert_eq!(
            result.fine_tune_params(),
            "array::join(own_goals, $_param_00000001)"
        );
        assert_eq!(result.to_raw().build(), "array::join(own_goals, ',')");
    }

    #[test]
    fn test_array_join_param() {
        let goals = Param::new("goals");

        let result = array::join!(goals, ",");
        assert_eq!(result.get_bindings().len(), 1);
        assert_eq!(
            result.fine_tune_params(),
            "array::join($goals, $_param_00000001)"
        );
        assert_eq!(result.to_raw().build(), "array::join($goals, ',')");
    }
}

#[cfg(test)]
mod tests {
    use umbriel::*;
    use umbriel::{functions::array::*, *};

    #[test]
    fn test_distinct() {
        let arr = arr![1, 2, 31];
        let result = distinct_fn(arr);

        assert_eq!(
            result.fine_tune_params(),
            "array::distinct([$_param_00000001, $_param_00000002, $_param_00000003])"
        );
        assert_eq!(result.to_raw().build(), "array::distinct([1, 2, 31])");
    }

    #[test]
    fn test_distinct_macro_with_vec_parameterization() {
        let arr = vec![1, 2, 3, 3, 2, 1];
        let result = distinct!(arr);
        assert_eq!(
            result.fine_tune_params(),
            "array::distinct($_param_00000001)"
        );
        assert_eq!(
            result.to_raw().to_string(),
            "array::distinct([1, 2, 3, 3, 2, 1])"
        );
    }

    #[test]
    fn test_distinct_macro_with_mixed_array_parametization() {
        let age = Field::new("age");
        let arr = arr![1, 2, 3, age];
        let result = distinct!(arr);
        assert_eq!(
            result.fine_tune_params(),
            "array::distinct([$_param_00000001, $_param_00000002, $_param_00000003, age])"
        );
        assert_eq!(
            result.to_raw().to_string(),
            "array::distinct([1, 2, 3, age])"
        );
    }

    #[test]
    fn test_len_on_diverse_array_custom_array_function() {
        let email = Field::new("email");
        let arr = arr![1, 21, "4334", "Charlie", email];
        let result = len_fn(arr);
        assert_eq!(
            result.fine_tune_params(),
            "array::len([$_param_00000001, $_param_00000002, $_param_00000003, $_param_00000004, email])"
        );
        assert_eq!(
            result.to_raw().build(),
            "array::len([1, 21, '4334', 'Charlie', email])"
        );
    }

    #[test]
    fn test_len_macro_on_diverse_array_custom_array_function() {
        let email = Field::new("email");
        let arr = arr![1, 21, "4334", "Charlie", email];
        let result = len!(arr);
        assert_eq!(
            result.fine_tune_params(),
            "array::len([$_param_00000001, $_param_00000002, $_param_00000003, $_param_00000004, email])"
        );
        assert_eq!(
            result.to_raw().to_string(),
            "array::len([1, 21, '4334', 'Charlie', email])"
        );
    }

    #[test]
    fn test_sort() {
        // Mono vector should use a single param
        let arr = vec![3, 2, 1];
        let result = sort_fn(arr.clone(), Ordering::Asc);
        assert_eq!(
            result.fine_tune_params(),
            "array::sort($_param_00000001, 'asc')"
        );
        assert_eq!(result.to_raw().build(), "array::sort([3, 2, 1], 'asc')");

        let result = sort_fn(arr.clone(), Ordering::Desc);
        assert_eq!(
            result.fine_tune_params(),
            "array::sort($_param_00000001, 'desc')"
        );
        assert_eq!(result.to_raw().build(), "array::sort([3, 2, 1], 'desc')");

        let result = sort_fn(arr.clone(), Ordering::Empty);
        assert_eq!(result.fine_tune_params(), "array::sort($_param_00000001)");
        assert_eq!(result.to_raw().build(), "array::sort([3, 2, 1])");

        let result = sort_fn(arr.clone(), Ordering::False);
        assert_eq!(
            result.fine_tune_params(),
            "array::sort($_param_00000001, false)"
        );
        assert_eq!(result.to_raw().build(), "array::sort([3, 2, 1], false)");
    }

    #[test]
    fn test_sort_macro_ordering_type() {
        let arr = vec![3, 2, 1];
        let result = sort!(arr.clone(), Ordering::Asc);
        assert_eq!(
            result.fine_tune_params(),
            "array::sort($_param_00000001, 'asc')"
        );
        assert_eq!(result.to_raw().build(), "array::sort([3, 2, 1], 'asc')");

        let result = sort!(arr.clone(), Ordering::Desc);
        assert_eq!(
            result.fine_tune_params(),
            "array::sort($_param_00000001, 'desc')"
        );
        assert_eq!(result.to_raw().build(), "array::sort([3, 2, 1], 'desc')");

        let result = sort!(arr.clone(), Ordering::Empty);
        assert_eq!(result.fine_tune_params(), "array::sort($_param_00000001)");
        assert_eq!(result.to_raw().build(), "array::sort([3, 2, 1])");

        let result = sort!(arr.clone(), Ordering::False);
        assert_eq!(
            result.fine_tune_params(),
            "array::sort($_param_00000001, false)"
        );
        assert_eq!(result.to_raw().build(), "array::sort([3, 2, 1], false)");
    }

    #[test]
    fn test_sort_macro() {
        let arr = vec![3, 2, 1];
        let result = sort!(arr.clone(), "asc");
        assert_eq!(
            result.fine_tune_params(),
            "array::sort($_param_00000001, 'asc')"
        );
        assert_eq!(result.to_raw().to_string(), "array::sort([3, 2, 1], 'asc')");

        let result = sort!(arr.clone(), "desc");
        assert_eq!(
            result.fine_tune_params(),
            "array::sort($_param_00000001, 'desc')"
        );
        assert_eq!(result.to_raw().build(), "array::sort([3, 2, 1], 'desc')");

        // Without ordering
        let result = sort!(arr.clone());
        assert_eq!(result.fine_tune_params(), "array::sort($_param_00000001)");
        assert_eq!(result.to_raw().build(), "array::sort([3, 2, 1])");

        let result = sort!(arr.clone(), false);
        assert_eq!(
            result.fine_tune_params(),
            "array::sort($_param_00000001, false)"
        );
        assert_eq!(result.to_raw().build(), "array::sort([3, 2, 1], false)");
    }

    #[test]
    fn test_sort_asc() {
        let arr = vec![3, 2, 1];
        let result = sort::asc_fn(arr);
        assert_eq!(
            result.fine_tune_params(),
            "array::sort::asc($_param_00000001)"
        );
        assert_eq!(result.to_raw().build(), "array::sort::asc([3, 2, 1])");
    }

    #[test]
    fn test_sort_asc_macro() {
        let arr = arr![3, 2, 1];
        let result = sort::asc!(arr);
        assert_eq!(
            result.fine_tune_params(),
            "array::sort::asc([$_param_00000001, $_param_00000002, $_param_00000003])"
        );
        assert_eq!(result.to_raw().build(), "array::sort::asc([3, 2, 1])");
    }

    #[test]
    fn test_sort_desc() {
        let arr = vec![3, 2, 1];
        let result = sort::desc_fn(arr);
        assert_eq!(
            result.fine_tune_params(),
            "array::sort::desc($_param_00000001)"
        );
        assert_eq!(result.to_raw().build(), "array::sort::desc([3, 2, 1])");
    }

    #[test]
    fn test_sort_desc_macro() {
        let arr = arr![3, 2, 1];
        let result = sort::desc!(arr);
        assert_eq!(
            result.fine_tune_params(),
            "array::sort::desc([$_param_00000001, $_param_00000002, $_param_00000003])"
        );
        assert_eq!(result.to_raw().build(), "array::sort::desc([3, 2, 1])");
    }

    #[test]
    fn test_slice() {
        let arr = vec![1, 2, 3, 4, 5];
        let result = slice_fn(arr, 2, 4);
        assert_eq!(
            result.fine_tune_params(),
            "array::slice($_param_00000001, $_param_00000002, $_param_00000003)"
        );
        assert_eq!(
            result.to_raw().build(),
            "array::slice([1, 2, 3, 4, 5], 2, 4)"
        );
    }

    #[test]
    fn test_slice_macro() {
        let arr = arr![1, 2, 3, 4, 5];
        let result = slice!(arr, 2, 4);
        assert_eq!(
            result.fine_tune_params(),
            "array::slice([$_param_00000001, $_param_00000002, $_param_00000003, $_param_00000004, $_param_00000005], $_param_00000006, $_param_00000007)"
        );
        assert_eq!(
            result.to_raw().build(),
            "array::slice([1, 2, 3, 4, 5], 2, 4)"
        );
    }
}

macro_rules! create_fn_with_single_array_arg_tests {
    ($(#[$attr:meta])* => $function_name:expr) => {
        paste::paste! {
            mod [<test_ $function_name>] {
                use umbriel::{functions::array, *};

                #[test]
                fn [<test $function_name _fn_on_array_macro_on_diverse_array>]() {
                    let age = Field::new("age");
                    let arr = arr![1, "Charlie", age];
                    let result = functions::array::[<$function_name _fn>](arr);
                    assert_eq!(
                        result.fine_tune_params(),
                        format!("array::{}([$_param_00000001, $_param_00000002, age])", $function_name)
                    );
                    assert_eq!(
                        result.to_raw().build(),
                        format!("array::{}([1, 'Charlie', age])", $function_name)
                    );
                }

                #[test]
                fn [<test $function_name _fn_on_same_element_types>]() {
                    let arr = arr![1, 2, 3];
                    let result = array::[<$function_name _fn>](arr);
                    assert_eq!(
                        result.fine_tune_params(),
                        format!("array::{}([$_param_00000001, $_param_00000002, $_param_00000003])", $function_name)
                    );

                    assert_eq!(
                        result.to_raw().build(),
                        format!("array::{}([1, 2, 3])", $function_name)
                    );
                }

                #[test]
                fn [<test $function_name _macro_on_array_macro_on_diverse_array>]() {
                    let age = Field::new("age");
                    let arr = arr![1, "Charlie", age];
                    let result = array::[<$function_name>]!(arr);
                    assert_eq!(
                        result.fine_tune_params(),
                        format!("array::{}([$_param_00000001, $_param_00000002, age])", $function_name)
                    );
                    assert_eq!(
                        result.to_raw().build(),
                        format!("array::{}([1, 'Charlie', age])", $function_name)
                    );
                }

                #[test]
                fn [<test $function_name _macro_on_same_element_types>]() {
                    let arr = umbriel::arr![1, 2, 3];
                    let result = umbriel::functions::array::[<$function_name>]!(arr);
                    assert_eq!(
                        result.fine_tune_params(),
                        format!("array::{}([$_param_00000001, $_param_00000002, $_param_00000003])", $function_name)
                    );

                    assert_eq!(
                        result.to_raw().build(),
                        format!("array::{}([1, 2, 3])", $function_name)
                    );
                }

                #[test]
                fn [<test $function_name _macro_on_fields>]() {
                    let students_ages = Field::new("students_ages");
                    let result = array::[<$function_name>]!(students_ages);
                    assert_eq!(
                        result.fine_tune_params(),
                        format!("array::{}(students_ages)", $function_name)
                    );

                    assert_eq!(
                        result.to_raw().build(),
                        format!("array::{}(students_ages)", $function_name)
                    );
                }
        }
        }
    };
}

create_fn_with_single_array_arg_tests!(
    =>
    "boolean_not"
);
create_fn_with_single_array_arg_tests!(
    =>
    "first"
);
create_fn_with_single_array_arg_tests!(
    =>
    "last"
);
create_fn_with_single_array_arg_tests!(
    =>
    "max"
);
create_fn_with_single_array_arg_tests!(
    =>
    "min"
);

macro_rules! create_fn_with_two_array_args_tests {
    ($(#[$attr:meta])* => $function_name:expr) => {
        paste::paste! {
            mod [<test_ $function_name>] {
                use umbriel::{functions::array, *};

                #[test]
                fn [<test $function_name fn_on_array_macro_on_diverse_array>]() {
                    let age = Field::new("age");
                    let arr1 = arr![1, "Charlie", age];
                    let arr2 = arr![4, "nett", 6];
                    let result = functions::array::[<$function_name _fn>](arr1, arr2);
                    assert_eq!(
                        result.fine_tune_params(),
                        format!("array::{}([$_param_00000001, $_param_00000002, age], [$_param_00000003, $_param_00000004, $_param_00000005])", $function_name)
                    );
                    assert_eq!(
                        result.to_raw().build(),
                        format!("array::{}([1, 'Charlie', age], [4, 'nett', 6])", $function_name)
                    );
                }

                #[test]
                fn [<test $function_name _fn_on_same_element_types>]() {
                    let arr1 = arr![1, 2, 3];
                    let arr2 = arr![4, 5, 6];
                    let result = array::[<$function_name _fn>](arr1, arr2);
                    assert_eq!(
                        result.fine_tune_params(),
                        format!("array::{}([$_param_00000001, $_param_00000002, $_param_00000003], \
                            [$_param_00000004, $_param_00000005, $_param_00000006])", $function_name)
                    );

                    assert_eq!(
                        result.to_raw().build(),
                        format!("array::{}([1, 2, 3], [4, 5, 6])", $function_name)
                    );
                }

                #[test]
                fn [<test $function_name _macro_on_array_macro_on_diverse_array>]() {
                    let age = Field::new("age");
                    let arr1 = arr![1, "Charlie", age];
                    let arr2 = arr![4, "nett", 6];
                    let result = array::[<$function_name>]!(arr1, arr2);
                    assert_eq!(
                        result.fine_tune_params(),
                        format!("array::{}([$_param_00000001, $_param_00000002, age], \
                            [$_param_00000003, $_param_00000004, $_param_00000005])", $function_name)
                    );
                    assert_eq!(
                        result.to_raw().build(),
                        format!("array::{}([1, 'Charlie', age], [4, 'nett', 6])", $function_name)
                    );
                }

                #[test]
                fn [<test $function_name _macro_on_same_element_types>]() {
                    let arr1 = umbriel::arr![1, 2, 3];
                    let arr2 = umbriel::arr![4, 5, 6];
                    let result = umbriel::functions::array::[<$function_name>]!(arr1, arr2);
                    assert_eq!(
                        result.fine_tune_params(),
                        format!("array::{}([$_param_00000001, $_param_00000002, $_param_00000003], \
                            [$_param_00000004, $_param_00000005, $_param_00000006])", $function_name)
                    );

                    assert_eq!(
                        result.to_raw().build(),
                        format!("array::{}([1, 2, 3], [4, 5, 6])", $function_name)
                    );
                }

                #[test]
                fn [<test $function_name _macro_on_fields>]() {
                    let students_ages = Field::new("students_ages");
                    let teachers_ages = Field::new("teachers_ages");
                    let result = array::[<$function_name>]!(students_ages, teachers_ages);
                    assert_eq!(
                        result.fine_tune_params(),
                        format!("array::{}(students_ages, teachers_ages)", $function_name)
                    );

                    assert_eq!(
                        result.to_raw().build(),
                        format!("array::{}(students_ages, teachers_ages)", $function_name)
                    );
                }
        }
        }
    };
}

create_fn_with_two_array_args_tests!(
=>
    "combine"
);
create_fn_with_two_array_args_tests!(
    =>
    "concat"
);
create_fn_with_two_array_args_tests!(
    =>
    "union"
);
create_fn_with_two_array_args_tests!(
    =>
    "difference"
);
create_fn_with_two_array_args_tests!(
    =>
    "intersect"
);
create_fn_with_two_array_args_tests!(
    =>
    "complement"
);
create_fn_with_two_array_args_tests!(
    =>
    "boolean_and"
);
create_fn_with_two_array_args_tests!(
    =>
    "boolean_or"
);
create_fn_with_two_array_args_tests!(
    =>
    "boolean_xor"
);
create_fn_with_two_array_args_tests!(
    =>
    "logical_and"
);
create_fn_with_two_array_args_tests!(
    =>
    "logical_or"
);
create_fn_with_two_array_args_tests!(
    =>
    "logical_xor"
);

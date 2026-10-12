#[cfg(test)]
mod tests {
    use umbriel::{Buildable, Field, NumberLike, ToRaw, arr};

    use umbriel::functions::rand;

    #[test]
    fn test_rand_fn() {
        let result = rand::rand_fn();
        assert_eq!(result.fine_tune_params(), "rand()");
        assert_eq!(result.to_raw().build(), "rand()");
    }

    #[test]
    fn test_rand_macro() {
        let result = rand!();
        assert_eq!(result.fine_tune_params(), "rand()");
        assert_eq!(result.to_raw().build(), "rand()");
    }

    #[test]
    fn test_rand_bool_fn() {
        let result = rand::bool_fn();
        assert_eq!(result.fine_tune_params(), "rand::bool()");
        assert_eq!(result.to_raw().build(), "rand::bool()");
    }

    #[test]
    fn test_rand_bool_macro() {
        let result = rand::bool!();
        assert_eq!(result.fine_tune_params(), "rand::bool()");
        assert_eq!(result.to_raw().build(), "rand::bool()");
    }

    #[test]
    fn test_rand_uuid_fn() {
        let result = rand::uuid_fn();
        assert_eq!(result.fine_tune_params(), "rand::uuid()");
        assert_eq!(result.to_raw().build(), "rand::uuid()");
    }

    #[test]
    fn test_rand_uuid() {
        let result = rand::uuid!();
        assert_eq!(result.fine_tune_params(), "rand::uuid()");
        assert_eq!(result.to_raw().build(), "rand::uuid()");
    }

    #[test]
    fn test_rand_uuid_v4_fn() {
        let result = rand::uuid::v4_fn();
        assert_eq!(result.fine_tune_params(), "rand::uuid::v4()");
        assert_eq!(result.to_raw().build(), "rand::uuid::v4()");
    }

    #[test]
    fn test_rand_uuid_v4() {
        let result = rand::uuid::v4!();
        assert_eq!(result.fine_tune_params(), "rand::uuid::v4()");
        assert_eq!(result.to_raw().build(), "rand::uuid::v4()");
    }

    #[test]
    fn test_rand_uuid_v7_fn() {
        let result = rand::uuid::v7_fn();
        assert_eq!(result.fine_tune_params(), "rand::uuid::v7()");
        assert_eq!(result.to_raw().build(), "rand::uuid::v7()");
    }

    #[test]
    fn test_rand_uuid_v7() {
        let result = rand::uuid::v7!();
        assert_eq!(result.fine_tune_params(), "rand::uuid::v7()");
        assert_eq!(result.to_raw().build(), "rand::uuid::v7()");
    }

    #[test]
    fn test_rand_enum_macro() {
        let result = rand::enum_!("one", "two", 3, 4.15385, "five", true);
        assert_eq!(
            result.fine_tune_params(),
            "rand::enum($_param_00000001, $_param_00000002, $_param_00000003, $_param_00000004, $_param_00000005, $_param_00000006)"
        );
        assert_eq!(
            result.to_raw().build(),
            "rand::enum('one', 'two', 3, 4.15385f, 'five', true)"
        );
    }

    #[test]
    fn test_rand_enum_macro_with_array() {
        let result = rand::enum_!(arr!["one", "two", 3, 4.15385, "five", true]);
        assert_eq!(
            result.fine_tune_params(),
            "rand::enum($_param_00000001, $_param_00000002, $_param_00000003, $_param_00000004, $_param_00000005, $_param_00000006)"
        );
        assert_eq!(
            result.to_raw().build(),
            "rand::enum('one', 'two', 3, 4.15385f, 'five', true)"
        );
    }

    #[test]
    fn test_rand_string_macro_with_one_arg_length() {
        let result = rand::string!(34);
        assert_eq!(result.fine_tune_params(), "rand::string($_param_00000001)");
        assert_eq!(result.to_raw().build(), "rand::string(34)");
    }

    #[test]
    fn test_rand_string_macro_with_one_arg_field() {
        let length_of_name = Field::new("length_of_name");
        let result = rand::string!(length_of_name);
        assert_eq!(result.fine_tune_params(), "rand::string(length_of_name)");
        assert_eq!(result.to_raw().build(), "rand::string(length_of_name)");
    }

    // Test Guid
    #[test]
    fn test_rand_guid_function_empty() {
        let result = rand::guid_fn(None as Option<NumberLike>);
        assert_eq!(result.fine_tune_params(), "rand::guid()");
        assert_eq!(result.to_raw().build(), "rand::guid()");
    }

    #[test]
    fn test_rand_guid_macro_empty() {
        let result = rand::guid!();
        assert_eq!(result.fine_tune_params(), "rand::guid()");
        assert_eq!(result.to_raw().build(), "rand::guid()");
    }

    #[test]
    fn test_rand_guid_macro_with_range() {
        let result = rand::guid!(34);
        assert_eq!(result.fine_tune_params(), "rand::guid($_param_00000001)");
        assert_eq!(result.to_raw().build(), "rand::guid(34)");
    }

    #[test]
    fn test_rand_guid_fn_with_field_input() {
        let length = Field::new("length");

        let result = rand::guid_fn(Some(length));
        assert_eq!(result.fine_tune_params(), "rand::guid(length)");
        assert_eq!(result.to_raw().build(), "rand::guid(length)");
    }

    #[test]
    fn test_rand_guid_macro_with_field_input() {
        let length = Field::new("length");

        let result = rand::guid!(length);
        assert_eq!(result.fine_tune_params(), "rand::guid(length)");
        assert_eq!(result.to_raw().build(), "rand::guid(length)");
    }
}

macro_rules! create_test_for_fn_with_two_args_tests {
    ($function_ident: expr) => {
        paste::paste! {
            mod [<test_ $function_ident _fn>] {
                use umbriel::*;
                use umbriel::functions::rand;

                #[test]
                fn [<test_rand_ $function_ident _function_empty>]() {
                    let result = rand::[< $function_ident _fn>](None as Option<umbriel::NumberLike>, None as Option<umbriel::NumberLike>);
                    assert_eq!(result.fine_tune_params(), format!("rand::{}()", $function_ident));
                    assert_eq!(result.to_raw().build(), format!("rand::{}()", $function_ident));
                }

                #[test]
                fn [<test_rand_ $function_ident _macro_empty>]() {
                    let result = rand::[< $function_ident>]!();
                    assert_eq!(result.fine_tune_params(), format!("rand::{}()", $function_ident));
                    assert_eq!(result.to_raw().build(), format!("rand::{}()", $function_ident));
                }

                #[test]
                fn [<test_rand_ $function_ident _macro_with_range>]() {
                    let result = rand::[< $function_ident>]!(34, 65);
                    assert_eq!(result.fine_tune_params(), format!("rand::{}($_param_00000001, $_param_00000002)", $function_ident));
                    assert_eq!(result.to_raw().build(), format!("rand::{}(34, 65)", $function_ident));
                }

                #[test]
                fn [<test_rand_ $function_ident fn_with_field_inputs>]() {
                    let start = Field::new("start");
                    let end = Field::new("end");

                    let result = rand::[< $function_ident _fn>](Some(start), Some(end));
                    assert_eq!(result.fine_tune_params(), format!("rand::{}(start, end)", $function_ident));
                    assert_eq!(result.to_raw().build(), format!("rand::{}(start, end)", $function_ident));
                }

                #[test]
                fn [<test_rand_ $function_ident macro_with_field_inputs>]() {
                    let start = Field::new("start");
                    let end = Field::new("end");

                    let result = rand::[< $function_ident>]!(start, end);
                    assert_eq!(result.fine_tune_params(), format!("rand::{}(start, end)", $function_ident));
                    assert_eq!(result.to_raw().build(), format!("rand::{}(start, end)", $function_ident));
                }
            }
        }
    };
}

create_test_for_fn_with_two_args_tests!("float");
create_test_for_fn_with_two_args_tests!("int");
create_test_for_fn_with_two_args_tests!("string");
create_test_for_fn_with_two_args_tests!("time");

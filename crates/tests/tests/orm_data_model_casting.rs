macro_rules! create_cast_function_tests {
    ($(#[$attr:meta])* => $function_name: expr) => {
        paste::paste! {
            mod [<test_ $function_name>] {
                use umbriel::*;
                use umbriel::*;

                #[test]
                fn [<test_ $function_name _cast_function >] () {
                    let value = 39;
                    let casted_value = [<$function_name>](value);
                    assert_eq!(casted_value.fine_tune_params(), format!("<{}> $_param_00000001", $function_name));
                    assert_eq!(casted_value.to_raw().build(), format!("<{}> {}", $function_name, value));
                }

                #[test]
                fn [<test_ $function_name _cast_function_with_field >] () {
                    let value = umbriel::Field::new("test");
                    let casted_value = [<$function_name>](value.clone());
                    assert_eq!(casted_value.fine_tune_params(), format!("<{}> test", $function_name));
                    assert_eq!(casted_value.to_raw().build(), format!("<{}> test", $function_name));
                }

                #[test]
                fn [<test_ $function_name _cast_function_with_param >] () {
                    let value = umbriel::Param::new("test");
                    let casted_value = [<$function_name>](value.clone());
                    assert_eq!(casted_value.fine_tune_params(), format!("<{}> $test", $function_name));
                    assert_eq!(casted_value.to_raw().build(), format!("<{}> $test", $function_name));
                }
            }
        }
    };
}

create_cast_function_tests!(
    => "bool"
);
create_cast_function_tests!(
    => "datetime"
);
create_cast_function_tests!(
    => "decimal"
);
create_cast_function_tests!(
    => "duration"
);
create_cast_function_tests!(
    => "float"
);
create_cast_function_tests!(
    => "int"
);
create_cast_function_tests!(
    => "number"
);
create_cast_function_tests!(
    => "string"
);

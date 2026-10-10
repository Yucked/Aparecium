#[cfg(test)]
mod tests {
    use aparecium::*;
    use aparecium::{functions::math::*, *};
    #[test]
    fn test_fixed_fn_with_field_data() {
        let land_size = Field::new("land_size");
        let decimal_place = Field::new("decimal_place");
        let result = fixed_fn(land_size, decimal_place);

        assert_eq!(
            result.fine_tune_params(),
            "math::fixed(land_size, decimal_place)"
        );

        assert_eq!(
            result.to_raw().build(),
            "math::fixed(land_size, decimal_place)"
        );
    }

    #[test]
    fn test_fixed_fn_with_raw_numbers() {
        let result = fixed_fn(13.45423, 4);
        assert_eq!(
            result.fine_tune_params(),
            "math::fixed($_param_00000001, $_param_00000002)"
        );
        assert_eq!(result.to_raw().build(), "math::fixed(13.45423f, 4)");
    }

    #[test]
    fn test_fixed_fn_with_raw_number_with_field() {
        let land_mass = Field::new("country.land_mass");
        let result = fixed_fn(land_mass, 4);
        assert_eq!(
            result.fine_tune_params(),
            "math::fixed(country.land_mass, $_param_00000001)"
        );
        assert_eq!(result.to_raw().build(), "math::fixed(country.land_mass, 4)");
    }

    // Macro versions
    #[test]
    fn test_fixed_macro_with_field_data() {
        let land_size = Field::new("land_size");
        let decimal_place = Field::new("decimal_place");
        let result = fixed!(land_size, decimal_place);

        assert_eq!(
            result.fine_tune_params(),
            "math::fixed(land_size, decimal_place)"
        );

        assert_eq!(
            result.to_raw().build(),
            "math::fixed(land_size, decimal_place)"
        );
    }

    #[test]
    fn test_fixed_macro_with_params() {
        let land_size = Param::new("land_size");
        let decimal_place = Param::new("decimal_place");
        let result = fixed!(land_size, decimal_place);

        assert_eq!(
            result.fine_tune_params(),
            "math::fixed($land_size, $decimal_place)"
        );

        assert_eq!(
            result.to_raw().build(),
            "math::fixed($land_size, $decimal_place)"
        );
    }

    #[test]
    fn test_fixed_macro_with_raw_numbers() {
        let result = fixed!(13.45423, 4);
        assert_eq!(
            result.fine_tune_params(),
            "math::fixed($_param_00000001, $_param_00000002)"
        );
        assert_eq!(result.to_raw().build(), "math::fixed(13.45423f, 4)");
    }

    #[test]
    fn test_fixed_macro_with_raw_number_with_field() {
        let land_mass = Field::new("country.land_mass");
        let result = fixed!(land_mass, 4);
        assert_eq!(
            result.fine_tune_params(),
            "math::fixed(country.land_mass, $_param_00000001)"
        );
        assert_eq!(result.to_raw().build(), "math::fixed(country.land_mass, 4)");
    }
}

macro_rules! create_test_for_fn_with_single_arg_tests {
    ($(#[$attr:meta])* => $function_name: expr) => {
        paste::paste! {
            mod [<test_ $function_name>] {
                use aparecium::functions::math::*;
                use aparecium::*;
                use aparecium::functions::math;

                #[test]
                fn [<test_ $function_name _fn_with_field_data >] () {
                    let temparate = Field::new("temperature");
                    let result = math::[<$function_name _fn>](temparate);

                    assert_eq!(result.fine_tune_params(), format!("math::{}(temperature)", $function_name));
                    assert_eq!(result.to_raw().build(), format!("math::{}(temperature)", $function_name));
                }

                #[test]
                fn [<test_ $function_name _fn_with_fraction>]() {
                    let result = math::[<$function_name _fn>](45.23);
                    assert_eq!(result.fine_tune_params(), format!("math::{}($_param_00000001)", $function_name));
                    assert_eq!(result.to_raw().build(), format!("math::{}(45.23f)", $function_name));
                }

                #[test]
                fn [<test_ $function_name _fn_with_negative_number>]() {
                    let result = math::[<$function_name _fn>](-454);
                    assert_eq!(result.fine_tune_params(), format!("math::{}($_param_00000001)", $function_name));
                    assert_eq!(result.to_raw().build(), format!("math::{}(-454)", $function_name));
                }

                // Macro version
                #[test]
                fn [<test_ $function_name _macro_with_field_data >] () {
                    let temparate = Field::new("temperature");
                    let result = math::[<$function_name>]!(temparate);

                    assert_eq!(result.fine_tune_params(), format!("math::{}(temperature)", $function_name));
                    assert_eq!(result.to_raw().build(), format!("math::{}(temperature)", $function_name));
                }

                #[test]
                fn [<test_ $function_name _macro_with_param >] () {
                    let temparate = Param::new("temperature");
                    let result = math::[<$function_name>]!(temparate);

                    assert_eq!(result.fine_tune_params(), format!("math::{}($temperature)", $function_name));
                    assert_eq!(result.to_raw().build(), format!("math::{}($temperature)", $function_name));
                }

                #[test]
                fn [<test_ $function_name _macro_with_fraction>]() {
                    let result = math::[<$function_name>]!(45.23);
                    assert_eq!(result.fine_tune_params(), format!("math::{}($_param_00000001)", $function_name));
                    assert_eq!(result.to_raw().build(), format!("math::{}(45.23f)", $function_name));
                }

                #[test]
                fn [<test_ $function_name _macro_with_negative_number>]() {
                    let result = math::[<$function_name>]!(-454);
                    assert_eq!(result.fine_tune_params(), format!("math::{}($_param_00000001)", $function_name));
                    assert_eq!(result.to_raw().build(), format!("math::{}(-454)", $function_name));
                }
            }
        }
    };
}

create_test_for_fn_with_single_arg_tests!(
=>
    "abs"
);
create_test_for_fn_with_single_arg_tests!(
=>
    "ceil"
);
create_test_for_fn_with_single_arg_tests!(
=>
    "floor"
);
create_test_for_fn_with_single_arg_tests!(
=>
    "round"
);
create_test_for_fn_with_single_arg_tests!(
=>
    "sqrt"
);

macro_rules! create_test_for_fn_with_single_array_arg_tests {
    ($(#[$attr:meta])* => $function_name: expr) => {
        paste::paste! {
            mod [<test_ $function_name>] {
                use aparecium::functions::math::*;
                use aparecium::*;
                use aparecium::functions::math;

                #[test]
                fn [<test_ $function_name _fn_with_field_data >] () {
                    let size_list = Field::new("size_list");
                    let result = math::[<$function_name _fn>](size_list);

                    assert_eq!(result.fine_tune_params(), format!("math::{}(size_list)", $function_name));
                    assert_eq!(result.to_raw().build(), format!("math::{}(size_list)", $function_name));
                }

                #[test]
                fn [<test_ $function_name _fn_with_number_array>]() {
                    let arr1 = array![1, 2, 3, 3.5];
                    let result = math::[<$function_name _fn>](arr1);
                    assert_eq!(result.fine_tune_params(), format!("math::{}($_param_00000001)", $function_name));
                    assert_eq!(result.to_raw().build(), format!("math::{}([1, 2, 3, 3.5f])", $function_name));
                }

                #[test]
                fn [<test_ $function_name _fn_with_mixed_array>]() {
                    let age = Field::new("age");
                    let arr = arr![1, 2, "4334", "Oyelowo", age];
                    let result = math::[<$function_name _fn>](arr);
                    assert_eq!(result.fine_tune_params(),
                        format!("math::{}([$_param_00000001, $_param_00000002, $_param_00000003, $_param_00000004, age])", $function_name));
                    assert_eq!(result.to_raw().build(), format!("math::{}([1, 2, '4334', 'Oyelowo', age])", $function_name));
                }

                // Macro version
                #[test]
                fn [<test_ $function_name _macro_with_field_data >] () {
                    let size_list = Field::new("size_list");
                    let result = math::[<$function_name>]!(size_list);

                    assert_eq!(result.fine_tune_params(), format!("math::{}(size_list)", $function_name));
                    assert_eq!(result.to_raw().build(), format!("math::{}(size_list)", $function_name));
                }

                #[test]
                fn [<test_ $function_name _macro_with_param >] () {
                    let size_list = Param::new("size_list");
                    let result = math::[<$function_name>]!(size_list);

                    assert_eq!(result.fine_tune_params(), format!("math::{}($size_list)", $function_name));
                    assert_eq!(result.to_raw().build(), format!("math::{}($size_list)", $function_name));
                }

                #[test]
                fn [<test_ $function_name _macro_with_number_array>]() {
                    let arr1 = array![1, 2, 3, 3.5];
                    let result = math::[<$function_name>]!(arr1);
                    assert_eq!(result.fine_tune_params(), format!("math::{}($_param_00000001)", $function_name));
                    assert_eq!(result.to_raw().build(), format!("math::{}([1, 2, 3, 3.5f])", $function_name));
                }

                #[test]
                fn [<test_ $function_name _macro_with_mixed_array>]() {
                    let age = Field::new("age");
                    let arr = arr![1, 2, "4334", "Oyelowo", age];
                    let result = math::[<$function_name>]!(arr);
                    assert_eq!(result.fine_tune_params(),
                        format!("math::{}([$_param_00000001, $_param_00000002, $_param_00000003, $_param_00000004, age])", $function_name));
                    assert_eq!(result.to_raw().build(), format!("math::{}([1, 2, '4334', 'Oyelowo', age])", $function_name));
                }
            }
        }
    };
}

create_test_for_fn_with_single_array_arg_tests!(
=> "mean"
);
create_test_for_fn_with_single_array_arg_tests!(
=> "median"
);
create_test_for_fn_with_single_array_arg_tests!(
=> "mode"
);
create_test_for_fn_with_single_array_arg_tests!(
=> "min"
);
create_test_for_fn_with_single_array_arg_tests!(
=> "max"
);
create_test_for_fn_with_single_array_arg_tests!(
=> "product"
);
create_test_for_fn_with_single_array_arg_tests!(
=> "sum"
);

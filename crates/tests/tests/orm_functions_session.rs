macro_rules! create_function_tests {
    ($(#[$attr:meta])* => $function_name: expr) => {
        paste::paste! {
            mod [<test_ $function_name>] {
                use umbriel::*;
                use umbriel::functions::session;

                #[test]
                fn [<test_ $function_name _fn>]() {
                    let result = session::[<$function_name _fn>]();
                    assert_eq!(result.fine_tune_params(), format!("{}()", $function_name));
                    assert_eq!(result.to_raw().build(), format!("{}()", $function_name));
                }

                #[test]
                fn [<test_ $function_name _macro>]() {
                    let result = session::[<$function_name>]!();
                    assert_eq!(result.fine_tune_params(), format!("{}()", $function_name));
                    assert_eq!(result.to_raw().build(), format!("{}()", $function_name));
                }
            }
        }
    };
}

create_function_tests!(
    =>
"db");
create_function_tests!(
    =>
"id");
create_function_tests!(
    =>
"ip");
create_function_tests!(
    =>
"ns");
create_function_tests!(
    =>
"origin");
create_function_tests!(
    =>
"sc");

macro_rules! create_test_for_fn_with_single_arg_tests {
    ($(#[$attr:meta])* => $module_name:expr, $function_name:expr, $arg:expr) => {
        paste::paste! {
            mod [<test_ $function_name>] {
                use aparecium::*;
                use aparecium::functions::parse;

                #[test]
                fn [<test_ $function_name _fn_with_field_data >] () {
                    let field = Field::new("field");
                    let result = parse::[<$module_name>]::[<$function_name _fn>](field);

                    let function_path = format!("parse::{}::{}", $module_name, $function_name);
                    assert_eq!(result.fine_tune_params(), format!("{}(field)", function_path));
                    assert_eq!(result.to_raw().build(), format!("{}(field)", function_path));
                }

                #[test]
                fn [<test_ $function_name _fn_with_fraction>]() {
                    let result = parse::[<$module_name>]::[<$function_name _fn>]($arg);
                    let function_path = format!("parse::{}::{}", $module_name, $function_name);
                    assert_eq!(result.fine_tune_params(), format!("{}($_param_00000001)", function_path));
                    assert_eq!(result.to_raw().build(), format!("{}('{}')", function_path, $arg));
                }

                #[test]
                fn [<test_ $function_name _macro_with_field_data >] () {
                    let field = Field::new("field");
                    let result = parse::[<$module_name>]::[<$function_name>]!(field);

                    let function_path = format!("parse::{}::{}", $module_name, $function_name);
                    assert_eq!(result.fine_tune_params(), format!("{}(field)", function_path));
                    assert_eq!(result.to_raw().build(), format!("{}(field)", function_path));
                }

                #[test]
                fn [<test_ $function_name _macro_with_param >] () {
                    let param = Param::new("param");
                    let result = parse::[<$module_name>]::[<$function_name>]!(param);

                    let function_path = format!("parse::{}::{}", $module_name, $function_name);
                    assert_eq!(result.fine_tune_params(), format!("{}($param)", function_path));
                    assert_eq!(result.to_raw().build(), format!("{}($param)", function_path));
                }

                #[test]
                fn [<test_ $function_name _macro_with_fraction>]() {
                    let result = parse::[<$module_name>]::[<$function_name>]!($arg);
                    let function_path = format!("parse::{}::{}", $module_name, $function_name);
                    assert_eq!(result.fine_tune_params(), format!("{}($_param_00000001)", function_path));
                    assert_eq!(result.to_raw().build(), format!("{}('{}')", function_path, $arg));
                }
            }
        }
    };
}

mod email {
    create_test_for_fn_with_single_arg_tests!(
        => "email", "host", "oyelowo@codebreather.com");
    create_test_for_fn_with_single_arg_tests!(
        =>
        "email", "user", "oyelowo@codebreather.com");
}
mod url {
    create_test_for_fn_with_single_arg_tests!(
        =>
        "url",
        "domain",
        "https://codebreather.com:443/topics?arg=value#fragment"
    );
    create_test_for_fn_with_single_arg_tests!(
        =>
        "url",
        "fragment",
        "https://codebreather.com:443/topics?arg=value#fragment"
    );
    create_test_for_fn_with_single_arg_tests!(
        =>
        "url",
        "host",
        "https://codebreather.com:443/topics?arg=value#fragment"
    );
    create_test_for_fn_with_single_arg_tests!(
        =>
        "url",
        "path",
        "https://codebreather.com:443/topics?arg=value#fragment"
    );
    create_test_for_fn_with_single_arg_tests!(
        =>
        "url",
        "port",
        "https://codebreather.com:443/topics?arg=value#fragment"
    );
    create_test_for_fn_with_single_arg_tests!(
        =>
        "url",
        "query",
        "https://codebreather.com:443/topics?arg=value#fragment"
    );
}

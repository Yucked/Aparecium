macro_rules! create_fn_with_url_and_head_tests {
    ($(#[$attr:meta])* => $function_name: expr) => {
        paste::paste! {
            mod [<test_ $function_name>] {
                use aparecium::functions::http::*;
                use aparecium::*;
                use functions::http;
                use std::collections::hash_map;

                #[test]
                fn [<test_ $function_name _method_with_empty_header>]() {
                    let result = [<$function_name _fn>]("https://codebreather.com", None as Option<ObjectLike>);
                    assert_eq!(result.fine_tune_params(), format!("http::{}($_param_00000001)", $function_name));
                    assert_eq!(
                        result.to_raw().build(),
                        format!("http::{}('https://codebreather.com')", $function_name)
                    );
                }

                #[test]
                fn [<test_ $function_name _method_with_field_and_empty_header>]() {
                    let homepage = Field::new("homepage");

                    let result = http::[<$function_name _fn>](homepage, None as Option<ObjectLike>);
                    assert_eq!(result.fine_tune_params(), format!("http::{}(homepage)", $function_name));
                    assert_eq!(result.to_raw().build(), format!("http::{}(homepage)", $function_name));
                }

                #[test]
                fn [<test_ $function_name _method_with_plain_custom_header>]() {
                    let headers = hash_map::HashMap::from([("x-my-header".to_string(), "some unique string".into())]);
                    let result = http::[< $function_name _fn>]("https://codebreather.com", Some(headers));
                    assert_eq!(
                        result.fine_tune_params(),
                        format!("http::{}($_param_00000001, $_param_00000002)", $function_name)
                    );
                    assert_eq!(
                        result.to_raw().build(),
                        format!("http::{}('https://codebreather.com', {})", $function_name, "{ \"x-my-header\": 'some unique string' }")
                    );
                }

                #[test]
                fn [<test_ $function_name _method_with_field_custom_header>]() {
                    let homepage = Field::new("homepage");
                    let headers = Field::new("headers");

                    let result = http::[<$function_name _fn>](homepage, Some(headers));
                    assert_eq!(
                        result.fine_tune_params(),
                        format!("http::{}(homepage, headers)", $function_name)
                    );
                    assert_eq!(result.to_raw().build(), format!("http::{}(homepage, headers)", $function_name));
                }

                // Macro version
                #[test]
                fn [<test_ $function_name _macro_method_with_empty_header_not_listed>]() {
                    let result = http::[<$function_name>]!("https://codebreather.com");
                    assert_eq!(result.fine_tune_params(), format!("http::{}($_param_00000001)", $function_name));
                    assert_eq!(
                        result.to_raw().build(),
                        format!("http::{}('https://codebreather.com')", $function_name)
                    );
                }

                #[test]
                fn [<test_ $function_name _macro_method_with_empty_header>]() {
                    let homepage = Field::new("homepage");

                    let result = http::[<$function_name>]!(homepage);
                    assert_eq!(result.fine_tune_params(), format!("http::{}(homepage)", $function_name));
                    assert_eq!(result.to_raw().build(), format!("http::{}(homepage)", $function_name));
                }

                #[test]
                fn [<test_ $function_name _macro_method_with_plain_custom_header>]() {
                    let headers = hash_map::HashMap::from([("x-my-header".to_string(), "some unique string".into())]);
                    let result = http::[<$function_name>]!("https://codebreather.com", headers);
                    assert_eq!(
                        result.fine_tune_params(),
                        format!("http::{}($_param_00000001, $_param_00000002)", $function_name)
                    );
                    assert_eq!(
                        result.to_raw().build(),
                        format!("http::{}('https://codebreather.com', {})", $function_name, "{ \"x-my-header\": 'some unique string' }")
                    );
                }

                #[test]
                fn [<test_ $function_name _macro_method_with_field_custom_header>]() {
                    let homepage = Field::new("homepage");
                    let headers = Field::new("headers");

                    let result = http::[<$function_name>]!(homepage, headers);
                    assert_eq!(
                        result.fine_tune_params(),
                        format!("http::{}(homepage, headers)", $function_name)
                    );
                    assert_eq!(result.to_raw().build(), format!("http::{}(homepage, headers)", $function_name));
                }
            }
        }
    };
}

create_fn_with_url_and_head_tests!(
    => "head"
);
create_fn_with_url_and_head_tests!(
    => "get"
);
create_fn_with_url_and_head_tests!(
    => "delete"
);

macro_rules! create_fn_with_3args_url_body_and_head_tests {
    ($(#[$attr:meta])* => $function_name: expr) => {
        paste::paste! {
            mod [<test_ $function_name>] {
                use aparecium::functions::http::*;
                use aparecium::*;
                use std::collections::hash_map::HashMap;

                #[test]
                fn [<test_field_ $function_name _method_with_empty_body_and_headers>]() {
                    let result = [<$function_name _fn>]("https://codebreather.com", None as Option<ObjectLike>, None as Option<ObjectLike>);
                    assert_eq!(
                        result.fine_tune_params(),
                        format!("http::{}($_param_00000001, {{ }}, {{ }})", $function_name)
                    );
                    assert_eq!(
                        result.to_raw().build(),
                        format!("http::{}('https://codebreather.com', {{ }}, {{ }})", $function_name)
                    );
                }

                #[test]
                fn [<test_field_ $function_name _method_with_fields_as_args>]() {
                    let homepage = Field::new("homepage");
                    let request_body = Field::new("request_body");
                    let headers = Field::new("headers");

                    let result = [<$function_name _fn>](homepage, Some(request_body), Some(headers));
                    assert_eq!(
                        result.fine_tune_params(),
                        format!("http::{}(homepage, request_body, headers)", $function_name)
                    );
                    assert_eq!(
                        result.to_raw().build(),
                        format!("http::{}(homepage, request_body, headers)", $function_name)
                    );
                }

                #[test]
                fn [<test_field_ $function_name _method_with_params_as_args>]() {
                    let homepage = Param::new("homepage");
                    let request_body = Param::new("request_body");
                    let headers = Param::new("headers");

                    let result = [<$function_name _fn>](homepage, Some(request_body), Some(headers));
                    assert_eq!(
                        result.fine_tune_params(),
                        format!("http::{}($homepage, $request_body, $headers)", $function_name)
                    );
                    assert_eq!(
                        result.to_raw().build(),
                        format!("http::{}($homepage, $request_body, $headers)", $function_name)
                    );
                }

                #[test]
                fn [<test_ $function_name _method_with_body_and_custom_headers_as_plain_values>]() {
                    let body = HashMap::from([
                        ("id".to_string(), 1.into()),
                        ("body".to_string(), "This is some awesome thinking!".into()),
                        ("postId".to_string(), 100.into()),
                    ]);
                    let headers = HashMap::from([("x-my-header".to_string(), "some unique string".into())]);
                    let result = [<$function_name _fn>]("https://codebreather.com", Some(body), Some(headers));

                    assert_eq!(
                        result.fine_tune_params(),
                        format!("http::{}($_param_00000001, $_param_00000002, $_param_00000003)", $function_name)
                    );
                    assert_eq!(
                        result.to_raw().build(),
                        format!(
                            "http::{}('https://codebreather.com', {}, {})",
                            $function_name,
                            "{ body: 'This is some awesome thinking!', id: 1, postId: 100 }",  "{ \"x-my-header\": 'some unique string' }" )
                    );
                }

                // Macro versions
                #[test]
                fn [<test_field_ $function_name _macro_method_with_no_custom_headers>]() {
                    let body = Field::new("body");
                    let result = [<$function_name>]!("https://codebreather.com", body);

                    assert_eq!(
                        result.fine_tune_params(),
                        format!("http::{}($_param_00000001, body, {{ }})", $function_name)
                    );
                    assert_eq!(
                        result.to_raw().build(),
                        format!("http::{}('https://codebreather.com', body, {{ }})", $function_name)
                    );
                }
                #[test]
                fn [<test_field_ $function_name _macro_method_with_empty_body_and_headers>]() {
                    let result = [<$function_name>]!("https://codebreather.com", None, None);

                    assert_eq!(
                        result.fine_tune_params(),
                        format!("http::{}($_param_00000001, $_param_00000002, $_param_00000003)", $function_name)
                    );
                    assert_eq!(
                        result.to_raw().build(),
                        format!("http::{}('https://codebreather.com', {{  }}, {{  }})", $function_name)
                    );
                }

                #[test]
                fn [<test_field_ $function_name _macro_method_with_fields_as_args>]() {
                    let homepage = Field::new("homepage");
                    let request_body = Field::new("request_body");
                    let headers = Field::new("headers");

                    let result = [<$function_name>]!(homepage, request_body, headers);
                    assert_eq!(
                        result.fine_tune_params(),
                        format!("http::{}(homepage, request_body, headers)", $function_name)
                    );
                    assert_eq!(
                        result.to_raw().build(),
                        format!("http::{}(homepage, request_body, headers)", $function_name)
                    );
                }

                #[test]
                fn [<test_field_ $function_name _macro_method_with_params_as_args>]() {
                    let homepage = Param::new("homepage");
                    let request_body = Param::new("request_body");
                    let headers = Param::new("headers");

                    let result = [<$function_name>]!(homepage, request_body, headers);
                    assert_eq!(
                        result.fine_tune_params(),
                        format!("http::{}($homepage, $request_body, $headers)", $function_name)
                    );
                    assert_eq!(
                        result.to_raw().build(),
                        format!("http::{}($homepage, $request_body, $headers)", $function_name)
                    );
                }

                #[test]
                fn [<test_ $function_name _macro_method_with_body_and_custom_headers_as_plain_values>]() {
                    let body = HashMap::from([
                        ("id".to_string(), 1.into()),
                        ("body".to_string(), "This is some awesome thinking!".into()),
                        ("postId".to_string(), 100.into()),
                    ]);
                    let headers = HashMap::from([("x-my-header".to_string(), "some unique string".into())]);
                    let result = [<$function_name>]!("https://codebreather.com", body, headers);

                    assert_eq!(
                        result.fine_tune_params(),
                        format!("http::{}($_param_00000001, $_param_00000002, $_param_00000003)", $function_name)
                    );
                    assert_eq!(
                        result.to_raw().build(),
                        format!(
                            "http::{}('https://codebreather.com', {}, {})",
                            $function_name,
                            "{ body: 'This is some awesome thinking!', id: 1, postId: 100 }",  "{ \"x-my-header\": 'some unique string' }" )
                    );
                }
            }
        }
    };
}

create_fn_with_3args_url_body_and_head_tests!(
=> "post"
);
create_fn_with_3args_url_body_and_head_tests!(
=> "put"
);
create_fn_with_3args_url_body_and_head_tests!(
=> "patch"
);

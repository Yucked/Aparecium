#[cfg(test)]
mod test_search_highlight {
    use umbriel::{functions::search, *};

    // seach::highlight('<b>', '</b>', 1);

    #[test]
    fn test_search_highlight_fn_with_strands() {
        let str1 = "<b>";
        let str2 = "</b>";
        let predicate_ref_number = 1;
        let result = search::highlight_fn(str1, str2, predicate_ref_number);
        assert_eq!(
            result.to_raw().build(),
            "search::highlight('<b>', '</b>', 1)"
        );
    }

    #[test]
    fn test_search_highlight_macro_with_strands() {
        let str1 = "<b>";
        let str2 = "</b>";
        let predicate_ref_number = 1;
        let result = search::highlight!(str1, str2, predicate_ref_number);
        assert_eq!(
            result.to_raw().build(),
            "search::highlight('<b>', '</b>', 1)"
        );
    }

    #[test]
    fn test_search_highlight_fn_with_fields() {
        let str1 = Field::new("str1");
        let str2 = Field::new("str2");
        let predicate_ref_number = Field::new("predicate_ref_number");
        let result = search::highlight_fn(str1, str2, predicate_ref_number);
        assert_eq!(
            result.to_raw().build(),
            "search::highlight(str1, str2, predicate_ref_number)"
        );
    }

    #[test]
    fn test_search_highlight_macro() {
        let str1 = Field::new("str1");
        let str2 = Field::new("str2");
        let predicate_ref_number = Field::new("predicate_ref_number");
        let result = search::highlight!(str1, str2, predicate_ref_number);
        assert_eq!(
            result.to_raw().build(),
            "search::highlight(str1, str2, predicate_ref_number)"
        );
    }

    #[test]
    fn test_search_highlight_fn_with_parameters() {
        let str1 = Param::new("str1");
        let str2 = Param::new("str2");
        let predicate_ref_number = Param::new("predicate_ref_number");
        let result = search::highlight_fn(str1, str2, predicate_ref_number);
        assert_eq!(
            result.to_raw().build(),
            "search::highlight($str1, $str2, $predicate_ref_number)"
        );
    }

    #[test]
    fn test_search_highlight_macro_with_parameters() {
        let str1 = Param::new("str1");
        let str2 = Param::new("str2");
        let predicate_ref_number = Param::new("predicate_ref_number");
        let result = search::highlight!(str1, str2, predicate_ref_number);
        assert_eq!(
            result.to_raw().build(),
            "search::highlight($str1, $str2, $predicate_ref_number)"
        );
    }
}

macro_rules! create_fn_with_single_search_arg_tests {
    ($(#[$attr:meta])* => $function_name:expr, $function_path:expr) => {
        paste::paste! {
            mod [<test_ $function_name>] {
                use umbriel::{functions::search, *};

                #[test]
                fn [<test $function_name _fn>]() {
                    let field = Field::new("field");
                    let result = functions::search::[<$function_name _fn>](field);
                    assert_eq!(
                        result.to_raw().build(),
                        format!("search::{}(field)", $function_path)
                    );
                }

                #[test]
                fn [<test $function_name _macro>]() {
                    let field = Field::new("field");
                    let result = search::[<$function_name>]!(field);
                    assert_eq!(
                        result.to_raw().build(),
                        format!("search::{}(field)", $function_path)
                    );
                }
            }
        }
    };
}

create_fn_with_single_search_arg_tests!(
    =>
    "score",
    "score"
);
create_fn_with_single_search_arg_tests!(
    =>
    "offsets",
    "offsets"
);

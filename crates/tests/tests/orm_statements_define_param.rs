#[cfg(test)]
mod tests {
    use umbriel::{ToRaw, create_param_name_fn};

    use umbriel::{statements::*, *};
    // fn endpoint_base() -> Param {
    //     Param::new("endpoint_base")
    // }

    create_param_name_fn!(endpoint_base_without_doc);

    #[test]
    fn test_define_param_statement_without_doc() {
        let statement = define_param(endpoint_base_without_doc()).value("https://example.com");
        assert_eq!(
            statement.to_raw().build(),
            "DEFINE PARAM $endpoint_base_without_doc VALUE 'https://example.com';"
        );

        assert_eq!(
            statement.fine_tune_params(),
            "DEFINE PARAM $endpoint_base_without_doc VALUE $_param_00000001;"
        );
    }

    create_param_name_fn!(
        /// endpoint of example.com
        =>
        endpoint_base_with_doc
    );

    #[test]
    fn test_define_param_statement() {
        let statement = define_param(endpoint_base_with_doc()).value("https://example.com");
        assert_eq!(
            statement.to_raw().build(),
            "DEFINE PARAM $endpoint_base_with_doc VALUE 'https://example.com';"
        );

        assert_eq!(
            statement.fine_tune_params(),
            "DEFINE PARAM $endpoint_base_with_doc VALUE $_param_00000001;"
        );
    }
}

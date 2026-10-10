#[cfg(test)]
mod tests {
    use aparecium::{ToRaw, Buildable};
    use aparecium::{statements::*, *};

    #[test]
    fn test_remove_function_build_without_prefix() {
        let statement = remove_function("update_author");
        assert_eq!(
            statement.to_raw().build(),
            "REMOVE FUNCTION fn::update_author;"
        );
        assert_eq!(
            statement.fine_tune_params(),
            "REMOVE FUNCTION $_param_00000001;"
        );
        assert_eq!(statement.get_bindings().len(), 1);
    }

    #[test]
    fn test_remove_function_build() {
        let statement = remove_function("fn::update_author");
        assert_eq!(
            statement.to_raw().build(),
            "REMOVE FUNCTION fn::update_author;"
        );
        assert_eq!(
            statement.fine_tune_params(),
            "REMOVE FUNCTION $_param_00000001;"
        );
        assert_eq!(statement.get_bindings().len(), 1);
    }
}

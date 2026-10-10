#[cfg(test)]
mod tests {
    use aparecium::{Field, Param, ToRaw, Buildable};
    use aparecium::{statements::*, *};
    use pretty_assertions::assert_eq;

    #[test]
    fn test_remove_model_build() {
        let statement = remove_model("recommendation").version("1.0.0");
        assert_eq!(
            statement.to_raw().build(),
            "REMOVE MODEL ml::recommendation<1.0.0>;"
        );
        assert_eq!(
            statement.fine_tune_params(),
            "REMOVE MODEL ml::$_param_00000001<$_param_00000002>;"
        );
        assert_eq!(statement.get_bindings().len(), 2);
    }

    #[test]
    fn test_remove_model_build_with_field() {
        let field = Field::new("field");
        let statement = remove_model(field);

        assert_eq!(statement.to_raw().build(), "REMOVE MODEL ml::field;");
        assert_eq!(statement.fine_tune_params(), "REMOVE MODEL ml::field;");
        assert_eq!(statement.get_bindings().len(), 0);
    }

    #[test]
    fn test_remove_model_build_with_param() {
        let param_model_to_remove = Param::new("param_model_to_remove");
        let statement = remove_model(param_model_to_remove);

        assert_eq!(
            statement.to_raw().build(),
            "REMOVE MODEL ml::$param_model_to_remove;"
        );
        assert_eq!(
            statement.fine_tune_params(),
            "REMOVE MODEL ml::$param_model_to_remove;"
        );
        assert_eq!(statement.get_bindings().len(), 0);
    }
}

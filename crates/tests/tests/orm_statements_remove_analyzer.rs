#[cfg(test)]
mod tests {
    use umbriel::{Buildable, Field, Param, ToRaw};
    use umbriel::{statements::*, *};

    #[test]
    fn test_remove_analyzer_build() {
        let statement = remove_analyzer("analyzer::standard");
        assert_eq!(
            statement.to_raw().build(),
            "REMOVE ANALYZER analyzer::standard;"
        );
        assert_eq!(
            statement.fine_tune_params(),
            "REMOVE ANALYZER $_param_00000001;"
        );
        assert_eq!(statement.get_bindings().len(), 1);
    }

    #[test]
    fn test_remove_analyzer_build_with_field() {
        let field = Field::new("field");
        let statement = remove_analyzer(field);
        assert_eq!(statement.to_raw().build(), "REMOVE ANALYZER field;");
        assert_eq!(
            statement.fine_tune_params(),
            "REMOVE ANALYZER $_param_00000001;"
        );
        assert_eq!(statement.get_bindings().len(), 1);
    }

    #[test]
    fn test_remove_analyzer_build_with_param() {
        let param_analyzer_to_remove = Param::new("param_analyzer_to_remove");
        let statement = remove_analyzer(param_analyzer_to_remove.clone());
        assert_eq!(
            statement.to_raw().build(),
            "REMOVE ANALYZER $param_analyzer_to_remove;"
        );
        assert_eq!(
            statement.fine_tune_params(),
            "REMOVE ANALYZER $_param_00000001;"
        );
        assert_eq!(statement.get_bindings().len(), 1);
    }
}

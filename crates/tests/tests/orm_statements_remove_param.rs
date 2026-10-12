#[cfg(test)]
mod tests {
    use umbriel::{Buildable, Param, ToRaw};
    use umbriel::{statements::*, *};

    #[test]
    fn test_remove_param_build() {
        let statement = remove_param("website_name");
        assert_eq!(statement.to_raw().build(), "REMOVE PARAM $website_name;");
        assert_eq!(statement.fine_tune_params(), "REMOVE PARAM $website_name;");
        assert_eq!(statement.get_bindings().len(), 0);
    }

    #[test]
    fn test_remove_param_build_with_param() {
        let param_variable = Param::new("param_variable");
        let statement = remove_param(param_variable);
        assert_eq!(statement.to_raw().build(), "REMOVE PARAM $param_variable;");
        assert_eq!(
            statement.fine_tune_params(),
            "REMOVE PARAM $param_variable;"
        );
        assert_eq!(statement.get_bindings().len(), 0);
    }
}

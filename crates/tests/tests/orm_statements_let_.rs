#[cfg(test)]
mod tests {
    use umbriel::{All, Table, ToRaw, statements::select};

    use umbriel::{statements::*, *};

    #[test]
    fn test_let_statement() {
        let statement = let_("name").equal_to(5);

        assert_eq!(
            statement.fine_tune_params(),
            "LET $name = $_param_00000001;"
        );

        assert_eq!(statement.to_raw().build(), "LET $name = 5;");

        assert_eq!(statement.get_param().build(), "$name");
    }

    #[test]
    fn test_let_statement_with_select_statement() {
        let user = Table::new("user");
        let statement = let_("name").equal_to(select(All).from(user));

        assert_eq!(
            statement.fine_tune_params(),
            "LET $name = $_param_00000001;"
        );

        assert_eq!(
            statement.to_raw().build(),
            "LET $name = (SELECT * FROM user);"
        );

        assert_eq!(statement.get_param().build(), "$name");
    }
}

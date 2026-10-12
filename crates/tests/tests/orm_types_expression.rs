#[cfg(test)]
mod tests {
    use umbriel::{All, NULL, Table, ToRaw, statements::select};

    use umbriel::*;

    #[test]
    fn expression_from_select_statement() {
        let users = Table::new("users");
        let select_statement = select(All).from(users);
        let expression = Expression::from(select_statement);
        assert_eq!(expression.fine_tune_params(), "$_param_00000001");
        assert_eq!(expression.to_raw().build(), "(SELECT * FROM users)");
    }

    #[test]
    fn expression_from_string() {
        let expression = Expression::from("hello");
        assert_eq!(expression.fine_tune_params(), "$_param_00000001");
        assert_eq!(expression.to_raw().build(), "'hello'");
    }

    #[test]
    fn expression_from_integer() {
        let expression = Expression::from(1);
        assert_eq!(expression.fine_tune_params(), "$_param_00000001");
        assert_eq!(expression.to_raw().build(), "1");
    }

    #[test]
    fn expression_from_float() {
        let expression = Expression::from(1.02);
        assert_eq!(expression.fine_tune_params(), "$_param_00000001");
        assert_eq!(expression.to_raw().build(), "1.02f");
    }

    #[test]
    fn expression_from_boolean() {
        let expression = Expression::from(true);
        assert_eq!(expression.fine_tune_params(), "$_param_00000001");
        assert_eq!(expression.to_raw().build(), "true");
    }

    #[test]
    fn expression_from_null() {
        let expression = Expression::from(NULL);
        assert_eq!(expression.fine_tune_params(), "NULL");
        assert_eq!(expression.to_raw().build(), "NULL");
    }
}

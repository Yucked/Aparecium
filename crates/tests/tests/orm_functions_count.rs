#[cfg(test)]
mod tests {
    use umbriel::*;
    use umbriel::{functions::count::*, *};

    #[test]
    fn test_count_withoout_arguments() {
        let result = count_fn(Empty);
        assert_eq!(result.fine_tune_params(), "count()");
        assert_eq!(result.to_raw().to_string(), "count()");
    }

    #[test]
    fn test_count_with_db_field() {
        let email = Field::new("email");
        let result = count_fn(email);
        assert_eq!(result.fine_tune_params(), "count(email)");
        assert_eq!(result.to_raw().to_string(), "count(email)");
    }

    #[test]
    fn test_count_with_simple_field_filter_operation() {
        let email = Field::new("email");
        let result = count_fn(email.greater_than(15));
        assert_eq!(result.fine_tune_params(), "count(email > $_param_00000001)");
        assert_eq!(result.to_raw().to_string(), "count(email > 15)");

        let email = Field::new("email");
        let result = count_fn(email.greater_than(15).or(true));
        assert_eq!(
            result.fine_tune_params(),
            "count(email > $_param_00000001 OR $_param_00000002)"
        );
        assert_eq!(result.to_raw().to_string(), "count(email > 15 OR true)");
    }

    #[test]
    fn test_count_with_complex_field_filter_operation() {
        let email = Field::new("email");
        let age = Field::new("age");
        let result = count_fn(cond(age.greater_than(15)).and(email.like("charlie@example.com")));
        assert_eq!(
            result.fine_tune_params(),
            "count((age > $_param_00000001) AND (email ~ $_param_00000002))"
        );
        assert_eq!(
            result.to_raw().to_string(),
            "count((age > 15) AND (email ~ 'charlie@example.com'))"
        );
    }

    #[test]
    fn test_count_with_array() {
        let result = count_fn(array![1, 2, 3, 4, 5, "4334", "Charlie"]);
        assert_eq!(result.fine_tune_params(), "count($_param_00000001)");
        assert_eq!(
            result.to_raw().to_string(),
            "count([1, 2, 3, 4, 5, '4334', 'Charlie'])"
        );
    }

    #[test]
    fn test_count_macro_withoout_arguments() {
        let result = count!();
        assert_eq!(result.fine_tune_params(), "count()");
        assert_eq!(result.to_raw().to_string(), "count()");
    }

    #[test]
    fn test_count_macro_withoout_arguments_aliased() {
        let head_count = AliasName::new("head_count");
        let result = count!().__as__(head_count);

        assert_eq!(result.fine_tune_params(), "count() AS head_count");
        assert_eq!(result.to_raw().to_string(), "count() AS head_count");
    }

    #[test]
    fn test_count_macro_with_db_field() {
        let email = Field::new("email");
        let result = count!(email);
        assert_eq!(result.fine_tune_params(), "count(email)");
        assert_eq!(result.to_raw().to_string(), "count(email)");
    }

    #[test]
    fn test_count_macro_with_simple_field_filter_operation() {
        let email = Field::new("email");
        let result = count!(email.greater_than(15));
        assert_eq!(result.fine_tune_params(), "count(email > $_param_00000001)");
        assert_eq!(result.to_raw().to_string(), "count(email > 15)");

        let email = Field::new("email");
        let result = count!(email.greater_than(15).or(true));
        assert_eq!(
            result.fine_tune_params(),
            "count(email > $_param_00000001 OR $_param_00000002)"
        );
        assert_eq!(result.to_raw().to_string(), "count(email > 15 OR true)");
    }

    #[test]
    fn test_count_macro_with_complex_field_filter_operation() {
        let email = Field::new("email");
        let age = Field::new("age");
        let result = count!(cond(age.greater_than(15)).and(email.like("charlie@example.com")));
        assert_eq!(
            result.fine_tune_params(),
            "count((age > $_param_00000001) AND (email ~ $_param_00000002))"
        );
        assert_eq!(
            result.to_raw().to_string(),
            "count((age > 15) AND (email ~ 'charlie@example.com'))"
        );
    }

    #[test]
    fn test_count_macro_with_complex_field_filter_operation_aliased() {
        let email = Field::new("email");
        let age = Field::new("age");
        let student_count = AliasName::new("student_count");
        let result = count!(cond(age.greater_than(15)).and(email.like("charlie@example.com")))
            .__as__(student_count);
        assert_eq!(
            result.fine_tune_params(),
            "count((age > $_param_00000001) AND (email ~ $_param_00000002)) AS student_count"
        );
        assert_eq!(
            result.to_raw().to_string(),
            "count((age > 15) AND (email ~ 'charlie@example.com')) AS student_count"
        );
    }
}

#[cfg(test)]
mod tests {
    use umbriel::{Field, Operatable, ToRaw, cond};

    use umbriel::*;

    #[test]
    fn test_filter_simple() {
        let age = Field::new("age");

        let filter = age.greater_than(18);

        assert_eq!(filter.to_raw().build(), "age > 18");
    }

    #[test]
    fn test_filter_simple_with_cond_macro() {
        let age = Field::new("age");
        let filter = cond!(age > 18);

        assert_eq!(filter.to_raw().build(), "age > 18");
    }

    #[test]
    fn test_filter() {
        let age = Field::new("age");
        let title = Field::new("title");

        let filter = cond(age.greater_than(18))
            .or(title.equal("Professor"))
            .and(age.less_than(100));

        assert_eq!(
            filter.to_raw().build(),
            "(age > 18) OR (title = 'Professor') AND (age < 100)"
        );
    }

    #[test]
    fn test_filter_with_cond_macro() {
        let age = Field::new("age");
        let title = Field::new("title");

        let filter = cond!((age > 18) OR (title = "Professor") AND (age < 100));

        assert_eq!(
            filter.to_raw().build(),
            "(age > 18) OR (title = 'Professor') AND (age < 100)"
        );
    }

    #[test]
    fn test_filter_parenthesized() {
        let age = Field::new("age");
        let title = Field::new("title");

        let filter = cond(age.greater_than(18))
            .or(title.equal("Professor"))
            .and(age.less_than(100));

        let parenthesized_filter = filter.parenthesized();
        assert_eq!(
            parenthesized_filter.to_raw().build(),
            "((age > 18) OR (title = 'Professor') AND (age < 100))"
        );
    }

    #[test]
    fn test_filter_parenthesized_with_cond_macro() {
        let age = Field::new("age");
        let title = Field::new("title");

        let filter = cond!((age > 18) OR (title = "Professor") AND (age < 100));

        let parenthesized_filter = filter.parenthesized();
        assert_eq!(
            parenthesized_filter.fine_tune_params(),
            "((age > $_param_00000001) OR (title = $_param_00000002) AND (age < $_param_00000003))"
        );
        assert_eq!(
            parenthesized_filter.to_raw().build(),
            "((age > 18) OR (title = 'Professor') AND (age < 100))"
        );
    }
    #[test]
    fn test_filter_parenthesized_with_cond_macro_nested() {
        let age = Field::new("age");
        let title = Field::new("title");

        let filter = cond!((age OR cond!(age >= 18)) OR (title = "Professor") AND (age < 100));

        let parenthesized_filter = filter.parenthesized();
        assert_eq!(
            parenthesized_filter.to_raw().build(),
            "((age OR age >= 18) OR (title = 'Professor') AND (age < 100))"
        );
    }

    #[test]
    fn test_filter_parenthesized_with_cond_macro_mixed() {
        let age = Field::new("age");
        let title = Field::new("title");

        let filter = cond!((age.or(4).or(545).or(232)) OR (title = "Professor") AND (age < 100));

        let parenthesized_filter = filter.parenthesized();
        assert_eq!(
            parenthesized_filter.fine_tune_params(),
            "((age OR $_param_00000001 OR $_param_00000002 OR $_param_00000003) OR (title = $_param_00000004) AND (age < $_param_00000005))"
        );

        assert_eq!(
            parenthesized_filter.to_raw().build(),
            "((age OR 4 OR 545 OR 232) OR (title = 'Professor') AND (age < 100))"
        );
    }
}

#[cfg(test)]
mod tests {
    use umbriel::{AliasName, ToRaw};

    use umbriel::*;

    #[test]
    fn test_field() {
        let age = Field::new("age");
        let operation = age.greater_than_or_equal(18).less_than_or_equal(56);

        assert_eq!(
            operation.fine_tune_params(),
            "age >= $_param_00000001 <= $_param_00000002"
        );
        assert_eq!(operation.clone().to_raw().to_string(), "age >= 18 <= 56");
    }

    #[test]
    fn test_field_alias() {
        let age = Field::new("age");
        let age_of_human = AliasName::new("age_of_human");

        assert_eq!(age.__as__(age_of_human).build(), "age AS age_of_human");
    }

    #[test]
    fn test_field_with_operation_alias() {
        let age = Field::new("age");
        let legal_age = AliasName::new("legal_age");

        let operation = age.greater_than_or_equal(18).less_than_or_equal(56);
        let operation_aliased = operation.__as__(legal_age);

        assert_eq!(
            operation_aliased.fine_tune_params(),
            "age >= $_param_00000001 <= $_param_00000002 AS legal_age"
        );
        assert_eq!(
            operation_aliased.to_raw().to_string(),
            "age >= 18 <= 56 AS legal_age"
        );
    }
}

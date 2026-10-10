#[cfg(test)]
mod tests {

    use aparecium::{statements::*, *};

    #[test]
    fn test_use_statement() {
        assert_eq!(
            use_().database(Database::from("root".to_string())).build(),
            "USE DB root;"
        );
        assert_eq!(
            use_()
                .namespace(Namespace::from("mars".to_string()))
                .to_string(),
            "USE NS mars;"
        );

        assert_eq!(
            use_()
                .database(Database::from("root".to_string()))
                .namespace(Namespace::from("mars".to_string()))
                .build(),
            "USE DB root NS mars;"
        );
    }
}

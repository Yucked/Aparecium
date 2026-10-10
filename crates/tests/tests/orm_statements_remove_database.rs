#[cfg(test)]
mod tests {
    use aparecium::{statements::*, *};

    #[test]
    fn test_remove_statement() {
        assert_eq!(
            remove_database("oyelowo").build(),
            "REMOVE DATABASE oyelowo;"
        );
    }
}

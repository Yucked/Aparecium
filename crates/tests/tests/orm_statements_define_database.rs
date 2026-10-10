#[cfg(test)]
mod tests {
    use aparecium::{statements::*, *};

    #[test]
    fn test_define_statement() {
        assert_eq!(
            define_database("oyelowo").build(),
            "DEFINE DATABASE oyelowo;"
        );
    }
}

#[cfg(test)]
mod tests {
    use umbriel::{statements::*, *};

    #[test]
    fn test_define_statement() {
        assert_eq!(
            define_database("charlie").build(),
            "DEFINE DATABASE charlie;"
        );
    }
}

#[cfg(test)]
mod tests {
    use umbriel::{statements::*, *};

    #[test]
    fn test_remove_statement() {
        assert_eq!(
            remove_database("charlie").build(),
            "REMOVE DATABASE charlie;"
        );
    }
}

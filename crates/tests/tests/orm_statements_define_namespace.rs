#[cfg(test)]
mod tests {
    use umbriel::{statements::*, *};

    #[test]
    fn test_let_statement() {
        assert_eq!(
            define_namespace("charlie").build(),
            "DEFINE NAMESPACE charlie;"
        );
    }
}

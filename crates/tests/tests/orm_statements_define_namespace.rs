#[cfg(test)]
mod tests {
    use aparecium::{statements::*, *};

    #[test]
    fn test_let_statement() {
        assert_eq!(
            define_namespace("oyelowo").build(),
            "DEFINE NAMESPACE oyelowo;"
        );
    }
}

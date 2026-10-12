#[cfg(test)]
mod tests {
    use umbriel::{statements::*, *};
    use pretty_assertions::assert_eq;

    #[test]
    fn test_remove_scope() {
        let scope = Scope::new("scope");
        let statement = remove_scope(scope);
        assert_eq!(statement.build(), "REMOVE SCOPE scope;");
    }
}

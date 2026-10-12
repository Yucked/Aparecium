#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use umbriel::{statements::*, *};

    #[test]
    fn test_remove_scope() {
        let scope = Scope::new("scope");
        let statement = remove_scope(scope);
        assert_eq!(statement.build(), "REMOVE SCOPE scope;");
    }
}

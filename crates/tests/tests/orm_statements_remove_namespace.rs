#[cfg(test)]
mod tests {
    use aparecium::{statements::*, *};

    #[test]
    fn test_remove_namespace() {
        let namespace = Namespace::new("namespace");
        let statement = remove_namespace(namespace);
        assert_eq!(statement.build(), "REMOVE NAMESPACE namespace;");
    }
}

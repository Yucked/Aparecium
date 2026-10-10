#[cfg(test)]
mod test {
    use aparecium::{statements::*, *};

    #[test]
    fn remove_token_on_namespace() {
        let login = Token::new("login");
        let statement = remove_token(login).on_namespace();
        assert_eq!(statement.build(), "REMOVE TOKEN login ON NAMESPACE;");
    }

    #[test]
    fn remove_token_on_database() {
        let login = Token::new("login");
        let statement = remove_token(login).on_database();
        assert_eq!(statement.build(), "REMOVE TOKEN login ON DATABASE;");
    }
}

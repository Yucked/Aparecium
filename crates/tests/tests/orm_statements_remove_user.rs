#[cfg(test)]
mod test {
    use aparecium::{statements::*, *};

    #[test]
    fn remove_user_on_namespace() {
        let login = User::new("login");
        let statement = remove_user(login).on_namespace();
        assert_eq!(statement.build(), "REMOVE USER login ON NAMESPACE;");
    }

    #[test]
    fn remove_user_on_database() {
        let login = User::new("login");
        let statement = remove_user(login).on_database();
        assert_eq!(statement.build(), "REMOVE USER login ON DATABASE;");
    }
}

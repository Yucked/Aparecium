#[cfg(test)]
mod tests {
    use umbriel::{statements::*, *};

    #[test]
    fn remove_login_on_namespace() {
        let login = Login::new("login");
        let statement = remove_login(login).on_namespace();
        assert_eq!(statement.build(), "REMOVE LOGIN login ON NAMESPACE;");
    }

    #[test]
    fn remove_login_on_database() {
        let login = Login::new("login");
        let statement = remove_login(login).on_database();
        assert_eq!(statement.build(), "REMOVE LOGIN login ON DATABASE;");
    }
}

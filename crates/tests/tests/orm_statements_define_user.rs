#[cfg(test)]
mod tests {
    use umbriel::{ToRaw, User};

    use umbriel::{statements::*, *};

    #[test]
    fn test_define_user_statement_with_passhash_and_role() {
        let username = User::new("username");
        let user_with_passhash_and_role = define_user(username)
            .on_database()
            .passhash("$argon2id$v=19$m=19456,t=2,p=1$u1CPdtdC0Ek5GE1gvidj/g$fjFa7PZM+4hp4hlUJN1fz/FaDAf7KY1Qu48F5m5P0V8")
            .role(UserRole::Owner);

        assert_eq!(
            user_with_passhash_and_role.fine_tune_params(),
            "DEFINE USER $_param_00000001 ON DATABASE PASSHASH $_param_00000002 ROLES OWNER;"
        );

        assert_eq!(
            user_with_passhash_and_role.to_raw().build(),
            "DEFINE USER username ON DATABASE PASSHASH '$argon2id$v=19$m=19456,t=2,p=1$u1CPdtdC0Ek5GE1gvidj/g$fjFa7PZM+4hp4hlUJN1fz/FaDAf7KY1Qu48F5m5P0V8' ROLES OWNER;"
        );

        assert_eq!(user_with_passhash_and_role.get_bindings().len(), 2);
    }

    #[test]
    fn test_define_user_statement_with_password_and_role() {
        let username = User::new("username");
        let user_with_password_and_role = define_user(username)
            .on_root()
            .password("123456")
            .role(UserRole::Owner);

        assert_eq!(
            user_with_password_and_role.fine_tune_params(),
            "DEFINE USER $_param_00000001 ON ROOT PASSWORD $_param_00000002 ROLES OWNER;"
        );

        assert_eq!(
            user_with_password_and_role.to_raw().build(),
            "DEFINE USER username ON ROOT PASSWORD '123456' ROLES OWNER;"
        );
        assert_eq!(user_with_password_and_role.get_bindings().len(), 2);
    }

    #[test]
    fn test_define_user_statement_with_namespace_and_editor_role() {
        let user_with_namespace_and_editor = define_user("username")
            .on_namespace()
            .password("123456")
            .role(UserRole::Editor);

        assert_eq!(
            user_with_namespace_and_editor.fine_tune_params(),
            "DEFINE USER $_param_00000001 ON NAMESPACE PASSWORD $_param_00000002 ROLES EDITOR;"
        );

        assert_eq!(
            user_with_namespace_and_editor.to_raw().build(),
            "DEFINE USER username ON NAMESPACE PASSWORD '123456' ROLES EDITOR;"
        );

        assert_eq!(user_with_namespace_and_editor.get_bindings().len(), 2);
    }

    #[test]
    fn test_define_user_statement_with_database_and_viewer_role() {
        let user_with_database_and_viewer = define_user("username")
            .on_database()
            .password("123456")
            .role(UserRole::Viewer);

        assert_eq!(
            user_with_database_and_viewer.fine_tune_params(),
            "DEFINE USER $_param_00000001 ON DATABASE PASSWORD $_param_00000002 ROLES VIEWER;"
        );

        assert_eq!(
            user_with_database_and_viewer.to_raw().build(),
            "DEFINE USER username ON DATABASE PASSWORD '123456' ROLES VIEWER;"
        );

        assert_eq!(user_with_database_and_viewer.get_bindings().len(), 2);
    }
}

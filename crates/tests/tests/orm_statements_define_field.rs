#[cfg(test)]
mod tests {
    use umbriel::{CrudType::*, statements::for_permission};
    use umbriel::{NONE, Operatable, ToRaw, cond, value};
    use umbriel::{statements::*, *};

    #[test]
    fn test_define_field_statement_full() {
        let name = Field::new("name");
        let user_table = Table::from("user");
        let age = Field::new("age");
        let email = Field::new("email");
        use FieldType::*;

        let statement = define_field(email)
            .on_table(user_table)
            .type_(String)
            .value("example@example.com")
            .assert(cond(value().is_not(NONE)).and(value().like("is_email")))
            .permissions(for_permission(Select).where_(age.greater_than_or_equal(18))) // Single works
            .permissions(for_permission([Create, Update]).where_(name.is("Bennett"))) //Multiple
            .permissions([
                for_permission([Create, Delete]).where_(name.is("Bennett")),
                for_permission(Update).where_(age.less_than_or_equal(130)),
            ]);

        assert_eq!(
            statement.fine_tune_params(),
            "DEFINE FIELD email ON TABLE user TYPE string VALUE $value OR $_param_00000001 \
                ASSERT ($value IS NOT NONE) AND ($value ~ $_param_00000002)\n\
                PERMISSIONS\n\
                FOR select\n\tWHERE age >= $_param_00000003\n\
                FOR create, update\n\tWHERE name IS $_param_00000004\n\
                FOR create, delete\n\tWHERE name IS $_param_00000005\n\
                FOR update\n\tWHERE age <= $_param_00000006;"
        );

        assert_eq!(
            statement.to_raw().build(),
            "DEFINE FIELD email ON TABLE user TYPE string VALUE $value OR 'example@example.com' \
                ASSERT ($value IS NOT NONE) AND ($value ~ 'is_email')\n\
                PERMISSIONS\n\
                FOR select\n\tWHERE age >= 18\n\
                FOR create, update\n\tWHERE name IS 'Bennett'\n\
                FOR create, delete\n\tWHERE name IS 'Bennett'\n\
                FOR update\n\tWHERE age <= 130;"
        );
        insta::assert_snapshot!(statement.fine_tune_params());
        assert_eq!(statement.get_bindings().len(), 6);
    }

    #[test]
    fn test_define_field_statement_simple() {
        use FieldType::*;

        let email = Field::new("email");
        let user_table = Table::from("user");
        let statement = define_field(email).on_table(user_table).type_(String);

        assert_eq!(
            statement.build(),
            "DEFINE FIELD email ON TABLE user TYPE string;"
        );
        insta::assert_snapshot!(statement.fine_tune_params());
        assert_eq!(statement.get_bindings().len(), 0);
    }
}

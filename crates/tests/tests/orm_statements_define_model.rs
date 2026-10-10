#[cfg(test)]
mod tests {
    use aparecium::{CrudType::*, statements::for_permission};
    use aparecium::{Field, Operatable, ToRaw};
    use aparecium::{statements::*, *};
    use pretty_assertions::assert_eq;

    #[test]
    fn test_define_model_statement_full() {
        let model_name = "recommendation";
        let age = Field::new("age");
        let name = Field::new("name");

        let statement = define_model(model_name)
            .version("1.2.3")
            .comment("My ultimate goal in life is to die empty. Share all my ideas, experiences, and values with the world. And leave nothing left inside of me. I want to die empty.")
            .permissions(for_permission(Select).where_(age.greater_than_or_equal(18))) // Single works
            .permissions(for_permission([Create, Update]).where_(name.is("Oyedayo"))) //Multiple
            .permissions([
                for_permission([Create, Delete]).where_(name.is("Oyedayo")),
                for_permission(Update).where_(age.less_than_or_equal(130)),
            ]);

        assert_eq!(
            statement.fine_tune_params(),
            "DEFINE MODEL ml::$_param_00000001<$_param_00000002>\n\
                COMMENT $_param_00000003\n\
                PERMISSIONS\n\
                FOR select\n\tWHERE age >= $_param_00000004\n\
                FOR create, update\n\tWHERE name IS $_param_00000005\n\
                FOR create, delete\n\tWHERE name IS $_param_00000006\n\
                FOR update\n\tWHERE age <= $_param_00000007;"
        );

        assert_eq!(
            statement.to_raw().build(),
            "DEFINE MODEL ml::recommendation<1.2.3>\n\
                COMMENT 'My ultimate goal in life is to die empty. Share all my ideas, experiences, and values with the world. And leave nothing left inside of me. I want to die empty.'\n\
                PERMISSIONS\n\
                FOR select\n\tWHERE age >= 18\n\
                FOR create, update\n\tWHERE name IS 'Oyedayo'\n\
                FOR create, delete\n\tWHERE name IS 'Oyedayo'\n\
                FOR update\n\tWHERE age <= 130;"
        );
        insta::assert_snapshot!(statement.fine_tune_params());
        assert_eq!(statement.get_bindings().len(), 7);
    }

    #[test]
    fn test_define_model_statement_simple() {
        let model_name = "recommendation";
        let statement = define_model(model_name);

        assert_eq!(
            statement.to_raw().build(),
            "DEFINE MODEL ml::recommendation;"
        );
        insta::assert_snapshot!(statement.fine_tune_params());
        assert_eq!(statement.get_bindings().len(), 1);
    }

    #[test]
    fn test_define_model_statement_version() {
        let model_name = "recommendation";
        let statement = define_model(model_name).version("v1.2.3");

        assert_eq!(
            statement.to_raw().build(),
            "DEFINE MODEL ml::recommendation<v1.2.3>;"
        );
        insta::assert_snapshot!(statement.fine_tune_params());
        assert_eq!(statement.get_bindings().len(), 2);
    }
}

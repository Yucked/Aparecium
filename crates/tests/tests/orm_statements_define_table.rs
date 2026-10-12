#[cfg(test)]
mod tests {
    use umbriel::{statements::*, *};

    use CrudType::*;
    use umbriel::{
        statements::{for_permission, select},
        *,
    };
    use surrealdb::sql;

    #[test]
    fn test_define_statement_schemaless_permissions_none() {
        let user = Table::from("user");
        let statement = define_table(user).schemaless().permissions_none();

        assert_eq!(
            statement.fine_tune_params(),
            "DEFINE TABLE user SCHEMALESS PERMISSIONS NONE;"
        );
        assert_eq!(
            statement.to_raw().build(),
            "DEFINE TABLE user SCHEMALESS PERMISSIONS NONE;"
        );
        assert_eq!(statement.get_bindings().len(), 0);
    }

    #[test]
    fn test_define_statement_schemaless() {
        let user = Table::from("user");
        let statement = define_table(user).schemaless().permissions_full();

        assert_eq!(
            statement.fine_tune_params(),
            "DEFINE TABLE user SCHEMALESS PERMISSIONS FULL;"
        );

        assert_eq!(
            statement.to_raw().build(),
            "DEFINE TABLE user SCHEMALESS PERMISSIONS FULL;"
        );

        assert_eq!(statement.get_bindings().len(), 0);
    }

    #[test]
    fn test_define_statement_multiple() {
        let name = Field::new("name");
        let user_table = Table::from("user");
        let age = Field::new("age");
        let country = Field::new("country");
        let fake_id2 = sql::Thing::from(("user".to_string(), "bennett".to_string()));

        let statement = define_table(user_table)
            .drop()
            .as_(
                select(All)
                    .from(fake_id2)
                    .where_(country.is("INDONESIA"))
                    .order_by(age.numeric().desc())
                    .limit(20)
                    .start(5),
            )
            .schemafull()
            .permissions(for_permission(Select).where_(age.greater_than_or_equal(18))) // Single works
            .permissions(for_permission([Create, Delete]).where_(name.is("Bennett"))) //Multiple
            .permissions([
                for_permission([Create, Delete]).where_(name.is("Bennett")),
                for_permission(Update).where_(age.less_than_or_equal(130)),
            ]);

        assert_eq!(
            statement.fine_tune_params(),
            "DEFINE TABLE user DROP SCHEMAFULL AS \n\tSELECT * FROM $_param_00000001 \
                WHERE country IS $_param_00000002 ORDER BY age NUMERIC DESC \
                LIMIT $_param_00000003 START AT $_param_00000004\nPERMISSIONS\n\
                FOR select\n\tWHERE age >= $_param_00000005\nFOR create, delete\n\tWHERE name IS $_param_00000006\n\
                FOR create, delete\n\tWHERE name IS $_param_00000007\nFOR update\n\t\
                WHERE age <= $_param_00000008;"
        );

        assert_eq!(
            statement.to_raw().build(),
            "DEFINE TABLE user DROP SCHEMAFULL AS \n\tSELECT * FROM user:bennett \
                WHERE country IS 'INDONESIA' ORDER BY age NUMERIC DESC LIMIT 20 \
                START AT 5\nPERMISSIONS\nFOR select\n\tWHERE age >= 18\n\
                FOR create, delete\n\tWHERE name IS 'Bennett'\n\
                FOR create, delete\n\tWHERE name IS 'Bennett'\nFOR update\n\tWHERE age <= 130;"
        );
        assert_eq!(statement.get_bindings().len(), 8);
    }
}

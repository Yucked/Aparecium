#[cfg(test)]
mod tests {
    use select::select_value;

    use umbriel::{
        Field, Model, Operatable, Table, TestUser, ToRaw, block, bracket, chain,
        functions::{count, math},
        statements::{define_field, let_, select},
    };

    use umbriel::{statements::*, *};

    #[test]
    fn test_return() {
        let query = return_(1);
        assert_eq!(query.fine_tune_params(), "RETURN $_param_00000001;");
        assert_eq!(query.to_raw().build(), "RETURN 1;");
    }

    #[test]
    fn test_return_with_record() {
        let user = TestUser::create_id("charlie");
        let account = Field::new("account");
        let connection = Field::new("connection");

        let query = return_(user).fetch(&[account, connection]);
        assert_eq!(
            query.fine_tune_params(),
            "RETURN $_param_00000001 FETCH account, connection;"
        );
        assert_eq!(
            query.to_raw().build(),
            "RETURN user:charlie FETCH account, connection;"
        );
    }

    #[test]
    fn test_return_fetch() {
        let query = return_(1).fetch("a");
        assert_eq!(query.fine_tune_params(), "RETURN $_param_00000001 FETCH a;");
        assert_eq!(query.to_raw().build(), "RETURN 1 FETCH a;");
    }

    #[test]
    fn test_return_fetches() {
        let name = Field::new("name");
        let age = Field::new("age");
        let query = return_(1).fetch(vec![name, age]);
        assert_eq!(
            query.fine_tune_params(),
            "RETURN $_param_00000001 FETCH name, age;"
        );
        assert_eq!(query.to_raw().build(), "RETURN 1 FETCH name, age;");
    }

    #[test]
    fn test_return_with_math() {
        let sales = Table::new("sales");
        let metrics = Table::new("metrics");
        let quantity = Field::new("quantity");
        let average_sales = Field::new("average_sales");

        let sales = &let_("sales").equal_to(select_value(quantity).from(sales));
        let total = &let_("total").equal_to(math::sum!(sales));
        let count = &let_("count").equal_to(count!(sales));

        let returned = return_(bracket(total.divide(count)));

        let def = define_field(average_sales).on_table(metrics).value(block(
            chain(sales).chain(total).chain(count).chain(returned),
        ));

        insta::assert_snapshot!(def.to_raw());

        insta::assert_snapshot!(def.fine_tune_params());

        assert_eq!(
            def.to_raw().build(),
            "DEFINE FIELD average_sales ON TABLE metrics VALUE $value OR {\n\
                LET $sales = (SELECT VALUE quantity FROM sales);\n\n\
                LET $total = math::sum($sales);\n\n\
                LET $count = count($sales);\n\n\
                RETURN ($total / $count);\n\
                };"
        );
    }
}

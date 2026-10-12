#[cfg(test)]
mod tests {
    use insta::assert_snapshot;
    use surrealdb::sql;

    use umbriel::{
        statements::{order, select},
        *,
    };

    use umbriel::{statements::*, *};

    #[test]
    fn test_if_statement_primitive() {
        let age = Field::new("age");

        let if_statement = if_(cond(age.gte(18)).and(age.lte(120)))
            .then("Valid".to_string())
            .else_("Invalid".to_string())
            .end();

        assert_eq!(if_statement.get_bindings().len(), 4);
        insta::assert_snapshot!(if_statement.fine_tune_params());
        insta::assert_snapshot!(if_statement.to_raw().build());
    }

    #[test]
    fn test_if_statement1() {
        let age = Field::new("age");
        let user_table = Table::new("user");

        let if_statement1 = if_(age.greater_than_or_equal(18).less_than_or_equal(120))
            .then(select(All).from(user_table))
            .end();

        assert_eq!(if_statement1.get_bindings().len(), 3);
        insta::assert_snapshot!(if_statement1.fine_tune_params());
        insta::assert_snapshot!(if_statement1.to_raw().build());
    }

    #[test]
    fn test_if_statement2() {
        let age = Field::new("age");
        let user_table = Table::new("user");
        let book_table = Table::new("book");

        let if_statement2 = if_(cond(age.gte(18)).and(age.lte(120)))
            .then(select(All).from(user_table).where_(age.gt(34)))
            .else_(select(All).from(book_table))
            .end();
        assert_eq!(if_statement2.get_bindings().len(), 4);

        insta::assert_snapshot!(if_statement2.fine_tune_params());
        insta::assert_snapshot!(if_statement2.to_raw().build());
    }

    #[test]
    fn test_if_statement3() {
        let name = Field::new("name");
        let age = Field::new("age");
        let user_table = Table::new("user");
        let book_table = Table::new("book");

        let if_statement = if_(cond(age.gte(18)).and(age.lte(120)))
            .then(select(All).from(user_table))
            .else_if(name.like("Charlie Bennett"))
            .then(select(All).from(book_table))
            .end();

        assert_eq!(if_statement.get_bindings().len(), 5);
        assert_snapshot!(if_statement.fine_tune_params());
        assert_snapshot!(if_statement.to_raw().build());
    }

    #[test]
    fn test_if_statement4() {
        let name = Field::new("name");
        let age = Field::new("age");

        let user_table = Table::new("user");
        let book_table = Table::new("book");
        let fruit_table = Table::new("fruit");

        let if_statement4 = if_(age.greater_than_or_equal(18).less_than_or_equal(120))
            .then(select(All).from(user_table))
            .else_if(name.like("Charlie Bennett"))
            .then(select(All).from(book_table))
            .else_(select(All).from(fruit_table))
            .end();

        assert_eq!(if_statement4.get_bindings().len(), 6);
        assert_snapshot!(if_statement4.fine_tune_params());
        assert_snapshot!(if_statement4.to_raw().build());
    }

    #[test]
    fn test_if_statement5() {
        let name = Field::new("name");
        let age = Field::new("age");
        let country = Field::new("country");

        let user_table = Table::new("user");
        let book_table = Table::new("book");
        let fruit_table = Table::new("fruit");

        let if_statement5 = if_(age.greater_than_or_equal(18).less_than_or_equal(120))
            .then(select(All).from(user_table.clone()))
            .else_if(name.like("Charlie Bennett"))
            .then(select(All).from(book_table))
            .else_if(cond(country.is("Canada")).or(country.is("Norway")))
            .then(select(All).from(fruit_table))
            .else_(select(All).from(user_table))
            .end();

        assert_eq!(if_statement5.get_bindings().len(), 9);
        assert_snapshot!(if_statement5.fine_tune_params());
        assert_snapshot!(if_statement5.to_raw().build());
    }

    #[test]
    fn test_if_statement6() {
        let name = Field::new("name");
        let age = Field::new("age");
        let country = Field::new("country");
        let city = Field::new("city");
        let fake_id = sql::Thing::from(("user".to_string(), "charlie".to_string()));
        let fake_id2 = sql::Thing::from(("user".to_string(), "bennett".to_string()));

        let statement1 = select(All)
            .from(fake_id)
            .where_(
                cond(city.is("Prince Edward Island"))
                    .and(city.is("NewFoundland"))
                    .or(city.like("Toronto")),
            )
            .order_by(order(&age).numeric())
            .limit(153)
            .start(10)
            .parallel();

        let statement2 = select(All)
            .from(fake_id2)
            .where_(country.is("INDONESIA"))
            .order_by(order(&age).numeric())
            .limit(20)
            .start(5);

        let book_table = Table::new("book");
        let fruit_table = Table::new("fruit");

        let if_statement5 = if_(age.greater_than_or_equal(18).less_than_or_equal(120))
            .then(statement1)
            .else_if(name.like("Charlie Bennett"))
            .then(statement2)
            .else_if(cond(country.is("Canada")).or(country.is("Norway")))
            .then(select(All).from(fruit_table))
            .else_(select(All).from(book_table))
            .end();

        assert_eq!(if_statement5.get_bindings().len(), 9);
        assert_snapshot!(if_statement5.fine_tune_params());
        assert_snapshot!(if_statement5.to_raw().build());
    }
}

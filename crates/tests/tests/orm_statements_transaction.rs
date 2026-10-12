#[cfg(test)]
mod tests {
    use select::CanOrder;

    use umbriel::{statements::select, *};

    use umbriel::{statements::*, *};

    #[test]
    fn test_transaction_commit() {
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
            .order_by(age.numeric().asc())
            .limit(153)
            .start(10)
            .parallel();

        let statement2 = select(All)
            .from(fake_id2)
            .where_(country.is("INDONESIA"))
            .order_by(age.numeric().asc())
            .limit(20)
            .start(5);

        let transaction = begin_transaction()
            .query(statement1)
            .query(statement2)
            .commit_transaction();

        assert_eq!(transaction.get_bindings().len(), 10);

        insta::assert_snapshot!(transaction.fine_tune_params());
        insta::assert_snapshot!(transaction.to_raw().build());
    }

    #[test]
    fn test_transaction_cancel() {
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
            .order_by(age.numeric().asc())
            .limit(153)
            .start(10)
            .parallel();

        let statement2 = select(All)
            .from(fake_id2)
            .where_(country.is("INDONESIA"))
            .order_by(age.numeric().asc())
            .limit(20)
            .start(5);

        let transaction = begin_transaction()
            .query(statement1)
            .query(statement2)
            .cancel_transaction();

        assert_eq!(transaction.get_bindings().len(), 10);

        insta::assert_snapshot!(transaction.fine_tune_params());
        insta::assert_snapshot!(transaction.to_raw().build());
    }
}

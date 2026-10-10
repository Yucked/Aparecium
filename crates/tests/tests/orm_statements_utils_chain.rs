#[cfg(test)]
mod tests {
    use aparecium::{
        All, Field, Operatable, ToRaw, chain, cond, statements::select, Buildable,
    };
    use insta::assert_snapshot;
    use select::CanOrder;
    use surrealdb::sql;

    #[test]
    fn test_chaining() {
        let age = Field::new("age");
        let country = Field::new("country");
        let city = Field::new("city");
        let fake_id = sql::Thing::from(("user".to_string(), "oyelowo".to_string()));
        let fake_id2 = sql::Thing::from(("user".to_string(), "oyedayo".to_string()));
        let fake_id3 = sql::Thing::from(("user".to_string(), "4".to_string()));

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

        let transaction = chain(statement1)
            .chain(statement2)
            .chain(select(All).from(fake_id3));

        assert_snapshot!(transaction.fine_tune_params());
        assert_snapshot!(transaction.to_raw().to_string());
    }
}

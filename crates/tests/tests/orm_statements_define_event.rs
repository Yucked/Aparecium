#[cfg(test)]
mod tests {
    use surrealdb::sql;

    use aparecium::{statements::select, *};
    use aparecium::{statements::*, *};

    #[test]
    fn test_define_event_statement_state_machine() {
        let age = Field::new("age");
        let city = Field::new("city");
        let fake_id = sql::Thing::from(("user".to_string(), "oyelowo".to_string()));
        let user_table = Table::new("user");
        let email_event = Event::new("email");

        let query = define_event(email_event)
            .on_table(user_table)
            .when(cond(age.greater_than_or_equal(18)))
            .then(
                select(All)
                    .from(fake_id)
                    .where_(
                        cond(city.is("Prince Edward Island"))
                            .and(city.is("NewFoundland"))
                            .or(city.like("Toronto")),
                    )
                    .limit(153)
                    .start(10)
                    .parallel(),
            );

        assert_eq!(
            query.fine_tune_params(),
            "DEFINE EVENT email ON TABLE user WHEN age >= $_param_00000001 THEN SELECT * FROM $_param_00000002 WHERE (city IS $_param_00000003) AND (city IS $_param_00000004) OR (city ~ $_param_00000005) LIMIT $_param_00000006 START AT $_param_00000007 PARALLEL;",
        );

        assert_eq!(
            query.to_raw().build(),
            "DEFINE EVENT email ON TABLE user \
                WHEN age >= 18 THEN \
                SELECT * FROM user:oyelowo WHERE (city IS 'Prince Edward Island') AND \
                (city IS 'NewFoundland') OR (city ~ 'Toronto') LIMIT 153 START AT 10 PARALLEL;",
        );
    }
}

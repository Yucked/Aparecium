#[cfg(test)]
mod tests {
    use umbriel::*;
    use umbriel::{statements::*, *};

    #[test]
    fn test_statement_with_alias() {
        let canadian_cities = AliasName::new("legal_age");
        let age = &Field::new("age");
        let city = &Field::new("city");
        let fake_id = sql::Thing::from(("user".to_string(), "charlie".to_string()));
        let ft_city = IndexName::new("ft_city");

        let statement = select(All)
            .omit([age, city])
            .from(fake_id)
            .with_index(ft_city)
            .where_(
                cond(city.is("Prince Edward Island"))
                    .and(city.is("NewFoundland"))
                    .or(city.like("Toronto")),
            )
            // You can call directly on field
            .order_by(age.desc().numeric())
            // Or use order function
            .order_by(order(city).asc())
            .limit(153)
            .start(10)
            .explain_full()
            .parallel();

        let statement_aliased = statement.__as__(canadian_cities);

        assert_eq!(
            statement_aliased.fine_tune_params(),
            "(SELECT * OMIT age, city FROM $_param_00000001 WITH INDEX ft_city WHERE (city IS $_param_00000002) \
                AND (city IS $_param_00000003) \
                OR (city ~ $_param_00000004) \
                ORDER BY age NUMERIC DESC, city ASC LIMIT \
                $_param_00000005 START AT $_param_00000006 PARALLEL EXPLAIN FULL) AS legal_age"
        );
        assert_eq!(
            statement_aliased.to_raw().to_string(),
            "(SELECT * OMIT age, city FROM user:charlie WITH INDEX ft_city WHERE (city IS 'Prince Edward Island') \
                    AND (city IS 'NewFoundland') OR (city ~ 'Toronto') \
                    ORDER BY age NUMERIC DESC, city ASC LIMIT 153 START AT 10 PARALLEL EXPLAIN FULL) AS legal_age"
        );
    }
}

#[cfg(test)]
mod tests {
    use surrealdb::sql;

    use umbriel::{functions::math, *};

    #[test]
    fn test_heterogeonous_array_values() {
        let country = Field::new("country");
        let age = Field::new("age");
        let gender = Field::new("gender");
        let total = AliasName::new("total");
        let mut values = arr![
            1,
            2,
            3,
            count!().__as__(total),
            math::sum!(age),
            gender,
            country,
            54,
            sql::Duration(std::time::Duration::from_secs(43))
        ];
        values.push(34.into());

        assert_eq!(
            values
                .into_iter()
                .map(|m| m.to_raw().build())
                .collect::<Vec<_>>()
                .join(", "),
            "1, 2, 3, count() AS total, math::sum(age), gender, country, 54, 43s, 34"
        );
    }
}

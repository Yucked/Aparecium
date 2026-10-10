#[cfg(test)]
mod tests {
    use surrealdb::sql;

    use aparecium::{Field, ToRaw, statements::let_};
    use aparecium::{functions::meta::*, *};

    #[test]
    fn test_id() {
        let record_id = sql::Thing::from(("person", "oyelowo"));
        let result = id!(record_id);
        assert_eq!(result.to_raw().build(), format!("meta::id(person:oyelowo)"));

        let id_field = Field::new("id_field");
        let result = id!(id_field);
        assert_eq!(result.to_raw().build(), format!("meta::id(id_field)"));

        let id_param = let_("id_param").equal_to("person:oyelowo").get_param();
        let result = id!(id_param);
        assert_eq!(result.to_raw().build(), format!("meta::id($id_param)"));
    }

    #[test]
    fn test_tb() {
        let record_id = sql::Thing::from(("person", "oyelowo"));
        let result = tb!(record_id);
        assert_eq!(result.to_raw().build(), format!("meta::tb(person:oyelowo)"));

        let id_field = Field::new("id_field");
        let result = tb!(id_field);
        assert_eq!(result.to_raw().build(), format!("meta::tb(id_field)"));

        let id_param = let_("id_param").equal_to("person:oyelowo").get_param();
        let result = tb!(id_param);
        assert_eq!(result.to_raw().build(), format!("meta::tb($id_param)"));
    }
}

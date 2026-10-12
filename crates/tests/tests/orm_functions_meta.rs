#[cfg(test)]
mod tests {
    use surrealdb::sql;

    use umbriel::{Field, ToRaw, statements::let_};
    use umbriel::{functions::meta::*, *};

    #[test]
    fn test_id() {
        let record_id = sql::Thing::from(("person", "charlie"));
        let result = id!(record_id);
        assert_eq!(result.to_raw().build(), format!("meta::id(person:charlie)"));

        let id_field = Field::new("id_field");
        let result = id!(id_field);
        assert_eq!(result.to_raw().build(), format!("meta::id(id_field)"));

        let id_param = let_("id_param").equal_to("person:charlie").get_param();
        let result = id!(id_param);
        assert_eq!(result.to_raw().build(), format!("meta::id($id_param)"));
    }

    #[test]
    fn test_tb() {
        let record_id = sql::Thing::from(("person", "charlie"));
        let result = tb!(record_id);
        assert_eq!(result.to_raw().build(), format!("meta::tb(person:charlie)"));

        let id_field = Field::new("id_field");
        let result = tb!(id_field);
        assert_eq!(result.to_raw().build(), format!("meta::tb(id_field)"));

        let id_param = let_("id_param").equal_to("person:charlie").get_param();
        let result = tb!(id_param);
        assert_eq!(result.to_raw().build(), format!("meta::tb($id_param)"));
    }
}

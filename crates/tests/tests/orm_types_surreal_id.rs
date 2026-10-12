#[cfg(test)]
mod tests {
    use surrealdb::sql::Uuid;
    use umbriel::*;

    #[test]
    fn test_create_id_with_model() {
        let id = TestUser::create_id(1);
        assert_eq!(id.to_string(), "user:1");
    }

    #[test]
    fn test_create_id_string_with_model() {
        let id = TestUser::create_id("charlie");
        assert_eq!(id.to_string(), "user:charlie");
    }

    #[test]
    fn test_create_id_uuid_with_model() {
        let id = TestUser::create_id(Uuid::default());
        assert!(id.to_string().contains("user:"));
        assert_eq!(
            id.to_string(),
            "user:⟨00000000-0000-0000-0000-000000000000⟩"
        );
    }

    #[test]
    fn test_create_uuid_with_model() {
        let id = TestUser::create_uuid();
        assert!(id.to_string().contains("user:"));
        assert_eq!(id.to_string().len(), 47);
    }

    #[test]
    fn test_create_surreal_id() {
        let id = TestUserNumberId::new(1);
        assert_eq!(id.to_string(), "user:1");
    }

    #[test]
    fn test_create_surreal_id_with_string() {
        let id = TestUserStringId::new("charlie".into());
        assert_eq!(id.to_string(), "user:charlie");
    }

    #[test]
    fn test_create_surreal_id_with_uuid() {
        let id = TestUserUuid::new();
        assert!(id.to_string().contains("user:"));
        assert_eq!(id.to_string().len(), 47);
    }

    #[test]
    fn test_create_uuid() {
        let id = SurrealUuid::<TestUser>::new();
        assert!(id.to_string().contains("user:"));
        assert_eq!(id.to_string().len(), 47);
    }

    #[test]
    fn test_surreal_id() {
        let id = TestUserNumberId::try_from("table:1").unwrap();
        assert_eq!(id.to_string(), "table:1");
    }

    #[test]
    fn test_surreal_id_from_string() {
        let id = TestUserStringId::try_from("table:charlie").unwrap();
        assert_eq!(id.to_string(), "table:charlie");
    }

    #[test]
    fn test_surreal_id_from_number() {
        let id = TestUserNumberId::try_from("table:1").unwrap();
        assert_eq!(id.to_string(), "table:1");
    }

    #[test]
    fn test_surreal_id_from_str_err() {
        let id = TestUserNumberId::try_from("table1").unwrap_err();
        assert!(
            id.to_string().contains(
                "Invalid id. Problem deserializing string to surrealdb::sql::Thing. \
                Check that the id is in the format 'table:id'"
            ),
            "Invalid id"
        );
    }

    #[test]
    fn test_surreal_id_from_str_err2() {
        let id = TestUserStringId::try_from("table:").unwrap_err();
        assert!(
            id.to_string().contains(
                "Invalid id. Problem deserializing string to surrealdb::sql::Thing. \
                Check that the id is in the format 'table:id'"
            ),
            "Invalid id"
        );
    }

    #[test]
    fn test_surreal_id_from_str_err3() {
        let id = TestUserNumberId::try_from("table:1:2:3").unwrap_err();
        assert!(
            id.to_string().contains(
                "Invalid id. Problem deserializing string to surrealdb::sql::Thing. \
                Check that the id is in the format 'table:id'"
            ),
            "Invalid id"
        );
    }

    #[test]
    fn test_surreal_id_from_str_err4() {
        let id = TestUserNumberId::try_from("table:1:2:3").unwrap_err();
        assert!(
            id.to_string().contains(
                "Invalid id. Problem deserializing string to surrealdb::sql::Thing. \
                Check that the id is in the format 'table:id'"
            ),
            "Invalid id"
        );
    }
}

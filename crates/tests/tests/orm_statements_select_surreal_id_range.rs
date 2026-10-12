#[cfg(test)]
mod tests {
    use umbriel::{statements::select, *};

    macro_rules! gen_test {
        ($range:expr) => {
            let statement = select(All).from($range);
            assert_eq!(
                statement.fine_tune_params(),
                "SELECT * FROM $_param_00000001;"
            );
            let bindings = statement.get_bindings();
            assert_eq!(bindings.len(), 1);
            assert_eq!(
                statement.to_raw().build(),
                format!("SELECT * FROM {};", bindings[0].get_raw_value())
            );
        };
    }

    #[test]
    fn test_range_from_for_simple_surreal_id() {
        let id1 = TestUser::create_simple_id();
        gen_test!(id1..);
    }

    #[test]
    fn test_range_to_for_simple_surreal_id() {
        let id1 = TestUser::create_simple_id();
        gen_test!(..id1);
    }

    #[test]
    fn test_range_to_inclusive_for_simple_surreal_id() {
        let id1 = TestUser::create_simple_id();
        gen_test!(..=id1);
    }

    #[test]
    fn test_range_for_simple_surreal_id() {
        let id1 = TestUser::create_simple_id();
        let id2 = TestUser::create_simple_id();
        gen_test!(id1..id2);
    }

    #[test]
    fn test_range_inclusive_for_surreal_uuid() {
        let id1 = TestUser::create_uuid();
        let id2 = TestUser::create_uuid();
        gen_test!(id1..=id2);
    }

    #[test]
    fn test_range_from_for_surreal_uuid() {
        let id1 = TestUser::create_uuid();
        gen_test!(id1..);
    }

    #[test]
    fn test_range_to_for_surreal_uuid() {
        let id1 = TestUser::create_uuid();
        gen_test!(..id1);
    }

    #[test]
    fn test_range_to_inclusive_for_surreal_uuid() {
        let id1 = TestUser::create_uuid();
        gen_test!(..=id1);
    }

    #[test]
    fn test_range_for_surreal_uuid() {
        let id1 = TestUser::create_uuid();
        let id2 = TestUser::create_uuid();
        gen_test!(id1..id2);
    }

    #[test]
    fn test_range_inclusive_for_surreal_ulid() {
        let id1 = TestUser::create_ulid();
        let id2 = TestUser::create_ulid();
        gen_test!(id1..=id2);
    }

    #[test]
    fn test_range_from_for_surreal_ulid() {
        let id1 = TestUser::create_ulid();
        gen_test!(id1..);
    }

    #[test]
    fn test_range_to_for_surreal_ulid() {
        let id1 = TestUser::create_ulid();
        gen_test!(..id1);
    }

    #[test]
    fn test_range_to_inclusive_for_surreal_ulid() {
        let id1 = TestUser::create_ulid();
        gen_test!(..=id1);
    }

    #[test]
    fn test_range_for_surreal_ulid() {
        let id1 = TestUser::create_ulid();
        let id2 = TestUser::create_ulid();
        gen_test!(id1..id2);
    }

    #[test]
    fn test_range_inclusive_for_surreal_custom() {
        let id1 = TestUser::create_id("charlie");
        let id2 = TestUser::create_id("bennett");
        gen_test!(id1..=id2);
    }

    #[test]
    fn test_range_from_for_surreal_custom() {
        let id1 = TestUser::create_id("charlie");
        gen_test!(id1..);
    }

    #[test]
    fn test_range_to_for_surreal_custom() {
        let id1 = TestUser::create_id("charlie");
        gen_test!(..id1);
    }

    #[test]
    fn test_range_to_inclusive_for_surreal_custom() {
        let id1 = TestUser::create_id("charlie");
        gen_test!(..=id1);
    }

    #[test]
    fn test_range_for_surreal_custom() {
        let id1 = TestUser::create_id("charlie");
        let id2 = TestUser::create_id("bennett");
        gen_test!(id1..id2);
    }
}

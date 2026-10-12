#[cfg(test)]
mod tests {
    use umbriel::{Buildable, ToRaw};
    use umbriel::{statements::*, *};

    #[test]
    fn test_show_changes_for_table_build() {
        let timestamp = chrono::DateTime::from_timestamp(4343434434, 232).unwrap();
        let statement = show_changes_for_table("reading").since(timestamp).limit(10);
        assert_eq!(
            statement.fine_tune_params(),
            "SHOW CHANGES FOR TABLE $_param_00000001 SINCE \"$_param_00000002\" LIMIT $_param_00000003;"
        );
        assert_eq!(
            statement.to_raw().build(),
            "SHOW CHANGES FOR TABLE reading SINCE \"'2107-08-22T05:33:54.000000232Z'\" LIMIT 10;"
        );
    }
}

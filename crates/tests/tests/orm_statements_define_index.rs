#[cfg(test)]
mod tests {
    use umbriel::{ToRaw, arr};

    use umbriel::{statements::*, *};

    #[test]
    fn define_index_with_search_analyzer() {
        let email = Field::new("email");

        let query = define_index("userEmailIndex")
            .on_table("user")
            .fields([email])
            .search_analyzer(
                search_analyzer("ascii")
                    .highlight()
                    .bm25(1.2, 0.75)
                    .doc_ids_order(1)
                    .doc_lengths_order(1)
                    .postings_order(1)
                    .terms_order(1),
            );

        assert_eq!(
            query.to_raw().build(),
            "DEFINE INDEX userEmailIndex ON TABLE user FIELDS email SEARCH ANALYZER ascii HIGHLIGHTS BM25 1.2f 0.75f DOC_IDS_ORDER 1 DOC_LENGTHS_ORDER 1 POSTINGS_ORDER 1 TERMS_ORDER 1;"
        );
        assert_eq!(
            query.fine_tune_params(),
            "DEFINE INDEX userEmailIndex ON TABLE $_param_00000001 FIELDS email SEARCH ANALYZER $_param_00000002 HIGHLIGHTS BM25 $_param_00000003 $_param_00000004 DOC_IDS_ORDER $_param_00000005 DOC_LENGTHS_ORDER $_param_00000006 POSTINGS_ORDER $_param_00000007 TERMS_ORDER $_param_00000008;"
        );
        assert_eq!(query.get_bindings().len(), 8);
    }

    #[test]
    fn test_define_index_statement_single_field() {
        let email = Field::new("email");

        let query = define_index("userEmailIndex")
            .on_table("user")
            .fields(email)
            .unique();

        assert_eq!(
            query.to_raw().build(),
            "DEFINE INDEX userEmailIndex ON TABLE user FIELDS email UNIQUE;"
        );
        assert_eq!(query.get_bindings().len(), 1);
    }

    #[test]
    fn test_define_index_statement_single_column() {
        let email = Field::new("email");

        let query = define_index("userEmailIndex")
            .on_table("user")
            .columns(email)
            .unique();

        assert_eq!(
            query.to_raw().build(),
            "DEFINE INDEX userEmailIndex ON TABLE user COLUMNS email UNIQUE;"
        );
        assert_eq!(query.get_bindings().len(), 1);
    }

    #[test]
    fn test_define_index_statement_multiple_fields() {
        let age = Field::new("age");
        let name = Field::new("name");
        let email = Field::new("email");
        let dob = Field::new("dob");

        let query = define_index("alien_index")
            .on_table("alien")
            .fields(arr![age, name, email, dob])
            .unique();

        assert_eq!(
            query.to_raw().build(),
            "DEFINE INDEX alien_index ON TABLE alien FIELDS age, name, email, dob UNIQUE;"
        );
        assert_eq!(query.get_bindings().len(), 1);
    }

    #[test]
    fn test_define_index_statement_multiple_columns() {
        let age = Field::new("age");
        let name = Field::new("name");
        let email = Field::new("email");
        let dob = Field::new("dob");

        let query = define_index("alien_index")
            .on_table("alien")
            .columns([age, name, email, dob])
            .unique();

        assert_eq!(
            query.to_raw().build(),
            "DEFINE INDEX alien_index ON TABLE alien COLUMNS age, name, email, dob UNIQUE;"
        );
        assert_eq!(query.get_bindings().len(), 1);
    }
}

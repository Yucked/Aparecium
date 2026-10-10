#[cfg(test)]
mod tests {
    use aparecium::Buildable;
    use aparecium::{statements::*, *};

    #[test]
    fn test_info_for_kv_build() {
        let statement = info_for().kv().build();
        assert_eq!(statement, "INFO FOR KV;");
    }

    #[test]
    fn test_info_for_namespace_build() {
        let statement = info_for().namespace().build();
        assert_eq!(statement, "INFO FOR NS;");
    }

    #[test]
    fn test_info_for_database_build() {
        let statement = info_for().database().build();
        assert_eq!(statement, "INFO FOR DB;");
    }

    #[test]
    fn test_info_for_scope_build() {
        let statement = info_for().scope("test_scope").build();
        assert_eq!(statement, "INFO FOR SCOPE test_scope;");
    }

    #[test]
    fn test_info_for_table_build() {
        let statement = info_for().table("test_table").build();
        assert_eq!(statement, "INFO FOR TABLE test_table;");
    }
}

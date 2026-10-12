// A proc-macro crate can only export macros, so its expansion modules are compiled into this
// test target from source; only part of each module is exercised here.
#![allow(dead_code)]

#[path = "../../macros/src/query/mod.rs"]
mod query;
#[path = "../../macros/src/surql.rs"]
mod surql;

mod tests {
    use crate::query::*;
    use quote::{ToTokens, quote};
    use syn::{Expr, parse_quote, parse2};

    #[test]
    fn test_parse_args_without_placeholders() {
        let input = quote! {
            my_db, "SELECT * FROM users"
        };

        let expected_db_conn: Expr = parse_quote! { my_db };
        let expected_query = "SELECT * FROM users".to_string();
        let expected_bindings: Vec<Binding> = vec![];

        let (db_conn, query, bindings) = parse_args(input).unwrap();

        assert_eq!(
            db_conn.to_token_stream().to_string(),
            expected_db_conn.to_token_stream().to_string()
        );
        assert_eq!(query, vec![expected_query]);
        assert_eq!(bindings.len(), expected_bindings.len());
    }

    #[test]
    fn test_parse_args_with_placeholders() {
        let input = quote! {
            my_db, "SELECT * FROM users WHERE id = $id", {
                id: 1
            }
        };

        let expected_db_conn: Expr = parse_quote! { my_db };
        let expected_query = "SELECT * FROM users WHERE id = $id".to_string();
        let expected_bindings = [Binding {
            key: parse_quote! { id },
            value: parse_quote! { 1 },
        }];

        let (db_conn, query, bindings) = parse_args(input).unwrap();

        assert_eq!(
            db_conn.to_token_stream().to_string(),
            expected_db_conn.to_token_stream().to_string()
        );
        assert_eq!(query, vec![expected_query]);
        assert_eq!(bindings.len(), expected_bindings.len());
        assert_eq!(bindings[0].key, expected_bindings[0].key);
        assert_eq!(
            bindings[0].value.to_token_stream().to_string(),
            expected_bindings[0].value.to_token_stream().to_string()
        );
    }

    #[test]
    fn test_parse_args_unexpected_tokens() {
        let input = quote! {
            my_db, "SELECT * FROM users", {} something_else
        };

        assert!(parse_args(input).is_err());
    }

    #[test]
    fn test_parse_args_multiple_queries() {
        let input = quote! {
            my_db,
            [
                "SELECT * FROM users WHERE id = $id",
                "CREATE user:charlie SET name = 'Charlie', company = 'ExampleCo', skills = ['Rust', 'python', 'typescript']"
            ], {
                id: 1
            }
        };

        let expected_db_conn: Expr = parse2(quote! { my_db }).unwrap();
        let expected_queries = vec![
            "SELECT * FROM users WHERE id = $id".to_string(),
            "CREATE user:charlie SET name = 'Charlie', company = 'ExampleCo', skills = ['Rust', 'python', 'typescript']".to_string()
        ];
        let expected_bindings = [Binding {
            key: parse2(quote! { id }).unwrap(),
            value: parse2(quote! { 1 }).unwrap(),
        }];

        let (db_conn, queries, bindings) = parse_args(input).expect("Failed to parse arguments");

        assert_eq!(
            quote! { #db_conn }.to_string(),
            quote! { #expected_db_conn }.to_string(),
            "Database connection parsing failed or mismatched."
        );

        assert_eq!(
            queries, expected_queries,
            "Queries parsing failed or mismatched."
        );
        assert_eq!(
            bindings.len(),
            expected_bindings.len(),
            "Number of bindings parsed does not match expected."
        );

        for (binding, expected_binding) in bindings.iter().zip(expected_bindings.iter()) {
            let binding_key = binding.key.to_string();
            let binding_value = binding.value.to_token_stream().to_string();

            let expected_binding_key = expected_binding.key.to_string();
            let expected_binding_value = expected_binding.value.to_token_stream().to_string();

            assert_eq!(binding_key, expected_binding_key, "Binding key mismatch.");
            assert_eq!(
                binding_value, expected_binding_value,
                "Binding value mismatch."
            );
        }
    }
}

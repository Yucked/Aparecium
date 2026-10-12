// A proc-macro crate can only export macros, so its expansion modules are compiled into this
// test target from source; only part of each module is exercised here.
#![allow(dead_code)]

#[path = "../../macros/src/helpers.rs"]
mod helpers;
#[path = "../../macros/src/migrations/mod.rs"]
mod migrations;

mod tests {
    use crate::migrations::table::*;
    use darling::FromDeriveInput;
    use quote::{ToTokens, quote};
    use syn::parse_quote;

    #[test]
    fn test_table_migration_schema_derive_attributes() {
        let input = parse_quote! {
            struct TableMigrationSchema;
        };

        let derive_input = TableMigrationSchemaDeriveAttributes::from_derive_input(&input).unwrap();
        let mut tokens = proc_macro2::TokenStream::new();
        derive_input.to_tokens(&mut tokens);

        let expected = quote! {
            impl umbriel::TableResources for TableMigrationSchema {}
        };

        assert_eq!(tokens.to_string(), expected.to_string());
    }
}

use crate::helpers::get_crate_name;
use darling::FromDeriveInput;
use quote::{ToTokens, quote};

#[derive(Debug, FromDeriveInput)]
pub struct TableMigrationSchemaDeriveAttributes {
    pub(crate) ident: syn::Ident,
    pub(crate) generics: syn::Generics,
}

impl ToTokens for TableMigrationSchemaDeriveAttributes {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let crate_name = get_crate_name(false);
        let TableMigrationSchemaDeriveAttributes {
            ident: struct_name_ident,
            generics,
        } = self;
        let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

        tokens.extend(quote! {
            impl #impl_generics #crate_name::TableResources for #struct_name_ident #ty_generics #where_clause {}
        });
    }
}

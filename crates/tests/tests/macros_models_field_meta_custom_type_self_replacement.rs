// A proc-macro crate can only export macros, so its expansion modules are compiled into this
// test target from source; only part of each module is exercised here.
#![allow(dead_code)]

#[path = "../../macros/src/field_type.rs"]
mod field_type;
#[path = "../../macros/src/helpers.rs"]
mod helpers;
#[path = "../../macros/src/models/mod.rs"]
mod models;

mod tests {
    use quote::{format_ident, quote};

    use crate::models::field_meta::custom_type_self_replacement::*;
    use syn::{AngleBracketedGenericArguments, Ident, Path, Type, TypePath, parse_quote};

    #[test]
    fn replace_self() {
        let struct_ident = Ident::new("User", proc_macro2::Span::call_site());
        let generics = AngleBracketedGenericArguments {
            colon2_token: None,
            lt_token: Default::default(),
            args: vec_to_punctuated(vec![
                parse_quote!('a),
                parse_quote!('b),
                parse_quote!(U),
                parse_quote!(T),
                // GenericArgument::Lifetime(syn::Lifetime::new("'a", proc_macro2::Span::call_site())),
                // GenericArgument::Lifetime(syn::Lifetime::new("'b", proc_macro2::Span::call_site())),
                // GenericArgument::Type(Type::Path(TypePath {
                //     qself: None,
                //     path: Path::from(Ident::new("U", proc_macro2::Span::call_site())),
                // })),
                // GenericArgument::Type(Type::Path(TypePath {
                //     qself: None,
                //     path: Path::from(Ident::new("V", proc_macro2::Span::call_site())),
                // })),
            ]),
            gt_token: Default::default(),
        };

        let ty_to_replace = Type::Path(TypePath {
            qself: None,
            path: Path::from(Ident::new("Self", proc_macro2::Span::call_site())),
        });

        let mut replacer = ReplaceSelfVisitor {
            struct_ident,
            generics,
        };
        let ty_to_replace = replacer.replace_self(&ty_to_replace.into());

        assert_eq!(
            quote!(#ty_to_replace).to_string(),
            quote!(User<'a, 'b, U, T , >).to_string()
        );
    }

    #[test]
    fn replace_self_using_quote() {
        let struct_ident = format_ident!("User");
        let generics = quote!(<'a, 'b, U, V>);
        let generics: AngleBracketedGenericArguments = syn::parse2(generics).unwrap();
        let type_with_self = quote!(Self<'a, 'b, U, V>);

        let mut replacer = ReplaceSelfVisitor {
            struct_ident,
            generics,
        };
        let type_with_self: syn::Type = syn::parse2(type_with_self).unwrap();
        let ty_to_replace = replacer.replace_self(&type_with_self.into());

        assert_eq!(
            quote::quote!(#ty_to_replace).to_string(),
            quote::quote!(User<'a, 'b, U, V>).to_string()
        );
    }
}

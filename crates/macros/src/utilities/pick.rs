/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 */
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::{
    Generics, Ident, Path, Result, Token,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
};

use crate::models::create_tokenstream_wrapper;

// `pub(crate)` fields are read by `crates/tests`, which compiles this module via `#[path]`.
pub struct PickedMeta {
    attributes: Vec<syn::Attribute>,
    pub(crate) new_struct: Ident,
    pub(crate) old_struct: Ident,
    old_struct_trait: Path,
    pub(crate) generics_without_bounds: Generics,
    pub(crate) field_names: Vec<FieldPicked>,
}

create_tokenstream_wrapper!( => GenericsWithOmiitedAsPhantomData);
create_tokenstream_wrapper!( => FilteredEmptyGenerics);

impl PickedMeta {
    fn map_empty_generics_to_phantom_placeholder(&self) -> GenericsWithOmiitedAsPhantomData {
        let generics = &self.generics_without_bounds;
        let new_generics_params = generics.params.iter().enumerate().map(|(i, param)| {
            let is_last = i == generics.params.len() - 1;
            let separator = if is_last {
                quote! {}
            } else {
                quote! {,}
            };
            match param {
                syn::GenericParam::Type(type_param) => {
                    let ident = &type_param.ident;
                    if *ident == "_" {
                        quote! { ::std::marker::PhantomData<dyn ::std::any::Any> #separator }
                    } else {
                        quote! { #ident #separator }
                    }
                }
                syn::GenericParam::Lifetime(lifetime) => {
                    let lifetime = &lifetime.lifetime;
                    quote! { #lifetime #separator }
                }
                syn::GenericParam::Const(const_param) => {
                    let ident = &const_param.ident;
                    quote! { #ident #separator }
                }
            }
        });
        let no_generics = generics.into_token_stream().is_empty();
        let new_generics_type_args = if no_generics {
            quote! {}
        } else {
            quote!( <#(#new_generics_params)* >)
        };
        new_generics_type_args.into()
    }

    fn filter_empty_generic_params(&self) -> FilteredEmptyGenerics {
        let generics = &self.generics_without_bounds;

        let filtered_generics = generics.params.iter().filter(|param| match param {
            syn::GenericParam::Type(type_param) => {
                let ident = &type_param.ident;
                *ident != "_"
            }
            _ => true,
        });

        let mut generics = Generics::default();
        for param in filtered_generics {
            generics.params.push(param.clone());
        }

        quote! {#generics}.into()
    }
}

pub(crate) struct FieldPicked {
    pub(crate) name: Ident,
    attrs: Vec<syn::Attribute>,
}

impl Parse for FieldPicked {
    fn parse(input: ParseStream) -> Result<Self> {
        let attrs = input.call(syn::Attribute::parse_outer)?;
        let name = input.parse()?;
        Ok(FieldPicked { name, attrs })
    }
}

impl Parse for PickedMeta {
    fn parse(input: ParseStream) -> Result<Self> {
        let attributes = input.call(syn::Attribute::parse_outer)?;
        let new_struct = input.parse()?;
        input.parse::<Token![,]>()?;

        let old_struct = input.parse()?;
        let generics = input.parse::<Generics>()?;

        let _as = input.parse::<Token![as]>()?;

        let pickee_struct_trait = input.parse::<Path>()?;

        input.parse::<Token![,]>()?;

        let content;
        let _brace_token = syn::bracketed!(content in input);

        let fields_names = Punctuated::<FieldPicked, Token![,]>::parse_terminated(&content)?;

        Ok(PickedMeta {
            attributes,
            new_struct,
            old_struct,
            old_struct_trait: pickee_struct_trait,
            generics_without_bounds: generics,
            field_names: fields_names.into_iter().collect(),
        })
    }
}

impl ToTokens for PickedMeta {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self {
            new_struct,
            old_struct,
            old_struct_trait,
            field_names,
            attributes,
            ..
        } = &self;

        let new_generics_type_args = self.map_empty_generics_to_phantom_placeholder().0;
        let filtered_generics = self.filter_empty_generic_params();

        let fields = field_names.iter().map(|field| {
            let FieldPicked { name, attrs } = field;
            quote! {
                #(#attrs)*
                #name: <#old_struct #new_generics_type_args as #old_struct_trait> ::#name,
            }
        });

        tokens.extend(quote! {
            #(#attributes)*
            // #[derive(::serde::Deserialize)]
            pub struct #new_struct #filtered_generics {
                #(
                    #fields
                )*
                // #(
                //     #field_names: <#old_struct #new_generics_type_args as #old_struct_trait> ::#field_names,
                // )*
            }
        });
    }
}

use proc_macro_crate::{FoundCrate, crate_name};
use proc_macro2::{Span, TokenStream, TokenTree};
use quote::quote;

use syn::Ident;

pub fn get_crate_name(internal: bool) -> TokenStream {
    if internal {
        quote! { crate }
    } else {
        let name = match crate_name("umbriel") {
            Ok(FoundCrate::Name(name)) => name,
            Ok(FoundCrate::Itself) | Err(_) => "umbriel".to_string(),
        };
        TokenTree::from(Ident::new(&name, Span::call_site())).into()
    }
}

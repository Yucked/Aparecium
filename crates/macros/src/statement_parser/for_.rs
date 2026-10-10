/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 */

use crate::helpers::get_crate_name;
use proc_macro::TokenStream;
use quote::{ToTokens, quote};
use syn::{
    Expr, Ident,
    parse::{Parse, ParseStream},
    token::{self, Brace},
};

use super::{
    helpers::generate_variable_name,
    query_chain::{GeneratedCode, QueriesChainParser},
};

#[derive(Debug, Clone)]
pub enum Iterable {
    Expr(Expr),
    Ident(Ident),
}

impl ToTokens for Iterable {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            Iterable::Expr(expr) => expr.to_tokens(tokens),
            Iterable::Ident(ident) => ident.to_tokens(tokens),
        }
    }
}

impl Parse for Iterable {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let fork = input.fork();
        fork.parse::<Ident>()?;

        if (fork.is_empty()) || (fork.peek(token::Brace)) {
            let ident = input.parse()?;
            Ok(Iterable::Ident(ident))
        } else {
            let expr = input.parse()?;
            Ok(Iterable::Expr(expr))
        }
    }
}

#[derive(Debug, Clone)]
pub struct ForLoopMetaParser {
    pub iteration_param: Ident,
    pub iterable: Iterable,
    pub body: QueriesChainParser,
    pub generated_ident: Ident,
}

impl Parse for ForLoopMetaParser {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let generated_ident = generate_variable_name();

        // The iteration parameter and the iterable in the start of the for loop
        if input.peek(token::Paren) {
            let iter_content;
            let _paranthesized_iter_content_token = syn::parenthesized!(iter_content in input);
            let iteration_param = iter_content.parse()?;

            iter_content.parse::<syn::Token![in]>()?;

            let iterable = iter_content.parse()?;
            // The body
            let content;
            let _brace_token: Brace = syn::braced!(content in input);

            let body = content.parse()?;

            input.parse::<syn::Token![;]>()?;

            Ok(ForLoopMetaParser {
                iteration_param,
                iterable,
                body,
                generated_ident,
            })
        } else {
            let iteration_param = input.parse()?;

            input.parse::<syn::Token![in]>()?;

            let iterable = input.parse()?;
            // The body
            let content;
            let _brace_token: Brace = syn::braced!(content in input);

            let body = content.parse()?;

            input.parse::<syn::Token![;]>()?;

            Ok(ForLoopMetaParser {
                iteration_param,
                iterable,
                body,
                generated_ident,
            })
        }
    }
}

#[derive(Debug, Clone)]
pub struct ForLoopStatementParser {
    pub meta_content: Box<ForLoopMetaParser>,
}

impl std::ops::Deref for ForLoopStatementParser {
    type Target = ForLoopMetaParser;

    fn deref(&self) -> &Self::Target {
        &self.meta_content
    }
}

impl Parse for ForLoopStatementParser {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse::<syn::Token![for]>()?;
        let for_loop = input.parse::<ForLoopMetaParser>()?;
        Ok(ForLoopStatementParser {
            meta_content: Box::new(for_loop),
        })
    }
}

pub struct TokenizedForLoop {
    pub code_to_render: TokenStream,
    pub query_chain: TokenStream,
}

impl ForLoopMetaParser {
    pub fn tokenize(&self) -> TokenizedForLoop {
        let ForLoopMetaParser {
            iteration_param,
            iterable,
            body,
            generated_ident: _,
        } = self;

        // let iterable = match iterable {
        //     Iterable::Expr(expr) => quote!(expr),
        //     Iterable::Ident(ident) => {
        //         let crate_name = get_crate_name(false);
        //         quote!(#crate_name::iterable::Iterable::from(#ident))
        //     }
        // };
        let GeneratedCode {
            to_render,
            query_chain_var_ident,
        } = body.generate_code();

        let crate_name = get_crate_name(false);

        let whole_stmts = quote!(
        {
            let #iteration_param = #crate_name::Param::new(stringify!(#iteration_param));

            #( #to_render )*

            #crate_name::statements::for_(#iteration_param).in_(#iterable)
            .block(
                #query_chain_var_ident
            )
        });

        let to_render = quote! {
            {
                #( #to_render )*

                #whole_stmts
            }
        }
        .into();
        let to_chain = quote!(#whole_stmts);

        TokenizedForLoop {
            code_to_render: to_render,
            query_chain: to_chain.into(),
        }
    }
}

pub fn for_loop_without_for_keyword(input: TokenStream) -> TokenStream {
    let for_loop_content = syn::parse_macro_input!(input as ForLoopMetaParser);

    let z = &for_loop_content.tokenize();
    let to_render: proc_macro2::TokenStream = z.code_to_render.clone().into();
    let to_chain: proc_macro2::TokenStream = z.query_chain.clone().into();

    quote!(
        #to_render

        #to_chain

    )
    .into()
}
///
/// A helper function to create a for loop
/// ```rust, ignore
/// use aparecium::{*, statements::{for_, select, select_value}};
///
/// let ref person_table = Table::from("person");
/// let ref user_name = Field::from("user_name");
///
/// for_!(name in vec!["Oyelowo", "Oyedayo"] {
///    select(All).from(person_table).where_(user_name.eq(name));
///    select(All).from(person_table).where_(user_name.eq(name));
///
///    for_!((name in select_value(user_name).from_only(person_table)) {
///         select(All).from(person_table).where_(user_name.eq(name));
///         select(All).from(person_table).where_(user_name.eq(name));
///    });
/// });
/// ```
// #[macro_export]
macro_rules! _for_loop {
    (($param:ident in $iterable:expr) { $($stmt:expr;)+ }) => {{
        // let ref $param = $crate::Param::new(stringify!($param));
        // $crate::statements::for_($param).in_($iterable).block($crate::internal_tools::query_turbo! {
        //     $($stmt;)+
        // })
    }};
    (($param:ident IN $iterable:expr) { $($stmt:expr;)+ }) => {{
        // let ref $param = $crate::Param::new(stringify!($param));
        // $crate::statements::for_($param).in_($iterable).block($crate::internal_tools::query_turbo! {
        //     $($stmt;)+
        // })
    }};
}
// pub use _for_loop as for_;

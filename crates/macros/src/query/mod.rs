use proc_macro2::{Ident, Span, TokenStream, TokenTree};
use quote::ToTokens;
use syn::parse::{Parse, ParseStream};
use syn::parse2;
use syn::punctuated::Punctuated;
use syn::token::{Brace, Comma};
use syn::{Expr, LitStr, Result, Token};

// `pub(crate)` items here are reached by `crates/tests`, which compiles this module via `#[path]`.
pub(crate) struct Binding {
    pub(crate) key: Ident,
    pub(crate) value: Expr,
}

struct Bindings {
    pairs: Punctuated<Binding, Comma>,
}

impl Parse for Binding {
    fn parse(input: ParseStream) -> Result<Self> {
        let key: Ident = input.parse()?;
        input.parse::<Token![:]>()?;
        let value: Expr = input.parse()?;
        Ok(Binding { key, value })
    }
}

impl Parse for Bindings {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        let _brace_token: Brace = syn::braced!(content in input);
        let pairs = Punctuated::parse_terminated(&content)?;
        Ok(Bindings { pairs })
    }
}

pub(crate) struct Queries {
    query_strings: Vec<LitStr>,
}

impl Parse for Queries {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        let _bracket_token: syn::token::Bracket = syn::bracketed!(content in input);
        let query_strings = Punctuated::<LitStr, Comma>::parse_terminated(&content)?
            .into_iter()
            .collect();
        Ok(Queries { query_strings })
    }
}

pub(crate) fn parse_args(input: TokenStream) -> Result<(Expr, Vec<String>, Vec<Binding>)> {
    let input2: TokenStream = input.clone();
    let mut iter = input2.into_iter();

    let database_connection = iter
        .next()
        .ok_or_else(|| syn::Error::new(Span::call_site(), "Expected a db connection"))
        .and_then(|tt| syn::parse2::<Expr>(tt.into()))?;

    match iter.next() {
        Some(TokenTree::Punct(ref punct)) if punct.as_char() == ',' => {}
        _ => {
            return Err(syn::Error::new(
                Span::call_site(),
                "Expected a comma after the DB connection",
            ));
        }
    }

    let literals = match iter.next() {
        Some(proc_macro2::TokenTree::Literal(lit)) => {
            vec![syn::parse2::<LitStr>(lit.to_token_stream())?]
        }
        Some(TokenTree::Group(group)) if group.delimiter() == proc_macro2::Delimiter::Bracket => {
            parse2::<Queries>(group.into_token_stream())?.query_strings
        }
        _ => {
            return Err(syn::Error::new(
                Span::call_site(),
                "Expected a SQL query string literal",
            ));
        }
    };

    for literal in &literals {
        crate::surql::validate(&literal.value(), literal.span())?;
    }
    let queries: Vec<String> = literals.iter().map(LitStr::value).collect();

    let has_placeholders = queries.join(";").contains('$');
    let mut bindings = Vec::new();

    if has_placeholders {
        match iter.next() {
            Some(TokenTree::Punct(ref punct)) if punct.as_char() == ',' => {}
            _ => {
                return Err(syn::Error::new(
                    Span::call_site(),
                    "Expected a comma after the SQL query string literal",
                ));
            }
        }

        let bindings_tokens = match iter.next() {
            Some(TokenTree::Group(group)) if group.delimiter() == proc_macro2::Delimiter::Brace => {
                group.into_token_stream()
            }
            _ => {
                return Err(syn::Error::new(
                    Span::call_site(),
                    "Expected curly braces for bindings",
                ));
            }
        };

        let Bindings { pairs } = parse2(bindings_tokens)?;
        bindings = pairs.into_iter().collect::<Vec<_>>();

        if iter.next().is_some() {
            return Err(syn::Error::new(
                Span::call_site(),
                "Unexpected tokens after bindings block",
            ));
        }
    } else {
        // If there are no placeholders, bindings are optional.
        if let Some(TokenTree::Punct(ref punct)) = iter.next() {
            if punct.as_char() == ',' {
                let bindings_tokens = match iter.next() {
                    Some(TokenTree::Group(group))
                        if group.delimiter() == proc_macro2::Delimiter::Brace =>
                    {
                        group.into_token_stream()
                    }
                    _ => {
                        return Err(syn::Error::new(
                            Span::call_site(),
                            "Expected curly braces for bindings",
                        ));
                    }
                };

                let Bindings { pairs } = parse2(bindings_tokens)?;
                bindings = pairs.into_iter().collect::<Vec<_>>();

                if iter.next().is_some() {
                    return Err(syn::Error::new(
                        Span::call_site(),
                        "Unexpected tokens after bindings block",
                    ));
                }
            } else {
                return Err(syn::Error::new(
                    Span::call_site(),
                    "Expected a comma after the SQL query string literal",
                ));
            }
        }
    }
    Ok((database_connection, queries, bindings))
}

fn validate_and_parse_sql_query(query: &str, bindings: &[Binding]) -> syn::Result<String> {
    for binding in bindings {
        let placeholder = format!("${}", binding.key);
        if !query.contains(&placeholder) {
            return Err(syn::Error::new(
                Span::call_site(),
                format!(
                    "No placeholder found in SQL query for binding '{}'",
                    binding.key
                ),
            ));
        }
    }

    let placeholders = query.match_indices('$').peekable();

    for (start, _) in placeholders {
        let end = query[start + 1..]
            .find(|c: char| !c.is_alphanumeric() && c != '_')
            .map_or(query.len(), |i| start + i + 1);

        let placeholder_name = &query[start + 1..end];

        if !bindings.iter().any(|b| b.key == placeholder_name) {
            return Err(syn::Error::new(
                Span::call_site(),
                format!("No binding found for placeholder '${}'", placeholder_name),
            ));
        }
    }

    Ok(query.to_owned())
}

pub fn query(args: TokenStream) -> TokenStream {
    let (db_con, query_strs, bindings) = parse_args(args).expect("Failed to parse arguments");
    let mut output = TokenStream::new();

    let _sql_queries = validate_and_parse_sql_query(&query_strs.join(";"), &bindings)
        .expect("Failed to validate SQL query");

    for (i, query_str) in query_strs.iter().enumerate() {
        let is_first = i == 0;
        if is_first {
            output.extend(quote::quote! {
                #db_con.query(#query_str)
            });
        } else {
            output.extend(quote::quote! {
                .query(#query_str)
            });
        }
    }

    // Binding names are parameter names chosen by the caller: an invalid one, including one using
    // the prefix reserved for bound values, fails compilation at the name.
    let crate_name = crate::helpers::get_crate_name(false);
    let mut name_checks = TokenStream::new();
    for Binding { key, value } in bindings {
        let name = key.to_string();
        name_checks.extend(quote::quote_spanned! {key.span()=>
            const _: () = #crate_name::Param::assert_valid(#name);
        });
        output.extend(quote::quote! {
            .bind((#name, #value))
        });
    }

    quote::quote!({
        #name_checks
        #output
    })
}

// Field types are known to the macros as the SDK's `Kind` and reach generated code as tokens that
// rebuild the same value. `ToTokens` cannot be implemented here for the external `Kind` (orphan
// rule), hence a function.

use proc_macro2::TokenStream;
use quote::quote;
use surrealdb_types::{GeometryKind, Kind, KindLiteral};

/// Tokens constructing `kind` through `<crate_name>::types`, so renamed dependencies resolve.
pub(crate) fn kind_tokens(kind: &Kind, crate_name: &TokenStream) -> TokenStream {
    let types = quote!(#crate_name::types);
    let kinds = |kinds: &[Kind]| {
        let kinds = kinds.iter().map(|kind| kind_tokens(kind, crate_name));
        quote!(::std::vec![#(#kinds),*])
    };
    let tables = |tables: &[surrealdb_types::Table]| {
        let tables = tables.iter().map(surrealdb_types::Table::as_str);
        quote!(::std::vec![#(#types::Table::new(#tables)),*])
    };
    let boxed = |kind: &Kind| {
        let kind = kind_tokens(kind, crate_name);
        quote!(::std::boxed::Box::new(#kind))
    };
    let length = |length: &Option<u64>| match length {
        Some(length) => quote!(::std::option::Option::Some(#length)),
        None => quote!(::std::option::Option::None),
    };

    match kind {
        Kind::Any => quote!(#types::Kind::Any),
        Kind::None => quote!(#types::Kind::None),
        Kind::Null => quote!(#types::Kind::Null),
        Kind::Bool => quote!(#types::Kind::Bool),
        Kind::Bytes => quote!(#types::Kind::Bytes),
        Kind::Datetime => quote!(#types::Kind::Datetime),
        Kind::Decimal => quote!(#types::Kind::Decimal),
        Kind::Duration => quote!(#types::Kind::Duration),
        Kind::Float => quote!(#types::Kind::Float),
        Kind::Int => quote!(#types::Kind::Int),
        Kind::Number => quote!(#types::Kind::Number),
        Kind::Object => quote!(#types::Kind::Object),
        Kind::String => quote!(#types::Kind::String),
        Kind::Uuid => quote!(#types::Kind::Uuid),
        Kind::Regex => quote!(#types::Kind::Regex),
        Kind::Range => quote!(#types::Kind::Range),
        Kind::Table(names) => {
            let names = tables(names);
            quote!(#types::Kind::Table(#names))
        }
        Kind::Record(names) => {
            let names = tables(names);
            quote!(#types::Kind::Record(#names))
        }
        Kind::Geometry(geometries) => {
            let geometries = geometries.iter().map(|geometry| match geometry {
                GeometryKind::Point => quote!(Point),
                GeometryKind::Line => quote!(Line),
                GeometryKind::Polygon => quote!(Polygon),
                GeometryKind::MultiPoint => quote!(MultiPoint),
                GeometryKind::MultiLine => quote!(MultiLine),
                GeometryKind::MultiPolygon => quote!(MultiPolygon),
                GeometryKind::Collection => quote!(Collection),
            });
            quote!(#types::Kind::Geometry(::std::vec![#(#types::GeometryKind::#geometries),*]))
        }
        Kind::Either(variants) => {
            let variants = kinds(variants);
            quote!(#types::Kind::Either(#variants))
        }
        Kind::Set(item, max) => {
            let (item, max) = (boxed(item), length(max));
            quote!(#types::Kind::Set(#item, #max))
        }
        Kind::Array(item, max) => {
            let (item, max) = (boxed(item), length(max));
            quote!(#types::Kind::Array(#item, #max))
        }
        Kind::Function(arguments, returns) => {
            let arguments = match arguments {
                Some(arguments) => {
                    let arguments = kinds(arguments);
                    quote!(::std::option::Option::Some(#arguments))
                }
                None => quote!(::std::option::Option::None),
            };
            let returns = match returns {
                Some(returns) => {
                    let returns = boxed(returns);
                    quote!(::std::option::Option::Some(#returns))
                }
                None => quote!(::std::option::Option::None),
            };
            quote!(#types::Kind::Function(#arguments, #returns))
        }
        Kind::Literal(literal) => {
            let literal = literal_tokens(literal, crate_name);
            quote!(#types::Kind::Literal(#literal))
        }
        Kind::File(buckets) => {
            quote!(#types::Kind::File(::std::vec![#(::std::string::String::from(#buckets)),*]))
        }
    }
}

fn literal_tokens(literal: &KindLiteral, crate_name: &TokenStream) -> TokenStream {
    let types = quote!(#crate_name::types);
    match literal {
        KindLiteral::String(value) => {
            quote!(#types::KindLiteral::String(::std::string::String::from(#value)))
        }
        KindLiteral::Integer(value) => quote!(#types::KindLiteral::Integer(#value)),
        KindLiteral::Float(value) => {
            // `proc_macro2` has no literal for non-finite floats.
            let value = if value.is_nan() {
                quote!(::core::f64::NAN)
            } else if value.is_infinite() && value.is_sign_positive() {
                quote!(::core::f64::INFINITY)
            } else if value.is_infinite() {
                quote!(::core::f64::NEG_INFINITY)
            } else {
                quote!(#value)
            };
            quote!(#types::KindLiteral::Float(#value))
        }
        KindLiteral::Decimal(value) => {
            let (mantissa, scale) = (value.mantissa(), value.scale());
            quote!(#types::KindLiteral::Decimal(
                #types::Decimal::from_i128_with_scale(#mantissa, #scale)
            ))
        }
        KindLiteral::Duration(value) => {
            let value = value.into_inner();
            let (secs, nanos) = (value.as_secs(), value.subsec_nanos());
            quote!(#types::KindLiteral::Duration(#types::Duration::new(#secs, #nanos)))
        }
        KindLiteral::Array(items) => {
            let items = items.iter().map(|item| kind_tokens(item, crate_name));
            quote!(#types::KindLiteral::Array(::std::vec![#(#items),*]))
        }
        KindLiteral::Object(entries) => {
            let entries = entries.iter().map(|(key, kind)| {
                let kind = kind_tokens(kind, crate_name);
                quote!((::std::string::String::from(#key), #kind))
            });
            quote!(#types::KindLiteral::Object(
                ::std::collections::BTreeMap::from([#(#entries),*])
            ))
        }
        KindLiteral::Bool(value) => quote!(#types::KindLiteral::Bool(#value)),
    }
}

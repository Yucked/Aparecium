// The only module that names `surrealdb_syn`: it is SurrealDB's internal parser with no SemVer
// guarantee, so its version moves with `surrealdb` and its API stays contained here.

use proc_macro2::Span;

/// Checks that `query` is syntactically valid SurrealQL, reporting the parser's message at `span`.
///
/// Syntax only: tables, fields, permissions and parameter types are not checked.
pub(crate) fn validate(query: &str, span: Span) -> syn::Result<()> {
    surrealdb_syn::parse(query)
        .map(drop)
        .map_err(|err| syn::Error::new(span, err))
}

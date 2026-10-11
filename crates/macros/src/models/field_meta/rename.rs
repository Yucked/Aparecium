use syn::{Attribute, Expr, LitStr, Token};

/// The string value of `setting` in an item's `#[surreal(…)]` attributes, e.g. `rename = "…"`.
///
/// Stored names come from the same attributes the SDK's `SurrealValue` derive reads, so the
/// schema and the stored record cannot disagree. Other settings are skipped.
pub fn surreal_setting(attrs: &[Attribute], setting: &str) -> syn::Result<Option<LitStr>> {
    let mut value = None;
    for attr in attrs.iter().filter(|attr| attr.path().is_ident("surreal")) {
        attr.parse_nested_meta(|meta| {
            if meta.path.is_ident(setting) {
                value = Some(meta.value()?.parse::<LitStr>()?);
            } else if meta.input.peek(Token![=]) {
                meta.value()?.parse::<Expr>()?;
            }
            Ok(())
        })?;
    }
    Ok(value)
}

/// The bare flag `flag`, e.g. `flatten`, if an item's `#[surreal(…)]` attributes include it.
pub fn surreal_flag(attrs: &[Attribute], flag: &str) -> syn::Result<Option<syn::Path>> {
    let mut found = None;
    for attr in attrs.iter().filter(|attr| attr.path().is_ident("surreal")) {
        attr.parse_nested_meta(|meta| {
            if meta.path.is_ident(flag) {
                found = Some(meta.path.clone());
            } else if meta.input.peek(Token![=]) {
                meta.value()?.parse::<Expr>()?;
            }
            Ok(())
        })?;
    }
    Ok(found)
}


use crate::helpers::get_crate_name;
use quote::format_ident;
use syn::Ident;

pub fn generate_variable_name() -> Ident {
    // Already a valid identifier: the crate path is one, and a simple uuid is lowercase hex.
    let sanitized_uuid = uuid::Uuid::new_v4().simple();
    let crate_name = get_crate_name(false);
    format_ident!("_{crate_name}__private__internal_variable_prefix__{sanitized_uuid}")
}

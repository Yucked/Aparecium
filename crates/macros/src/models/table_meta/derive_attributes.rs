use crate::helpers::get_crate_name;
use darling::{FromDeriveInput, ast::Data, util};
use proc_macro2::TokenStream;
use quote::{ToTokens, format_ident, quote};
use syn::Ident;

use super::table::TableNameIdent;
use crate::models::*;

create_ident_wrapper!(StructIdent);
create_ident_wrapper!(StructPatchIdent);

impl StructIdent {
    pub fn patch_ident(&self) -> StructPatchIdent {
        StructPatchIdent(format_ident!("{}Patch", self.to_string()))
    }

    pub fn is_same_name(&self, other: impl Into<CustomType>) -> ExtractorResult<bool> {
        let other: CustomType = other.into();
        Ok(other.type_name()? == self.to_string())
    }
}

/// A struct marked with `#[aparecium::model(..)]`. The attribute macro forwards its arguments as
/// `#[orm(node | edge | object, table = ..)]`; the other `#[orm(..)]` settings are the user's.
#[derive(Clone, Debug, FromDeriveInput)]
#[darling(attributes(orm), forward_attrs(surreal), supports(struct_named))]
pub struct ModelReceiver {
    pub(crate) ident: Ident,
    pub(crate) vis: syn::Visibility,
    /// The struct's `#[surreal(…)]` attributes, read for `rename_all`.
    pub(crate) attrs: Vec<syn::Attribute>,
    pub(crate) generics: StructGenerics,
    pub data: Data<util::Ignored, MyFieldReceiver>,

    #[darling(default)]
    pub(crate) node: bool,

    #[darling(default)]
    pub(crate) edge: bool,

    #[darling(default)]
    pub(crate) object: bool,

    #[darling(default)]
    pub(crate) table: Option<TableNameIdent>,

    #[darling(default)]
    pub(crate) relax_table: Option<bool>,

    #[darling(default)]
    pub(crate) schemafull: Option<bool>,

    #[darling(default)]
    pub(crate) drop: Option<bool>,

    #[darling(default)]
    pub(crate) flexible: Option<bool>,

    pub(crate) as_: Option<AttributeAs>,

    pub(crate) permissions: Option<Permissions>,

    #[darling(default)]
    pub(crate) define: Option<AttributeDefine>,
}

impl ModelReceiver {
    pub fn ident(&self) -> StructIdent {
        StructIdent(self.ident.clone())
    }

    pub fn generics(&self) -> &StructGenerics {
        &self.generics
    }

    pub fn data_type(&self) -> ExtractorResult<DataType> {
        match (self.node, self.edge, self.object) {
            (true, false, false) => Ok(DataType::Node),
            (false, true, false) => Ok(DataType::Edge),
            (false, false, true) => Ok(DataType::Object),
            _ => Err(syn::Error::new_spanned(
                &self.ident,
                "a model is exactly one of `node`, `edge` or `object`",
            )
            .into()),
        }
    }

    /// The table of a node or edge; objects are stored inside other records and have none.
    pub fn table(&self) -> ExtractorResult<Option<&TableNameIdent>> {
        match (self.data_type()?, &self.table) {
            (DataType::Object, None) => Ok(None),
            (DataType::Object, Some(table)) => Err(syn::Error::new_spanned(
                table,
                "an embedded `object` model has no table",
            )
            .into()),
            (_, Some(table)) => Ok(Some(
                table.validate_and_return(&self.ident(), &self.relax_table)?,
            )),
            (_, None) => Err(syn::Error::new_spanned(
                &self.ident,
                "a `node` or `edge` model needs `table = \"…\"`",
            )
            .into()),
        }
    }

    /// The struct's `#[surreal(rename_all = "…")]`.
    pub fn casing(&self) -> ExtractorResult<StructLevelCasing> {
        struct_casing(&self.attrs)
    }

    pub fn get_table_definition_token(&self) -> ExtractorResult<TableDefinitions> {
        let ModelReceiver {
            ref drop,
            ref flexible,
            ref schemafull,
            ref as_,
            ref permissions,
            ref define,
            ..
        } = *self;

        let crate_name = get_crate_name(false);

        if (define.is_some())
            && (drop.is_some()
                || as_.is_some()
                || schemafull.is_some()
                || flexible.is_some()
                || permissions.is_some())
        {
            return Err(
                syn::Error::new_spanned(
                    self.ident.clone(),
                    "Invalid combination. When `define`, the following attributes cannot be use in combination to prevent confusion:
                            drop,
                            flexible,
                            as,
                            schemafull,
                            permissions",
                )
                .into(),
            );
        }

        let definition = match define {
            Some(define) => define.to_token_stream(),
            None => {
                let mut define_table_methods: Vec<TokenStream> = vec![];

                if drop.is_some() {
                    define_table_methods.push(quote!(.drop()))
                }

                if flexible.is_some() {
                    define_table_methods.push(quote!(.flexible()))
                }

                if let Some(as_) = as_ {
                    define_table_methods.push(quote!(.as_(#as_)))
                }

                if schemafull.is_some() {
                    define_table_methods.push(quote!(.schemafull()))
                }

                if let Some(permissions) = permissions {
                    define_table_methods.push(permissions.to_token_stream());
                }

                quote!(
                    #crate_name::statements::define_table(<Self as #crate_name::Model>::table())
                    #( #define_table_methods) *
                )
            }
        };

        // Definitions write their values inline; `inline` keeps the builder's errors.
        Ok(quote!(#crate_name::Raw::inline(&#definition)).into())
    }
}

use crate::helpers::get_crate_name;
use quote::{ToTokens, quote};

use super::Codegen;
use crate::models::*;

impl Codegen<'_> {
    /// Checks a field's definition settings. The definitions themselves are part of the field's
    /// metadata, which `define_fields` reads.
    pub fn create_field_definition_checks(&mut self) -> ExtractorResult<()> {
        self.validate_field_attributes()?;

        self.static_assertions
            .push(self.static_assertion_field_value(self.table_derive_attributes())?);
        Ok(())
    }

    /// Relations are read through graph traversals and never stored, so they have no definition.
    pub fn skip_field(&self) -> bool {
        self.field_receiver()
            .to_relation_type(self.table_derive_attributes())
            .is_relate_graph()
    }

    pub fn field_defintion_db(&self) -> ExtractorResult<Vec<DefineFieldStatementToken>> {
        let field_receiver = self.field_receiver();
        let model_attributes = self.table_derive_attributes();
        if self.skip_field() {
            return Ok(vec![]);
        }

        let crate_name = get_crate_name(false);
        let VariablesModelMacro { _____table, .. } = VariablesModelMacro::new();
        let casing = model_attributes.casing()?;
        let db_field_name = field_receiver.db_field_name(&casing)?;
        // Every stored field has a type: inference fails rather than leave it out.
        let field_type_in_db_token = field_receiver
            .field_type_db_token(model_attributes)?
            .ok_or_else(|| {
                syn::Error::new_spanned(
                    &field_receiver.ty,
                    "the field's database type is unknown; specify `#[orm(ty = \"…\")]`",
                )
            })?;

        let mut define_field_methods = vec![];
        let mut define_array_field_item_methods = vec![];
        let mut all_field_defintions: Vec<DefineFieldStatementToken> = vec![];

        if let Some(assert) = field_receiver.assert.as_ref() {
            define_field_methods.push(quote!(.assert(#assert)));
        }

        if let Some(item_assert) = field_receiver.item_assert.as_ref() {
            define_array_field_item_methods.push(quote!(.assert(#item_assert)));
        }

        // `#[orm(value = …)]` is documented as the field's default, so it emits `DEFAULT`.
        if let Some(value) = field_receiver.value.as_ref() {
            define_field_methods.push(quote!(.default(#value)));
        }

        if let Some(permissions) = field_receiver.permissions.as_ref() {
            define_field_methods.push(permissions.into_token_stream());
        }

        // Definitions write their values inline; `inline` keeps the builder's errors.
        let raw = |definition: TokenStream| -> DefineFieldStatementToken {
            quote!(#crate_name::Raw::inline(&#definition)).into()
        };

        all_field_defintions.push(raw(quote!(
            #crate_name::statements::define_field(#crate_name::Field::ident(#db_field_name))
            .on_table(#_____table.clone())
            .type_(#field_type_in_db_token)
            #( # define_field_methods) *
        )));

        if !define_array_field_item_methods.is_empty() {
            // `field.*` defines every element of an array field.
            all_field_defintions.push(raw(quote!(
                #crate_name::statements::define_field(#crate_name::Field::ident(#db_field_name).all())
                    .on_table(#_____table.clone())
                    #( # define_array_field_item_methods) *
            )));
        };

        if let Some(define) = field_receiver.define.as_ref() {
            all_field_defintions.push(raw(quote!(#define)));
        }

        Ok(all_field_defintions)
    }

    fn static_assertion_field_value(
        &self,
        model_attrs: &ModelAttributes,
    ) -> ExtractorResult<StaticAssertionToken> {
        let field_receiver = self.field_receiver();
        let field_type = field_receiver.field_type_db_original(model_attrs)?;

        match field_type {
            Some(field_type) => {
                let static_assertion = field_receiver
                    .value
                    .as_ref()
                    .map_or(StaticAssertionToken::default(), |v| {
                        v.get_default_value_static_assertion(field_type.inner())
                    });
                Ok(static_assertion)
            }
            None => Ok(StaticAssertionToken::default()),
        }
    }

    fn validate_field_attributes(&self) -> ExtractorResult<()> {
        let field_receiver = self.field_receiver();
        let MyFieldReceiver {
            define,
            assert: assert_,
            value,
            permissions,
            item_assert,
            relate,
            ..
        } = field_receiver;
        let ident = field_receiver.ident()?;
        let db_field_name =
            field_receiver.db_field_name(&self.table_derive_attributes().casing()?)?;

        if define.is_some()
            && (assert_.is_some()
                || value.is_some()
                || permissions.is_some()
                || item_assert.is_some())
        {
            return Err(
                syn::Error::new_spanned(
                    db_field_name,
                    r#"Invalid combination. When `define`, the following attributes cannot be use in combination to prevent confusion:
    assert,
    value,
    permissions,
    item_assert"#).into());
        }

        if relate.is_some()
            && (define.is_some()
                || assert_.is_some()
                || value.is_some()
                || permissions.is_some()
                || item_assert.is_some())
        {
            return Err(syn::Error::new_spanned(
                ident,
                r#"This is a read-only relation field and does not allow the following attributes:
    define,
    assert,
    value,
    permissions,
    item_assert"#,
            )
            .into());
        }

        Ok(())
    }
}

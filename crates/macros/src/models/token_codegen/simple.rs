use crate::models::*;

use super::Codegen;

impl Codegen<'_> {
    /// Records the Rust field holding the record's `id`, which `Model::record_id` returns.
    pub fn create_table_id_type_token(&mut self) -> ExtractorResult<()> {
        let field_receiver = self.field_receiver();
        let casing = self.table_derive_attributes().casing()?;

        if field_receiver.db_field_name(&casing)?.is_id() {
            self.record_id_field = Some(field_receiver.ident()?.into_inner());
        }

        Ok(())
    }

    pub fn create_db_field_names_token(&mut self) -> ExtractorResult<()> {
        let field_receiver = self.field_receiver();
        let db_field_name =
            field_receiver.db_field_name(&self.table_derive_attributes().casing()?)?;

        // Caught here so the error names the field; `StoredFields::assert_valid` still checks
        // the names flattened objects add, which are only known once the code compiles.
        if self
            .serialized_field_names_normalised
            .contains(&db_field_name)
        {
            return Err(syn::Error::new_spanned(
                field_receiver.ident()?.into_inner(),
                format!(
                    "another field is already stored as `{db_field_name}`; check `#[surreal(rename)]` and `rename_all`"
                ),
            )
            .into());
        }

        self.serialized_field_names_normalised.push(db_field_name);

        Ok(())
    }
}

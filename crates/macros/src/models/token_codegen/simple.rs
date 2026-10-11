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
        let db_field_name = self
            .field_receiver()
            .db_field_name(&self.table_derive_attributes().casing()?)?;

        self.serialized_field_names_normalised
            .push(db_field_name.to_token_stream().into());

        Ok(())
    }
}

use quote::quote;

use crate::models::*;

use super::Codegen;

impl Codegen<'_> {
    pub fn create_field_connection_builder_token(&mut self) -> ExtractorResult<()> {
        let table_derive_attrs = self.table_derive_attributes();
        let field_receiver = self.field_receiver();
        let VariablesModelMacro {
            _____field_names,
            ___________path,
            ..
        } = VariablesModelMacro::new();

        if field_receiver
            .to_relation_type(table_derive_attrs)
            .is_relate_graph()
        {
            // Relate graph fields are readonly and derived and only used in
            // our rust code but not in the database schema
            return Ok(());
        };

        let casing = table_derive_attrs.casing()?;
        let field_ident_normalized = field_receiver.field_ident_normalized(&casing)?;
        let field_name_serialized = field_receiver.db_field_name(&casing)?;
        let field_name_pascalized = field_receiver.field_name_pascalized(&casing)?;

        self.schema_struct_fields_types_kv.push(
            quote!(pub #field_ident_normalized: #_____field_names::#field_name_pascalized, ).into(),
        );

        // The field's path is the model's path followed by its stored name.
        self.schema_struct_fields_names_kv.push(
            quote!(#field_ident_normalized: #___________path.clone().with_field(#field_name_serialized).into(),)
                .into(),
        );

        Ok(())
    }
}

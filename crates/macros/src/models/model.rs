use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::{Expr, Meta, Token, punctuated::Punctuated};

use crate::models::*;

/// Expands `#[aparecium::model(node | edge | object, table = "…", value = manual)]`.
///
/// The struct gains the SDK's `SurrealValue` derive (unless `value = manual`) and the hidden
/// `ModelCodegen` derive, which receives the model kind and table as `#[orm(..)]`.
pub fn model_attribute(args: TokenStream, item: TokenStream) -> TokenStream {
    match expand_model_attribute(args, item.clone()) {
        Ok(tokens) => tokens,
        Err(error) => {
            let error = error.to_compile_error();
            quote!(#error #item)
        }
    }
}

fn expand_model_attribute(args: TokenStream, item: TokenStream) -> syn::Result<TokenStream> {
    let crate_name = get_crate_name(false);
    let args = syn::parse::Parser::parse2(Punctuated::<Meta, Token![,]>::parse_terminated, args)?;

    let mut kind = None;
    let mut table = None;
    let mut manual_value = false;
    for arg in &args {
        match arg {
            Meta::Path(path)
                if path.is_ident("node") || path.is_ident("edge") || path.is_ident("object") =>
            {
                if kind.replace(path.clone()).is_some() {
                    return Err(syn::Error::new_spanned(
                        path,
                        "a model is exactly one of `node`, `edge` or `object`",
                    ));
                }
            }
            Meta::NameValue(name_value) if name_value.path.is_ident("table") => {
                table = Some(name_value.value.clone());
            }
            Meta::NameValue(name_value) if name_value.path.is_ident("value") => {
                match &name_value.value {
                    Expr::Path(path) if path.path.is_ident("manual") => manual_value = true,
                    value => {
                        return Err(syn::Error::new_spanned(
                            value,
                            "the only `value` setting is `manual`, for a hand-written `SurrealValue`",
                        ));
                    }
                }
            }
            other => {
                return Err(syn::Error::new_spanned(
                    other,
                    "expected `node`, `edge` or `object`, `table = \"…\"` or `value = manual`",
                ));
            }
        }
    }

    let kind = kind.ok_or_else(|| {
        syn::Error::new(
            proc_macro2::Span::call_site(),
            "expected the model kind: `node`, `edge` or `object`",
        )
    })?;
    let table = table.map(|table| quote!(, table = #table));

    // `SurrealValue` names its runtime through the consumer's own path to Aparecium, so a
    // renamed dependency works without a direct `surrealdb` dependency.
    let surreal_value = (!manual_value).then(|| {
        let types_path = quote!(#crate_name::types).to_string();
        quote!(
            #[derive(#crate_name::types::SurrealValue)]
            #[surreal(crate = #types_path)]
        )
    });

    Ok(quote!(
        #surreal_value
        #[derive(#crate_name::__private::ModelCodegen)]
        #[orm(#kind #table)]
        #item
    ))
}

impl ToTokens for ModelReceiver {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self.expand() {
            Ok(expanded) => tokens.extend(expanded),
            Err(error) => tokens.extend(error.write_errors()),
        }
    }
}

impl ModelReceiver {
    fn expand(&self) -> ExtractorResult<TokenStream> {
        let crate_name = get_crate_name(false);
        let model_attributes = ModelAttributes::new(self)?;
        let data_type = model_attributes.to_data_type();
        let struct_name_ident = &self.ident();
        let (impl_generics, ty_generics, where_clause) = self.generics().split_for_impl();
        let struct_marker = self.generics().phantom_marker_type();
        let explicit_generics = model_attributes.explicit_fully_qualified_generics_path();
        let VariablesModelMacro {
            ___________path,
            __________connect,
            _____field_names,
            _____struct_marker_ident,
        } = VariablesModelMacro::new();

        let code_gen = Codegen::parse_fields(&model_attributes)?;
        let Codegen {
            schema_struct_fields_types_kv,
            schema_struct_fields_names_kv,
            field_wrapper_type_custom_implementations,
            static_assertions,
            imports_referenced_node_schema,
            record_link_fields_methods,
            ..
        } = &code_gen;
        let imports_referenced_node_schema = imports_referenced_node_schema.iter();

        let CommonIdents {
            module_name_internal,
            module_name_rexported,
            test_function_name,
            _____schema_def,
            ..
        } = code_gen.common_idents();

        let record_model = match data_type {
            DataType::Node | DataType::Edge => Some(self.record_model_tokens(&code_gen)?),
            DataType::Object => None,
        };

        let model_kind_impl = match data_type {
            DataType::Node => self.node_tokens(&code_gen)?,
            DataType::Edge => self.edge_tokens(&code_gen)?,
            DataType::Object => quote!(
                impl #impl_generics #crate_name::Object for #struct_name_ident #ty_generics #where_clause {}
            ),
        };

        // Lets `assert_fields!` check that relations name a real node or edge table.
        let table_name_checker = model_attributes.table()?.map(|table| {
            quote!(
                pub struct TableNameStaticChecker {
                    pub #table: ::std::string::String,
                }
            )
        });

        let node_only = matches!(data_type, DataType::Node).then(|| {
            let Codegen {
                aliases_struct_fields_types_kv,
                aliases_struct_fields_names_kv,
                node_edge_metadata,
                ..
            } = &code_gen;
            let aliases_struct_name = &code_gen.common_idents().aliases_struct_name;
            quote!(
                #[derive(Debug, Clone)]
                pub struct #aliases_struct_name {
                   #( #aliases_struct_fields_types_kv) *
                }

                impl #aliases_struct_name {
                    pub fn new() -> Self {
                        Self {
                           #( #aliases_struct_fields_names_kv) *
                        }
                    }
                }

                #node_edge_metadata
            )
        });

        Ok(quote!(
            impl #impl_generics #crate_name::SchemaGetter for #struct_name_ident #ty_generics #where_clause {
                type Schema = #module_name_rexported::Schema #ty_generics;

                fn schema() -> Self::Schema {
                    #module_name_rexported::Schema #explicit_generics ::new()
                }

                fn schema_prefixed(prefix: impl ::std::convert::Into<#crate_name::Field>) -> Self::Schema {
                    #module_name_rexported::Schema #explicit_generics ::new_prefixed(prefix)
                }
            }

            #model_kind_impl

            #record_model

            #[allow(non_snake_case)]
            pub mod #module_name_rexported {
                pub use super::#module_name_internal::#_____schema_def::__Schema__ as Schema;
            }

            #[allow(non_snake_case)]
            mod #module_name_internal {
                #![allow(clippy::builtin_type_shadow)]

                use super::*;

                #table_name_checker

                #( #imports_referenced_node_schema) *

                pub(super) mod #_____field_names {
                    use super::super::*;

                    #( #field_wrapper_type_custom_implementations) *
                }

                pub mod #_____schema_def {
                    use super::#_____field_names;
                    use super::super::*;

                    /// Paths to the model's fields, from the query root or a graph traversal.
                    #[allow(non_snake_case)]
                    #[derive(Debug, Clone)]
                    pub struct __Schema__ #ty_generics #where_clause {
                       #( #schema_struct_fields_types_kv) *
                        pub(super) #___________path: #crate_name::Field,
                        pub(super) #_____struct_marker_ident: #struct_marker,
                    }
                }
                pub type #struct_name_ident #ty_generics = #_____schema_def::__Schema__ #ty_generics;

                impl #impl_generics #crate_name::Render for #struct_name_ident #ty_generics #where_clause {
                    fn render(&self, sql: &mut #crate_name::Sql) {
                        sql.push(&self.#___________path);
                    }
                }

                impl #impl_generics #crate_name::Aliasable for #struct_name_ident #ty_generics #where_clause {}

                impl #impl_generics ::std::convert::From<&#struct_name_ident #ty_generics> for #crate_name::Field #where_clause {
                    fn from(schema: &#struct_name_ident #ty_generics) -> Self {
                        schema.#___________path.clone()
                    }
                }

                impl #impl_generics ::std::convert::From<#struct_name_ident #ty_generics> for #crate_name::Field #where_clause {
                    fn from(schema: #struct_name_ident #ty_generics) -> Self {
                        schema.#___________path
                    }
                }

                impl #impl_generics ::std::convert::From<#struct_name_ident #ty_generics> for #crate_name::ValueLike #where_clause {
                    fn from(schema: #struct_name_ident #ty_generics) -> Self {
                        #crate_name::ValueLike::from(schema.#___________path)
                    }
                }

                impl #impl_generics #struct_name_ident #ty_generics #where_clause {
                    pub fn new() -> Self {
                        Self::___________with_path(#crate_name::Field::default())
                    }

                    pub fn new_prefixed(prefix: impl ::std::convert::Into<#crate_name::Field>) -> Self {
                        Self::___________with_path(prefix.into())
                    }

                    /// The schema at the end of `path` extended by `clause`.
                    #[doc(hidden)]
                    pub fn #__________connect(
                        path: impl ::std::convert::Into<#crate_name::Field>,
                        clause: #crate_name::Clause,
                    ) -> Self {
                        let mut path = path.into();
                        path.push_clause(clause);
                        Self::___________with_path(path)
                    }

                    fn ___________with_path(#___________path: #crate_name::Field) -> Self {
                        Self {
                           #( #schema_struct_fields_names_kv) *
                            #___________path,
                            #_____struct_marker_ident: ::std::marker::PhantomData,
                        }
                    }

                    #( #record_link_fields_methods) *
                }

                #node_only
            }

            #[allow(non_snake_case, unreachable_code, dead_code)]
            fn #test_function_name #impl_generics() #where_clause {
                #( #static_assertions) *
            }
        ))
    }

    /// `Model`, its patch type and the helpers `object!` checks keys with.
    fn record_model_tokens(&self, code_gen: &Codegen) -> ExtractorResult<TokenStream> {
        let crate_name = get_crate_name(false);
        let struct_name_ident = &self.ident();
        let (impl_generics, ty_generics, where_clause) = self.generics().split_for_impl();
        let struct_marker = self.generics().phantom_marker_type();
        let vis = &self.vis;
        let table = self
            .table()?
            .expect("a node or edge has a table")
            .as_string();
        let table_definitions = self.get_table_definition_token()?;
        let patch_ident = struct_name_ident.patch_ident();
        let VariablesModelMacro {
            _____struct_marker_ident,
            ..
        } = VariablesModelMacro::new();
        let Codegen {
            serialized_fmt_db_field_names_instance: serializable_fields,
            linked_fields,
            link_one_fields,
            link_self_fields,
            link_one_and_self_fields,
            link_many_fields,
            field_definitions,
            field_metadata,
            renamed_serialized_fields_kv,
            patch_fields,
            patch_defaults,
            patch_setters,
            record_id_field,
            ..
        } = code_gen;
        let CommonIdents {
            struct_with_renamed_serialized_fields,
            ..
        } = code_gen.common_idents();
        let serializable_fields_count = serializable_fields.len();
        let record_id_field = record_id_field.as_ref().ok_or_else(|| {
            syn::Error::new_spanned(
                &self.ident,
                "a node or edge needs its record id: `id: RecordId<Self>`",
            )
        })?;
        let patch_doc = format!(
            "A partial update of [`{}`]: each field left as `Patch::Keep` is unchanged.",
            self.ident
        );

        Ok(quote!(
            impl #impl_generics #struct_name_ident #ty_generics #where_clause {
                #[doc(hidden)]
                pub const fn __get_serializable_field_names() -> [&'static str; #serializable_fields_count] {
                    [#( #serializable_fields ), *]
                }
            }

            #[doc(hidden)]
            #[allow(non_snake_case)]
            pub struct #struct_with_renamed_serialized_fields {
               #(#renamed_serialized_fields_kv), *
            }

            impl #impl_generics #crate_name::Model for #struct_name_ident #ty_generics #where_clause {
                type StructRenamedCreator = #struct_with_renamed_serialized_fields;

                fn table() -> #crate_name::Table {
                    #crate_name::Table::new(#table)
                }

                fn record_id(&self) -> &#crate_name::RecordId<Self> {
                    &self.#record_id_field
                }

                fn get_serializable_fields() -> ::std::vec::Vec<#crate_name::Field> {
                    ::std::vec![#( #crate_name::Field::ident(#serializable_fields) ), *]
                }

                fn get_linked_fields() -> ::std::vec::Vec<#crate_name::Field> {
                    ::std::vec![#( #linked_fields ), *]
                }

                fn get_link_one_fields() -> ::std::vec::Vec<#crate_name::Field> {
                    ::std::vec![#( #link_one_fields ), *]
                }

                fn get_link_self_fields() -> ::std::vec::Vec<#crate_name::Field> {
                    ::std::vec![#( #link_self_fields ), *]
                }

                fn get_link_one_and_self_fields() -> ::std::vec::Vec<#crate_name::Field> {
                    ::std::vec![#( #link_one_and_self_fields ), *]
                }

                fn get_link_many_fields() -> ::std::vec::Vec<#crate_name::Field> {
                    ::std::vec![#( #link_many_fields ), *]
                }

                fn define_table() -> #crate_name::Raw {
                    #table_definitions
                }

                fn define_fields() -> ::std::vec::Vec<#crate_name::Raw> {
                    ::std::vec![#( #field_definitions ), *]
                }

                fn get_field_meta() -> ::std::vec::Vec<#crate_name::FieldMetadata> {
                    ::std::vec![#( #field_metadata ), *]
                }
            }

            #[doc = #patch_doc]
            #[allow(non_snake_case)]
            #vis struct #patch_ident #impl_generics #where_clause {
                #( #patch_fields, )*
                #[doc(hidden)]
                #vis #_____struct_marker_ident: #struct_marker,
            }

            impl #impl_generics ::std::default::Default for #patch_ident #ty_generics #where_clause {
                fn default() -> Self {
                    Self {
                        #( #patch_defaults, )*
                        #_____struct_marker_ident: ::std::marker::PhantomData,
                    }
                }
            }

            impl #impl_generics #crate_name::ModelPatch for #patch_ident #ty_generics #where_clause {
                type Model = #struct_name_ident #ty_generics;

                fn into_setters(self) -> ::std::vec::Vec<#crate_name::Setter> {
                    let mut setters = ::std::vec::Vec::new();
                    #( #patch_setters )*
                    setters
                }
            }
        ))
    }

    fn node_tokens(&self, code_gen: &Codegen) -> ExtractorResult<TokenStream> {
        let crate_name = get_crate_name(false);
        let struct_name_ident = &self.ident();
        let (impl_generics, ty_generics, where_clause) = self.generics().split_for_impl();
        let table = self
            .table()?
            .expect("a node has a table")
            .as_string();
        let explicit_generics = ModelAttributes::new(self)?.explicit_fully_qualified_generics_path();
        let VariablesModelMacro {
            __________connect, ..
        } = VariablesModelMacro::new();
        let CommonIdents {
            module_name_internal,
            aliases_struct_name,
            ..
        } = code_gen.common_idents();
        let fields_relations_aliased = &code_gen.fields_relations_aliased;

        Ok(quote!(
            impl #impl_generics #crate_name::Node for #struct_name_ident #ty_generics #where_clause {
                type TableNameChecker = #module_name_internal::TableNameStaticChecker;
                type Aliases = #module_name_internal::#aliases_struct_name;

                fn with(clause: impl ::std::convert::Into<#crate_name::NodeClause>) -> <Self as #crate_name::SchemaGetter>::Schema {
                    let clause: #crate_name::NodeClause = clause.into();
                    #module_name_internal::#struct_name_ident #explicit_generics ::#__________connect(
                        #crate_name::Field::default(),
                        clause.with_table(#table).into_inner(),
                    )
                }

                fn aliases() -> Self::Aliases {
                    #module_name_internal::#aliases_struct_name::new()
                }

                fn get_table() -> #crate_name::Table {
                    #crate_name::Table::new(#table)
                }

                fn get_fields_relations_aliased() -> ::std::vec::Vec<#crate_name::Alias> {
                    ::std::vec![#( #fields_relations_aliased ), *]
                }
            }
        ))
    }

    fn edge_tokens(&self, code_gen: &Codegen) -> ExtractorResult<TokenStream> {
        let crate_name = get_crate_name(false);
        let struct_name_ident = &self.ident();
        let (impl_generics, ty_generics, where_clause) = self.generics().split_for_impl();
        let table = self
            .table()?
            .expect("an edge has a table")
            .as_string();
        let CommonIdents {
            module_name_internal,
            ..
        } = code_gen.common_idents();

        let has_in_and_out_fields = code_gen
            .serialized_field_names_normalised
            .iter()
            .map(|name| name.to_string().replace('"', ""))
            .filter(|name| name == "in" || name == "out")
            .count()
            == 2;
        if !has_in_and_out_fields {
            return Err(syn::Error::new_spanned(
                &self.ident,
                "an edge model needs `in` and `out` fields",
            )
            .into());
        }

        Ok(quote!(
            impl #impl_generics #crate_name::Edge for #struct_name_ident #ty_generics #where_clause {
                type In = In;
                type Out = Out;
                type TableNameChecker = #module_name_internal::TableNameStaticChecker;

                fn get_table() -> #crate_name::Table {
                    #crate_name::Table::new(#table)
                }
            }
        ))
    }
}

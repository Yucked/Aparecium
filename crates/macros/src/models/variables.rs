use quote::format_ident;

/// Names of private items in generated schema code, chosen not to collide with model fields.
pub struct VariablesModelMacro {
    /// The schema's path from the query root, e.g. `->writes->book`; every field extends it.
    pub ___________path: syn::Ident,
    /// Builds a schema at the end of a path extended by a clause, joining one model's schema to
    /// another's in a graph traversal.
    pub __________connect: syn::Ident,
    pub _____field_names: syn::Ident,
    pub _____struct_marker_ident: syn::Ident,
    /// The table field definitions are written for: the model's own, or the table of a model
    /// that flattens this object.
    pub _____table: syn::Ident,
}

impl Default for VariablesModelMacro {
    fn default() -> Self {
        Self::new()
    }
}

impl VariablesModelMacro {
    pub fn new() -> Self {
        Self {
            ___________path: format_ident!("___________path"),
            __________connect: format_ident!("__________connect"),
            _____field_names: format_ident!("_____field_names"),
            _____struct_marker_ident: format_ident!("_____struct_marker_ident"),
            _____table: format_ident!("_____table"),
        }
    }
}

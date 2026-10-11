use std::str::FromStr;

use convert_case::{Case, Casing};
use strum_macros::EnumString;
use syn::Attribute;

use super::{ExtractorResult, surreal_setting};

/// A struct's casing from its `#[surreal(rename_all = "…")]` attribute.
pub fn struct_casing(attrs: &[Attribute]) -> ExtractorResult<StructLevelCasing> {
    let casing = match surreal_setting(attrs, "rename_all")? {
        Some(casing) => CaseString::from_str(&casing.value())
            .map_err(|_| syn::Error::new(casing.span(), "unknown `rename_all` casing"))?,
        None => CaseString::None,
    };
    Ok(casing.into())
}

/// A struct's `#[surreal(rename_all = "…")]`, with the values the SDK's derive accepts.
#[derive(Debug, Clone, Copy, EnumString, Default)]
pub enum CaseString {
    #[default]
    None,

    #[strum(serialize = "camelCase")]
    Camel,
    #[strum(serialize = "snake_case")]
    Snake,
    #[strum(serialize = "PascalCase")]
    Pascal,

    #[strum(serialize = "lowercase")]
    Lower,

    #[strum(serialize = "UPPERCASE")]
    Upper,

    #[strum(serialize = "SCREAMING_SNAKE_CASE")]
    ScreamingSnake,

    #[strum(serialize = "kebab-case")]
    Kebab,

    #[strum(serialize = "SCREAMING-KEBAB-CASE")]
    ScreamingKebab,
}

impl CaseString {
    /// The stored name of a field, cased as the SDK's derive cases it: `lowercase` and
    /// `UPPERCASE` only change letter case, the others re-split the words.
    pub fn apply(self, ident: &str) -> String {
        match self {
            CaseString::None => ident.to_owned(),
            CaseString::Lower => ident.to_lowercase(),
            CaseString::Upper => ident.to_uppercase(),
            CaseString::Camel => ident.to_case(Case::Camel),
            CaseString::Snake => ident.to_case(Case::Snake),
            CaseString::Pascal => ident.to_case(Case::Pascal),
            CaseString::ScreamingSnake => ident.to_case(Case::UpperSnake),
            CaseString::Kebab => ident.to_case(Case::Kebab),
            CaseString::ScreamingKebab => ident.to_case(Case::UpperKebab),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct StructLevelCasing(CaseString);

impl From<CaseString> for StructLevelCasing {
    fn from(value: CaseString) -> Self {
        Self(value)
    }
}

impl std::ops::Deref for StructLevelCasing {
    type Target = CaseString;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

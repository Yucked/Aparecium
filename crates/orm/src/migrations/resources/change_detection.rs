use std::{fmt::Display, ops::Deref};

use crate::migrations::*;
use crate::{DbResources, Field, FieldChangeMeta, FieldMetadata, Render, Table};

pub enum DeltaTypeField {
    NoChange,
    Create {
        right: DefineStatementRaw,
    },
    Remove {
        left: DefineStatementRaw,
    },
    Update {
        left: DefineStatementRaw,
        right: DefineStatementRaw,
    },
    Rename {
        new_name: Field,
        old_name: Field,
    },
}

// Change detection
pub(crate) struct FieldChangeDetectionMeta<'a, R: DbResources> {
    pub(crate) field_name: Field,
    pub(crate) left_defs: Fields,
    pub(crate) right_defs: Fields,
    pub(crate) table: Table,
    pub(crate) codebase_resources: &'a R,
}

impl<R: DbResources> TryFrom<FieldChangeDetectionMeta<'_, R>> for DeltaTypeField {
    type Error = MigrationError;

    fn try_from(value: FieldChangeDetectionMeta<R>) -> MigrationResult<Self> {
        let FieldChangeDetectionMeta {
            field_name,
            left_defs,
            right_defs,
            table,
            codebase_resources,
        } = value;

        let foundfield_by_newname = RightDatabase::find_field_has_old_name(
            codebase_resources,
            &table,
            By::NewName(field_name.clone()),
        )
        .map(FieldMetadataWrapper);

        let found_field_by_oldname = RightDatabase::find_field_has_old_name(
            codebase_resources,
            &table,
            By::OldName(field_name.clone()),
        )
        .map(FieldMetadataWrapper);

        let res = match (found_field_by_oldname, foundfield_by_newname) {
            (Some(_l_meta), Some(_r_meta)) => {
                return Err(MigrationError::RenamingToSameOldFieldDisallowed {
                    field: field_name,
                    table,
                });
            }
            (None, Some(r_meta)) => {
                let old_name = r_meta.old_name.clone().unwrap().to_debug_sql();
                if right_defs.get_definition(&old_name).is_some() {
                    return Err(MigrationError::CannotRenameFromOldFieldInUSeInTable {
                        new_field: field_name.to_debug_sql(),
                        old_field: old_name,
                        table,
                        renamables: left_defs.get_names().join(", "),
                    });
                }

                DeltaTypeField::Rename {
                    new_name: r_meta.name.to_owned(),
                    old_name: r_meta.old_name.to_owned().unwrap(),
                }
            }
            (Some(_), None) => {
                // Skip change since Renaming has already been handled up there on the right
                DeltaTypeField::NoChange
            }
            // No explicit rename attribute used on the field e.g old_name = "OldFieldName"
            (None, None) => {
                let field_name = field_name.to_debug_sql();
                let left_def = left_defs.get_definition(&field_name).cloned();
                let right_def = right_defs.get_definition(&field_name).cloned();

                match (left_def, right_def) {
                    (None, Some(r)) => DeltaTypeField::Create { right: r },
                    (Some(l), None) => DeltaTypeField::Remove { left: l },
                    (Some(l), Some(r)) => {
                        if l.trim() != r.trim() {
                            DeltaTypeField::Update { left: l, right: r }
                        } else {
                            DeltaTypeField::NoChange
                        }
                    }
                    _ => unreachable!(),
                }
            }
        };

        Ok(res)
    }
}

#[derive(Debug, Clone)]
struct FieldMetadataWrapper(FieldMetadata);
impl Deref for FieldMetadataWrapper {
    type Target = FieldMetadata;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Clone)]
pub enum SingleFieldChangeType {
    Delete(FieldChangeMeta),
    Rename(FieldChangeMeta),
}

impl Display for SingleFieldChangeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SingleFieldChangeType::Delete(change) => write!(
                f,
                "Delete old field '{}' and create a new one '{}' on table '{}'",
                change.old_name.to_debug_sql(),
                change.new_name.to_debug_sql(),
                change.table
            ),
            SingleFieldChangeType::Rename(change) => write!(
                f,
                "Rename old field '{}' to new field '{}' on table '{}'",
                change.old_name.to_debug_sql(),
                change.new_name.to_debug_sql(),
                change.table
            ),
        }
    }
}

#[derive(Debug, Clone)]
pub enum DeltaTypeResource {
    NoChange,
    Create {
        right: DefineStatementRaw,
    },
    Remove {
        left: DefineStatementRaw,
    },
    Update {
        left: DefineStatementRaw,
        right: DefineStatementRaw,
    },
}

impl From<(Option<DefineStatementRaw>, Option<DefineStatementRaw>)> for DeltaTypeResource {
    fn from(value: (Option<DefineStatementRaw>, Option<DefineStatementRaw>)) -> Self {
        match value {
            (None, Some(r)) => DeltaTypeResource::Create { right: r },
            (Some(l), None) => DeltaTypeResource::Remove { left: l },
            (Some(l), Some(r)) => {
                if l.trim() != r.trim() {
                    DeltaTypeResource::Update { left: l, right: r }
                } else {
                    DeltaTypeResource::NoChange
                }
            }
            (None, None) => unreachable!(),
        }
    }
}

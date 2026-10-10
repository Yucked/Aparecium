/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 */

// Type strings are parsed at compile time by `aparecium-macros`, which constructs these values
// directly; the runtime only needs the types and their SurrealQL rendering.

use std::fmt::{self, Display};

use serde::{Deserialize, Serialize};

/// Geometry types supported by surrealdb
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Serialize, Deserialize, Hash)]
pub enum GeometryType {
    /// Define a field with any geometric type
    Feature,
    /// Define a field with point geometric type
    Point,
    /// Define a field with LineStrigWgeometric type
    LineString,
    /// Define a field with polygon geometric type
    Polygon,
    /// Define a field with multipoint geometric type
    MultiPoint,
    /// Define a field with multiline geometric type
    MultiLine,
    /// Define a field with multpipolygon geometric type
    MultiPolygon,
    /// Define a field with collection of geometry types
    Collection,
}

impl Display for GeometryType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let geom = match self {
            GeometryType::Feature => "feature",
            GeometryType::Point => "point",
            GeometryType::LineString => "LineString",
            GeometryType::Polygon => "polygon",
            GeometryType::MultiPoint => "multipoint",
            GeometryType::MultiLine => "multiline",
            GeometryType::MultiPolygon => "multipolygon",
            GeometryType::Collection => "collection",
        };
        write!(f, "{}", geom)
    }
}

#[allow(missing_docs)]
#[derive(Clone, Default, Debug, PartialEq)]
pub enum FieldType {
    /// Use this when you explicitly don't want to specify the field's data type. The field will
    /// allow any data type supported by SurrealDB.
    #[default]
    Any,
    Null,
    /// true of false
    Bool,
    Bytes,
    /// An ISO 8601 compliant data type that stores a date with time and time zone.
    Datetime,
    /// Uses BigDecimal for storing any real number with arbitrary precision.
    Decimal,
    /// Store a value representing a length of time. Can be added or subtracted from datetimes or
    /// other durations.
    Duration,
    /// Store a value in a 64 bit float.
    Float,
    /// Store a value in a 64 bit integer.
    Int,
    /// Store numbers without specifying the type. SurrealDB will detect the type of number and
    /// store it using the minimal number of bytes. For numbers passed in as a string, this field
    /// will store the number in a BigDecimal.
    Number,
    /// Store formatted objects containing values of any supported type with no limit to object
    /// depth or nesting.
    Object,
    String,
    Uuid,
    /// Store a reference to another record. The value must be a Record ID.
    Record(Vec<crate::Table>), // record<user | admin> or record<user> or record
    /// RFC 7946 compliant data type for storing geometry in the GeoJson format.
    Geometry(Vec<GeometryType>), // geometry<point | LineStrigW| polygon>
    Option(Box<FieldType>),           // option<string>
    Union(Vec<FieldType>),            // string | int | object
    Set(Box<FieldType>, Option<u64>), // set<string, 10>, set<string>, set
    /// a list
    Array(Box<FieldType>, Option<u64>), // array<string, 10>, array<string>, array
}

impl Display for FieldType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FieldType::Any => write!(f, "any"),
            FieldType::Null => write!(f, "null"),
            FieldType::Bool => write!(f, "bool"),
            FieldType::Bytes => write!(f, "bytes"),
            FieldType::Datetime => write!(f, "datetime"),
            FieldType::Decimal => write!(f, "decimal"),
            FieldType::Duration => write!(f, "duration"),
            FieldType::Float => write!(f, "float"),
            FieldType::Int => write!(f, "int"),
            FieldType::Number => write!(f, "number"),
            FieldType::Object => write!(f, "object"),
            FieldType::String => write!(f, "string"),
            FieldType::Uuid => write!(f, "uuid"),
            FieldType::Record(ref_tables) => {
                if ref_tables.is_empty() {
                    write!(f, "record<any>")
                } else {
                    write!(
                        f,
                        "record<{}>",
                        ref_tables
                            .iter()
                            .map(|t| t.to_string())
                            .collect::<Vec<_>>()
                            .join("|")
                    )
                }
            }
            FieldType::Geometry(ref_tables) => {
                if ref_tables.is_empty() {
                    write!(f, "geometry<feature>")
                } else {
                    write!(
                        f,
                        "geometry<{}>",
                        ref_tables
                            .iter()
                            .map(|t| t.to_string())
                            .collect::<Vec<_>>()
                            .join("|")
                    )
                }
            }
            FieldType::Option(ft) => write!(f, "option<{}>", ft),
            FieldType::Union(ref_tables) => write!(
                f,
                "{}",
                ref_tables
                    .iter()
                    .map(|t| t.to_string())
                    .collect::<Vec<_>>()
                    .join(" | ")
            ),
            FieldType::Set(ft, size) => {
                if let Some(size) = size {
                    write!(f, "set<{}, {}>", ft, size)
                } else {
                    write!(f, "set<{}>", ft)
                }
            }
            FieldType::Array(ft, size) => {
                if let Some(size) = size {
                    write!(f, "array<{}, {}>", ft, size)
                } else {
                    write!(f, "array<{}>", ft)
                }
            }
        }
    }
}

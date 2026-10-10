// A proc-macro crate can only export macros, so its expansion modules are compiled into this
// test target from source; only part of each module is exercised here.
#![allow(dead_code)]

#[path = "../../macros/src/field_type.rs"]
mod field_type;
#[path = "../../macros/src/helpers.rs"]
mod helpers;

mod tests {
    use crate::field_type::*;
    use pretty_assertions::assert_eq;

    macro_rules! test_parse_db_field_type {
        ($name:ident, $input:expr, $output:expr) => {
            paste::paste! {
                #[test]
                fn [< test_field_type_$name >]() {
                    let result = parse_field_type($input);
                    let (input, ouput) = result.unwrap();
                    assert_eq!(input, "");
                    assert_eq!(ouput, $output);
                }
            }
        };
    }

    test_parse_db_field_type!(any, "any", FieldType::Any);
    test_parse_db_field_type!(null, "null", FieldType::Null);
    test_parse_db_field_type!(bool, "bool", FieldType::Bool);
    test_parse_db_field_type!(bytes, "bytes", FieldType::Bytes);
    test_parse_db_field_type!(datetime, "datetime", FieldType::Datetime);
    test_parse_db_field_type!(decimal, "decimal", FieldType::Decimal);
    test_parse_db_field_type!(duration, "duration", FieldType::Duration);
    test_parse_db_field_type!(flaot, "float", FieldType::Float);
    test_parse_db_field_type!(int, "int", FieldType::Int);
    test_parse_db_field_type!(number, "number", FieldType::Number);
    test_parse_db_field_type!(object, "object", FieldType::Object);
    test_parse_db_field_type!(string, "string", FieldType::String);
    test_parse_db_field_type!(uuild, "uuid", FieldType::Uuid);
    test_parse_db_field_type!(record_any, "record", FieldType::Record(vec![]));
    test_parse_db_field_type!(
        record_single_alien,
        "record<alien> ",
        FieldType::Record(vec!["alien".into()])
    );
    test_parse_db_field_type!(
        record_spaced,
        "record      < lowo | dayo  |     oye>",
        FieldType::Record(vec!["lowo".into(), "dayo".into(), "oye".into()])
    );
    test_parse_db_field_type!(
        record_no_space,
        "record<lowo|dayo|oye>",
        FieldType::Record(vec!["lowo".into(), "dayo".into(), "oye".into()])
    );

    test_parse_db_field_type!(
        geometry_empty_optional,
        "geometry",
        FieldType::Geometry(vec![])
    );

    test_parse_db_field_type!(
        geometry_single,
        "geometry<point>",
        FieldType::Geometry(vec![GeometryType::Point])
    );

    test_parse_db_field_type!(
        geometry_spaced,
        "geometry    < point | LineString  |     polygon>",
        FieldType::Geometry(vec![
            GeometryType::Point,
            GeometryType::LineString,
            GeometryType::Polygon
        ])
    );

    test_parse_db_field_type!(
        geometry_no_space,
        "geometry<collection| point|multipolygon|LineString|polygon>",
        FieldType::Geometry(vec![
            GeometryType::Collection,
            GeometryType::Point,
            GeometryType::MultiPolygon,
            GeometryType::LineString,
            GeometryType::Polygon
        ])
    );
    test_parse_db_field_type!(
        option_string,
        "option<string>",
        FieldType::Option(Box::new(FieldType::String))
    );
    test_parse_db_field_type!(
        option_float,
        "option<float>",
        FieldType::Option(Box::new(FieldType::Float))
    );
    test_parse_db_field_type!(
        option_bool,
        "option<bool>",
        FieldType::Option(Box::new(FieldType::Bool))
    );
    test_parse_db_field_type!(
        option_decimal,
        "option<decimal>",
        FieldType::Option(Box::new(FieldType::Decimal))
    );
    test_parse_db_field_type!(
        option_duration,
        "option<duration>",
        FieldType::Option(Box::new(FieldType::Duration))
    );
    test_parse_db_field_type!(
        option_datetime,
        "option<datetime>",
        FieldType::Option(Box::new(FieldType::Datetime))
    );
    test_parse_db_field_type!(
        option_uuid,
        "option<uuid>",
        FieldType::Option(Box::new(FieldType::Uuid))
    );
    test_parse_db_field_type!(
        option_number,
        "option<number>",
        FieldType::Option(Box::new(FieldType::Number))
    );
    test_parse_db_field_type!(
        option_object,
        "option<object>",
        FieldType::Option(Box::new(FieldType::Object))
    );
    test_parse_db_field_type!(
        option_bytes,
        "option<bytes>",
        FieldType::Option(Box::new(FieldType::Bytes))
    );
    test_parse_db_field_type!(
        option_any,
        "option<any>",
        FieldType::Option(Box::new(FieldType::Any))
    );
    test_parse_db_field_type!(
        option_null,
        "option<null>",
        FieldType::Option(Box::new(FieldType::Null))
    );
    test_parse_db_field_type!(
        option_geometry,
        "option<geometry>",
        FieldType::Option(Box::new(FieldType::Geometry(vec![])))
    );
    test_parse_db_field_type!(
        option_geometry_single,
        "option<geometry<point>>",
        FieldType::Option(Box::new(FieldType::Geometry(vec![GeometryType::Point])))
    );
    test_parse_db_field_type!(
        option_geometry_spaced,
        "option<geometry    < point | LineString |     polygon>>",
        FieldType::Option(Box::new(FieldType::Geometry(vec![
            GeometryType::Point,
            GeometryType::LineString,
            GeometryType::Polygon
        ])))
    );
    test_parse_db_field_type!(
        option_geometry_no_space,
        "option<geometry<collection| point|multipolygon|LineString|polygon>>",
        FieldType::Option(Box::new(FieldType::Geometry(vec![
            GeometryType::Collection,
            GeometryType::Point,
            GeometryType::MultiPolygon,
            GeometryType::LineString,
            GeometryType::Polygon
        ])))
    );
    test_parse_db_field_type!(
        option_int,
        "option<int>",
        FieldType::Option(Box::new(FieldType::Int))
    );
    test_parse_db_field_type!(
        option_simple_record,
        "option<record>",
        FieldType::Option(Box::new(FieldType::Record(vec![])))
    );
    test_parse_db_field_type!(
        option_record,
        "option<record<alien>>",
        FieldType::Option(Box::new(FieldType::Record(vec!["alien".into()])))
    );
    test_parse_db_field_type!(
        option_record_spaced,
        "option<record    < lowo | dayo  |     oye>>",
        FieldType::Option(Box::new(FieldType::Record(vec![
            "lowo".into(),
            "dayo".into(),
            "oye".into()
        ])))
    );
    test_parse_db_field_type!(
        option_record_no_space,
        "option<record<lowo|dayo|oye>>",
        FieldType::Option(Box::new(FieldType::Record(vec![
            "lowo".into(),
            "dayo".into(),
            "oye".into()
        ])))
    );

    // Union/Either
    test_parse_db_field_type!(
        union_string_int,
        "string|int",
        FieldType::Union(vec![FieldType::String, FieldType::Int])
    );

    // Array
    test_parse_db_field_type!(
        option_array,
        "option<array>",
        FieldType::Option(Box::new(FieldType::Array(Box::new(FieldType::Any), None)))
    );
    test_parse_db_field_type!(
        option_array_string,
        "option<array<string>>",
        FieldType::Option(Box::new(FieldType::Array(
            Box::new(FieldType::String),
            None
        )))
    );
    test_parse_db_field_type!(
        option_array_string_10_nospace,
        "option<array<string,10>>",
        FieldType::Option(Box::new(FieldType::Array(
            Box::new(FieldType::String),
            Some(10)
        )))
    );

    test_parse_db_field_type!(
        option_array_string_10,
        "option<array<string, 10>>",
        FieldType::Option(Box::new(FieldType::Array(
            Box::new(FieldType::String),
            Some(10)
        )))
    );
    test_parse_db_field_type!(
        option_array_string_10_spaced,
        "option<array    < string , 10> >",
        FieldType::Option(Box::new(FieldType::Array(
            Box::new(FieldType::String),
            Some(10)
        )))
    );
    test_parse_db_field_type!(
        option_array_string_10_spaced2,
        "option   <  array    < string ,   10> >",
        FieldType::Option(Box::new(FieldType::Array(
            Box::new(FieldType::String),
            Some(10)
        )))
    );
    // parse for array
    test_parse_db_field_type!(
        array_any,
        "array",
        FieldType::Array(Box::new(FieldType::Any), None)
    );
    test_parse_db_field_type!(
        array_string,
        "array<string>",
        FieldType::Array(Box::new(FieldType::String), None)
    );
    test_parse_db_field_type!(
        array_object_10_nospace,
        "array<object,69>",
        FieldType::Array(Box::new(FieldType::Object), Some(69))
    );
    test_parse_db_field_type!(
        array_geometry_10,
        "array<geometry<point | polygon | multipolygon>, 10>",
        FieldType::Array(
            Box::new(FieldType::Geometry(vec![
                GeometryType::Point,
                GeometryType::Polygon,
                GeometryType::MultiPolygon
            ])),
            Some(10)
        )
    );
    test_parse_db_field_type!(
        array_null_10_spaced,
        "array    < null , 10> ",
        FieldType::Array(Box::new(FieldType::Null), Some(10))
    );
    test_parse_db_field_type!(
        array_nested_array,
        "array<array<float, 42> , 10> ",
        FieldType::Array(
            Box::new(FieldType::Array(Box::new(FieldType::Float), Some(42))),
            Some(10)
        )
    );

    // SET TESTS
    test_parse_db_field_type!(
        set_any,
        "set",
        FieldType::Set(Box::new(FieldType::Any), None)
    );
    test_parse_db_field_type!(
        set_string,
        "set<string>",
        FieldType::Set(Box::new(FieldType::String), None)
    );
    test_parse_db_field_type!(
        set_object_10_nospace,
        "set<object,69>",
        FieldType::Set(Box::new(FieldType::Object), Some(69))
    );
    test_parse_db_field_type!(
        set_geometry_10,
        "set<geometry<point | polygon | multipolygon>, 10>",
        FieldType::Set(
            Box::new(FieldType::Geometry(vec![
                GeometryType::Point,
                GeometryType::Polygon,
                GeometryType::MultiPolygon
            ])),
            Some(10)
        )
    );
    test_parse_db_field_type!(
        set_null_10_spaced,
        "set    < null , 10> ",
        FieldType::Set(Box::new(FieldType::Null), Some(10))
    );
    test_parse_db_field_type!(
        set_nested_array,
        "set<set<float, 42> , 10> ",
        FieldType::Set(
            Box::new(FieldType::Set(Box::new(FieldType::Float), Some(42))),
            Some(10)
        )
    );

    /////// Test Union
    // test_parse_db_field_type!(union_any_single, "any", FieldTypee::Any);

    test_parse_db_field_type!(
        union_any_null,
        "any | null",
        FieldType::Union(vec![FieldType::Any, FieldType::Null])
    );

    test_parse_db_field_type!(
        union_any_null_bool,
        "any | null | bool",
        FieldType::Union(vec![FieldType::Any, FieldType::Null, FieldType::Bool])
    );

    test_parse_db_field_type!(
        union_any_null_bool_bytes,
        "any | null | bool | bytes",
        FieldType::Union(vec![
            FieldType::Any,
            FieldType::Null,
            FieldType::Bool,
            FieldType::Bytes
        ])
    );

    test_parse_db_field_type!(
        union_any_null_bool_bytes_datetime,
        "any | null | bool | bytes | datetime",
        FieldType::Union(vec![
            FieldType::Any,
            FieldType::Null,
            FieldType::Bool,
            FieldType::Bytes,
            FieldType::Datetime
        ])
    );

    test_parse_db_field_type!(
        union_any_null_bool_bytes_datetime_decimal,
        "any | null | bool | bytes | datetime | decimal",
        FieldType::Union(vec![
            FieldType::Any,
            FieldType::Null,
            FieldType::Bool,
            FieldType::Bytes,
            FieldType::Datetime,
            FieldType::Decimal
        ])
    );

    test_parse_db_field_type!(
        union_any_null_bool_bytes_datetime_decimal_duration,
        "any | null | bool | bytes | datetime | decimal | duration",
        FieldType::Union(vec![
            FieldType::Any,
            FieldType::Null,
            FieldType::Bool,
            FieldType::Bytes,
            FieldType::Datetime,
            FieldType::Decimal,
            FieldType::Duration
        ])
    );

    test_parse_db_field_type!(
        union_any_null_bool_bytes_datetime_decimal_duration_float,
        "any | null | bool | bytes | datetime | decimal | duration | float",
        FieldType::Union(vec![
            FieldType::Any,
            FieldType::Null,
            FieldType::Bool,
            FieldType::Bytes,
            FieldType::Datetime,
            FieldType::Decimal,
            FieldType::Duration,
            FieldType::Float
        ])
    );

    test_parse_db_field_type!(
        union_any_null_bool_bytes_datetime_decimal_duration_float_int,
        "any | null | bool | bytes | datetime | decimal | duration | float | int",
        FieldType::Union(vec![
            FieldType::Any,
            FieldType::Null,
            FieldType::Bool,
            FieldType::Bytes,
            FieldType::Datetime,
            FieldType::Decimal,
            FieldType::Duration,
            FieldType::Float,
            FieldType::Int
        ])
    );

    test_parse_db_field_type!(
        union_any_null_bool_bytes_datetime_decimal_duration_float_int_number,
        "any | null | bool | bytes | datetime | decimal | duration | float | int | number",
        FieldType::Union(vec![
            FieldType::Any,
            FieldType::Null,
            FieldType::Bool,
            FieldType::Bytes,
            FieldType::Datetime,
            FieldType::Decimal,
            FieldType::Duration,
            FieldType::Float,
            FieldType::Int,
            FieldType::Number
        ])
    );

    test_parse_db_field_type!(
        union_any_null_bool_bytes_datetime_decimal_duration_float_int_number_object,
        "any | null | bool | bytes | datetime | decimal | duration | float | int | number | object",
        FieldType::Union(vec![
            FieldType::Any,
            FieldType::Null,
            FieldType::Bool,
            FieldType::Bytes,
            FieldType::Datetime,
            FieldType::Decimal,
            FieldType::Duration,
            FieldType::Float,
            FieldType::Int,
            FieldType::Number,
            FieldType::Object
        ])
    );

    test_parse_db_field_type!(
        union_any_null_bool_bytes_datetime_decimal_duration_float_int_number_object_string,
        "any | null | bool | bytes | datetime | decimal | duration | float | int | number | object | string",
        FieldType::Union(vec![
            FieldType::Any,
            FieldType::Null,
            FieldType::Bool,
            FieldType::Bytes,
            FieldType::Datetime,
            FieldType::Decimal,
            FieldType::Duration,
            FieldType::Float,
            FieldType::Int,
            FieldType::Number,
            FieldType::Object,
            FieldType::String
        ])
    );

    test_parse_db_field_type!(
        union_any_null_bool_bytes_datetime_decimal_duration_float_int_number_object_string_uuid,
        "any | null | bool | bytes | datetime | decimal | duration | float | int | number | object | string | uuid",
        FieldType::Union(vec![
            FieldType::Any,
            FieldType::Null,
            FieldType::Bool,
            FieldType::Bytes,
            FieldType::Datetime,
            FieldType::Decimal,
            FieldType::Duration,
            FieldType::Float,
            FieldType::Int,
            FieldType::Number,
            FieldType::Object,
            FieldType::String,
            FieldType::Uuid
        ])
    );

    test_parse_db_field_type!(
        union_any_null_bool_bytes_datetime_decimal_duration_float_int_number_object_string_uuid_record,
        "any | null | bool | bytes | datetime | decimal | duration | float | int | number | object | string | uuid | record",
        FieldType::Union(vec![
            FieldType::Any,
            FieldType::Null,
            FieldType::Bool,
            FieldType::Bytes,
            FieldType::Datetime,
            FieldType::Decimal,
            FieldType::Duration,
            FieldType::Float,
            FieldType::Int,
            FieldType::Number,
            FieldType::Object,
            FieldType::String,
            FieldType::Uuid,
            FieldType::Record(vec![])
        ])
    );

    test_parse_db_field_type!(
        union_any_null_bool_bytes_datetime_decimal_duration_float_int_number_object_string_uuid_record_geometry,
        "any | null | bool | bytes | datetime | decimal | duration | float | int | number | object | string | uuid | record | geometry",
        FieldType::Union(vec![
            FieldType::Any,
            FieldType::Null,
            FieldType::Bool,
            FieldType::Bytes,
            FieldType::Datetime,
            FieldType::Decimal,
            FieldType::Duration,
            FieldType::Float,
            FieldType::Int,
            FieldType::Number,
            FieldType::Object,
            FieldType::String,
            FieldType::Uuid,
            FieldType::Record(vec![]),
            FieldType::Geometry(vec![])
        ])
    );

    test_parse_db_field_type!(
        union_any_null_bool_bytes_datetime_decimal_duration_float_int_number_object_string_uuid_record_geometry_array,
        "any | null | bool | bytes | datetime | decimal | duration | float | int | number | object | string | uuid | record | geometry | array",
        FieldType::Union(vec![
            FieldType::Any,
            FieldType::Null,
            FieldType::Bool,
            FieldType::Bytes,
            FieldType::Datetime,
            FieldType::Decimal,
            FieldType::Duration,
            FieldType::Float,
            FieldType::Int,
            FieldType::Number,
            FieldType::Object,
            FieldType::String,
            FieldType::Uuid,
            FieldType::Record(vec![]),
            FieldType::Geometry(vec![]),
            FieldType::Array(Box::new(FieldType::Any), None)
        ])
    );

    test_parse_db_field_type!(
        union_any_null_bool_bytes_datetime_decimal_duration_float_int_number_object_string_uuid_record_geometry_array_set,
        "any | null | bool | bytes | datetime | decimal | duration | float | int | number | object | string | uuid | record | geometry | array | set",
        FieldType::Union(vec![
            FieldType::Any,
            FieldType::Null,
            FieldType::Bool,
            FieldType::Bytes,
            FieldType::Datetime,
            FieldType::Decimal,
            FieldType::Duration,
            FieldType::Float,
            FieldType::Int,
            FieldType::Number,
            FieldType::Object,
            FieldType::String,
            FieldType::Uuid,
            FieldType::Record(vec![]),
            FieldType::Geometry(vec![]),
            FieldType::Array(Box::new(FieldType::Any), None),
            FieldType::Set(Box::new(FieldType::Any), None)
        ])
    );

    test_parse_db_field_type!(
        union_any_null_bool_bytes_datetime_decimal_duration_float_int_number_object_string_uuid_record_geometry_array_set_option,
        "any | null | bool | bytes | datetime | decimal | duration | float | int | number | object | string | uuid | record | geometry | array | set | option<int>",
        FieldType::Union(vec![
            FieldType::Any,
            FieldType::Null,
            FieldType::Bool,
            FieldType::Bytes,
            FieldType::Datetime,
            FieldType::Decimal,
            FieldType::Duration,
            FieldType::Float,
            FieldType::Int,
            FieldType::Number,
            FieldType::Object,
            FieldType::String,
            FieldType::Uuid,
            FieldType::Record(vec![]),
            FieldType::Geometry(vec![]),
            FieldType::Array(Box::new(FieldType::Any), None),
            FieldType::Set(Box::new(FieldType::Any), None),
            FieldType::Option(Box::new(FieldType::Int))
        ])
    );

    test_parse_db_field_type!(
        union_array_of_string_and_int_or_null,
        "array<string|int|null>",
        FieldType::Array(
            Box::new(FieldType::Union(vec![
                FieldType::String,
                FieldType::Int,
                FieldType::Null
            ])),
            None
        )
    );

    test_parse_db_field_type!(
        union_array_of_string_and_int_or_null_10,
        "int | option<float> | array<option<string>|int|null, 10> | set<option<number>|float|null, 10> | option<array> | option<set<option<int>>>",
        FieldType::Union(vec![
            FieldType::Int,
            FieldType::Option(Box::new(FieldType::Float)),
            FieldType::Array(
                Box::new(FieldType::Union(vec![
                    FieldType::Option(Box::new(FieldType::String)),
                    FieldType::Int,
                    FieldType::Null
                ])),
                Some(10)
            ),
            FieldType::Set(
                Box::new(FieldType::Union(vec![
                    FieldType::Option(Box::new(FieldType::Number)),
                    FieldType::Float,
                    FieldType::Null
                ])),
                Some(10)
            ),
            FieldType::Option(Box::new(FieldType::Array(Box::new(FieldType::Any), None))),
            FieldType::Option(Box::new(FieldType::Set(
                Box::new(FieldType::Option(Box::new(FieldType::Int))),
                None
            )))
        ])
    );
}

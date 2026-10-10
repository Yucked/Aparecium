#[cfg(test)]
mod tests {
    use aparecium::functions::type_;
    use aparecium::*;

    #[test]
    fn test_bool_with_macro_with_plain_number() {
        let result = type_::bool!(43545);
        assert_eq!(result.fine_tune_params(), "type::bool($_param_00000001)");
        assert_eq!(result.to_raw().build(), "type::bool(43545)");
    }

    #[test]
    fn test_bool_with_macro_with_plain_false() {
        let result = type_::bool!(false);
        assert_eq!(result.fine_tune_params(), "type::bool($_param_00000001)");
        assert_eq!(result.to_raw().build(), "type::bool(false)");
    }

    #[test]
    fn test_bool_with_macro_with_plain_true() {
        let result = type_::bool!(true);
        assert_eq!(result.fine_tune_params(), "type::bool($_param_00000001)");
        assert_eq!(result.to_raw().build(), "type::bool(true)");
    }

    #[test]
    fn test_datetime_macro_with_plain_datetime() {
        let dt = chrono::DateTime::from_timestamp(61, 0).unwrap();
        let result = type_::datetime!(dt);
        assert_eq!(
            result.fine_tune_params(),
            "type::datetime($_param_00000001)"
        );
        assert_eq!(
            result.to_raw().build(),
            "type::datetime('1970-01-01T00:01:01Z')"
        );
    }

    #[test]
    fn test_datetime_macro_with_datetime_field() {
        let rebirth_date = Field::new("rebirth_date");
        let result = type_::datetime!(rebirth_date);

        assert_eq!(result.fine_tune_params(), "type::datetime(rebirth_date)");
        assert_eq!(result.to_raw().build(), "type::datetime(rebirth_date)");
    }

    #[test]
    fn test_datetime_macro_with_param() {
        let rebirth_date = Param::new("rebirth_date");
        let result = type_::datetime!(rebirth_date);

        assert_eq!(result.fine_tune_params(), "type::datetime($rebirth_date)");
        assert_eq!(result.to_raw().build(), "type::datetime($rebirth_date)");
    }

    #[test]
    fn test_point_macro_with_plain_values() {
        let result = type_::point!(51.509865, -0.118092);
        assert_eq!(
            result.fine_tune_params(),
            "type::point($_param_00000001, $_param_00000002)"
        );
        assert_eq!(
            result.to_raw().build(),
            "type::point(51.509865f, -0.118092f)"
        );
    }

    #[test]
    fn test_point_macro_with_fields() {
        let home = Field::new("home");
        let away = Field::new("away");
        let result = type_::point!(home, away);
        assert_eq!(result.fine_tune_params(), "type::point(home, away)");
        assert_eq!(result.to_raw().build(), "type::point(home, away)");
    }

    #[test]
    fn test_thing_macro_with_plain_values() {
        let user = Table::from("user");
        let id = "oyelowo";
        let result = type_::thing!(user, id);
        assert_eq!(
            result.fine_tune_params(),
            "type::thing($_param_00000001, $_param_00000002)"
        );
        assert_eq!(result.to_raw().build(), "type::thing(user, 'oyelowo')");
    }

    #[test]
    fn test_thing_macro_with_datetime_field() {
        let table = Table::new("table");
        let id = Field::new("id");
        let result = type_::thing!(table, id);

        assert_eq!(
            result.fine_tune_params(),
            "type::thing($_param_00000001, id)"
        );
        assert_eq!(result.to_raw().build(), "type::thing(table, id)");
    }
}

macro_rules! create_type_tests {
    ($(#[$attr:meta])* => $function_name:expr, $value_type: ty, $test_data_input:expr, $test_stringified_data_output: expr) => {
        paste::paste! {
            mod [<test_ $function_name>] {
                use aparecium::functions::type_::*;
                use aparecium::*;
                use aparecium::functions::type_;

                #[test]
                fn [<test_ $function_name _with_field>]() {
                    let name = Field::new("name");
                    let result = type_::[<$function_name _fn>](name);
                    assert_eq!(result.fine_tune_params(), format!("type::{}(name)", $function_name));
                    assert_eq!(result.to_raw().build(), format!("type::{}(name)", $function_name));
                }

                #[test]
                fn [<test_ $function_name _with_plain_string>]() {
                    let result = type_::[<$function_name _fn>]($test_data_input);
                    assert_eq!(result.fine_tune_params(), format!("type::{}($_param_00000001)", $function_name));
                    assert_eq!(result.to_raw().build(), format!("type::{}({})", $function_name, $test_stringified_data_output));
                }
            }
        }
    };
}

create_type_tests!(
    => 
    "bool", ValueLike, "toronto", "'toronto'");
create_type_tests!(
    =>
    "datetime",
    DatetimeLike,
    chrono::DateTime::from_timestamp(61, 0).unwrap(),
    "'1970-01-01T00:01:01Z'"
);
create_type_tests!(
    =>
    "duration",
    DurationLike,
    std::time::Duration::from_secs(24 * 60 * 60 * 7),
    "1w"
);
create_type_tests!(
    =>
    "field", ValueLike, "name", "'name'"
);
create_type_tests!(
    =>
    "fields", ArrayLike, ["name"], "['name']"
);
create_type_tests!(
    =>
    "float", NumberLike, 43.5, "43.5f"
);
create_type_tests!(
    =>
    "int", NumberLike, 99, 99
);
create_type_tests!(
    =>
    "number", NumberLike, 5, 5
);
create_type_tests!(
    =>
    "string", ValueLike, 5454, "5454"
);
create_type_tests!(
    =>
    "regex", StrandLike, "/[A-Z]{3}/", "'/[A-Z]{3}/'"
);
create_type_tests!(
    =>
    "table", TableLike, Table::new("user"), "user"
);

macro_rules! create_is_function_tests {
    ($(#[$attr:meta])* => $function_name:expr, $value_type: ty, $test_data_input:expr, $test_stringified_data_output: expr) => {
        paste::paste! {
            mod [<test_is_ $function_name>] {
                use aparecium::functions::type_::is::*;
                use aparecium::*;
                use aparecium::functions::type_;

                #[test]
                fn [<test_is_ $function_name _with_field>]() {
                    let name = Field::new("name");
                    let result = type_::is::[<$function_name _fn>](name);
                    assert_eq!(result.fine_tune_params(), format!("type::is::{}(name)", $function_name));
                    assert_eq!(result.to_raw().build(), format!("type::is::{}(name)", $function_name));
                }

                #[test]
                fn [<test_is_ $function_name _with_plain_string>]() {
                    let result = type_::is::[<$function_name _fn>]($test_data_input);
                    assert_eq!(result.fine_tune_params(), format!("type::is::{}($_param_00000001)", $function_name));
                    assert_eq!(result.to_raw().build(), format!("type::is::{}({})", $function_name, $test_stringified_data_output));
                }
            }
        }
    };
}

mod is {
    create_is_function_tests!(
        =>
        "array", ValueLike, 5454, "5454"
    );
    create_is_function_tests!(
        =>
        "bool", ValueLike, 5454, "5454"
    );
    create_is_function_tests!(
        =>
        "bytes", ValueLike, 5454, "5454"
    );
    create_is_function_tests!(
        =>
        "datetime",
        DatetimeLike,
        chrono::DateTime::from_timestamp(61, 0).unwrap(),
        "'1970-01-01T00:01:01Z'"
    );
    create_is_function_tests!(
        =>
        "collection", ValueLike, 5454, "5454"
    );
    create_is_function_tests!(
        =>
        "decimal", ValueLike, 5454, "5454"
    );
    create_is_function_tests!(
        =>
        "duration",
        DurationLike,
        std::time::Duration::from_secs(24 * 60 * 60 * 7),
        "1w"
    );
    create_is_function_tests!(
        =>
        "float", NumberLike, 5454, "5454"
    );
    create_is_function_tests!(
        =>
        "int", NumberLike, 5454, "5454"
    );
    create_is_function_tests!(
        =>
        "null", ValueLike, 5454, "5454"
    );
    create_is_function_tests!(
        =>
        "number", NumberLike, 5454, "5454"
    );
    create_is_function_tests!(
        =>
        "string", StrandLike, "Oyelowo", "'Oyelowo'"
    );
    create_is_function_tests!(
        =>
        "geometry", aparecium::GeometryLike, geo::point!(x: 51.509865, y: -0.118092), "(51.509865, -0.118092)"
    );
    create_is_function_tests!(
        =>
        "line", aparecium::GeometryLike, geo::LineString(vec![
        geo::Coord {
            x: -122.33583,
            y: 47.60621,
        },
        geo::Coord {
            x: -122.33583,
            y: 47.60622,
        },
        geo::Coord {
            x: -122.33584,
            y: 47.60622,
        },
        geo::Coord {
            x: -122.33584,
            y: 47.60621,
        },
        geo::Coord {
            x: -122.33583,
            y: 47.60621,
        },
    ]), "{ type: 'LineString', coordinates: [[-122.33583, 47.60621], [-122.33583, 47.60622], [-122.33584, 47.60622], [-122.33584, 47.60621], [-122.33583, 47.60621]] }"
    );
    create_is_function_tests!(
        =>
        "multiline", aparecium::GeometryLike, geo::MultiLineString(vec![]), "{ type: 'MultiLineString', coordinates: [] }"
    );
    create_is_function_tests!(
        =>
        "multipoint", aparecium::GeometryLike, geo::MultiPoint(vec![
        geo::Point::new(0.0, 0.0),
        geo::Point::new(1.0, 1.0),
        (2.0, 35.0).into(),
    ]), "{ type: 'MultiPoint', coordinates: [[0, 0], [1, 1], [2, 35]] }"
    );
    create_is_function_tests!(
    =>
    "multipolygon", aparecium::GeometryLike, geo::MultiPolygon(vec![]), "{ type: 'MultiPolygon', coordinates: [] }"
    );
    create_is_function_tests!(
        =>
        "point", aparecium::GeometryLike, geo::Point::new(0.0, 0.0), "(0, 0)"
    );
    create_is_function_tests!(
    =>
    "polygon", aparecium::GeometryLike, geo::Polygon::new(
        geo::LineString(vec![
            geo::Coord {
                x: -122.33583,
                y: 47.60621,
            },
            geo::Coord {
                x: -122.33583,
                y: 47.60622,
            },
        ]),
        vec![geo::LineString(vec![
            geo::Coord {
                x: -122.33583,
                y: 47.60621,
            },
            geo::Coord {
                x: -122.33583,
                y: 47.60622,
            },
        ])],
    ), "{ type: 'Polygon', coordinates: [[[-122.33583, 47.60621], [-122.33583, 47.60622], [-122.33583, 47.60621]], [[[-122.33583, 47.60621], [-122.33583, 47.60622], [-122.33583, 47.60621]]]] }"
    );
    create_is_function_tests!(
        =>
        "object", ObjectLike, sql::Object::default(), "{  }"
    );
    create_is_function_tests!(
        =>
        "record", ThingLike, sql::thing("user:1").unwrap(), "user:1"
    );
    create_is_function_tests!(
        =>
        "uuid", ValueLike, uuid::Uuid::parse_str("936DA01F9ABD4d9d80C702AF85C822A8").unwrap(), "'936da01f-9abd-4d9d-80c7-02af85c822a8'"
    );
}

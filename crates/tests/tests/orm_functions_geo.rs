#[cfg(test)]
mod tests {
    use ::geo::{point, polygon};
    use umbriel::*;
    use umbriel::{functions::geo::*, *};
    use functions::geo;

    #[test]
    fn test_area_with_field() {
        let city = Field::new("city");
        let result = geo::area_fn(city);

        assert_eq!(result.fine_tune_params(), "geo::area(city)");
        assert_eq!(result.to_raw().build(), "geo::area(city)");
    }

    #[test]
    fn test_area_with_raw_polygon() {
        let poly = polygon!(
            exterior: [
                (x: -111., y: 45.),
                (x: -111., y: 41.),
                (x: -104., y: 41.),
                (x: -104., y: 45.),
            ],
            interiors: [
                [
                    (x: -110., y: 44.),
                    (x: -110., y: 42.),
                    (x: -105., y: 42.),
                    (x: -105., y: 44.),
                ],
            ],
        );
        let result = geo::area_fn(poly);
        assert_eq!(result.fine_tune_params(), "geo::area($_param_00000001)");
        assert_eq!(
            result.to_raw().build(),
            "geo::area({ type: 'Polygon', coordinates: [[[-111, 45], [-111, 41], [-104, 41], [-104, 45], [-111, 45]], [[[-110, 44], [-110, 42], [-105, 42], [-105, 44], [-110, 44]]]] })"
        );
    }

    #[test]
    fn test_area_macro_with_raw_polygon() {
        let poly = polygon!(
            exterior: [
                (x: -111., y: 45.),
                (x: -111., y: 41.),
                (x: -104., y: 41.),
                (x: -104., y: 45.),
            ],
            interiors: [
                [
                    (x: -110., y: 44.),
                    (x: -110., y: 42.),
                    (x: -105., y: 42.),
                    (x: -105., y: 44.),
                ],
            ],
        );
        let result = geo::area!(poly);
        assert_eq!(result.fine_tune_params(), "geo::area($_param_00000001)");
        assert_eq!(
            result.to_raw().build(),
            "geo::area({ type: 'Polygon', coordinates: [[[-111, 45], [-111, 41], [-104, 41], [-104, 45], [-111, 45]], [[[-110, 44], [-110, 42], [-105, 42], [-105, 44], [-110, 44]]]] })"
        );
    }

    #[test]
    fn test_area_macro_with_fields() {
        let poly = Field::new("poly");
        let result = geo::area!(poly);
        assert_eq!(result.fine_tune_params(), "geo::area(poly)");
        assert_eq!(result.to_raw().build(), "geo::area(poly)");
    }

    #[test]
    fn test_area_macro_with_params() {
        let poly = Param::new("poly");
        let result = geo::area!(poly);
        assert_eq!(result.fine_tune_params(), "geo::area($poly)");
        assert_eq!(result.to_raw().build(), "geo::area($poly)");
    }

    #[test]
    fn test_bearing_with_raw_points() {
        let point1 = point! {
            x: 40.02f64,
            y: 116.34,
        };

        let point2 = point! {
            x: 80.02f64,
            y: 103.19,
        };
        let result = geo::bearing_fn(point1, point2);
        assert_eq!(
            result.fine_tune_params(),
            "geo::bearing($_param_00000001, $_param_00000002)"
        );
        assert_eq!(
            result.to_raw().build(),
            "geo::bearing((40.02, 116.34), (80.02, 103.19))"
        );
    }

    #[test]
    fn test_bearing_with_raw_point_with_field() {
        let hometown = Field::new("hometown");

        let point2 = point! {
            x: 80.02f64,
            y: 103.19,
        };
        let result = geo::bearing_fn(hometown, point2);
        assert_eq!(
            result.fine_tune_params(),
            "geo::bearing(hometown, $_param_00000001)"
        );
        assert_eq!(
            result.to_raw().build(),
            "geo::bearing(hometown, (80.02, 103.19))"
        );
    }

    #[test]
    fn test_bearing_macro_with_raw_points() {
        let point1 = point! {
            x: 40.02f64,
            y: 116.34,
        };

        let point2 = point! {
            x: 80.02f64,
            y: 103.19,
        };
        let result = geo::bearing!(point1, point2);
        assert_eq!(
            result.fine_tune_params(),
            "geo::bearing($_param_00000001, $_param_00000002)"
        );
        assert_eq!(
            result.to_raw().build(),
            "geo::bearing((40.02, 116.34), (80.02, 103.19))"
        );
    }

    #[test]
    fn test_bearing_macro_with_raw_point_with_field() {
        let hometown = Field::new("hometown");

        let point2 = point! {
            x: 80.02f64,
            y: 103.19,
        };
        let result = geo::bearing!(hometown, point2);
        assert_eq!(
            result.fine_tune_params(),
            "geo::bearing(hometown, $_param_00000001)"
        );
        assert_eq!(
            result.to_raw().build(),
            "geo::bearing(hometown, (80.02, 103.19))"
        );
    }

    #[test]
    fn test_bearing_macro_with_raw_params() {
        let hometown = Param::new("hometown");
        let point2 = Param::new("point2");

        let result = geo::bearing!(hometown, point2);
        assert_eq!(
            result.fine_tune_params(),
            "geo::bearing($hometown, $point2)"
        );
        assert_eq!(result.to_raw().build(), "geo::bearing($hometown, $point2)");
    }

    #[test]
    fn test_centroid_with_field() {
        let city = Field::new("city");
        let result = centroid_fn(city);

        assert_eq!(result.fine_tune_params(), "geo::centroid(city)");
        assert_eq!(result.to_raw().build(), "geo::centroid(city)");
    }

    #[test]
    fn test_centroid_with_raw_polygon() {
        let poly = polygon!(
            exterior: [
                (x: -111., y: 45.),
                (x: -111., y: 41.),
                (x: -104., y: 41.),
                (x: -104., y: 45.),
            ],
            interiors: [
                [
                    (x: -110., y: 44.),
                    (x: -110., y: 42.),
                    (x: -105., y: 42.),
                    (x: -105., y: 44.),
                ],
            ],
        );
        let result = geo::centroid_fn(poly);
        assert_eq!(result.fine_tune_params(), "geo::centroid($_param_00000001)");
        assert_eq!(
            result.to_raw().build(),
            "geo::centroid({ type: 'Polygon', coordinates: [[[-111, 45], [-111, 41], [-104, 41], [-104, 45], [-111, 45]], [[[-110, 44], [-110, 42], [-105, 42], [-105, 44], [-110, 44]]]] })"
        );
    }

    #[test]
    fn test_centroid_macro_with_field() {
        let city = Field::new("city");
        let result = geo::centroid!(city);

        assert_eq!(result.fine_tune_params(), "geo::centroid(city)");
        assert_eq!(result.to_raw().build(), "geo::centroid(city)");
    }

    #[test]
    fn test_centroid_macro_with_param() {
        let city = Param::new("city");
        let result = geo::centroid!(city);

        assert_eq!(result.fine_tune_params(), "geo::centroid($city)");
        assert_eq!(result.to_raw().build(), "geo::centroid($city)");
    }

    #[test]
    fn test_centroid_macro_with_raw_polygon() {
        let poly = polygon!(
            exterior: [
                (x: -111., y: 45.),
                (x: -111., y: 41.),
                (x: -104., y: 41.),
                (x: -104., y: 45.),
            ],
            interiors: [
                [
                    (x: -110., y: 44.),
                    (x: -110., y: 42.),
                    (x: -105., y: 42.),
                    (x: -105., y: 44.),
                ],
            ],
        );
        let result = geo::centroid!(poly);
        assert_eq!(result.fine_tune_params(), "geo::centroid($_param_00000001)");
        assert_eq!(
            result.to_raw().build(),
            "geo::centroid({ type: 'Polygon', coordinates: [[[-111, 45], [-111, 41], [-104, 41], [-104, 45], [-111, 45]], [[[-110, 44], [-110, 42], [-105, 42], [-105, 44], [-110, 44]]]] })"
        );
    }

    #[test]
    fn test_distance_with_raw_points() {
        let point1 = point! {
            x: 40.02f64,
            y: 116.34,
        };

        let point2 = point! {
            x: 80.02f64,
            y: 103.19,
        };
        let result = geo::distance_fn(point1, point2);
        assert_eq!(
            result.fine_tune_params(),
            "geo::distance($_param_00000001, $_param_00000002)"
        );
        assert_eq!(
            result.to_raw().build(),
            "geo::distance((40.02, 116.34), (80.02, 103.19))"
        );
    }

    #[test]
    fn test_distance_with_raw_point_with_field() {
        let hometown = Field::new("hometown");

        let point2 = point! {
            x: 80.02f64,
            y: 103.19,
        };
        let result = geo::distance_fn(hometown, point2);
        assert_eq!(
            result.fine_tune_params(),
            "geo::distance(hometown, $_param_00000001)"
        );
        assert_eq!(
            result.to_raw().build(),
            "geo::distance(hometown, (80.02, 103.19))"
        );
    }

    #[test]
    fn test_distance_with_only_fields() {
        let hometown = Field::new("hometown");
        let yukon = Field::new("yukon");

        let result = geo::distance_fn(hometown, yukon);
        assert_eq!(result.fine_tune_params(), "geo::distance(hometown, yukon)");
        assert_eq!(result.to_raw().build(), "geo::distance(hometown, yukon)");
    }

    #[test]
    fn test_distance_macro_with_raw_points() {
        let point1 = point! {
            x: 40.02f64,
            y: 116.34,
        };

        let point2 = point! {
            x: 80.02f64,
            y: 103.19,
        };
        let result = geo::distance!(point1, point2);
        assert_eq!(
            result.fine_tune_params(),
            "geo::distance($_param_00000001, $_param_00000002)"
        );
        assert_eq!(
            result.to_raw().build(),
            "geo::distance((40.02, 116.34), (80.02, 103.19))"
        );
    }

    #[test]
    fn test_distance_macro_with_raw_point_with_field() {
        let hometown = Field::new("hometown");

        let point2 = point! {
            x: 80.02f64,
            y: 103.19,
        };
        let result = geo::distance!(hometown, point2);
        assert_eq!(
            result.fine_tune_params(),
            "geo::distance(hometown, $_param_00000001)"
        );
        assert_eq!(
            result.to_raw().build(),
            "geo::distance(hometown, (80.02, 103.19))"
        );
    }

    #[test]
    fn test_distance_macro_with_only_fields() {
        let hometown = Field::new("hometown");
        let yukon = Field::new("yukon");

        let result = geo::distance!(hometown, yukon);
        assert_eq!(result.fine_tune_params(), "geo::distance(hometown, yukon)");
        assert_eq!(result.to_raw().build(), "geo::distance(hometown, yukon)");
    }

    #[test]
    fn test_distance_macro_with_only_params() {
        let hometown = Param::new("hometown");
        let yukon = Param::new("yukon");

        let result = geo::distance!(hometown, yukon);
        assert_eq!(
            result.fine_tune_params(),
            "geo::distance($hometown, $yukon)"
        );
        assert_eq!(result.to_raw().build(), "geo::distance($hometown, $yukon)");
    }

    #[test]
    fn test_hash_decode_with_field() {
        let city = Field::new("city");
        let result = geo::hash::decode_fn(city);

        assert_eq!(result.fine_tune_params(), "geo::hash::decode(city)");
        assert_eq!(result.to_raw().build(), "geo::hash::decode(city)");
    }

    #[test]
    fn test_hash_decode_with_string() {
        let result = hash::decode_fn("mpuxk4s24f51");
        assert_eq!(
            result.fine_tune_params(),
            "geo::hash::decode($_param_00000001)"
        );
        assert_eq!(result.to_raw().build(), "geo::hash::decode('mpuxk4s24f51')");
    }

    #[test]
    fn test_hash_decode_macro_with_field() {
        let city = Field::new("city");
        let result = geo::hash::decode!(city);

        assert_eq!(result.fine_tune_params(), "geo::hash::decode(city)");
        assert_eq!(result.to_raw().build(), "geo::hash::decode(city)");
    }

    #[test]
    fn test_hash_decode_macro_with_param() {
        let city = Param::new("city");
        let result = geo::hash::decode!(city);

        assert_eq!(result.fine_tune_params(), "geo::hash::decode($city)");
        assert_eq!(result.to_raw().build(), "geo::hash::decode($city)");
    }

    #[test]
    fn test_hash_decode_macro_with_string() {
        let result = geo::hash::decode!("mpuxk4s24f51");
        assert_eq!(
            result.fine_tune_params(),
            "geo::hash::decode($_param_00000001)"
        );
        assert_eq!(result.to_raw().build(), "geo::hash::decode('mpuxk4s24f51')");
    }

    #[test]
    fn test_hash_encode_with_field_and_empty_accuracy() {
        let city = Field::new("city");
        let result = geo::hash::encode_fn(city, None as Option<NumberLike>);

        assert_eq!(result.fine_tune_params(), "geo::hash::encode(city)");
        assert_eq!(result.to_raw().build(), "geo::hash::encode(city)");
    }

    #[test]
    fn test_hash_encode_with_field_and_field_accuracy() {
        let city = Field::new("city");
        let accuracy = Field::new("accuracy");
        let result = geo::hash::encode_fn(city, Some(accuracy));

        assert_eq!(
            result.fine_tune_params(),
            "geo::hash::encode(city, accuracy)"
        );
        assert_eq!(result.to_raw().build(), "geo::hash::encode(city, accuracy)");
    }

    #[test]
    fn test_hash_encode_with_field_and_number_accuracy() {
        let city = Field::new("city");
        let result = geo::hash::encode_fn(city, Some(5));

        assert_eq!(
            result.fine_tune_params(),
            "geo::hash::encode(city, $_param_00000001)"
        );
        assert_eq!(result.to_raw().build(), "geo::hash::encode(city, 5)");
    }

    #[test]
    fn test_hash_encode_with_point() {
        let point = point! {
            x: 40.02f64,
            y: 116.34,
        };

        let result = geo::hash::encode_fn(point, Some(5));
        assert_eq!(
            result.fine_tune_params(),
            "geo::hash::encode($_param_00000001, $_param_00000002)"
        );
        assert_eq!(
            result.to_raw().build(),
            "geo::hash::encode((40.02, 116.34), 5)"
        );
    }
    // Macro version
    #[test]
    fn test_hash_encode_macro_with_field_and_empty_accuracy_not_mentioned_at_all() {
        let city = Field::new("city");
        let result = geo::hash::encode!(city);

        assert_eq!(result.fine_tune_params(), "geo::hash::encode(city)");
        assert_eq!(result.to_raw().build(), "geo::hash::encode(city)");
    }

    #[test]
    fn test_hash_encode_macro_with_field_and_empty_accuracy() {
        let city = Field::new("city");
        let result = geo::hash::encode!(city);

        assert_eq!(result.fine_tune_params(), "geo::hash::encode(city)");
        assert_eq!(result.to_raw().build(), "geo::hash::encode(city)");
    }

    #[test]
    fn test_hash_encode_macro_with_field_and_field_accuracy() {
        let city = Field::new("city");
        let accuracy = Field::new("accuracy");
        let result = geo::hash::encode!(city, accuracy);

        assert_eq!(
            result.fine_tune_params(),
            "geo::hash::encode(city, accuracy)"
        );
        assert_eq!(result.to_raw().build(), "geo::hash::encode(city, accuracy)");
    }

    #[test]
    fn test_hash_encode_macro_with_param_and_no_accuracy_listed() {
        let city = Param::new("city");
        let result = geo::hash::encode!(city);

        assert_eq!(result.fine_tune_params(), "geo::hash::encode($city)");
        assert_eq!(result.to_raw().build(), "geo::hash::encode($city)");
    }

    #[test]
    fn test_hash_encode_macros_with_field_and_number_accuracy() {
        let city = Field::new("city");
        let result = geo::hash::encode!(city, 5);

        assert_eq!(
            result.fine_tune_params(),
            "geo::hash::encode(city, $_param_00000001)"
        );
        assert_eq!(result.to_raw().build(), "geo::hash::encode(city, 5)");
    }

    #[test]
    fn test_hash_encode_macro_with_point() {
        let point = point! {
            x: 40.02f64,
            y: 116.34,
        };

        let result = geo::hash::encode!(point, 5);
        assert_eq!(
            result.fine_tune_params(),
            "geo::hash::encode($_param_00000001, $_param_00000002)"
        );
        assert_eq!(
            result.to_raw().build(),
            "geo::hash::encode((40.02, 116.34), 5)"
        );
    }
}

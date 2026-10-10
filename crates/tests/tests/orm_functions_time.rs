#[cfg(test)]
mod tests {
    use aparecium::functions::time;
    use aparecium::*;

    #[test]
    fn test_floor_macro_with_datetime_field() {
        let rebirth_date = Field::new("rebirth_date");
        let duration = Field::new("duration");
        let result = time::floor!(rebirth_date, duration);

        assert_eq!(
            result.fine_tune_params(),
            "time::floor(rebirth_date, duration)"
        );
        assert_eq!(
            result.to_raw().build(),
            "time::floor(rebirth_date, duration)"
        );
    }

    #[test]
    fn test_floor_macro_with_plain_datetime_and_duration() {
        let dt = chrono::DateTime::from_timestamp(61, 0).unwrap();
        let duration = std::time::Duration::from_secs(24 * 60 * 60 * 7);
        let result = time::floor!(dt, duration);
        assert_eq!(
            result.fine_tune_params(),
            "time::floor($_param_00000001, $_param_00000002)"
        );
        assert_eq!(
            result.to_raw().build(),
            "time::floor('1970-01-01T00:01:01Z', 1w)"
        );
    }

    #[test]
    fn test_round_macro_with_datetime_field() {
        let rebirth_date = Field::new("rebirth_date");
        let duration = Field::new("duration");
        let result = time::round!(rebirth_date, duration);

        assert_eq!(
            result.fine_tune_params(),
            "time::round(rebirth_date, duration)"
        );
        assert_eq!(
            result.to_raw().build(),
            "time::round(rebirth_date, duration)"
        );
    }

    #[test]
    fn test_round_macro_with_plain_datetime_and_duration() {
        let dt = chrono::DateTime::from_timestamp(61, 0).unwrap();
        let duration = std::time::Duration::from_secs(24 * 60 * 60 * 7);
        let result = time::round!(dt, duration);
        assert_eq!(
            result.fine_tune_params(),
            "time::round($_param_00000001, $_param_00000002)"
        );
        assert_eq!(
            result.to_raw().build(),
            "time::round('1970-01-01T00:01:01Z', 1w)"
        );
    }

    #[test]
    fn test_group_macro_with_datetime_field() {
        let rebirth_date = Field::new("rebirth_date");
        let duration = Field::new("duration");
        let result = time::group!(rebirth_date, duration);

        assert_eq!(
            result.fine_tune_params(),
            "time::group(rebirth_date, duration)"
        );
        assert_eq!(
            result.to_raw().build(),
            "time::group(rebirth_date, duration)"
        );
    }

    #[test]
    fn test_group_macro_with_datetime_params() {
        let rebirth_date = Param::new("rebirth_date");
        let duration = Param::new("duration");
        let result = time::group!(rebirth_date, duration);

        assert_eq!(
            result.fine_tune_params(),
            "time::group($rebirth_date, $duration)"
        );
        assert_eq!(
            result.to_raw().build(),
            "time::group($rebirth_date, $duration)"
        );
    }

    macro_rules! test_group_with_interval {
        ($interval_name:ident, $interval: expr) => {
            paste::paste! {
                #[test]
                fn [<test_group_macro_with_plain_datetime_and_ $interval_name>]() {
                    let dt = chrono::DateTime::from_timestamp(61, 0).unwrap();
                    let result = time::group!(dt, $interval);
                    assert_eq!(
                        result.fine_tune_params(),
                        "time::group($_param_00000001, $_param_00000002)"
                    );
                    assert_eq!(
                        result.to_raw().build(),
                        format!("time::group('1970-01-01T00:01:01Z', '{}')", $interval)
                    );
                }
            }
        };
    }

    test_group_with_interval!(year, "year");
    test_group_with_interval!(month, "month");
    test_group_with_interval!(week, "week");
    test_group_with_interval!(day, "day");
    test_group_with_interval!(hour, "hour");
    test_group_with_interval!(minute, "minute");
    test_group_with_interval!(second, "second");

    test_group_with_interval!(year_with_enum, Interval::Year);
    test_group_with_interval!(month_with_enum, Interval::Month);
    test_group_with_interval!(week_with_enum, Interval::Week);
    test_group_with_interval!(day_with_enum, Interval::Day);
    test_group_with_interval!(hour_with_enum, Interval::Hour);
    test_group_with_interval!(minute_with_enum, Interval::Minute);
    test_group_with_interval!(second_with_enum, Interval::Second);

    #[test]
    fn test_now_fn() {
        let result = time::now_fn();
        assert_eq!(result.fine_tune_params(), "now()");
        assert_eq!(result.to_raw().build(), "now()");
    }

    #[test]
    fn test_now_macro() {
        let result = time::now!();
        assert_eq!(result.fine_tune_params(), "now()");
        assert_eq!(result.to_raw().build(), "now()");
    }
}

macro_rules! create_time_fn_with_single_datetime_arg_tests {
    ($(#[$attr:meta])* => $function_name: expr) => {
        paste::paste! {
            mod [<test_ $function_name _fn>] {
                use aparecium::*;
                use aparecium::functions::time;


                #[test]
                fn [<test_ $function_name _fn_with_datetime_field>]() {
                    let rebirth_date = aparecium::Field::new("rebirth_date");
                    let result = time::[<$function_name>]!(rebirth_date);

                    assert_eq!(result.fine_tune_params(), format!("time::{}(rebirth_date)", $function_name));
                    assert_eq!(result.to_raw().build(), format!("time::{}(rebirth_date)", $function_name));
                }

                #[test]
                fn [<test_ $function_name _fn_with_plain_datetime>]() {
                    let dt = chrono::DateTime::from_timestamp(61, 0).unwrap();
                    let result = time::[<$function_name>]!(dt);
                    assert_eq!(result.fine_tune_params(), format!("time::{}($_param_00000001)", $function_name));
                    assert_eq!(
                        result.to_raw().build(),
                        format!("time::{}('1970-01-01T00:01:01Z')", $function_name)
                    );
                }
            }
        }
    };
}

create_time_fn_with_single_datetime_arg_tests!(
    =>
    "day"
);
create_time_fn_with_single_datetime_arg_tests!(
    =>
    "hour"
);
create_time_fn_with_single_datetime_arg_tests!(
    =>
    "minute"
);
create_time_fn_with_single_datetime_arg_tests!(
    =>
    "month"
);
create_time_fn_with_single_datetime_arg_tests!(
    =>
    "nano"
);
create_time_fn_with_single_datetime_arg_tests!(
    =>
    "second"
);
create_time_fn_with_single_datetime_arg_tests!(
    =>
    "unix"
);
create_time_fn_with_single_datetime_arg_tests!(
    =>
    "wday"
);
create_time_fn_with_single_datetime_arg_tests!(
    =>
    "week"
);
create_time_fn_with_single_datetime_arg_tests!(
    =>
    "yday"
);
create_time_fn_with_single_datetime_arg_tests!(
    =>
    "year"
);

macro_rules! create_time_fn_with_single_array_of_datetime_arg_tests {
    ($(#[$attr:meta])* => $function_name: expr) => {
        paste::paste! {
            mod [<test_ $function_name _fn>] {
                use aparecium::*;
                use aparecium::functions::time;


                #[test]
                fn [<test_ $function_name _fn_with_datetime_field>]() {
                    let rebirth_date = aparecium::Field::new("rebirth_date");
                    let result = time::[<$function_name>]!(rebirth_date);

                    assert_eq!(result.fine_tune_params(), format!("time::{}(rebirth_date)", $function_name));
                    assert_eq!(result.to_raw().build(), format!("time::{}(rebirth_date)", $function_name));
                }

                #[test]
                fn [<test_ $function_name _fn_with_plain_datetime>]() {
                    let dt = chrono::DateTime::from_timestamp(61, 0).unwrap();
                    let dt2 = aparecium::Param::new("dt2");
                    let result = time::[<$function_name>]!(arr![dt, dt2]);
                    assert_eq!(result.fine_tune_params(), format!("time::{}([$_param_00000001, $dt2])", $function_name));
                    assert_eq!(
                        result.to_raw().build(),
                        format!("time::{}(['1970-01-01T00:01:01Z', $dt2])", $function_name)

                    );
                }
            }
        }
    };
}

create_time_fn_with_single_array_of_datetime_arg_tests!(
    =>
    "max"
);
create_time_fn_with_single_array_of_datetime_arg_tests!(
    =>
    "min"
);

macro_rules! create_time_fn_with_single_number_arg_tests {
    ($(#[$attr:meta])* => $function_name: expr, $function_path: expr) => {
        paste::paste! {
            mod [<test_ $function_name _fn>] {
                use aparecium::*;
                use aparecium::functions::time;


                #[test]
                fn [<test_ $function_name _fn_with_number_field>]() {
                    let rebirth_date = aparecium::Field::new("rebirth_date");
                    let result = time::from::[<$function_name>]!(rebirth_date);

                    assert_eq!(result.fine_tune_params(), format!("time::{}(rebirth_date)", $function_path));
                    assert_eq!(result.to_raw().build(), format!("time::{}(rebirth_date)", $function_path));
                }

                #[test]
                fn [<test_ $function_name _fn_with_plain_number>]() {
                    let number = aparecium::Param::new("number");
                    let result = time::from::[<$function_name>]!(number);
                    assert_eq!(result.fine_tune_params(), format!("time::{}($number)", $function_path));
                    assert_eq!(
                        result.to_raw().build(),
                        format!("time::{}($number)", $function_path)

                    );
                }

                #[test]
                fn [<test_ $function_name _fn_with_plain_number_and_duration>]() {
                    let result = time::from::[<$function_name>]!(20000);
                    assert_eq!(result.fine_tune_params(), format!("time::{}($_param_00000001)", $function_path));
                    assert_eq!(
                        result.to_raw().build(),
                        format!("time::{}(20000)", $function_path)

                    );
                }
            }
        }
    };
}

mod from {
    create_time_fn_with_single_number_arg_tests!(
        =>
        "micros",
        "from::micros"
    );
    create_time_fn_with_single_number_arg_tests!(
        =>
        "millis",
        "from::millis"
    );
    create_time_fn_with_single_number_arg_tests!(
        =>
        "secs",
        "from::secs"
    );
    create_time_fn_with_single_number_arg_tests!(
        =>
        "unix",
        "from::unix"
    );
}

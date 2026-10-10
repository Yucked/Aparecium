#[cfg(test)]
mod tests {
    use aparecium::functions::string;
    use aparecium::*;
    #[test]
    fn test_concat_macro() {
        let title = Field::new("title");
        let result = string::concat!(title, "one", 3, 4.15385, "  ", true);
        assert_eq!(
            result.fine_tune_params(),
            "string::concat(title, $_param_00000001, $_param_00000002, $_param_00000003, $_param_00000004, $_param_00000005)"
        );
        assert_eq!(
            result.to_raw().build(),
            "string::concat(title, 'one', 3, 4.15385f, '  ', true)"
        );
    }

    #[test]
    fn test_concat_macro_with_array() {
        let result = string::concat!(arr!["one", "two", 3, 4.15385, "five", true]);
        assert_eq!(
            result.fine_tune_params(),
            "string::concat($_param_00000001, $_param_00000002, $_param_00000003, $_param_00000004, $_param_00000005, $_param_00000006)"
        );
        assert_eq!(
            result.to_raw().build(),
            "string::concat('one', 'two', 3, 4.15385f, 'five', true)"
        );
    }

    #[test]
    fn test_join_macro() {
        let title = Field::new("title");
        let result = string::join!(title, "one", 3, 4.15385, "  ", true);
        assert_eq!(
            result.fine_tune_params(),
            "string::join(title, $_param_00000001, $_param_00000002, $_param_00000003, $_param_00000004, $_param_00000005)"
        );
        assert_eq!(
            result.to_raw().build(),
            "string::join(title, 'one', 3, 4.15385f, '  ', true)"
        );
    }

    #[test]
    fn test_join_macro_with_array() {
        let result = string::join!(arr!["one", "two", 3, 4.15385, "five", true]);
        assert_eq!(
            result.fine_tune_params(),
            "string::join($_param_00000001, $_param_00000002, $_param_00000003, $_param_00000004, $_param_00000005, $_param_00000006)"
        );
        assert_eq!(
            result.to_raw().build(),
            "string::join('one', 'two', 3, 4.15385f, 'five', true)"
        );
    }

    #[test]
    fn test_ends_with_macro_with_field_and_string() {
        let name = Field::new("name");
        let result = string::ends_with!(name, "lowo");
        assert_eq!(
            result.fine_tune_params(),
            "string::ends_with(name, $_param_00000001)"
        );
        assert_eq!(result.to_raw().build(), "string::ends_with(name, 'lowo')");
    }

    #[test]
    fn test_ends_with_macro_with_plain_strings() {
        let result = string::ends_with!("Oyelowo", "lowo");
        assert_eq!(
            result.fine_tune_params(),
            "string::ends_with($_param_00000001, $_param_00000002)"
        );
        assert_eq!(
            result.to_raw().build(),
            "string::ends_with('Oyelowo', 'lowo')"
        );
    }

    #[test]
    fn test_starts_with_macro_with_field_and_string() {
        let name = Field::new("name");
        let result = string::starts_with!(name, "lowo");
        assert_eq!(
            result.fine_tune_params(),
            "string::starts_with(name, $_param_00000001)"
        );
        assert_eq!(result.to_raw().build(), "string::starts_with(name, 'lowo')");
    }

    #[test]
    fn test_starts_with_macro_with_plain_strings() {
        let result = string::starts_with!("Oyelowo", "Oye");
        assert_eq!(
            result.fine_tune_params(),
            "string::starts_with($_param_00000001, $_param_00000002)"
        );
        assert_eq!(
            result.to_raw().build(),
            "string::starts_with('Oyelowo', 'Oye')"
        );
    }
    #[test]
    fn test_split_macro_with_field_and_string() {
        let phrase = Field::new("phrase");
        let result = string::split!(phrase, ", ");
        assert_eq!(
            result.fine_tune_params(),
            "string::split(phrase, $_param_00000001)"
        );
        assert_eq!(result.to_raw().build(), "string::split(phrase, ', ')");
    }

    #[test]
    fn test_split_macro_with_plain_strings() {
        let result = string::split!("With great power, comes great responsibility", ", ");
        assert_eq!(
            result.fine_tune_params(),
            "string::split($_param_00000001, $_param_00000002)"
        );
        assert_eq!(
            result.to_raw().build(),
            "string::split('With great power, comes great responsibility', ', ')"
        );
    }

    #[test]
    fn test_length_with_macro_with_field() {
        let name = Field::new("name");
        let result = string::len!(name);
        assert_eq!(result.fine_tune_params(), "string::length(name)");
        assert_eq!(result.to_raw().build(), "string::length(name)");
    }

    #[test]
    fn test_length_with_macro_with_plain_string() {
        let result = string::len!("toronto");
        assert_eq!(
            result.fine_tune_params(),
            "string::length($_param_00000001)"
        );
        assert_eq!(result.to_raw().build(), "string::length('toronto')");
    }

    #[test]
    fn test_reverse_with_macro_with_field() {
        let name = Field::new("name");
        let result = string::reverse!(name);
        assert_eq!(result.fine_tune_params(), "string::reverse(name)");
        assert_eq!(result.to_raw().build(), "string::reverse(name)");
    }

    #[test]
    fn test_reverse_with_macro_with_plain_string() {
        let result = string::reverse!("oyelowo");
        assert_eq!(
            result.fine_tune_params(),
            "string::reverse($_param_00000001)"
        );
        assert_eq!(result.to_raw().build(), "string::reverse('oyelowo')");
    }

    #[test]
    fn test_trim_with_macro_with_field() {
        let name = Field::new("name");
        let result = string::trim!(name);
        assert_eq!(result.fine_tune_params(), "string::trim(name)");
        assert_eq!(result.to_raw().build(), "string::trim(name)");
    }

    #[test]
    fn test_trim_with_macro_with_plain_string() {
        let result = string::trim!("oyelowo");
        assert_eq!(result.fine_tune_params(), "string::trim($_param_00000001)");
        assert_eq!(result.to_raw().build(), "string::trim('oyelowo')");
    }

    #[test]
    fn test_slug_with_macro_with_field() {
        let name = Field::new("name");
        let result = string::slug!(name);
        assert_eq!(result.fine_tune_params(), "string::slug(name)");
        assert_eq!(result.to_raw().build(), "string::slug(name)");
    }

    #[test]
    fn test_slug_with_macro_with_plain_string() {
        let result = string::slug!("Codebreather is from #Jupiter");
        assert_eq!(result.fine_tune_params(), "string::slug($_param_00000001)");
        assert_eq!(
            result.to_raw().build(),
            "string::slug('Codebreather is from #Jupiter')"
        );
    }

    #[test]
    fn test_lowercase_with_macro_with_field() {
        let name = Field::new("name");
        let result = string::lowercase!(name);
        assert_eq!(result.fine_tune_params(), "string::lowercase(name)");
        assert_eq!(result.to_raw().build(), "string::lowercase(name)");
    }

    #[test]
    fn test_lowercase_with_macro_with_plain_string() {
        let result = string::lowercase!("OYELOWO");
        assert_eq!(
            result.fine_tune_params(),
            "string::lowercase($_param_00000001)"
        );
        assert_eq!(result.to_raw().build(), "string::lowercase('OYELOWO')");
    }

    #[test]
    fn test_uppercase_with_macro_with_field() {
        let name = Field::new("name");
        let result = string::uppercase!(name);
        assert_eq!(result.fine_tune_params(), "string::uppercase(name)");
        assert_eq!(result.to_raw().build(), "string::uppercase(name)");
    }

    #[test]
    fn test_uppercase_with_macro_with_plain_string() {
        let result = string::uppercase!("oyelowo");
        assert_eq!(
            result.fine_tune_params(),
            "string::uppercase($_param_00000001)"
        );
        assert_eq!(result.to_raw().build(), "string::uppercase('oyelowo')");
    }

    #[test]
    fn test_words_with_macro_with_field() {
        let sentence = Field::new("sentence");
        let result = string::words!(sentence);
        assert_eq!(result.fine_tune_params(), "string::words(sentence)");
        assert_eq!(result.to_raw().build(), "string::words(sentence)");
    }

    #[test]
    fn test_words_with_macro_with_plain_string() {
        let result = string::words!(
            "This is the first day of the rest of my life. I will make every single moment count!"
        );
        assert_eq!(result.fine_tune_params(), "string::words($_param_00000001)");
        assert_eq!(
            result.to_raw().build(),
            "string::words('This is the first day of the rest of my life. I will make every single moment count!')"
        );
    }

    #[test]
    fn test_repeat_with_macro_with_fields() {
        let name = Field::new("name");
        let count = Field::new("count");
        let result = string::repeat!(name, count);
        assert_eq!(result.fine_tune_params(), "string::repeat(name, count)");
        assert_eq!(result.to_raw().build(), "string::repeat(name, count)");
    }

    #[test]
    fn test_repeat_with_macro_with_plain_string() {
        let result = string::repeat!("Oyelowo", 5);
        assert_eq!(
            result.fine_tune_params(),
            "string::repeat($_param_00000001, $_param_00000002)"
        );
        assert_eq!(result.to_raw().build(), "string::repeat('Oyelowo', 5)");
    }

    #[test]
    fn test_replace_with_macro_with_fields() {
        let name = Field::new("name");
        let last_name = Field::new("last_name");
        let first_name = Field::new("first_name");

        let result = string::replace!(name, last_name, first_name);
        assert_eq!(
            result.fine_tune_params(),
            "string::replace(name, last_name, first_name)"
        );
        assert_eq!(
            result.to_raw().build(),
            "string::replace(name, last_name, first_name)"
        );
    }

    #[test]
    fn test_slice_with_macro_with_plain_string() {
        let result = string::slice!("Oyelowo", 3, 5);
        assert_eq!(
            result.fine_tune_params(),
            "string::slice($_param_00000001, $_param_00000002, $_param_00000003)"
        );
        assert_eq!(result.to_raw().build(), "string::slice('Oyelowo', 3, 5)");
    }

    #[test]
    fn test_slice_with_macro_with_fields() {
        let name = Field::new("name");
        let last_name = Field::new("last_name");
        let first_name = Field::new("first_name");

        let result = string::slice!(name, last_name, first_name);
        assert_eq!(
            result.fine_tune_params(),
            "string::slice(name, last_name, first_name)"
        );
        assert_eq!(
            result.to_raw().build(),
            "string::slice(name, last_name, first_name)"
        );
    }

    #[test]
    fn test_replace_with_macro_with_plain_strings() {
        let result = string::replace!("Oyelowo Oyedayo", "Oyelowo", "Oyedayo");
        assert_eq!(
            result.fine_tune_params(),
            "string::replace($_param_00000001, $_param_00000002, $_param_00000003)"
        );
        assert_eq!(
            result.to_raw().build(),
            "string::replace('Oyelowo Oyedayo', 'Oyelowo', 'Oyedayo')"
        );
    }
}

macro_rules! create_validation_with_tests_tests {
    ($(#[$attr:meta])* => $function_name: expr) => {
        paste::paste! {
            mod [<test_ $function_name>] {
                use aparecium::*;
                use aparecium::functions::string::is;

                #[test]
                fn [<test_ $function_name _with_field>] ()  {
                    let username = Field::new("username");
                    let result = is::[<$function_name _fn>](username);

                    assert_eq!(result.fine_tune_params(), format!("string::is::{}(username)", $function_name));
                    assert_eq!(result.to_raw().to_string(), format!("string::is::{}(username)", $function_name));
                    }

                #[test]
                fn [<test_ $function_name _string_username>] ()  {
                    let result = is::[<$function_name _fn>]("oyelowo1234");

                    assert_eq!(result.fine_tune_params(), format!("string::is::{}($_param_00000001)", $function_name));
                    assert_eq!(result.to_raw().to_string(), format!("string::is::{}('oyelowo1234')", $function_name));
                }

                #[test]
                fn [<test_ $function_name _with_number>] ()  {
                    let result = is::[<$function_name _fn>](123456423);

                    assert_eq!(result.fine_tune_params(), format!("string::is::{}($_param_00000001)", $function_name));
                    assert_eq!(result.to_raw().to_string(), format!("string::is::{}(123456423)", $function_name));
                }

                #[test]
                fn [<test_ $function_name _with_fraction>] ()  {
                    let result = is::[<$function_name _fn>](12.3456423);

                    assert_eq!(result.fine_tune_params(), format!("string::is::{}($_param_00000001)", $function_name));
                    assert_eq!(result.to_raw().to_string(), format!("string::is::{}(12.3456423f)", $function_name));
                }

                // Macro versions
                #[test]
                fn [<test_ $function_name _macro_with_field>] ()  {
                    let username = Field::new("username");
                    let result = is::[<$function_name>]!(username);

                    assert_eq!(result.fine_tune_params(), format!("string::is::{}(username)", $function_name));
                    assert_eq!(result.to_raw().to_string(), format!("string::is::{}(username)", $function_name));
                }

                #[test]
                fn [<test_ $function_name _macro_string_username>] ()  {
                    let result = is::[<$function_name>]!("oyelowo1234");

                    assert_eq!(result.fine_tune_params(), format!("string::is::{}($_param_00000001)", $function_name));
                    assert_eq!(result.to_raw().to_string(), format!("string::is::{}('oyelowo1234')", $function_name));
                }

                #[test]
                fn [<test_ $function_name _macro_with_number>] ()  {
                    let result = is::[<$function_name>]!(123456423);

                    assert_eq!(result.fine_tune_params(), format!("string::is::{}($_param_00000001)", $function_name));
                    assert_eq!(result.to_raw().to_string(), format!("string::is::{}(123456423)", $function_name));
                }

                #[test]
                fn [<test_ $function_name _macro_with_fraction>] ()  {
                    let result = is::[<$function_name>]!(12.3456423);

                    assert_eq!(result.fine_tune_params(), format!("string::is::{}($_param_00000001)", $function_name));
                    assert_eq!(result.to_raw().to_string(), format!("string::is::{}(12.3456423f)", $function_name));
                }
            }
        }
    };
}

mod is {
    create_validation_with_tests_tests!(
        =>
        "alphanum"
    );
    create_validation_with_tests_tests!(
        =>
        "alpha"
    );
    create_validation_with_tests_tests!(
        =>
        "ascii"
    );
    create_validation_with_tests_tests!(
        =>
        "domain"
    );
    create_validation_with_tests_tests!(
        =>
        "email"
    );
    create_validation_with_tests_tests!(
            =>
            "hexadecimal"
    );
    create_validation_with_tests_tests!(
        =>
        "latitude"
    );
    create_validation_with_tests_tests!(
        =>
        "longitude"
    );
    create_validation_with_tests_tests!(
        =>
        "numeric"
    );
    create_validation_with_tests_tests!(
        =>
        "semver"
    );
    create_validation_with_tests_tests!(
        =>
        "uuid"
    );
    create_validation_with_tests_tests!(
        =>
        "datetime"
    );
}

macro_rules! create_fn_with_two_strings_args_tests {
    ($(#[$attr:meta])* => $function_name:expr, $function_path:expr) => {
        paste::paste! {
            mod [<test_ $function_name>] {
                 use aparecium::[<string_ $function_name>] as [<$function_name>];
                use aparecium::{functions::string, *};

                #[test]
                fn [<test $function_name fn_on_strand_macro_on_diverse_strands>]() {
                    let name = Field::new("name");
                    let result = functions::string::[<$function_name _fn>](name, "Oyelowo");
                    assert_eq!(
                        result.fine_tune_params(),
                        format!("string::{}(name, $_param_00000001)", $function_path)
                    );
                    assert_eq!(
                        result.to_raw().build(),
                        format!("string::{}(name, 'Oyelowo')", $function_path)
                    );
                }

                #[test]
                fn [<test $function_name _fn_on_same_element_types>]() {
                    let result = string::[<$function_name _fn>]("Oyelowo", "Oyedayo");
                    assert_eq!(
                        result.fine_tune_params(),
                        format!("string::{}($_param_00000001, $_param_00000002)", $function_path)
                    );

                    assert_eq!(
                        result.to_raw().build(),
                        format!("string::{}('Oyelowo', 'Oyedayo')", $function_path)
                    );
                }

                #[test]
                fn [<test $function_name _macro_on_strand_macro_on_diverse_strands>]() {
                    let name = Field::new("name");
                    let result = self::[<$function_name>]!(name, "Oyelowo");
                    assert_eq!(
                        result.fine_tune_params(),
                        format!("string::{}(name, $_param_00000001)", $function_path)
                    );
                    assert_eq!(
                        result.to_raw().build(),
                        format!("string::{}(name, 'Oyelowo')", $function_path)
                    );
                }

                #[test]
                fn [<test $function_name _macro_on_same_element_types>]() {
                    let result = self::[<$function_name>]!("Oyelowo", "Oyedayo");
                    assert_eq!(
                        result.fine_tune_params(),
                        format!("string::{}($_param_00000001, $_param_00000002)", $function_path)
                    );

                    assert_eq!(
                        result.to_raw().build(),
                        format!("string::{}('Oyelowo', 'Oyedayo')", $function_path)
                    );
                }
            }
        }
    };
}

create_fn_with_two_strings_args_tests!(
    =>
    "is_format",
    "is::format"
);
create_fn_with_two_strings_args_tests!(
    =>
    "distance_hamming",
    "distance::hamming"
);
create_fn_with_two_strings_args_tests!(
    =>
    "distance_levenshtein",
    "distance::levenshtein"
);
create_fn_with_two_strings_args_tests!(
    =>
    "similarity_fuzzy",
    "similarity::fuzzy"
);
create_fn_with_two_strings_args_tests!(
    =>
    "similarity_jaro",
    "similarity::jaro"
);
create_fn_with_two_strings_args_tests!(
    =>
    "similarity_smithwaterman",
    "similarity::smithwaterman"
);

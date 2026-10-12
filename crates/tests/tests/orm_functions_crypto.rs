#[cfg(test)]
mod tests {
    use umbriel::{functions::crypto::*, *};
    use umbriel::{statements::let_, *};

    #[test]
    fn test_argon2_compare() {
        let result = argon2::compare_fn("Charlie", "Bennett");
        assert_eq!(
            result.fine_tune_params(),
            "crypto::argon2::compare($_param_00000001, $_param_00000002)"
        );
        assert_eq!(
            result.to_raw().build(),
            "crypto::argon2::compare('Charlie', 'Bennett')"
        );
    }

    #[test]
    fn test_argon2_compare_with_param() {
        let hash = let_("hash").equal_to("$argon2id$v=19$m=4096,t=3,p=1$pbZ6yJ2rPJKk4pyEMVwslQ$jHzpsiB+3S/H+kwFXEcr10vmOiDkBkydVCSMfRxV7CA");
        let pass = let_("pass").equal_to("the strongest password");

        let result = argon2::compare_fn(hash.get_param(), pass.get_param());
        assert_eq!(
            result.fine_tune_params(),
            "crypto::argon2::compare($hash, $pass)"
        );
        assert_eq!(
            result.to_raw().build(),
            "crypto::argon2::compare($hash, $pass)"
        );
    }

    #[test]
    fn test_argon2_generate() {
        let result = argon2::generate_fn("Charlie");
        assert_eq!(
            result.fine_tune_params(),
            "crypto::argon2::generate($_param_00000001)"
        );
        assert_eq!(
            result.to_raw().build(),
            "crypto::argon2::generate('Charlie')"
        );
    }

    #[test]
    fn test_argon2_compare_macro_with_raw_values() {
        let result = argon2::compare!(
            "$argon2id$v=19$m=4096,t=3,p=1$pbZ6yJ2rPJKk4pyEMVwslQ$jHzpsiB+3S/H+kwFXEcr10vmOiDkBkydVCSMfRxV7CA",
            "the strongest password"
        );
        assert_eq!(
            result.fine_tune_params(),
            "crypto::argon2::compare($_param_00000001, $_param_00000002)"
        );
        assert_eq!(
            result.to_raw().build(),
            "crypto::argon2::compare('$argon2id$v=19$m=4096,t=3,p=1$pbZ6yJ2rPJKk4pyEMVwslQ$jHzpsiB+3S/H+kwFXEcr10vmOiDkBkydVCSMfRxV7CA', 'the strongest password')"
        );
    }

    #[test]
    fn test_argon2_compare_macro_with_fields() {
        let hash = Field::new("hash");
        let pass = Field::new("pass");

        let result = argon2::compare!(hash, pass);
        assert_eq!(
            result.fine_tune_params(),
            "crypto::argon2::compare(hash, pass)"
        );
        assert_eq!(
            result.to_raw().build(),
            "crypto::argon2::compare(hash, pass)"
        );
    }

    #[test]
    fn test_argon2_compare_macro_with_param() {
        let hash = let_("hash").equal_to("$argon2id$v=19$m=4096,t=3,p=1$pbZ6yJ2rPJKk4pyEMVwslQ$jHzpsiB+3S/H+kwFXEcr10vmOiDkBkydVCSMfRxV7CA");
        let pass = let_("pass").equal_to("the strongest password");

        let result = argon2::compare!(hash.get_param(), pass.get_param());
        assert_eq!(
            result.fine_tune_params(),
            "crypto::argon2::compare($hash, $pass)"
        );
        assert_eq!(
            result.to_raw().build(),
            "crypto::argon2::compare($hash, $pass)"
        );
    }

    #[test]
    fn test_argon2_generate_fn() {
        let result = argon2::generate!("Charlie");
        assert_eq!(
            result.fine_tune_params(),
            "crypto::argon2::generate($_param_00000001)"
        );
        assert_eq!(
            result.to_raw().build(),
            "crypto::argon2::generate('Charlie')"
        );
    }

    // pbkdf2
    #[test]
    fn test_pbkdf2_compare_with_param() {
        let hash = let_("hash").equal_to("$argon2id$v=19$m=4096,t=3,p=1$pbZ6yJ2rPJKk4pyEMVwslQ$jHzpsiB+3S/H+kwFXEcr10vmOiDkBkydVCSMfRxV7CA");
        let pass = let_("pass").equal_to("the strongest password");

        let result = pbkdf2::compare_fn(hash.get_param(), pass.get_param());
        assert_eq!(
            result.fine_tune_params(),
            "crypto::pbkdf2::compare($hash, $pass)"
        );
        assert_eq!(
            result.to_raw().build(),
            "crypto::pbkdf2::compare($hash, $pass)"
        );
    }

    #[test]
    fn test_pbkdf2_generate() {
        let result = pbkdf2::generate_fn("Charlie");
        assert_eq!(
            result.fine_tune_params(),
            "crypto::pbkdf2::generate($_param_00000001)"
        );
        assert_eq!(
            result.to_raw().build(),
            "crypto::pbkdf2::generate('Charlie')"
        );
    }

    #[test]
    fn test_pbkdf2_compare_macro_with_raw_values() {
        let result = pbkdf2::compare!(
            "$argon2id$v=19$m=4096,t=3,p=1$pbZ6yJ2rPJKk4pyEMVwslQ$jHzpsiB+3S/H+kwFXEcr10vmOiDkBkydVCSMfRxV7CA",
            "the strongest password"
        );
        assert_eq!(
            result.fine_tune_params(),
            "crypto::pbkdf2::compare($_param_00000001, $_param_00000002)"
        );
        assert_eq!(
            result.to_raw().build(),
            "crypto::pbkdf2::compare('$argon2id$v=19$m=4096,t=3,p=1$pbZ6yJ2rPJKk4pyEMVwslQ$jHzpsiB+3S/H+kwFXEcr10vmOiDkBkydVCSMfRxV7CA', 'the strongest password')"
        );
    }

    #[test]
    fn test_pbkdf2_compare_macro_with_fields() {
        let hash = Field::new("hash");
        let pass = Field::new("pass");

        let result = pbkdf2::compare!(hash, pass);
        assert_eq!(
            result.fine_tune_params(),
            "crypto::pbkdf2::compare(hash, pass)"
        );
        assert_eq!(
            result.to_raw().build(),
            "crypto::pbkdf2::compare(hash, pass)"
        );
    }

    #[test]
    fn test_pbkdf2_compare_macro_with_param() {
        let hash = let_("hash").equal_to("$argon2id$v=19$m=4096,t=3,p=1$pbZ6yJ2rPJKk4pyEMVwslQ$jHzpsiB+3S/H+kwFXEcr10vmOiDkBkydVCSMfRxV7CA");
        let pass = let_("pass").equal_to("the strongest password");

        let result = pbkdf2::compare!(hash.get_param(), pass.get_param());
        assert_eq!(
            result.fine_tune_params(),
            "crypto::pbkdf2::compare($hash, $pass)"
        );
        assert_eq!(
            result.to_raw().build(),
            "crypto::pbkdf2::compare($hash, $pass)"
        );
    }

    #[test]
    fn test_pbkdf2_generate_fn() {
        let result = pbkdf2::generate!("Charlie");
        assert_eq!(
            result.fine_tune_params(),
            "crypto::pbkdf2::generate($_param_00000001)"
        );
        assert_eq!(
            result.to_raw().build(),
            "crypto::pbkdf2::generate('Charlie')"
        );
    }

    #[test]
    fn test_scrypt_compare_with_param() {
        let hash = let_("hash").equal_to("$argon2id$v=19$m=4096,t=3,p=1$pbZ6yJ2rPJKk4pyEMVwslQ$jHzpsiB+3S/H+kwFXEcr10vmOiDkBkydVCSMfRxV7CA");
        let pass = let_("pass").equal_to("the strongest password");

        let result = scrypt::compare_fn(hash.get_param(), pass.get_param());
        assert_eq!(
            result.fine_tune_params(),
            "crypto::scrypt::compare($hash, $pass)"
        );
        assert_eq!(
            result.to_raw().build(),
            "crypto::scrypt::compare($hash, $pass)"
        );
    }

    #[test]
    fn test_scrypt_generate() {
        let result = scrypt::generate_fn("Charlie");
        assert_eq!(
            result.fine_tune_params(),
            "crypto::scrypt::generate($_param_00000001)"
        );
        assert_eq!(
            result.to_raw().build(),
            "crypto::scrypt::generate('Charlie')"
        );
    }

    #[test]
    fn test_scrypt_compare_macro_with_raw_values() {
        let result = scrypt::compare!(
            "$argon2id$v=19$m=4096,t=3,p=1$pbZ6yJ2rPJKk4pyEMVwslQ$jHzpsiB+3S/H+kwFXEcr10vmOiDkBkydVCSMfRxV7CA",
            "the strongest password"
        );
        assert_eq!(
            result.fine_tune_params(),
            "crypto::scrypt::compare($_param_00000001, $_param_00000002)"
        );
        assert_eq!(
            result.to_raw().build(),
            "crypto::scrypt::compare('$argon2id$v=19$m=4096,t=3,p=1$pbZ6yJ2rPJKk4pyEMVwslQ$jHzpsiB+3S/H+kwFXEcr10vmOiDkBkydVCSMfRxV7CA', 'the strongest password')"
        );
    }

    #[test]
    fn test_scrypt_compare_macro_with_fields() {
        let hash = Field::new("hash");
        let pass = Field::new("pass");

        let result = scrypt::compare!(hash, pass);
        assert_eq!(
            result.fine_tune_params(),
            "crypto::scrypt::compare(hash, pass)"
        );
        assert_eq!(
            result.to_raw().build(),
            "crypto::scrypt::compare(hash, pass)"
        );
    }

    #[test]
    fn test_scrypt_compare_macro_with_param() {
        let hash = let_("hash").equal_to("$argon2id$v=19$m=4096,t=3,p=1$pbZ6yJ2rPJKk4pyEMVwslQ$jHzpsiB+3S/H+kwFXEcr10vmOiDkBkydVCSMfRxV7CA");
        let pass = let_("pass").equal_to("the strongest password");

        let result = scrypt::compare!(hash.get_param(), pass.get_param());
        assert_eq!(
            result.fine_tune_params(),
            "crypto::scrypt::compare($hash, $pass)"
        );
        assert_eq!(
            result.to_raw().build(),
            "crypto::scrypt::compare($hash, $pass)"
        );
    }

    #[test]
    fn test_scrypt_generate_fn() {
        let result = scrypt::generate!("Charlie");
        assert_eq!(
            result.fine_tune_params(),
            "crypto::scrypt::generate($_param_00000001)"
        );
        assert_eq!(
            result.to_raw().build(),
            "crypto::scrypt::generate('Charlie')"
        );
    }

    // bcrypt
    // Create all tests for bcrypt
    #[test]
    fn test_bcrypt_compare_with_param() {
        let hash = let_("hash").equal_to("$argon2id$v=19$m=4096,t=3,p=1$pbZ6yJ2rPJKk4pyEMVwslQ$jHzpsiB+3S/H+kwFXEcr10vmOiDkBkydVCSMfRxV7CA");
        let pass = let_("pass").equal_to("the strongest password");

        let result = bcrypt::compare_fn(hash.get_param(), pass.get_param());
        assert_eq!(
            result.fine_tune_params(),
            "crypto::bcrypt::compare($hash, $pass)"
        );
        assert_eq!(
            result.to_raw().build(),
            "crypto::bcrypt::compare($hash, $pass)"
        );
    }

    #[test]
    fn test_bcrypt_generate() {
        let result = bcrypt::generate_fn("Charlie");
        assert_eq!(
            result.fine_tune_params(),
            "crypto::bcrypt::generate($_param_00000001)"
        );
        assert_eq!(
            result.to_raw().build(),
            "crypto::bcrypt::generate('Charlie')"
        );
    }

    #[test]
    fn test_bcrypt_compare_macro_with_raw_values() {
        let result = bcrypt::compare!(
            "$argon2id$v=19$m=4096,t=3,p=1$pbZ6yJ2rPJKk4pyEMVwslQ$jHzpsiB+3S/H+kwFXEcr10vmOiDkBkydVCSMfRxV7CA",
            "the strongest password"
        );
        assert_eq!(
            result.fine_tune_params(),
            "crypto::bcrypt::compare($_param_00000001, $_param_00000002)"
        );
        assert_eq!(
            result.to_raw().build(),
            "crypto::bcrypt::compare('$argon2id$v=19$m=4096,t=3,p=1$pbZ6yJ2rPJKk4pyEMVwslQ$jHzpsiB+3S/H+kwFXEcr10vmOiDkBkydVCSMfRxV7CA', 'the strongest password')"
        );
    }

    #[test]
    fn test_bcrypt_compare_macro_with_fields() {
        let hash = Field::new("hash");
        let pass = Field::new("pass");

        let result = bcrypt::compare!(hash, pass);
        assert_eq!(
            result.fine_tune_params(),
            "crypto::bcrypt::compare(hash, pass)"
        );
        assert_eq!(
            result.to_raw().build(),
            "crypto::bcrypt::compare(hash, pass)"
        );
    }

    #[test]
    fn test_bcrypt_compare_macro_with_param() {
        let hash = let_("hash").equal_to("$argon2id$v=19$m=4096,t=3,p=1$pbZ6yJ2rPJKk4pyEMVwslQ$jHzpsiB+3S/H+kwFXEcr10vmOiDkBkydVCSMfRxV7CA");
        let pass = let_("pass").equal_to("the strongest password");

        let result = bcrypt::compare!(hash.get_param(), pass.get_param());
        assert_eq!(
            result.fine_tune_params(),
            "crypto::bcrypt::compare($hash, $pass)"
        );
        assert_eq!(
            result.to_raw().build(),
            "crypto::bcrypt::compare($hash, $pass)"
        );
    }

    #[test]
    fn test_bcrypt_generate_fn() {
        let result = bcrypt::generate!("Charlie");
        assert_eq!(
            result.fine_tune_params(),
            "crypto::bcrypt::generate($_param_00000001)"
        );
        assert_eq!(
            result.to_raw().build(),
            "crypto::bcrypt::generate('Charlie')"
        );
    }
}

macro_rules! create_fn_with_single_arg_value_tests {
    ($(#[$attr:meta])* => $function_name: expr) => {
        paste::paste! {
            mod [<test_ $function_name>] {
                use umbriel::functions::crypto::*;
                use umbriel::*;

                #[test]
                fn [<test_ $function_name _with_str>]() {
                    let result = [<$function_name _fn>]("Charlie");
                    assert_eq!(result.fine_tune_params(), format!("crypto::{}($_param_00000001)", $function_name));
                    assert_eq!(result.to_raw().build(), format!("crypto::{}('Charlie')", $function_name));
                }

                #[test]
                fn [<test_ $function_name _with_macro>]() {
                    let result = [<$function_name>]!("Charlie");
                    assert_eq!(result.fine_tune_params(), format!("crypto::{}($_param_00000001)", $function_name));
                    assert_eq!(result.to_raw().build(), format!("crypto::{}('Charlie')", $function_name));
                }

                #[test]
                fn [<test_ $function_name _with_field>]() {
                    let title = umbriel::Field::new("title");
                    let result = [<$function_name _fn>](title);
                    assert_eq!(result.fine_tune_params(), format!("crypto::{}(title)", $function_name));
                    assert_eq!(result.to_raw().build(), format!("crypto::{}(title)", $function_name));
                }

                #[test]
                fn [<test_ $function_name _macro_with_field>]() {
                    let title = umbriel::Field::new("title");
                    let result = [<$function_name>]!(title);
                    assert_eq!(result.fine_tune_params(), format!("crypto::{}(title)", $function_name));
                    assert_eq!(result.to_raw().build(), format!("crypto::{}(title)", $function_name));
                }
            }
        }
    };
}

create_fn_with_single_arg_value_tests!(
    => "md5"
);
create_fn_with_single_arg_value_tests!(
    => "sha1"
);
create_fn_with_single_arg_value_tests!(
    => "sha256"
);
create_fn_with_single_arg_value_tests!(
    => "sha512"
);

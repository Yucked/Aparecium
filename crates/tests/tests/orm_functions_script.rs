#[cfg(test)]
mod tests {
    use aparecium::{ToRaw, statements::let_};

    use aparecium::{functions::script::*, *};

    #[test]
    fn test_function_without_args() {
        let f2 = function!((), {
            return [1, 2, 3].map(v => v * 10);
        });
        assert_eq!(
            f2.build(),
            "function() { return [1, 2, 3].map(v => v * 10); }"
        );
        assert_eq!(
            f2.to_raw().build(),
            "function() { return [1, 2, 3].map(v => v * 10); }"
        );
    }

    #[test]
    fn test_function_with_params_with_variable_interpolation_in_the_function() {
        let value = let_("value").equal_to("SurrealDB").get_param();
        let words = let_("words")
            .equal_to(vec!["awesome", "advanced", "cool"])
            .get_param();

        let f2 = function!(
            (value, words),
            "{ return `${arguments[0]} is ${arguments[1]}`; }"
        );

        assert_eq!(
            f2.build(),
            "function($value, $words) { return `${arguments[0]} is ${arguments[1]}`; }"
        );

        assert_eq!(
            f2.to_raw().build(),
            "function($value, $words) { return `${arguments[0]} is ${arguments[1]}`; }"
        );
    }

    #[test]
    fn test_function_with_params_without_variable_interpolation_in_the_function() {
        let value = let_("value").equal_to("SurrealDB").get_param();
        let words = let_("words")
            .equal_to(vec!["awesome", "advanced", "cool"])
            .get_param();

        let f2 = function!((value, words), {
            return arguments[0] + " is " + arguments[1];
        });

        assert_eq!(
            f2.build(),
            "function($value, $words) { return arguments[0] + \" is \" + arguments[1]; }"
        );

        assert_eq!(
            f2.to_raw().build(),
            "function($value, $words) { return arguments[0] + \" is \" + arguments[1]; }"
        );
    }

    #[test]
    fn test_function_with_args() {
        let name = Param::new("name");
        let id = Param::new("id");

        let f2 = function!((name, id), {
            return [1, 2, 3].map(v => v * 10);
        });
        assert_eq!(
            f2.build(),
            "function($name, $id) { return [1, 2, 3].map(v => v * 10); }"
        );
        assert_eq!(
            f2.to_raw().build(),
            "function($name, $id) { return [1, 2, 3].map(v => v * 10); }"
        );
    }

    #[test]
    fn test_function_with_args_code_str() {
        let name = Param::new("name");
        let id = Param::new("id");

        let f2 = function!(
            (name, id),
            "{ return [1, 2, 3].map(v => v * 10 * $name * $id) ; }"
        );
        insta::assert_snapshot!(f2);
        insta::assert_snapshot!(f2.to_raw());
    }
}

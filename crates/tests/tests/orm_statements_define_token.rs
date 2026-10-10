#[cfg(test)]
mod tests {
    use aparecium::ToRaw;

    use aparecium::{statements::*, *};

    #[test]
    fn test_define_token_statement_on_namespace() {
        let statement = define_token("oyelowo_token")
            .on_namespace()
            .type_(TokenType::PS512)
            .value("abrakradabra");

        assert_eq!(
            statement.fine_tune_params(),
            "DEFINE TOKEN oyelowo_token ON NAMESPACE TYPE PS512 VALUE $_param_00000001;"
        );

        assert_eq!(
            statement.to_raw().build(),
            "DEFINE TOKEN oyelowo_token ON NAMESPACE TYPE PS512 VALUE 'abrakradabra';"
        );

        assert_eq!(statement.get_bindings().len(), 1);
    }

    #[test]
    fn test_define_token_statement_on_database() {
        let statement = define_token("oyelowo_token")
            .on_database()
            .type_(TokenType::HS512)
            .value("anaksunamun");

        assert_eq!(
            statement.fine_tune_params(),
            "DEFINE TOKEN oyelowo_token ON DATABASE TYPE HS512 VALUE $_param_00000001;"
        );

        assert_eq!(
            statement.to_raw().build(),
            "DEFINE TOKEN oyelowo_token ON DATABASE TYPE HS512 VALUE 'anaksunamun';"
        );

        assert_eq!(statement.get_bindings().len(), 1);
    }

    #[test]
    fn test_define_token_statement_on_scope() {
        let statement = define_token("oyelowo_token")
            .on_scope("planet")
            .type_(TokenType::EDDSA)
            .value("abcde");

        assert_eq!(
            statement.fine_tune_params(),
            "DEFINE TOKEN oyelowo_token ON SCOPE planet TYPE EDDSA VALUE $_param_00000001;"
        );

        assert_eq!(
            statement.to_raw().build(),
            "DEFINE TOKEN oyelowo_token ON SCOPE planet TYPE EDDSA VALUE 'abcde';"
        );

        assert_eq!(statement.get_bindings().len(), 1);
    }
}

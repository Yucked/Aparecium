#[cfg(test)]
mod tests {
    use aparecium::{
        functions::crypto,
        statements::{define_scope, select},
        *,
    };
    use std::time::Duration;

    #[test]
    fn test_define_scope_statement_on_namespace() {
        let user = Table::new("user");
        let email = Field::new("email");
        let pass = Field::new("pass");
        let pass_param = Param::new("pass_param");

        let token_def = define_scope("oyelowo_scope")
            .session(Duration::from_secs(45))
            .signup(Raw::new(
                "CREATE user SET email = $email, pass = crypto::argon2::generate($pass)",
            ))
            .signin(
                select(All).from(user).where_(
                    cond(email.equal("oyelowo@codebreather.com"))
                        .and(crypto::argon2::compare!(pass, pass_param)),
                ),
            );

        assert_eq!(
            token_def.fine_tune_params(),
            "DEFINE SCOPE $_param_00000001 SESSION $_param_00000002 \
                \n\tSIGNUP $_param_00000003 \
                \n\tSIGNIN $_param_00000004;"
        );
        assert_eq!(
            token_def.to_raw().build(),
            "DEFINE SCOPE oyelowo_scope SESSION 45s \
                \n\tSIGNUP (CREATE user SET email = $email, pass = crypto::argon2::generate($pass)) \
                \n\tSIGNIN (SELECT * FROM user WHERE (email = 'oyelowo@codebreather.com') AND (crypto::argon2::compare(pass, $pass_param)));"
        );

        assert_eq!(token_def.get_bindings().len(), 4);
    }
}

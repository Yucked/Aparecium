#[cfg(test)]
mod tests {
    use aparecium::*;

    #[test]
    fn test_token_type_display() {
        assert_eq!(format!("{}", TokenType::EDDSA), "EDDSA");
        assert_eq!(format!("{}", TokenType::ES256), "ES256");
        assert_eq!(format!("{}", TokenType::ES384), "ES384");
        assert_eq!(format!("{}", TokenType::ES512), "ES512");
        assert_eq!(format!("{}", TokenType::HS256), "HS256");
        assert_eq!(format!("{}", TokenType::HS384), "HS384");
        assert_eq!(format!("{}", TokenType::HS512), "HS512");
        assert_eq!(format!("{}", TokenType::PS256), "PS256");
        assert_eq!(format!("{}", TokenType::PS384), "PS384");
        assert_eq!(format!("{}", TokenType::PS512), "PS512");
        assert_eq!(format!("{}", TokenType::RS256), "RS256");
        assert_eq!(format!("{}", TokenType::RS384), "RS384");
        assert_eq!(format!("{}", TokenType::RS512), "RS512");
    }

    #[test]
    fn test_token_target_display() {
        assert_eq!(format!("{}", TokenTarget::Namespace), "NAMESPACE");
        assert_eq!(format!("{}", TokenTarget::Database), "DATABASE");
        assert_eq!(
            format!("{}", TokenTarget::Scope("test".into())),
            "SCOPE test"
        );
    }
}

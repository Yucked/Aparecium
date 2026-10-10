#[cfg(test)]
mod tests {
    use aparecium::*;
    use aparecium::*;

    #[test]
    fn test_return_type() {
        let return_type = ReturnType::None;
        assert_eq!(return_type.to_string(), "RETURN NONE ");

        let return_type = ReturnType::Before;
        assert_eq!(return_type.to_string(), "RETURN BEFORE ");

        let return_type = ReturnType::After;
        assert_eq!(return_type.to_string(), "RETURN AFTER ");

        let return_type = ReturnType::Diff;
        assert_eq!(return_type.to_string(), "RETURN DIFF ");

        let id = Field::new("id");
        let name = Field::new("name");
        let return_type = ReturnType::Projections(vec![id, name].into());
        assert_eq!(return_type.to_string(), "RETURN id, name ");
    }
}

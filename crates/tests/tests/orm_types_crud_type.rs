#[cfg(test)]
mod tests {
    use aparecium::*;

    #[test]
    fn crud_type_display() {
        assert_eq!(CrudType::Create.to_string(), "create");
        assert_eq!(CrudType::Select.to_string(), "select");
        assert_eq!(CrudType::Update.to_string(), "update");
        assert_eq!(CrudType::Delete.to_string(), "delete");
    }
}

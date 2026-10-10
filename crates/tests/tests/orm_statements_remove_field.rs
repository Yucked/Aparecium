#[cfg(test)]
mod tests {
    use aparecium::{Field, Table};
    use aparecium::{statements::*, *};

    #[test]
    fn test_remove_field() {
        let user = Table::new("user");
        let name = Field::new("name");

        let statement = remove_field(name).on_table(user);
        assert_eq!(statement.build(), "REMOVE FIELD name ON TABLE user;");
    }
}

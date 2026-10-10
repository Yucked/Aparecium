#[cfg(test)]
mod tests {
    use aparecium::TableIndex;
    use aparecium::{statements::*, *};

    #[test]
    fn test_remove_index() {
        let user = Table::new("user");
        let party = TableIndex::new("party");

        let statement = remove_index(party).on_table(user);
        assert_eq!(statement.build(), "REMOVE INDEX party ON TABLE user;");
    }
}

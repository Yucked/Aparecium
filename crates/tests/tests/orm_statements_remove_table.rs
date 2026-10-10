#[cfg(test)]
mod tests {
    use aparecium::{statements::*, *};

    #[test]
    fn test_remove_table() {
        let table = Table::new("table");
        let statement = remove_table(table);
        assert_eq!(statement.build(), "REMOVE TABLE table;");
    }
}

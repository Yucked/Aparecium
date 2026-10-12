#[cfg(test)]
mod tests {
    use umbriel::{statements::*, *};

    #[test]
    fn test_remove_event() {
        let user = Table::new("user");
        let party = Event::new("party");

        let statement = remove_event(party).on_table(user);
        assert_eq!(statement.build(), "REMOVE EVENT party ON TABLE user;");
    }
}

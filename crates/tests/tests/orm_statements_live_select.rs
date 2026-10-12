#[cfg(test)]
mod tests {
    use umbriel::{All, Buildable, Table, statements::select};
    use umbriel::{statements::*, *};

    #[test]
    fn test_live_select_build() {
        let person = Table::new("person");
        let statement = live(select(All).from(person)).build();

        assert_eq!(statement, "LIVE SELECT * FROM person;");
    }
}

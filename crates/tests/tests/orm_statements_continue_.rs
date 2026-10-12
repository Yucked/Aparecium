#[cfg(test)]
mod tests {
    use umbriel::Buildable;
    use umbriel::{statements::*, *};

    #[test]
    fn test_continue_stmt_build() {
        let statement = continue_().build();
        assert_eq!(statement, "CONTINUE;");
    }
}

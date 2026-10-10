#[cfg(test)]
mod tests {
    use aparecium::Buildable;
    use aparecium::{statements::*, *};

    #[test]
    fn test_continue_stmt_build() {
        let statement = continue_().build();
        assert_eq!(statement, "CONTINUE;");
    }
}

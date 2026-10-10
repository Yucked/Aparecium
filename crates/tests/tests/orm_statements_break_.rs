#[cfg(test)]
mod tests {
    use aparecium::Buildable;
    use aparecium::{statements::*, *};

    #[test]
    fn test_break_stmt_build() {
        let statement = break_().build();
        assert_eq!(statement, "BREAK;");
    }
}

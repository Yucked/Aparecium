#[cfg(test)]
mod tests {
    use aparecium::{Buildable, ToRaw};
    use aparecium::{statements::*, *};

    #[test]
    fn test_throw_stmt_build() {
        let statement = throw("some error message");
        assert_eq!(statement.fine_tune_params(), "THROW $_param_00000001;");
        assert_eq!(statement.to_raw().build(), "THROW 'some error message';");
    }
}

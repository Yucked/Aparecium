#[cfg(test)]
mod tests {
    use std::time::Duration;

    use umbriel::ToRaw;

    use umbriel::{statements::*, *};

    #[test]
    fn test_sleep_statement() {
        let statement = sleep(Duration::from_secs(43));
        assert_eq!(statement.fine_tune_params(), "SLEEP $_param_00000001;");
        assert_eq!(statement.to_raw().build(), "SLEEP 43s;");
        assert_eq!(statement.get_bindings().len(), 1);
    }
}

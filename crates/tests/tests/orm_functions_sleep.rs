#[cfg(test)]
mod tests {
    use std::time;

    use umbriel::*;
    use umbriel::{functions::sleep::*, *};

    #[test]
    fn test_sleep_fn_with_field_data() {
        let waiting_time = Field::new("waiting_time");
        let result = sleep_fn(waiting_time);

        assert_eq!(result.fine_tune_params(), "sleep(waiting_time)");
        assert_eq!(result.to_raw().to_string(), "sleep(waiting_time)");
    }

    #[test]
    fn test_sleep_fn() {
        let result = sleep_fn(time::Duration::from_secs(55));
        assert_eq!(result.fine_tune_params(), "sleep($_param_00000001)");
        assert_eq!(result.to_raw().to_string(), "sleep(55s)");
    }

    #[test]
    fn test_sleep_fn_over_long_period() {
        let result = sleep_fn(time::Duration::from_secs(55340223));
        assert_eq!(result.fine_tune_params(), "sleep($_param_00000001)");
        assert_eq!(result.to_raw().to_string(), "sleep(1y39w2d12h17m3s)");
    }

    // macro versions
    #[test]
    fn test_sleep_macro_with_field_data() {
        let waiting_time = Field::new("waiting_time");
        let result = sleep!(waiting_time);

        assert_eq!(result.fine_tune_params(), "sleep(waiting_time)");
        assert_eq!(result.to_raw().to_string(), "sleep(waiting_time)");
    }

    #[test]
    fn test_sleep_macro() {
        let result = sleep!(time::Duration::from_secs(55));
        assert_eq!(result.fine_tune_params(), "sleep($_param_00000001)");
        assert_eq!(result.to_raw().to_string(), "sleep(55s)");
    }

    #[test]
    fn test_sleep_macro_over_long_period() {
        let result = sleep!(time::Duration::from_secs(55340223));
        assert_eq!(result.fine_tune_params(), "sleep($_param_00000001)");
        assert_eq!(result.to_raw().to_string(), "sleep(1y39w2d12h17m3s)");
    }
}

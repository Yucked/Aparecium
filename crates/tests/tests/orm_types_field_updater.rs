#[cfg(test)]
mod tests {
    use umbriel::ToRaw;

    use umbriel::*;

    #[test]
    fn test_increment_by() {
        let score = Field::new("score");
        let updated_updater = updater(score).increment_by(5);
        assert_eq!(
            updated_updater.fine_tune_params(),
            "score += $_param_00000001"
        );
        assert_eq!(updated_updater.to_raw().build(), "score += 5");
    }

    #[test]
    fn test_append() {
        let names = Field::new("names");
        let updated_updater = updater(names).append("Charlie");
        assert_eq!(
            updated_updater.fine_tune_params(),
            "names += $_param_00000001"
        );
        assert_eq!(updated_updater.to_raw().build(), "names += 'Charlie'");
    }

    #[test]
    fn test_decrement_by() {
        let score = Field::new("score");
        let updated_updater = updater(score).decrement_by(5);
        assert_eq!(
            updated_updater.fine_tune_params(),
            "score -= $_param_00000001"
        );
        assert_eq!(updated_updater.to_raw().build(), "score -= 5");
    }

    #[test]
    fn test_remove() {
        let names = Field::new("names");
        let updated_updater = updater(names).remove("Charlie");
        assert_eq!(
            updated_updater.fine_tune_params(),
            "names -= $_param_00000001"
        );
        assert_eq!(updated_updater.to_raw().build(), "names -= 'Charlie'");
    }

    #[test]
    fn test_plus_equal() {
        let score = Field::new("score");
        let updated_updater = updater(score).plus_equal(10);
        assert_eq!(
            updated_updater.fine_tune_params(),
            "score += $_param_00000001"
        );
    }

    #[test]
    fn test_minus_equal() {
        let names = Field::new("names");
        let updated_updater = updater(names).minus_equal("Charlie");
        assert_eq!(
            updated_updater.fine_tune_params(),
            "names -= $_param_00000001"
        );
        assert_eq!(updated_updater.to_raw().build(), "names -= 'Charlie'");
    }
}

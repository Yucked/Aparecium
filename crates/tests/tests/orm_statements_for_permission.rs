#[cfg(test)]
mod tests {
    use aparecium::{Field, Operatable};
    use aparecium::{statements::*, *};

    #[test]
    fn test_define_for_statement_state_machine() {
        let name = Field::new("name");

        let for_res = for_permission(CrudType::Create).where_(name.like("Oyelowo"));
        assert_eq!(
            for_res.fine_tune_params(),
            "FOR create\n\tWHERE name ~ $_param_00000001"
        );
        assert_eq!(
            for_res.to_raw().build(),
            "FOR create\n\tWHERE name ~ 'Oyelowo'"
        );
    }

    #[test]
    fn test_define_for_statement_state_machine_multiple() {
        use CrudType::*;
        let name = Field::new("name");

        let for_res = for_permission([Create, Delete, Select, Update]).where_(name.is("Oyedayo"));
        assert_eq!(
            for_res.fine_tune_params(),
            "FOR create, delete, select, update\n\tWHERE name IS $_param_00000001"
        );

        assert_eq!(
            for_res.to_raw().build(),
            "FOR create, delete, select, update\n\tWHERE name IS 'Oyedayo'"
        );
    }
}

#[cfg(test)]
mod test {
    use aparecium::{
        Field, Param, ToRaw, cond,
        statements::{let_, select},
    };

    use aparecium::*;

    #[test]
    fn test_display_clause_with_empty() {
        let empty_clause = Clause::from(Empty);
        assert_eq!(empty_clause.build(), "");
        assert_eq!(empty_clause.to_raw().build(), "");
    }

    #[test]
    fn test_display_clause_with_where_filter() {
        let filter = cond(Field::new("age").equal(18));
        let where_clause = Clause::from(filter);
        assert_eq!(
            where_clause.fine_tune_params(),
            "[WHERE age = $_param_00000001]"
        );
        assert_eq!(where_clause.to_raw().build(), "[WHERE age = 18]");
    }

    #[test]
    fn test_display_clause_with_id_only_wors_with_node() {
        let id_clause =
            NodeClause::from(sql::Thing::from(("student".to_string(), "oye".to_string())));
        assert_eq!(id_clause.fine_tune_params(), "$_param_00000001");
        assert_eq!(id_clause.to_raw().build(), "student:oye");
    }

    #[test]
    fn test_display_clause_with_query() {
        let table = Table::new("students");
        let select_statement = select(All).from(table);
        let query_clause = Clause::from(select_statement);
        assert_eq!(query_clause.fine_tune_params(), "$_param_00000001");
        assert_eq!(query_clause.get_bindings().len(), 1);
        assert_eq!(query_clause.get_errors().len(), 0);
        assert_eq!(query_clause.to_raw().build(), "(SELECT * FROM students)");
    }

    #[test]
    fn test_display_clause_with_param() {
        let table = Table::new("students");
        let devs = let_("devs").equal_to(select(All).from(table));
        let clause = NodeClause::from(devs.get_param());
        assert_eq!(clause.build(), "$devs");
        assert_eq!(clause.to_raw().build(), "$devs");
    }

    #[test]
    fn test_display_clause_with_let_statement() {
        let table = Table::new("students");
        let devs = let_("devs").equal_to(select(All).from(table));
        let clause = NodeClause::from(devs);
        assert_eq!(clause.build(), "$devs");
        assert_eq!(clause.to_raw().build(), "$devs");
    }

    #[test]
    fn test_display_clause_with_all() {
        let all_clause = Clause::from(All);
        assert_eq!(all_clause.build(), "[*]");
        assert_eq!(all_clause.to_raw().build(), "[*]");
    }

    #[test]
    fn test_display_clause_with_last() {
        let last_clause = Clause::from(Last);
        assert_eq!(last_clause.build(), "[$]");
        assert_eq!(last_clause.to_raw().build(), "[$]");
    }

    #[test]
    fn test_display_clause_with_index() {
        let index_clause = Clause::from(index(42));
        assert_eq!(index_clause.fine_tune_params(), "[$_param_00000001]");
        assert_eq!(index_clause.to_raw().build(), "[42]");
    }

    #[test]
    fn test_display_clause_with_index_field() {
        let position = Field::new("position");
        let index_clause = Clause::from(index(position));
        assert_eq!(index_clause.fine_tune_params(), "[position]");
        assert_eq!(index_clause.to_raw().build(), "[position]");
    }

    #[test]
    fn test_display_clause_with_index_param() {
        let position = Param::new("position");
        let index_clause = Clause::from(index(position));
        assert_eq!(index_clause.fine_tune_params(), "[$position]");
        assert_eq!(index_clause.to_raw().build(), "[$position]");
    }

    #[test]
    fn test_display_clause_with_any_edge_condition_simple() {
        let writes = Table::new("writes");
        let reads = Table::new("reads");
        let purchased = Table::new("purchased");
        let amount = Field::new("amount");

        let age_edge_condition =
            any_other_edges(vec![writes, reads, purchased]).where_(amount.less_than_or_equal(120));

        assert_eq!(
            age_edge_condition.fine_tune_params(),
            "writes, reads, purchased  WHERE amount <= $_param_00000001"
        );
        assert_eq!(
            age_edge_condition.to_raw().build(),
            "writes, reads, purchased  WHERE amount <= 120"
        );
    }

    #[test]
    fn test_display_clause_with_any_edge_condition_complex() {
        let writes = Table::new("writes");
        let reads = Table::new("reads");
        let purchased = Table::new("purchased");
        let city = Field::new("city");

        let age_edge_condition = any_other_edges(vec![writes, reads, purchased]).where_(
            cond(city.is("Prince Edward Island"))
                .and(city.is("NewFoundland"))
                .or(city.like("Toronto")),
        );

        assert_eq!(
            age_edge_condition.fine_tune_params(),
            "writes, reads, purchased  WHERE (city IS $_param_00000001) AND (city IS $_param_00000002) OR (city ~ $_param_00000003)"
        );
        assert_eq!(
            age_edge_condition.to_raw().build(),
            "writes, reads, purchased  WHERE (city IS 'Prince Edward Island') AND (city IS 'NewFoundland') OR (city ~ 'Toronto')"
        );
    }
}

#[cfg(test)]
mod tests {
    use umbriel::*;
    use umbriel::*;

    #[test]
    fn can_build_patch_operation() {
        let email = Field::new("_email");

        let patch_op = PatchOp(patch(email));
        assert_eq!(patch_op.get_errors().len(), 0);
        assert_eq!(patch_op.get_bindings().len(), 1);
        assert_eq!(
            patch_op.fine_tune_params(),
            "{ op: 'add', path: $_param_00000001 }"
        );
        assert_eq!(patch_op.to_raw().build(), "{ op: 'add', path: '/_email' }");
    }

    #[test]
    fn gathers_errors_when_invalid_path_is_provided() {
        let email = Field::new("email[WHERE id = 1]");

        let patch_op = PatchOp(patch(email));

        assert_eq!(patch_op.get_errors().len(), 1);
        assert_eq!(
            patch_op.get_errors().first().unwrap(),
            "The path you have provided is invalid. \
            Make sure that there are no clauses or conditions included. Valid path include \
            e.g name, name(E).first, name(E).first(E).second, etc."
        );
        assert_eq!(patch_op.get_bindings().len(), 1);
        assert_eq!(
            patch_op.fine_tune_params(),
            "{ op: 'add', path: $_param_00000001 }"
        );
        assert_eq!(
            patch_op.to_raw().build(),
            "{ op: 'add', path: '/email[WHERE id = 1]' }"
        );
    }

    #[test]
    fn gathers_error_when_clauses_uses() {
        get_invalid_paths(Field::new("name[WHERE id = 1]"));
        get_invalid_paths(Field::new("name[WHERE id = 1].first"));
        get_invalid_paths(Field::new("name[0]"));
        get_invalid_paths(Field::new("name[1]"));
        get_invalid_paths(Field::new("name[$]"));
        get_invalid_paths(Field::new("name[*]"));
        get_invalid_paths(Field::new("name->"));
        get_invalid_paths(Field::new("name->writes"));
        get_invalid_paths(Field::new("->writes"));
        get_invalid_paths(Field::new("->"));
        get_invalid_paths(Field::new("->->"));
        get_invalid_paths(Field::new("-"));
        get_invalid_paths(Field::new("_-"));
        get_invalid_paths(Field::new("-something"));
        get_invalid_paths(Field::new("name->writes->book"));
        get_invalid_paths(Field::new("->writes->book"));
        get_invalid_paths(Field::new("user:cha->write->blog:mars"));
        get_invalid_paths(Field::new(
            "->knows->person->(knows WHERE influencer = true)",
        ));
        get_invalid_paths(Field::new("5book"));
        get_invalid_paths(Field::new("-book_"));
        get_invalid_paths(Field::new("*book_"));
        get_invalid_paths(Field::new("$book_"));
        get_invalid_paths(Field::new("%book_"));
        get_invalid_paths(Field::new("&book_"));
        get_invalid_paths(Field::new("#book_"));
        get_invalid_paths(Field::new("@book_"));
        get_invalid_paths(Field::new("(book_"));
        get_invalid_paths(Field::new(")book_"));
        get_invalid_paths(Field::new("book*"));
        get_invalid_paths(Field::new("bo$ok"));
    }

    fn get_invalid_paths(field: Field) {
        let patch_op = PatchOp(patch(field));

        assert_eq!(patch_op.get_errors().len(), 1);
        assert_eq!(
            patch_op.get_errors().first().unwrap(),
            "The path you have provided is invalid. \
            Make sure that there are no clauses or conditions included. Valid path include \
            e.g name, name(E).first, name(E).first(E).second, etc."
        );
    }

    #[test]
    fn can_build_add_operation() {
        let name = Field::new("_name.first");

        let patch_op = PatchOp(patch(name));
        assert_eq!(patch_op.get_errors().len(), 0);
        assert_eq!(patch_op.get_bindings().len(), 1);
        assert_eq!(
            patch_op.fine_tune_params(),
            "{ op: 'add', path: $_param_00000001 }"
        );
        assert_eq!(
            patch_op.to_raw().build(),
            "{ op: 'add', path: '/_name/first' }"
        );
    }

    #[test]
    fn can_build_change_operation() {
        let name = Field::new("name.first");

        let patch_op = PatchOp(patch(name));
        assert_eq!(patch_op.get_errors().len(), 0);
        assert_eq!(patch_op.get_bindings().len(), 1);
        assert_eq!(
            patch_op.fine_tune_params(),
            "{ op: 'add', path: $_param_00000001 }"
        );
        assert_eq!(
            patch_op.to_raw().build(),
            "{ op: 'add', path: '/name/first' }"
        );
    }

    #[test]
    fn can_build_remove_operation() {
        let name = Field::new("name.first");

        let patch_op = PatchOp(patch(name));
        assert_eq!(patch_op.get_errors().len(), 0);
        assert_eq!(patch_op.get_bindings().len(), 1);
        assert_eq!(
            patch_op.fine_tune_params(),
            "{ op: 'add', path: $_param_00000001 }"
        );
        assert_eq!(
            patch_op.to_raw().build(),
            "{ op: 'add', path: '/name/first' }"
        );
    }

    #[test]
    fn can_build_replace_operation() {
        let name = Field::new("name.first.title");

        let patch_op = PatchOp(patch(name));
        assert_eq!(patch_op.get_errors().len(), 0);
        assert_eq!(patch_op.get_bindings().len(), 1);
        assert_eq!(
            patch_op.fine_tune_params(),
            "{ op: 'add', path: $_param_00000001 }"
        );
        assert_eq!(
            patch_op.to_raw().build(),
            "{ op: 'add', path: '/name/first/title' }"
        );
    }
}

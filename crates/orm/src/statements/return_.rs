/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 */

use crate::{BindingsList, Buildable, Erroneous, ErrorList, Parametric, Queryable, ValueLike};

use super::select::Fetchables;

/// The `RETURN` statement.
///
/// # Arguemnt
/// * `return_value` - The value to return.
///
/// # Examples
/// ```
/// use aparecium::{*, statements::*, functions::*};
///
/// let user = TestUser::create_id("oyelowo");
/// let account = Field::new("account");
/// let connection = Field::new("connection");
///
/// let query = return_(user).fetch(&[account, connection]);
/// assert_eq!(
///     query.to_raw().build(),
///     "RETURN user:oyelowo FETCH account, connection;"
/// );
/// ```
pub fn return_(return_value: impl Into<ValueLike>) -> ReturnStatement {
    let return_value: ValueLike = return_value.into();
    let return_string = return_value.build();
    let bindings = return_value.get_bindings();
    let errors = return_value.get_errors();

    ReturnStatement {
        return_value: return_string,
        fetch: vec![],
        bindings,
        errors,
    }
}

/// The `RETURN` statement.
pub struct ReturnStatement {
    return_value: String,
    fetch: Vec<String>,
    bindings: BindingsList,
    errors: ErrorList,
}

impl ReturnStatement {
    /// Fetches the specified fields.
    pub fn fetch(mut self, fetchables: impl Into<Fetchables>) -> Self {
        let fields: Fetchables = fetchables.into();

        let fields = match fields {
            Fetchables::Field(one_field) => vec![one_field],
            Fetchables::Fields(many_fields) => many_fields,
        };

        fields.iter().for_each(|f| {
            self.fetch.push(f.build());
            self.bindings.extend(f.get_bindings());
            self.errors.extend(f.get_errors());
        });
        self
    }
}

impl Buildable for ReturnStatement {
    fn build(&self) -> String {
        let mut query = format!("RETURN {}", self.return_value);
        if !self.fetch.is_empty() {
            query = format!("{query} FETCH {}", self.fetch.join(", "));
        }

        format!("{query};")
    }
}

impl Parametric for ReturnStatement {
    fn get_bindings(&self) -> BindingsList {
        self.bindings.clone()
    }
}

impl Erroneous for ReturnStatement {
    fn get_errors(&self) -> ErrorList {
        self.errors.clone()
    }
}

impl Queryable for ReturnStatement {}

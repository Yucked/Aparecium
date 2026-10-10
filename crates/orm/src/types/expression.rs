/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 */

use crate::{statements::Subquery, Buildable, Erroneous, ErrorList, Parametric, ValueLike};

/// An expression is a value or statement that can be used within another query.
#[derive(Clone, Debug)]
pub enum Expression {
    Value(ValueLike),
    Subquery(Subquery),
}

impl Buildable for Expression {
    fn build(&self) -> String {
        match self {
            Expression::Value(value) => value.build(),
            Expression::Subquery(subquery) => subquery.build(),
        }
    }
}

impl Parametric for Expression {
    fn get_bindings(&self) -> Vec<crate::traits::Binding> {
        match self {
            Expression::Value(value) => value.get_bindings(),
            Expression::Subquery(subquery) => subquery.get_bindings(),
        }
    }
}

impl Erroneous for Expression {
    fn get_errors(&self) -> ErrorList {
        match self {
            Expression::Value(_value) => vec![],
            Expression::Subquery(subquery) => subquery.get_errors(),
        }
    }
}

impl<T> From<T> for Expression
where
    T: Into<ValueLike>,
{
    fn from(value: T) -> Self {
        Expression::Value(value.into())
    }
}


/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 */

use std::ops::Bound;

use crate::{
    Binding, Model, SurrealId, SurrealSimpleId, SurrealUlid, SurrealUuid, ValueLike,
    statements::select::TargettablesForSelect,
};
use surrealdb::sql;

impl<T, V> From<std::ops::RangeInclusive<SurrealId<T, V>>> for TargettablesForSelect
where
    T: Model,
    V: Into<sql::Id>,
{
    fn from(range: std::ops::RangeInclusive<SurrealId<T, V>>) -> Self {
        // e.g user:1..=5
        let range = sql::Range {
            tb: T::table().to_string(),
            beg: Bound::Included(range.start().to_thing().id),
            end: Bound::Included(range.end().to_thing().id),
        };
        let binding = Binding::new(range);
        TargettablesForSelect::RecordRange(ValueLike {
            string: binding.get_param_dollarised(),
            bindings: vec![binding],
            errors: vec![],
        })
    }
}

impl<T, V> From<std::ops::RangeFrom<SurrealId<T, V>>> for TargettablesForSelect
where
    T: Model,
    V: Into<sql::Id>,
{
    fn from(range: std::ops::RangeFrom<SurrealId<T, V>>) -> Self {
        // e.g user:1..
        let range = sql::Range {
            tb: T::table().to_string(),
            beg: Bound::Included(range.start.to_thing().id),
            end: Bound::Unbounded,
        };
        let binding = Binding::new(range);
        TargettablesForSelect::RecordRange(ValueLike {
            string: binding.get_param_dollarised(),
            bindings: vec![binding],
            errors: vec![],
        })
    }
}

impl<T, V> From<std::ops::RangeTo<SurrealId<T, V>>> for TargettablesForSelect
where
    T: Model,
    V: Into<sql::Id>,
{
    fn from(range: std::ops::RangeTo<SurrealId<T, V>>) -> Self {
        // e.g user:..5
        let range = sql::Range {
            tb: T::table().to_string(),
            beg: Bound::Unbounded,
            end: Bound::Excluded(range.end.to_thing().id),
        };
        let binding = Binding::new(range);
        TargettablesForSelect::RecordRange(ValueLike {
            string: binding.get_param_dollarised(),
            bindings: vec![binding],
            errors: vec![],
        })
    }
}

impl<T, V> From<std::ops::RangeToInclusive<SurrealId<T, V>>> for TargettablesForSelect
where
    T: Model,
    V: Into<sql::Id>,
{
    fn from(range: std::ops::RangeToInclusive<SurrealId<T, V>>) -> Self {
        // e.g user:..=5
        let range = sql::Range {
            tb: T::table().to_string(),
            beg: Bound::Unbounded,
            end: Bound::Included(range.end.to_thing().id),
        };
        let binding = Binding::new(range);
        TargettablesForSelect::RecordRange(ValueLike {
            string: binding.get_param_dollarised(),
            bindings: vec![binding],
            errors: vec![],
        })
    }
}

impl<T, V> From<std::ops::Range<SurrealId<T, V>>> for TargettablesForSelect
where
    T: Model,
    V: Into<sql::Id>,
{
    fn from(range: std::ops::Range<SurrealId<T, V>>) -> Self {
        // e.g user:1..5
        let range = sql::Range {
            tb: T::table().to_string(),
            beg: Bound::Included(range.start.to_thing().id),
            end: Bound::Excluded(range.end.to_thing().id),
        };
        let binding = Binding::new(range);
        TargettablesForSelect::RecordRange(ValueLike {
            string: binding.get_param_dollarised(),
            bindings: vec![binding],
            errors: vec![],
        })
    }
}

impl<T, V> From<std::ops::RangeInclusive<&SurrealId<T, V>>> for TargettablesForSelect
where
    T: Model,
    V: Into<sql::Id>,
{
    fn from(range: std::ops::RangeInclusive<&SurrealId<T, V>>) -> Self {
        let range = sql::Range {
            tb: T::table().to_string(),
            beg: Bound::Included(range.start().to_thing().id),
            end: Bound::Included(range.end().to_thing().id),
        };
        let binding = Binding::new(range);

        TargettablesForSelect::RecordRange(ValueLike {
            string: binding.get_param_dollarised(),
            bindings: vec![binding],
            errors: vec![],
        })
    }
}

impl<T, V> From<std::ops::RangeFrom<&SurrealId<T, V>>> for TargettablesForSelect
where
    T: Model,
    V: Into<sql::Id>,
{
    fn from(range: std::ops::RangeFrom<&SurrealId<T, V>>) -> Self {
        // e.g user:1..
        let range = sql::Range {
            tb: T::table().to_string(),
            beg: Bound::Included(range.start.to_thing().id),
            end: Bound::Unbounded,
        };
        let binding = Binding::new(range);
        TargettablesForSelect::RecordRange(ValueLike {
            string: binding.get_param_dollarised(),
            bindings: vec![binding],
            errors: vec![],
        })
    }
}

impl<T, V> From<std::ops::RangeTo<&SurrealId<T, V>>> for TargettablesForSelect
where
    T: Model,
    V: Into<sql::Id>,
{
    fn from(range: std::ops::RangeTo<&SurrealId<T, V>>) -> Self {
        // e.g user:..5
        let range = sql::Range {
            tb: T::table().to_string(),
            beg: Bound::Unbounded,
            end: Bound::Excluded(range.end.to_thing().id),
        };
        let binding = Binding::new(range);
        TargettablesForSelect::RecordRange(ValueLike {
            string: binding.get_param_dollarised(),
            bindings: vec![binding],
            errors: vec![],
        })
    }
}

impl<T, V> From<std::ops::RangeToInclusive<&SurrealId<T, V>>> for TargettablesForSelect
where
    T: Model,
    V: Into<sql::Id>,
{
    fn from(range: std::ops::RangeToInclusive<&SurrealId<T, V>>) -> Self {
        // e.g user:..=5
        let range = sql::Range {
            tb: T::table().to_string(),
            beg: Bound::Unbounded,
            end: Bound::Included(range.end.to_thing().id),
        };
        let binding = Binding::new(range);
        TargettablesForSelect::RecordRange(ValueLike {
            string: binding.get_param_dollarised(),
            bindings: vec![binding],
            errors: vec![],
        })
    }
}

impl<T, V> From<std::ops::Range<&SurrealId<T, V>>> for TargettablesForSelect
where
    T: Model,
    V: Into<sql::Id>,
{
    fn from(range: std::ops::Range<&SurrealId<T, V>>) -> Self {
        // e.g user:1..5
        let range = sql::Range {
            tb: T::table().to_string(),
            beg: Bound::Included(range.start.to_thing().id),
            end: Bound::Excluded(range.end.to_thing().id),
        };
        let binding = Binding::new(range);
        TargettablesForSelect::RecordRange(ValueLike {
            string: binding.get_param_dollarised(),
            bindings: vec![binding],
            errors: vec![],
        })
    }
}

//////////////////////////////////////////
macro_rules! create_range {
    ($id_type:ident) => {
        impl<T> From<std::ops::RangeInclusive<$id_type<T>>> for TargettablesForSelect
        where
            T: Model,
        {
            fn from(range: std::ops::RangeInclusive<$id_type<T>>) -> Self {
                // e.g user:1..=5
                let range = sql::Range {
                    tb: T::table().to_string(),
                    beg: Bound::Included(range.start().to_thing().id),
                    end: Bound::Included(range.end().to_thing().id),
                };
                let binding = Binding::new(range);
                TargettablesForSelect::RecordRange(ValueLike {
                    string: binding.get_param_dollarised(),
                    bindings: vec![binding],
                    errors: vec![],
                })
            }
        }

        impl<T> From<std::ops::RangeFrom<$id_type<T>>> for TargettablesForSelect
        where
            T: Model,
        {
            fn from(range: std::ops::RangeFrom<$id_type<T>>) -> Self {
                // e.g user:1..
                let range = sql::Range {
                    tb: T::table().to_string(),
                    beg: Bound::Included(range.start.to_thing().id),
                    end: Bound::Unbounded,
                };
                let binding = Binding::new(range);
                TargettablesForSelect::RecordRange(ValueLike {
                    string: binding.get_param_dollarised(),
                    bindings: vec![binding],
                    errors: vec![],
                })
            }
        }

        impl<T> From<std::ops::RangeTo<$id_type<T>>> for TargettablesForSelect
        where
            T: Model,
        {
            fn from(range: std::ops::RangeTo<$id_type<T>>) -> Self {
                // e.g user:..5
                let range = sql::Range {
                    tb: T::table().to_string(),
                    beg: Bound::Unbounded,
                    end: Bound::Excluded(range.end.to_thing().id),
                };
                let binding = Binding::new(range);
                TargettablesForSelect::RecordRange(ValueLike {
                    string: binding.get_param_dollarised(),
                    bindings: vec![binding],
                    errors: vec![],
                })
            }
        }

        impl<T> From<std::ops::RangeToInclusive<$id_type<T>>> for TargettablesForSelect
        where
            T: Model,
        {
            fn from(range: std::ops::RangeToInclusive<$id_type<T>>) -> Self {
                // e.g user:..=5
                let range = sql::Range {
                    tb: T::table().to_string(),
                    beg: Bound::Unbounded,
                    end: Bound::Included(range.end.to_thing().id),
                };
                let binding = Binding::new(range);
                TargettablesForSelect::RecordRange(ValueLike {
                    string: binding.get_param_dollarised(),
                    bindings: vec![binding],
                    errors: vec![],
                })
            }
        }

        impl<T> From<std::ops::Range<$id_type<T>>> for TargettablesForSelect
        where
            T: Model,
        {
            fn from(range: std::ops::Range<$id_type<T>>) -> Self {
                // e.g user:1..5
                let range = sql::Range {
                    tb: T::table().to_string(),
                    beg: Bound::Included(range.start.to_thing().id),
                    end: Bound::Excluded(range.end.to_thing().id),
                };
                let binding = Binding::new(range);
                TargettablesForSelect::RecordRange(ValueLike {
                    string: binding.get_param_dollarised(),
                    bindings: vec![binding],
                    errors: vec![],
                })
            }
        }
    };
}

create_range!(SurrealSimpleId);
create_range!(SurrealUuid);
create_range!(SurrealUlid);
///////////////////////////////////////////

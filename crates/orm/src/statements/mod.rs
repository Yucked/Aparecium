/*utils_chain
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 */

use std::fmt::Display;

pub(crate) mod break_;
pub(crate) mod continue_;
pub(crate) mod create;
pub(crate) mod create_v2;
pub(crate) mod define_analyzer;
pub(crate) mod define_database;
pub(crate) mod define_event;
pub(crate) mod define_field;
pub(crate) mod define_function;
pub(crate) mod define_index;
pub(crate) mod define_model;
pub(crate) mod define_namespace;
pub(crate) mod define_param;
pub(crate) mod define_scope;
pub(crate) mod define_table;
pub(crate) mod define_token;
pub(crate) mod define_user;
pub(crate) mod delete;
pub(crate) mod for_loop;
pub(crate) mod for_permission;
pub(crate) mod ifelse;
pub(crate) mod info;
pub(crate) mod insert;
pub(crate) mod let_;
pub(crate) mod live_select;
pub(crate) mod relate;
pub(crate) mod remove_analyzer;
pub(crate) mod remove_database;
pub(crate) mod remove_event;
pub(crate) mod remove_field;
pub(crate) mod remove_function;
pub(crate) mod remove_index;
pub(crate) mod remove_login;
pub(crate) mod remove_model;
pub(crate) mod remove_namespace;
pub(crate) mod remove_param;
pub(crate) mod remove_scope;
pub(crate) mod remove_table;
pub(crate) mod remove_token;
pub(crate) mod remove_user;
pub(crate) mod return_;
pub(crate) mod select;
pub(crate) mod select_surreal_id_range;
pub(crate) mod show;
pub(crate) mod sleep;
pub(crate) mod subquery;
pub(crate) mod throw;
pub(crate) mod transaction;
pub(crate) mod update;
pub(crate) mod use_;
pub(crate) mod utils_block;
pub(crate) mod utils_chain;

pub use break_::{BreakStatement, break_};
pub use continue_::{ContinueStatement, continue_};
pub use create::{CreateStatement, create, create_only};
pub use create_v2::{CreateStatementV2, create_v2};
pub use define_analyzer::{
    AnalyzerFilter, DefineAnalyzerStatement, SnowballLanguage, Tokenizer, define_analyzer,
};
pub use define_database::{DefineDatabaseStatement, define_database};
pub use define_event::{DefineEventStatement, define_event};
pub use define_field::{DefineFieldStatement, define_field};
pub use define_function::{DefineFunctionStatement, FunctionArgument, define_function};
pub use define_index::{DefineIndexStatement, Scoring, define_index, search_analyzer};
pub use define_model::{DefineModelStatement, ModelName, ModelVersion, define_model};
pub use define_namespace::{DefineNamespaceStatement, define_namespace};
pub use define_param::{DefineParamStatement, define_param};
pub use define_scope::{DefineScopeStatement, define_scope};
pub use define_table::{DefineTableStatement, define_table};
pub use define_token::{DefineTokenStatement, define_token};
pub use define_user::{DefineUserStatement, UserRole, define_user};
pub use delete::{DeleteStatement, DeleteStatementMini, delete, delete_only};
pub use for_loop::{ForLoopStatement, for_};
pub use for_permission::{ForPermission, Permissions, for_permission};
pub use ifelse::{IfElseStatement, if_};
pub use info::{InfoStatement, info_for};
pub use insert::{InsertStatement, Insertables, insert};
pub use let_::{LetStatement, let_};
pub use live_select::{LiveSelectStatement, live};
pub use relate::{RelateStatement, relate, relate_only};
pub use remove_analyzer::{RemoveAnalyzerStatement, remove_analyzer};
pub use remove_database::{RemoveDatabaseStatement, remove_database};
pub use remove_event::{RemoveEventStatement, remove_event};
pub use remove_field::{RemoveFieldStatement, remove_field};
pub use remove_function::{RemoveFunctionStatement, remove_function};
pub use remove_index::{RemoveIndexStatement, remove_index};
pub use remove_login::{RemoveLoginStatement, remove_login};
pub use remove_model::{RemoveModelStatement, remove_model};
pub use remove_namespace::{RemoveNamespaceStatement, remove_namespace};
pub use remove_param::{RemoveParamStatement, remove_param};
pub use remove_scope::{RemoveScopeStatement, remove_scope};
pub use remove_table::{RemoveTableStatement, remove_table};
pub use remove_token::{RemoveTokenStatement, remove_token};
pub use remove_user::{RemoveUserStatement, UserPermissionScope, remove_user};
pub use return_::{ReturnStatement, return_};
pub use select::{
    Order, Orderables, SelectStatement, SelectStatementMini, Selectables, Splittables, order,
    select, select_diff, select_value,
};
pub use show::{ShowChangesStatement, show_changes_for_table};
pub use sleep::{SleepStatement, sleep};
pub use subquery::Subquery;
pub use throw::{ThrowStatement, throw};
pub use transaction::{
    BeginTransactionStatement, TransactionCompletion, begin_transaction, transaction_deprecated,
};
pub use update::{UpdateStatement, update, update_only};
pub use use_::{UseStatement, use_};

/// helpers for statements
pub mod utils {
    pub use super::utils_block::{Block, block, block_deprecated};
    pub use super::utils_chain::{Chainable, QueryChain, chain};
}

#[allow(missing_docs)]
pub enum NamespaceOrDatabase {
    Namespace,
    Database,
}

impl Display for NamespaceOrDatabase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let stringified = match self {
            NamespaceOrDatabase::Namespace => "NAMESPACE",
            NamespaceOrDatabase::Database => "DATABASE",
        };
        write!(f, "{}", stringified)
    }
}

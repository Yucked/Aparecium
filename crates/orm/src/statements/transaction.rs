use crate::{Queryable, Render, Sql};

/// Creates a Transaction statement starting with BEGIN TRANSACTION and ends with COMMIT
/// TRANSACTION or CANCEL TRANSACTION.
///
/// Each statement within SurrealDB is run within its own transaction. If a set of changes need to
/// be made together, then groups of statements can be run together as a single transaction,
/// either succeeding as a whole, or failing without leaving any residual data modifications.
///
/// Example
///
/// ```rust
/// use aparecium::{*, statements::{begin_transaction, order, select}, types::RecordId};
/// # let name = Field::new("name");
/// # let age = Field::new("age");
/// # let country = Field::new("country");
/// # let city = Field::new("city");
/// # let fake_id = RecordId::new("user", "alice");
/// # let fake_id2 = RecordId::new("user", "bob");
///
/// let statement1 = select(All)
///     .from(fake_id)
///     .where_(cond(city.is("Prince Edward Island"))
///             .and(city.is("NewFoundland"))
///             .or(city.like("Toronto")),
///     )
///     .order_by(order(&age).numeric())
///     .limit(153)
///     .start(10)
///     .parallel();
///
/// let statement2 = select(All)
///     .from(fake_id2)
///     .where_(country.is("INDONESIA"))
///     .order_by(order(&age).numeric())
///     .limit(20)
///     .start(5);
///
/// let transaction = begin_transaction()
///     .query(statement1)
///     .query(statement2)
///     .commit_transaction();
/// ```
pub fn begin_transaction() -> QueryTransaction {
    QueryTransaction { queries: vec![] }
}

/// Creates a Transaction statement starting
#[macro_export]
macro_rules! transaction_deprecated {
    (BEGIN TRANSACTION; $($query:expr;)* COMMIT TRANSACTION;) => {
        {
            $crate::statements::begin_transaction().
            $(
            query(&$query).
            )*

            .commit_transaction()
        }
    };
    (begin transaction; $($query:expr;)* commit transaction;) => {
        {
            $crate::statements::begin_transaction().
            $(
            query(&$query).
            )*

            .commit_transaction()
        }
    };
    (BEGIN TRANSACTION; $($query:expr;)* CANCEL TRANSACTION;) => {
        {
            $crate::statements::begin_transaction().
            $(
            query(&$query).
            )*

            .cancel_transaction()
        }
    };
    (begin transaction; $($query:expr;)* cancel transaction;) => {
        {
            $crate::statements::begin_transaction().
            $(
            query(&$query).
            )*

            .cancel_transaction()
        }
    };
}
pub use transaction_deprecated;

/// A transaction collecting its statements.
pub struct QueryTransaction {
    queries: Vec<Sql>,
}

impl QueryTransaction {
    /// Adds a statement to the transaction.
    pub fn query(mut self, query: impl Queryable) -> Self {
        self.queries.push(query.to_sql());
        self
    }

    /// Ends the transaction with `COMMIT TRANSACTION`.
    pub fn commit_transaction(self) -> TransactionCompletion {
        TransactionCompletion {
            queries: self.queries,
            completion: Completion::Commit,
        }
    }

    /// Ends the transaction with `CANCEL TRANSACTION`.
    pub fn cancel_transaction(self) -> TransactionCompletion {
        TransactionCompletion {
            queries: self.queries,
            completion: Completion::Cancel,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Completion {
    Commit,
    Cancel,
}

/// Completed Transaction statements. Either COMMIT or CANCEL Transaction.
#[derive(Debug, Clone)]
pub struct TransactionCompletion {
    queries: Vec<Sql>,
    completion: Completion,
}

impl Render for TransactionCompletion {
    fn render(&self, sql: &mut Sql) {
        sql.push_str("BEGIN TRANSACTION;\n");
        for query in &self.queries {
            sql.push_str("\n").push(query).push_str("\n");
        }
        sql.push_str(match self.completion {
            Completion::Commit => "\nCOMMIT TRANSACTION;\n\t",
            Completion::Cancel => "\nCANCEL TRANSACTION;\n\t",
        });
    }
}

impl Queryable for TransactionCompletion {}

use surrealdb::types::Table;

use crate::{Event, Renderer};

use super::{Statement, impl_definition};

/// Remove event statement
///
/// # Arguments
///
/// * `event` - The name of the event to be removed. Can be a string or an Event type.
///
/// # Example
/// ```rust
/// use umbriel::{*, statements::remove_event};
/// # let user = Table::new("user");
/// # let party = Event::new("party");
///
/// let statement = remove_event(party).on_table(user);
/// assert_eq!(statement.to_debug_sql(), "REMOVE EVENT party ON TABLE user;");
/// ```
pub fn remove_event(event: impl Into<Event>) -> RemoveEventStatement {
    RemoveEventStatement {
        event: event.into(),
        table: None,
    }
}

/// Remove event statement
pub struct RemoveEventStatement {
    event: Event,
    table: Option<Table>,
}

impl RemoveEventStatement {
    /// Set the table to remove the event from.
    pub fn on_table(mut self, table: impl Into<Table>) -> Self {
        self.table = Some(table.into());
        self
    }
}

impl Statement for RemoveEventStatement {
    fn render_body(&self, out: &mut Renderer) {
        out.push_str("REMOVE EVENT ").push(&self.event);
        if let Some(table) = &self.table {
            out.push_str(" ON TABLE ").push(table);
        }
    }
}

impl_definition!(RemoveEventStatement);

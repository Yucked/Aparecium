use surrealdb::types::Table;

use crate::{Conditional, Event, Filter, Sql};

use super::{Statement, Subquery, impl_definition};

/// A DEFINE EVENT statement awaiting its condition.
pub struct EventBuilder {
    event: Event,
    on_table: Option<Table>,
}

/**
Define a new event.
Events can be triggered after any change or modification to the data in a record. Each trigger is able to see the $before and $after value of the record, enabling advanced custom logic with each trigger.

Requirements
You must be authenticated as a root, namespace, or database user before you can use the DEFINE EVENT statement.
You must select your namespace and database before you can use the DEFINE EVENT statement.

# Example
```rust
use aparecium::{*, statements::{define_event, select}, types::RecordId};

# let age = Field::new("age");
# let city = Field::new("city");
# let fake_id = RecordId::new("user", "alice");
# let user_table = Table::new("user");
# let email_event = Event::new("email");

 let query = define_event(email_event)
    .on_table(user_table)
     .when(age.greater_than_or_equal(18))
     .then(
         select(All)
             .from(fake_id)
             .where_(
                 cond(city.is("Prince Edward Island"))
                     .and(city.is("NewFoundland"))
                     .or(city.like("Toronto")),
             )
             .limit(153)
             .start(10)
             .parallel(),
     );

 assert_eq!(
     query.to_debug_sql(),
     "DEFINE EVENT email ON TABLE user WHEN age >= 18 THEN (SELECT * FROM user:alice \
WHERE ((city IS 'Prince Edward Island') AND (city IS 'NewFoundland')) OR (city ~ 'Toronto') \
LIMIT 153 START AT 10 PARALLEL);",
);
```
*/
pub fn define_event(event_name: impl Into<Event>) -> EventBuilder {
    EventBuilder {
        event: event_name.into(),
        on_table: None,
    }
}

impl EventBuilder {
    /// Set the event table
    pub fn on_table(mut self, table: impl Into<Table>) -> Self {
        self.on_table = Some(table.into());
        self
    }

    /// Set the event trigger
    pub fn when(self, condition: impl Conditional) -> Then {
        Then {
            builder: self,
            when: Filter::new(condition),
        }
    }
}

/// A DEFINE EVENT statement awaiting what it runs.
pub struct Then {
    builder: EventBuilder,
    when: Filter,
}

impl Then {
    /// What the event runs: a statement, rendered as a subquery, or a block.
    pub fn then(self, query: impl Into<Subquery>) -> DefineEventStatement {
        DefineEventStatement {
            event: self.builder.event,
            on_table: self.builder.on_table,
            when: self.when,
            then: query.into(),
        }
    }
}

/// A statement for defining an event.
pub struct DefineEventStatement {
    event: Event,
    on_table: Option<Table>,
    when: Filter,
    then: Subquery,
}

impl Statement for DefineEventStatement {
    fn render_body(&self, sql: &mut Sql) {
        sql.push_str("DEFINE EVENT ").push(&self.event);
        if let Some(table) = &self.on_table {
            sql.push_str(" ON TABLE ").push(table);
        }
        sql.push_str(" WHEN ")
            .push(&self.when)
            .push_str(" THEN ")
            .push(&self.then);
    }
}

impl_definition!(DefineEventStatement);

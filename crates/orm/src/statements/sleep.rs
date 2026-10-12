use crate::{DurationLike, Renderer};

use super::{Statement, impl_statement};

/// Creates a SLEEP statement.
///
/// Examples
///
/// ```rust
/// use std::time::Duration;
/// use umbriel::{*, statements::sleep};
///
/// assert_eq!(sleep(Duration::from_secs(43)).to_debug_sql(), "SLEEP 43s;");
/// ```
pub fn sleep(duration: impl Into<DurationLike>) -> SleepStatement {
    SleepStatement {
        duration: duration.into(),
    }
}

/// Sleep statement initialization builder
pub struct SleepStatement {
    duration: DurationLike,
}

impl Statement for SleepStatement {
    fn render_body(&self, out: &mut Renderer) {
        // SLEEP takes a duration literal.
        out.push_str("SLEEP ").push_inlined(&self.duration);
    }
}

impl_statement!(SleepStatement);

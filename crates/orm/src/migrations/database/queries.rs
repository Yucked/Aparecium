use std::fmt::{self, Display};

/// One line of a generated migration file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum QueryType {
    /// A statement, written with a single trailing `;`.
    Statement(String),
    /// A `--` comment.
    Comment(String),
    /// A blank line between groups of statements.
    Blank,
}

impl QueryType {
    /// The lines joined into a migration file's content.
    pub fn join(lines: &[QueryType]) -> String {
        lines
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join("\n")
    }
}

impl Display for QueryType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Statement(statement) => write!(f, "{};", statement.trim_end_matches(';')),
            Self::Comment(comment) => write!(f, "-- {comment}"),
            Self::Blank => Ok(()),
        }
    }
}

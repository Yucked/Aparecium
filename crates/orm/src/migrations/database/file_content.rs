use std::{fmt::Display, fs, path::PathBuf};

use crate::migrations::{Checksum, MigrationError, MigrationResult};

/// A migration file's SurrealQL.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct FileContent(String);

impl Display for FileContent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let FileContent(content) = self;
        write!(f, "{}", content)
    }
}

impl FileContent {
    /// No content.
    pub fn empty() -> Self {
        Self("".into())
    }

    /// The content's checksum.
    pub fn as_checksum(&self) -> MigrationResult<Checksum> {
        Checksum::generate_from_content(self)
    }

    /// The checksum of the file at `file_path`, not of this content.
    pub fn as_checksum_from_path(
        &self,
        file_path: impl Into<PathBuf>,
    ) -> MigrationResult<Checksum> {
        Checksum::generate_from_path(file_path)
    }

    /// Reads the file at `file_path`.
    pub fn from_file(file_path: impl Into<PathBuf>) -> MigrationResult<Self> {
        let file_path = file_path.into();
        let content = fs::read_to_string(file_path)
            .map_err(|e| MigrationError::IoError(format!("Error: {}", e)))?;
        Ok(Self(content))
    }

    /// The content as bytes.
    pub fn as_bytes(&self) -> &[u8] {
        self.0.as_bytes()
    }

    /// Whether there is no content.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl From<String> for FileContent {
    fn from(content: String) -> Self {
        Self(content)
    }
}

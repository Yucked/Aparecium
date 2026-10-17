use clap::ValueEnum;
use std::{fmt::Display, str::FromStr};

use crate::migrations::MigrationError;

/// Which way a migration is applied.
#[derive(Debug, Clone)]
pub enum Direction {
    /// From the previous schema to the new one.
    Up,
    /// Back from the new schema to the previous one.
    Down,
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let direction = match self {
            Self::Up => "up",
            Self::Down => "down",
        };
        write!(f, "{direction}")
    }
}

/// Whether migrations can be rolled back.
#[derive(Default, Debug, Clone, Copy)]
pub enum MigrationFlag {
    /// Each migration is an `.up.surql` and `.down.surql` pair.
    #[default]
    TwoWay,
    /// Each migration is a single `.surql` file.
    OneWay,
}

type Reversible = bool;

impl From<Reversible> for MigrationFlag {
    fn from(value: Reversible) -> Self {
        if value {
            MigrationFlag::TwoWay
        } else {
            MigrationFlag::OneWay
        }
    }
}

impl Display for MigrationFlag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let flag = match self {
            Self::TwoWay => "two_way",
            Self::OneWay => "one_way",
        };
        write!(f, "{flag}")
    }
}

impl MigrationFlag {
    /// Whether migrations are two-way.
    pub fn is_twoway(&self) -> bool {
        matches!(self, Self::TwoWay)
    }

    /// The flag names, as written in configuration.
    pub fn options() -> Vec<String> {
        vec![Self::TwoWay.to_string(), Self::OneWay.to_string()]
    }
}

impl TryFrom<String> for MigrationFlag {
    type Error = MigrationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "two_way" => Ok(Self::TwoWay),
            "one_way" => Ok(Self::OneWay),
            _ => Err(MigrationError::InvalidMigrationFlag(
                value,
                Self::options().join(", "),
            )),
        }
    }
}

/// How closely migration files are checked against the history.
#[derive(ValueEnum, Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Mode {
    /// File names and checksums must match the history.
    #[default]
    Strict,
    /// Files are not checked against the history.
    Lax,
}

impl FromStr for Mode {
    type Err = MigrationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_lowercase().as_str() {
            "strict" => Ok(Self::Strict),
            "lax" => Ok(Self::Lax),
            _ => Err(MigrationError::InvalidMigrationMode(
                s.to_string(),
                Self::options().join(", "),
            )),
        }
    }
}

impl Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mode = match self {
            Self::Strict => "strict",
            Self::Lax => "lax",
        };
        write!(f, "{mode}")
    }
}

impl Mode {
    /// The mode names, as written in configuration.
    pub fn options() -> Vec<String> {
        vec![Self::Strict.to_string(), Self::Lax.to_string()]
    }

    /// Whether the mode is strict.
    pub fn is_strict(&self) -> bool {
        matches!(self, Self::Strict)
    }

    /// Whether the mode is lax.
    pub fn is_relaxed(&self) -> bool {
        matches!(self, Self::Lax)
    }

    /// The mode's name.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Strict => "strict",
            Self::Lax => "lax",
        }
    }
}

//! The migrations-feature tests: generating, applying, listing, pruning, resetting and rolling back
//! migrations, embedded migrations and the CLI.
//!
//! They build only with this crate's `migrations` feature (`required-features` in the manifest),
//! which needs `umbriel`'s migrations module to compile; making it compile and bringing these
//! tests to the current API is #9.

mod cli;
mod down;
mod embed;
mod embedded;
mod generate;
mod init;
mod list;
mod prune;
mod reset;
mod up;

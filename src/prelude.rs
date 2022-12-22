//! Crate prelude

pub use crate::error::Error;
use clap::Parser;

pub type Result<T> = core::result::Result<T, Error>;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
pub struct Cli {
    /// The pattern to look for
    pub pattern: String,
    /// The path to the file to read
    pub path: std::path::PathBuf,
}

// Generic Wrapper tuple struct for newtype pattern
pub struct W<T>(pub T);

// Personal preference
pub use std::format as f;

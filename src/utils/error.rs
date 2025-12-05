use thiserror::Error;

/// Custom error type for Advent of Code solutions
#[derive(Error, Debug)]
pub enum AocError {
    #[error("Invalid day number: {0}. Day must be between 1 and 25.")]
    InvalidDay(u8),

    #[error("Input file not found for day {0}: {1}")]
    InputNotFound(u8, String),

    #[error("Failed to read input file: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Parse error: {0}")]
    ParseError(String),

    #[error("Solution error: {0}")]
    SolutionError(String),
}

/// Result type alias using AocError
pub type Result<T> = std::result::Result<T, AocError>;


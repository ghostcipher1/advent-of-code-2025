use crate::utils::error::{AocError, Result};
use std::fs;
use std::path::PathBuf;

/// Reads the input file for the specified day and part
///
/// # Arguments
/// * `day` - The day number (1-25)
/// * `part` - The part number (1 or 2)
/// * `use_sample` - If true, reads from sample file (day1p1s.txt format), 
///                  if false, reads from real file (day01p1.txt format)
///
/// # Returns
/// The contents of the input file as a String
///
/// # Errors
/// Returns `AocError::InvalidDay` if day is not between 1 and 25
/// Returns `AocError::InputNotFound` if the input file doesn't exist
/// Returns `AocError::IoError` if there's an error reading the file
pub fn read_input(day: u8, part: u8, use_sample: bool) -> Result<String> {
    // Validate day number
    if day < 1 || day > 25 {
        return Err(AocError::InvalidDay(day));
    }

    // Validate part number
    if part != 1 && part != 2 {
        return Err(AocError::ParseError(format!(
            "Invalid part number: {}. Part must be 1 or 2.",
            part
        )));
    }

    // Construct file path based on whether we're using sample or real input
    let file_path = if use_sample {
        // Sample files: day1p1s.txt, day1p2s.txt, etc. (no leading zero, 's' suffix)
        format!("inputs/day{}p{}s.txt", day, part)
    } else {
        // Real files: day01p1.txt, day01p2.txt, etc. (with leading zero)
        format!("inputs/day{:02}p{}.txt", day, part)
    };
    
    let path = PathBuf::from(&file_path);

    // Check if file exists
    if !path.exists() {
        return Err(AocError::InputNotFound(day, file_path));
    }

    // Read and return file contents
    fs::read_to_string(&path).map_err(AocError::from)
}


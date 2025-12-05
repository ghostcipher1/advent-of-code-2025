use crate::utils::{Result, AocError};

/// Checks if a product ID is invalid (made of a digit sequence repeated twice)
///
/// # Arguments
/// * `id` - The product ID to check
///
/// # Returns
/// `true` if the ID is invalid, `false` otherwise
fn is_invalid_id(id: u64) -> bool {
    let id_str = id.to_string();
    let len = id_str.len();
    
    // Must have even length to be split into two equal halves
    if len % 2 != 0 {
        return false;
    }
    
    // Split into two halves
    let half_len = len / 2;
    let first_half = &id_str[0..half_len];
    let second_half = &id_str[half_len..];
    
    // Check if the two halves are identical
    first_half == second_half
}

/// Solves Part 1 of Day 2
///
/// Finds all invalid product IDs (IDs made of a digit sequence repeated twice)
/// in the given ranges and returns their sum.
///
/// # Arguments
/// * `input` - The input string containing comma-separated ranges (format: start-end)
///
/// # Returns
/// The sum of all invalid IDs as a String
///
/// # Errors
/// Returns a `Result::Err` if there's an error parsing the input
pub fn solve_part1(input: &str) -> Result<String> {
    let mut total_sum: u64 = 0;
    
    // Parse ranges from input (comma-separated)
    for range_str in input.trim().split(',') {
        let range_str = range_str.trim();
        if range_str.is_empty() {
            continue;
        }
        
        // Parse range: "start-end"
        let parts: Vec<&str> = range_str.split('-').collect();
        if parts.len() != 2 {
            return Err(AocError::ParseError(format!(
                "Invalid range format: {}",
                range_str
            )));
        }
        
        let start: u64 = parts[0]
            .parse()
            .map_err(|_| AocError::ParseError(format!("Invalid start value: {}", parts[0])))?;
        
        let end: u64 = parts[1]
            .parse()
            .map_err(|_| AocError::ParseError(format!("Invalid end value: {}", parts[1])))?;
        
        // Check all IDs in this range
        for id in start..=end {
            if is_invalid_id(id) {
                total_sum += id;
            }
        }
    }
    
    Ok(total_sum.to_string())
}

/// Checks if a product ID is invalid (made of a digit sequence repeated at least twice)
/// This is for Part 2, where the sequence can be repeated 2, 3, 4, etc. times
///
/// # Arguments
/// * `id` - The product ID to check
///
/// # Returns
/// `true` if the ID is invalid, `false` otherwise
fn is_invalid_id_part2(id: u64) -> bool {
    let id_str = id.to_string();
    let len = id_str.len();
    
    // Try all possible part lengths (from 1 to len/2)
    // We need at least 2 repetitions, so part_len must be <= len/2
    for part_len in 1..=len / 2 {
        // Check if the length is divisible by part_len
        if len % part_len != 0 {
            continue;
        }
        
        // Extract the first part
        let first_part = &id_str[0..part_len];
        
        // Check if the entire string is made of repetitions of this part
        let num_repetitions = len / part_len;
        let reconstructed: String = first_part.repeat(num_repetitions);
        
        if reconstructed == id_str {
            return true;
        }
    }
    
    false
}

/// Solves Part 2 of Day 2
///
/// Finds all invalid product IDs (IDs made of a digit sequence repeated at least twice)
/// in the given ranges and returns their sum.
///
/// # Arguments
/// * `input` - The input string containing comma-separated ranges (format: start-end)
///
/// # Returns
/// The sum of all invalid IDs as a String
///
/// # Errors
/// Returns a `Result::Err` if there's an error parsing the input
pub fn solve_part2(input: &str) -> Result<String> {
    let mut total_sum: u64 = 0;
    
    // Parse ranges from input (comma-separated)
    for range_str in input.trim().split(',') {
        let range_str = range_str.trim();
        if range_str.is_empty() {
            continue;
        }
        
        // Parse range: "start-end"
        let parts: Vec<&str> = range_str.split('-').collect();
        if parts.len() != 2 {
            return Err(AocError::ParseError(format!(
                "Invalid range format: {}",
                range_str
            )));
        }
        
        let start: u64 = parts[0]
            .parse()
            .map_err(|_| AocError::ParseError(format!("Invalid start value: {}", parts[0])))?;
        
        let end: u64 = parts[1]
            .parse()
            .map_err(|_| AocError::ParseError(format!("Invalid end value: {}", parts[1])))?;
        
        // Check all IDs in this range
        for id in start..=end {
            if is_invalid_id_part2(id) {
                total_sum += id;
            }
        }
    }
    
    Ok(total_sum.to_string())
}


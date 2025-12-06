use crate::utils::{Result, AocError};

/// Parses the input into ranges and ingredient IDs
fn parse_input(input: &str) -> Result<(Vec<(u64, u64)>, Vec<u64>)> {
    let mut sections = input.split("\n\n");

    let ranges_section = sections
        .next()
        .ok_or_else(|| AocError::ParseError("Missing ranges section".to_string()))?;

    let ids_section = sections
        .next()
        .ok_or_else(|| AocError::ParseError("Missing ingredient IDs section".to_string()))?;

    // Parse ranges
    let mut ranges: Vec<(u64, u64)> = Vec::new();
    for line in ranges_section.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let parts: Vec<&str> = line.split('-').collect();
        if parts.len() != 2 {
            return Err(AocError::ParseError(format!("Invalid range format: {}", line)));
        }

        let start: u64 = parts[0]
            .parse()
            .map_err(|_| AocError::ParseError(format!("Invalid range start: {}", parts[0])))?;
        let end: u64 = parts[1]
            .parse()
            .map_err(|_| AocError::ParseError(format!("Invalid range end: {}", parts[1])))?;

        ranges.push((start, end));
    }

    // Parse ingredient IDs
    let mut ids: Vec<u64> = Vec::new();
    for line in ids_section.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let id: u64 = line
            .parse()
            .map_err(|_| AocError::ParseError(format!("Invalid ingredient ID: {}", line)))?;
        ids.push(id);
    }

    Ok((ranges, ids))
}

/// Checks if an ingredient ID is fresh (falls within any range)
fn is_fresh(id: u64, ranges: &[(u64, u64)]) -> bool {
    ranges.iter().any(|&(start, end)| id >= start && id <= end)
}

/// Solves Part 1 of Day 5
///
/// Count how many available ingredient IDs are fresh
/// (fall within at least one fresh range).
///
/// # Arguments
/// * `input` - The input string containing ranges and ingredient IDs
///
/// # Returns
/// The count of fresh ingredients as a String
///
/// # Errors
/// Returns a `Result::Err` if there's an error parsing the input
pub fn solve_part1(input: &str) -> Result<String> {
    let (ranges, ids) = parse_input(input)?;

    let fresh_count = ids.iter().filter(|&&id| is_fresh(id, &ranges)).count();

    Ok(fresh_count.to_string())
}

/// Merges overlapping ranges and returns sorted non-overlapping ranges
fn merge_ranges(mut ranges: Vec<(u64, u64)>) -> Vec<(u64, u64)> {
    if ranges.is_empty() {
        return ranges;
    }

    // Sort by start position
    ranges.sort_by_key(|r| r.0);

    let mut merged: Vec<(u64, u64)> = Vec::new();
    merged.push(ranges[0]);

    for &(start, end) in &ranges[1..] {
        let last = merged.last_mut().unwrap();
        // Check if ranges overlap or are adjacent
        if start <= last.1 + 1 {
            // Merge: extend the end if necessary
            last.1 = last.1.max(end);
        } else {
            // No overlap, add new range
            merged.push((start, end));
        }
    }

    merged
}

/// Solves Part 2 of Day 5
///
/// Count total unique ingredient IDs considered fresh by all ranges.
/// Ranges can overlap, so we merge them first.
///
/// # Arguments
/// * `input` - The input string containing ranges and ingredient IDs
///
/// # Returns
/// The total count of fresh IDs as a String
///
/// # Errors
/// Returns a `Result::Err` if there's an error parsing the input
pub fn solve_part2(input: &str) -> Result<String> {
    let (ranges, _ids) = parse_input(input)?;

    // Merge overlapping ranges
    let merged = merge_ranges(ranges);

    // Count total IDs in merged ranges
    let total: u64 = merged.iter().map(|&(start, end)| end - start + 1).sum();

    Ok(total.to_string())
}


use crate::utils::{Result, AocError};

/// Solves Part 1 of Day 1
///
/// The dial starts at 50 and can be rotated left (L) or right (R) by a given distance.
/// The dial is circular (0-99), so rotations wrap around.
/// Returns the count of how many times the dial points at 0 after any rotation.
///
/// # Arguments
/// * `input` - The input string containing rotation instructions (one per line)
///
/// # Returns
/// The solution as a String (the count of times the dial points at 0)
///
/// # Errors
/// Returns a `Result::Err` if there's an error parsing the input
pub fn solve_part1(input: &str) -> Result<String> {
    let mut position: i32 = 50;
    let mut count = 0;

    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        // Parse rotation: first character is direction (L or R), rest is distance
        if line.len() < 2 {
            return Err(AocError::ParseError(format!(
                "Rotation line too short: {}",
                line
            )));
        }

        let direction = line
            .chars()
            .next()
            .ok_or_else(|| AocError::ParseError("Empty rotation line".to_string()))?;
        
        let distance: i32 = line[1..]
            .parse()
            .map_err(|_| AocError::ParseError(format!("Invalid distance in rotation: {}", line)))?;

        // Apply rotation
        match direction {
            'L' => {
                // Left: subtract, wrapping at 0
                position = (position - distance + 100) % 100;
            }
            'R' => {
                // Right: add, wrapping at 99
                position = (position + distance) % 100;
            }
            _ => {
                return Err(AocError::ParseError(format!(
                    "Invalid direction '{}' in rotation: {}",
                    direction, line
                )));
            }
        }

        // Count if dial points at 0
        if position == 0 {
            count += 1;
        }
    }

    Ok(count.to_string())
}

/// Solves Part 2 of Day 1
///
/// The dial starts at 50 and can be rotated left (L) or right (R) by a given distance.
/// Counts EVERY time the dial points at 0 DURING a rotation (not just at the end).
/// This includes all times the dial passes through 0 while rotating, including wrapping.
///
/// # Arguments
/// * `input` - The input string containing rotation instructions (one per line)
///
/// # Returns
/// The solution as a String (the total count of times the dial points at 0)
///
/// # Errors
/// Returns a `Result::Err` if there's an error parsing the input
pub fn solve_part2(input: &str) -> Result<String> {
    let mut position: i32 = 50;
    let mut count = 0;

    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        // Parse rotation: first character is direction (L or R), rest is distance
        if line.len() < 2 {
            return Err(AocError::ParseError(format!(
                "Rotation line too short: {}",
                line
            )));
        }

        let direction = line
            .chars()
            .next()
            .ok_or_else(|| AocError::ParseError("Empty rotation line".to_string()))?;
        
        let distance: i32 = line[1..]
            .parse()
            .map_err(|_| AocError::ParseError(format!("Invalid distance in rotation: {}", line)))?;

        let start_position = position;

        // Count how many times we pass through 0 during the rotation
        // We check the position after each of the 'distance' clicks
        match direction {
            'L' => {
                // Left: subtract, wrapping backwards (99->98->...->1->0->99)
                // Count how many times position is 0 after each click
                let mut times_at_zero = 0;
                let mut current_pos = start_position;
                
                for _click in 1..=distance {
                    current_pos = (current_pos - 1 + 100) % 100;
                    if current_pos == 0 {
                        times_at_zero += 1;
                    }
                }
                
                count += times_at_zero;
                
                // Update position for next rotation
                position = current_pos;
            }
            'R' => {
                // Right: add, wrapping forwards (99->0->1->...)
                // Count how many times position is 0 after each click
                let mut times_at_zero = 0;
                let mut current_pos = start_position;
                
                for _click in 1..=distance {
                    current_pos = (current_pos + 1) % 100;
                    if current_pos == 0 {
                        times_at_zero += 1;
                    }
                }
                
                count += times_at_zero;
                
                // Update position for next rotation
                position = current_pos;
            }
            _ => {
                return Err(AocError::ParseError(format!(
                    "Invalid direction '{}' in rotation: {}",
                    direction, line
                )));
            }
        }
    }

    Ok(count.to_string())
}


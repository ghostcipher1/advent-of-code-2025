use crate::utils::{Result, AocError};

/// Solves Part 1 of Day 4
///
/// Count paper rolls (@) that have fewer than 4 adjacent rolls
/// in the 8 neighboring positions (including diagonals).
///
/// # Arguments
/// * `input` - The input string containing the grid
///
/// # Returns
/// The count of accessible rolls as a String
///
/// # Errors
/// Returns a `Result::Err` if there's an error parsing the input
pub fn solve_part1(input: &str) -> Result<String> {
    let grid: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().collect())
        .collect();

    if grid.is_empty() {
        return Err(AocError::ParseError("Empty grid".to_string()));
    }

    let rows = grid.len();
    let cols = grid[0].len();

    // 8 directions: up, down, left, right, and 4 diagonals
    let directions: [(i32, i32); 8] = [
        (-1, -1), (-1, 0), (-1, 1),
        (0, -1),           (0, 1),
        (1, -1),  (1, 0),  (1, 1),
    ];

    let mut accessible_count = 0;

    for row in 0..rows {
        for col in 0..cols {
            // Only check positions with paper rolls
            if grid[row][col] != '@' {
                continue;
            }

            // Count adjacent rolls
            let mut adjacent_rolls = 0;
            for (dr, dc) in &directions {
                let new_row = row as i32 + dr;
                let new_col = col as i32 + dc;

                // Check bounds
                if new_row >= 0 && new_row < rows as i32 && new_col >= 0 && new_col < cols as i32 {
                    if grid[new_row as usize][new_col as usize] == '@' {
                        adjacent_rolls += 1;
                    }
                }
            }

            // Accessible if fewer than 4 adjacent rolls
            if adjacent_rolls < 4 {
                accessible_count += 1;
            }
        }
    }

    Ok(accessible_count.to_string())
}

/// Counts adjacent rolls for a given position
fn count_adjacent_rolls(grid: &[Vec<char>], row: usize, col: usize) -> usize {
    let rows = grid.len();
    let cols = grid[0].len();

    let directions: [(i32, i32); 8] = [
        (-1, -1), (-1, 0), (-1, 1),
        (0, -1),           (0, 1),
        (1, -1),  (1, 0),  (1, 1),
    ];

    let mut count = 0;
    for (dr, dc) in &directions {
        let new_row = row as i32 + dr;
        let new_col = col as i32 + dc;

        if new_row >= 0 && new_row < rows as i32 && new_col >= 0 && new_col < cols as i32 {
            if grid[new_row as usize][new_col as usize] == '@' {
                count += 1;
            }
        }
    }
    count
}

/// Solves Part 2 of Day 4
///
/// Repeatedly remove accessible rolls (fewer than 4 adjacent)
/// until no more can be removed. Count total rolls removed.
///
/// # Arguments
/// * `input` - The input string containing the grid
///
/// # Returns
/// The total count of removed rolls as a String
///
/// # Errors
/// Returns a `Result::Err` if there's an error parsing the input
pub fn solve_part2(input: &str) -> Result<String> {
    let mut grid: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().collect())
        .collect();

    if grid.is_empty() {
        return Err(AocError::ParseError("Empty grid".to_string()));
    }

    let rows = grid.len();
    let cols = grid[0].len();

    let mut total_removed = 0;

    loop {
        // Find all accessible rolls this round
        let mut to_remove: Vec<(usize, usize)> = Vec::new();

        for row in 0..rows {
            for col in 0..cols {
                if grid[row][col] == '@' {
                    let adjacent = count_adjacent_rolls(&grid, row, col);
                    if adjacent < 4 {
                        to_remove.push((row, col));
                    }
                }
            }
        }

        // Stop if no rolls can be removed
        if to_remove.is_empty() {
            break;
        }

        // Remove all accessible rolls
        for (row, col) in &to_remove {
            grid[*row][*col] = '.';
        }

        total_removed += to_remove.len();
    }

    Ok(total_removed.to_string())
}


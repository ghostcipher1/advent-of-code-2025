use crate::utils::Result;

pub mod day01;
pub mod day02;
pub mod day03;

/// Type alias for a solve function that takes input and returns a result string
type SolveFn = fn(&str) -> Result<String>;

/// Registry mapping day numbers to their solve functions
struct DayRegistry {
    part1: Option<SolveFn>,
    part2: Option<SolveFn>,
}

/// Day registry map - add new days here as they are implemented
fn get_day_registry(day: u8) -> Option<DayRegistry> {
    match day {
        1 => Some(DayRegistry {
            part1: Some(day01::solve_part1),
            part2: Some(day01::solve_part2),
        }),
        2 => Some(DayRegistry {
            part1: Some(day02::solve_part1),
            part2: Some(day02::solve_part2),
        }),
        3 => Some(DayRegistry {
            part1: Some(day03::solve_part1),
            part2: Some(day03::solve_part2),
        }),
        _ => None,
    }
}

/// Solves Part 1 for the given day
///
/// # Arguments
/// * `day` - The day number (1-25)
/// * `input` - The input string for the day's challenge
///
/// # Returns
/// The solution as a String
///
/// # Errors
/// Returns an error if the day is not implemented or if solving fails
pub fn solve_part1(day: u8, input: &str) -> Result<String> {
    let registry = get_day_registry(day)
        .ok_or_else(|| crate::utils::AocError::InvalidDay(day))?;
    
    registry
        .part1
        .ok_or_else(|| {
            crate::utils::AocError::SolutionError(format!("Part 1 not implemented for day {}", day))
        })?
        (input)
}

/// Solves Part 2 for the given day
///
/// # Arguments
/// * `day` - The day number (1-25)
/// * `input` - The input string for the day's challenge
///
/// # Returns
/// The solution as a String
///
/// # Errors
/// Returns an error if the day is not implemented or if solving fails
pub fn solve_part2(day: u8, input: &str) -> Result<String> {
    let registry = get_day_registry(day)
        .ok_or_else(|| crate::utils::AocError::InvalidDay(day))?;
    
    registry
        .part2
        .ok_or_else(|| {
            crate::utils::AocError::SolutionError(format!("Part 2 not implemented for day {}", day))
        })?
        (input)
}


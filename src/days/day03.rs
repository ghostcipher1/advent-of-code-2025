use crate::utils::{Result, AocError};

/// Solves Part 1 of Day 3
///
/// Each line is a bank of batteries with digit joltage ratings (1-9).
/// We must turn on exactly 2 batteries per bank; the joltage produced
/// is the two-digit number formed by those digits (in order).
/// Find the maximum joltage from each bank and return the sum.
///
/// # Arguments
/// * `input` - The input string containing battery banks (one per line)
///
/// # Returns
/// The total output joltage as a String
///
/// # Errors
/// Returns a `Result::Err` if there's an error parsing the input
pub fn solve_part1(input: &str) -> Result<String> {
    let mut total: u64 = 0;

    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let digits: Vec<u32> = line
            .chars()
            .map(|c| c.to_digit(10).ok_or_else(|| {
                AocError::ParseError(format!("Invalid digit '{}' in line: {}", c, line))
            }))
            .collect::<Result<Vec<_>>>()?;

        if digits.len() < 2 {
            return Err(AocError::ParseError(format!(
                "Bank must have at least 2 batteries: {}",
                line
            )));
        }

        // Find maximum two-digit number by selecting two positions i < j
        let mut max_joltage: u32 = 0;

        for i in 0..digits.len() - 1 {
            for j in (i + 1)..digits.len() {
                let joltage = digits[i] * 10 + digits[j];
                if joltage > max_joltage {
                    max_joltage = joltage;
                }
            }
        }

        total += max_joltage as u64;
    }

    Ok(total.to_string())
}

/// Solves Part 2 of Day 3
///
/// Same as Part 1, but now we must turn on exactly 12 batteries per bank.
/// The joltage is the 12-digit number formed by those digits (in order).
/// Find the maximum joltage from each bank and return the sum.
///
/// # Arguments
/// * `input` - The input string containing battery banks (one per line)
///
/// # Returns
/// The total output joltage as a String
///
/// # Errors
/// Returns a `Result::Err` if there's an error parsing the input
pub fn solve_part2(input: &str) -> Result<String> {
    let mut total: u128 = 0;
    const K: usize = 12; // Number of digits to select

    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let digits: Vec<u32> = line
            .chars()
            .map(|c| c.to_digit(10).ok_or_else(|| {
                AocError::ParseError(format!("Invalid digit '{}' in line: {}", c, line))
            }))
            .collect::<Result<Vec<_>>>()?;

        let n = digits.len();

        if n < K {
            return Err(AocError::ParseError(format!(
                "Bank must have at least {} batteries, got {}: {}",
                K, n, line
            )));
        }

        // Greedy selection: pick K digits to maximize the resulting number
        // At each step, pick the largest digit that leaves enough remaining
        let mut result_digits: Vec<u32> = Vec::with_capacity(K);
        let mut start = 0;

        for i in 0..K {
            // Range of valid positions: [start, n - K + i]
            // We need (K - i - 1) more digits after this one
            let end = n - K + i;

            // Find the maximum digit in range [start, end] (leftmost if ties)
            let mut max_val = 0;
            let mut max_pos = start;
            for pos in start..=end {
                if digits[pos] > max_val {
                    max_val = digits[pos];
                    max_pos = pos;
                }
            }

            result_digits.push(max_val);
            start = max_pos + 1;
        }

        // Convert result_digits to a number
        let mut joltage: u128 = 0;
        for &d in &result_digits {
            joltage = joltage * 10 + d as u128;
        }

        total += joltage;
    }

    Ok(total.to_string())
}


use anyhow::{Context, Result as AnyhowResult};
use clap::Parser;
use aoc_2025::days;
use aoc_2025::utils;

/// Advent of Code 2025 Solver
#[derive(Parser, Debug)]
#[command(name = "aoc-2025")]
#[command(about = "Solve Advent of Code 2025 challenges", long_about = None)]
struct Args {
    /// Day number (1-25)
    #[arg(short, long, value_name = "DAY")]
    day: u8,

    /// Use sample input files instead of real input files
    #[arg(short, long)]
    sample: bool,
}

fn main() -> AnyhowResult<()> {
    let args = Args::parse();

    // Validate day number
    if args.day < 1 || args.day > 25 {
        anyhow::bail!("Day must be between 1 and 25, got: {}", args.day);
    }

    // Solve Part 1
    println!("=== Day {} Part 1 ===", args.day);
    let input_part1 = utils::read_input(args.day, 1, args.sample)
        .context(format!("Failed to read input for day {} part 1", args.day))?;
    match days::solve_part1(args.day, &input_part1) {
        Ok(answer) => println!("Answer: {}\n", answer),
        Err(e) => {
            eprintln!("Error solving Part 1: {}\n", e);
            return Err(anyhow::Error::from(e));
        }
    }

    // Solve Part 2
    println!("=== Day {} Part 2 ===", args.day);
    let input_part2 = match utils::read_input(args.day, 2, args.sample) {
        Ok(input) => input,
        Err(utils::AocError::InputNotFound(_, _)) => {
            println!("Unable to locate part 2 files.");
            return Ok(());
        }
        Err(e) => {
            return Err(anyhow::Error::from(e)
                .context(format!("Failed to read input for day {} part 2", args.day)));
        }
    };
    match days::solve_part2(args.day, &input_part2) {
        Ok(answer) => println!("Answer: {}", answer),
        Err(e) => {
            eprintln!("Error solving Part 2: {}", e);
            return Err(anyhow::Error::from(e));
        }
    }

    Ok(())
}


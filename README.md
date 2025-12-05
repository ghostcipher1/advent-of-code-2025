# 🎄 Advent of Code 2025

> *"The best way to spread Christmas cheer is coding puzzles loud for all to hear."*

[![Rust](https://img.shields.io/badge/Rust-1.75%2B-orange?logo=rust)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-green.svg)](LICENSE)
[![AoC 2025](https://img.shields.io/badge/Advent%20of%20Code-2025-yellow?logo=adventofcode)](https://adventofcode.com/2025)

A collection of solutions for [Advent of Code 2025](https://adventofcode.com/2025) implemented in **Rust** — combining the joy of holiday puzzles with the power of systems programming.

<div align="center">

```
        ⭐
        /\
       /  \
      /    \
     /  ()  \
    /   /\   \
   /   /  \   \
  /   / ** \   \
 /   /  ()  \   \
/   /   /\   \   \
^^^/^^^/^🎁^^^^^/^^^
       |||
       |||
```

</div>

---

## 🎅 About This Project

This repository contains my solutions for Advent of Code 2025, an annual coding challenge that runs from December 1st through December 25th. Each day presents a new two-part programming puzzle wrapped in a festive narrative.

### 🏆 The Challenge

This project serves dual purposes:
- **Professional Portfolio**: Demonstrating proficiency in Rust, algorithmic problem-solving, and clean code architecture
- **Friendly Competition**: Part of a coding challenge with a friend — may the best solutions win! ⭐

---

## 🛠️ Project Structure

```
aoc-2025/
├── Cargo.toml              # Project dependencies & configuration
├── inputs/                 # Puzzle input files
│   ├── day01p1.txt         # Real inputs (dayXXpY.txt)
│   ├── day1p1s.txt         # Sample inputs (dayXpYs.txt)
│   └── ...
└── src/
    ├── main.rs             # CLI entry point
    ├── lib.rs              # Library exports
    ├── days/               # Daily solutions
    │   ├── mod.rs          # Day registry & dispatch
    │   ├── day01.rs        # Day 1: Dial
    │   ├── day02.rs        # Day 2: Product IDs
    │   ├── day03.rs        # Day 3: Lobby
    │   └── ...
    └── utils/              # Shared utilities
        ├── error.rs        # Custom error types
        └── input.rs        # Input file handling
```

---

## 🚀 Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) 1.75 or later
- Cargo (included with Rust)

### Installation

```bash
git clone https://github.com/yourusername/aoc-2025.git
cd aoc-2025
cargo build --release
```

### Usage

```bash
# Solve a specific day with real input
cargo run -- -d <DAY>

# Solve with sample input (for testing)
cargo run -- -d <DAY> --sample

# Examples
cargo run -- -d 1           # Run Day 1
cargo run -- -d 3 --sample  # Run Day 3 with sample input
```

### Command Line Options

| Flag | Description |
|------|-------------|
| `-d, --day <DAY>` | Day number to solve (1-25) |
| `-s, --sample` | Use sample input files |
| `-h, --help` | Display help information |

---

## 🌟 Progress

| Day | Puzzle | Part 1 | Part 2 |
|:---:|--------|:------:|:------:|
| 1 | Dial | ⭐ | ⭐ |
| 2 | Product IDs | ⭐ | ⭐ |
| 3 | Lobby | ⭐ | ⭐ |
| 4 | *Locked* | 🔒 | 🔒 |
| 5 | *Locked* | 🔒 | 🔒 |
| ... | ... | ... | ... |
| 25 | *Locked* | 🔒 | 🔒 |

**Total Stars: 6/50** ⭐

---

## 🎁 Features

- **Modular Architecture**: Each day is self-contained with a clean interface
- **Robust Error Handling**: Custom error types with descriptive messages using `thiserror`
- **CLI Interface**: User-friendly command-line interface powered by `clap`
- **Sample Testing**: Easy switching between sample and real inputs for validation
- **Extensible Design**: Simple pattern for adding new days as they unlock

---

## 📚 Resources & References

This project was developed with the assistance of multiple resources. The following references were instrumental in building robust Rust solutions:

### Official Documentation

Rust Foundation. (2024). *The Rust programming language* (2021 ed.). https://doc.rust-lang.org/book/

Rust Foundation. (2024). *Rust by example*. https://doc.rust-lang.org/rust-by-example/

Rust Foundation. (2024). *The Cargo book*. https://doc.rust-lang.org/cargo/

### Books & Learning Materials

Klabnik, S., & Nichols, C. (2023). *The Rust programming language* (2nd ed.). No Starch Press. https://nostarch.com/rust-programming-language-2nd-edition

Blandy, J., Orendorff, J., & Tindall, L. F. S. (2021). *Programming Rust: Fast, safe systems development* (2nd ed.). O'Reilly Media.

McNamara, T. (2021). *Rust in action: Systems programming concepts and techniques*. Manning Publications.

### Crate Documentation

Tolnay, D. (2024). *anyhow: Flexible concrete error type built on std::error::Error* (Version 1.0) [Rust crate]. https://docs.rs/anyhow/

Tolnay, D. (2024). *thiserror: Derive(Error) for struct and enum error types* (Version 1.0) [Rust crate]. https://docs.rs/thiserror/

Clap Contributors. (2024). *clap: Command line argument parser for Rust* (Version 4.5) [Rust crate]. https://docs.rs/clap/

### Community Resources

Advent of Code. (2024). *About Advent of Code*. https://adventofcode.com/2024/about

Rust Community. (2024). *Rust subreddit*. Reddit. https://www.reddit.com/r/rust/

---

## 🤝 Acknowledgments

- [Eric Wastl](http://was.tl/) for creating Advent of Code
- The Rust community for excellent documentation and tooling
- My friend and competitor for making this challenge even more exciting! 🎯

---

## 📄 License

This project is licensed under the MIT License — see the [LICENSE](LICENSE) file for details.

---

<div align="center">

**Happy Holidays & Happy Coding!** 🎄✨

*Built with ❤️ and 🦀 Rust*

</div>


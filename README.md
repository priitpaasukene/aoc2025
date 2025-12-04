# Advent of Code 2025

Learning Rust through Advent of Code 2025 challenges.

## Project Structure

This project uses a Cargo workspace with separate crates for each day:
- `common/`: Shared utilities and helper functions
- `day01/` through `day12/`: Individual day solutions

Each day contains:
- `src/lib.rs`: Shared code between parts
- `src/bin/p1.rs`: Part 1 solution
- `src/bin/p2.rs`: Part 2 solution
- `src/input.txt`: Puzzle input

## Running Solutions

```bash
# From the workspace root
cargo run --bin day01-p1
cargo run --bin day01-p2

# Or specify the package explicitly
cargo run --bin day01-p1 -p day01
cargo run --bin day01-p2 -p day01

# Or from a specific day directory
cd day01
cargo run --bin day01-p1
cargo run --bin day01-p2

# Run with optimizations (recommended for complex solutions)
cargo run --release --bin day01-p1
cargo run --release --bin day01-p2

# Run tests
cargo test -p day01
cargo test --workspace
```

## Progress

| Day | Part 1 | Part 2 | Notes |
|-----|--------|--------|-------|
| 01  | ⬜     | ⬜     |       |
| 02  | ⬜     | ⬜     |       |
| 03  | ⬜     | ⬜     |       |
| 04  | ⬜     | ⬜     |       |
| 05  | ⬜     | ⬜     |       |
| 06  | ⬜     | ⬜     |       |
| 07  | ⬜     | ⬜     |       |
| 08  | ⬜     | ⬜     |       |
| 09  | ⬜     | ⬜     |       |
| 10  | ⬜     | ⬜     |       |
| 11  | ⬜     | ⬜     |       |
| 12  | ⬜     | ⬜     |       |

Legend: ⬜ Not started | 🟡 In progress | ✅ Complete

## Development Notes

- Using Rust edition 2021
- Each day is independent and can be developed/run separately
- Common utilities in `common/` crate for code reuse
- Tests included in each binary for example validation

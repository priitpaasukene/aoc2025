use std::fs;

/// Read input file from current directory (src/input.txt)
/// This should be called from within a day's binary (p1.rs or p2.rs)
pub fn read_input_local() -> String {
    fs::read_to_string("src/input.txt")
        .expect("Failed to read src/input.txt")
}

/// Read input file from a specific day directory
pub fn read_input(day: &str) -> String {
    let path = format!("{}/src/input.txt", day);
    fs::read_to_string(&path)
        .unwrap_or_else(|_| panic!("Failed to read input file at {}", path))
}

// Add more common utilities here as needed:
// - Grid structures and coordinate types
// - Parsing helpers
// - Algorithm implementations (BFS, DFS, pathfinding, etc.)
// - Mathematical utilities
// - String processing functions

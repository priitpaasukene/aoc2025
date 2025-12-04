use day09::parse_input;

fn main() {
    let input = include_str!("../input.txt");
    let data = parse_input(input);
    
    let result = solve(&data);
    println!("Part 1: {}", result);
}

fn solve(_data: &[String]) -> usize {
    // TODO: Implement part 1 solution
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = "";
        let data = parse_input(input);
        assert_eq!(solve(&data), 0);
    }
}

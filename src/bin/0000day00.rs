use std::fs;

const YEAR: &str = "2025";
const DAY: &str = "00";

fn main() {
    println!(">> Advent of Code {YEAR} Day {DAY}");

    let input = fs::read_to_string(format!("src/bin/{YEAR}day{DAY}.txt")).unwrap();

    // part 1
    let result1 = part1(&input);
    println!(">> part 1: {result1}");

    // part 2
    let result2 = part2(&input);
    println!(">> part 2: {result2}");
}

fn part1(input: &str) -> usize {
    todo!()
}

fn part2(input: &str) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    const TEST_INPUT: &str = r#""#;
    const TEST_SOLUTION_P1: usize = 0;
    const TEST_SOLUTION_P2: usize = 0;

    #[test]
    fn test_solution_p1() {
        let _vec_str = TEST_INPUT.split_whitespace();

        assert_eq!(TEST_SOLUTION_P1, 123);
    }

    #[test]
    fn test_solution_p2() {
        let _vec_str = TEST_INPUT.split_whitespace();

        assert_eq!(TEST_SOLUTION_P2, 123);
    }
}

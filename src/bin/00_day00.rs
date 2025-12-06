use std::fs;

const YEAR: &str = "2025";
const DAY: &str = "00";

fn main() {
    println!("day{DAY}");

    let string = fs::read_to_string(format!("src/bin/22025day{DAY}.txt")).unwrap();
    let _input = string.split_whitespace();

    // part 1
    let result1 = 0;
    println!(">> part 1: {result1}");

    // part 2
    let result2 = 0;
    println!(">> part 2: {result2}");
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r#""#;
    const TEST_SOLUTION_P1: usize = 0;
    const TEST_SOLUTION_P2: usize = 0;

    #[test]
    fn test_test_solution_p1() {
        let _vec_str = TEST_INPUT.split_whitespace();

        assert_eq!(TEST_SOLUTION_P1, 0);
    }

    #[test]
    fn test_test_solution_p2() {
        let _vec_str = TEST_INPUT.split_whitespace();

        assert_eq!(TEST_SOLUTION_P2, 0);
    }
}

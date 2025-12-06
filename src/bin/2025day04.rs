use std::fs;

const YEAR: &str = "2025";
const DAY: &str = "04";

fn main() {
    println!("year {YEAR}: day {DAY}");

    let string = fs::read_to_string(format!("src/bin/{YEAR}day{DAY}.txt")).unwrap();
    let input = string.split_whitespace().collect();

    // part 1
    let result1 = can_forklift(input);

    println!(">> part 1: {result1}");

    // part 2
    let result2 = 0;
    println!(">> part 2: {result2}");
}

fn can_forklift(input: Vec<&str>) -> usize {
    let rows = input.len();
    let mut can_forklift = 0;
    // a b c
    // h x d
    // g f e
    for row in 0..rows {
        let top_line = if row == 0 { None } else { input.get(row - 1) };
        let mid_line = input.get(row).unwrap();
        let bot_line = input.get(row + 1);

        // walk through the line
        for index in 0..mid_line.len() {
            let left_offset = if index == 0 { 0 } else { 1 };
            let right_offset = if index == mid_line.len() - 1 { 1 } else { 2 };
            if &mid_line[index..index + 1] == "@" {
                let abc = match top_line {
                    Some(line) => &line[(index - left_offset)..(index + right_offset)],
                    None => "...",
                };
                let hxd = &mid_line[(index - left_offset)..(index + right_offset)];
                let gfe = match bot_line {
                    Some(line) => &line[(index - left_offset)..(index + right_offset)],
                    None => "...",
                };
                let s = String::new();
                let x = s + abc + hxd + gfe;
                let rolls = x.replace(".", "");
                if rolls.len() <= 4 {
                    can_forklift += 1;
                }
            }
        }
    }

    can_forklift
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r#"
..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@."#;
    const TEST_SOLUTION_P1: usize = 13;
    const TEST_SOLUTION_P2: usize = 0;

    #[test]
    fn test_test_solution_p1() {
        let input: Vec<&str> = TEST_INPUT.split_whitespace().collect();
        let result = can_forklift(input);

        assert_eq!(TEST_SOLUTION_P1, result);
    }

    #[test]
    fn test_test_solution_p2() {
        let _vec_str = TEST_INPUT.split_whitespace();

        assert_eq!(TEST_SOLUTION_P2, 0);
    }
}

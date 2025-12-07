use std::fs;

const YEAR: &str = "2025";
const DAY: &str = "04";

fn main() {
    println!("year {YEAR}: day {DAY}");

    let string = fs::read_to_string(format!("src/bin/{YEAR}day{DAY}.txt")).unwrap();

    let mut input: Vec<String> = string.split_whitespace().map(|s| s.to_owned()).collect();

    // part 1
    let (result1, _vec) = can_forklift(&input);

    println!(">> part 1: {}", result1);

    // part 2
    let mut sum = 0;
    loop {
        let (count, coordinates) = can_forklift(&input);
        if count == 0 {
            break;
        }
        sum += count;
        for (row, index) in coordinates {
            let mut line = input.get(row).unwrap().clone();
            line.replace_range(index..index + 1, ".");
            input[row] = line;
        }
    }
    let result2 = sum;
    println!(">> part 2: {result2}");
}

fn can_forklift(input: &Vec<String>) -> (usize, Vec<(usize, usize)>) {
    let mut coordinates: Vec<(usize, usize)> = Vec::new();
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
                    coordinates.push((row, index));
                }
            }
        }
    }

    (can_forklift, coordinates)
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
    const TEST_SOLUTION_P2: usize = 43;

    #[test]
    fn test_test_solution_p1() {
        let input: Vec<String> = TEST_INPUT
            .split_whitespace()
            .map(|s| s.to_owned())
            .collect();
        let result = can_forklift(&input);

        assert_eq!(TEST_SOLUTION_P1, result.0);
    }

    #[test]
    fn test_test_solution_p2() {
        let mut input: Vec<String> = TEST_INPUT
            .split_whitespace()
            .map(|s| s.to_owned())
            .collect();
        let mut sum = 0;
        loop {
            let (count, coordinates) = can_forklift(&input);
            println!(">> count {count}");
            if count == 0 {
                break;
            }
            sum += count;
            for (row, index) in coordinates {
                let mut line = input.get(row).unwrap().clone();
                println!(">> line {line}");
                line.replace_range(index..index + 1, ".");
                println!(">> repl {line}");
                input[row] = line;
            }
        }

        assert_eq!(TEST_SOLUTION_P2, sum);
    }
}

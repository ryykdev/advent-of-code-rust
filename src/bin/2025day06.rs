use std::fs;

const YEAR: &str = "2025";
const DAY: &str = "06";

fn main() {
    println!("day{DAY}");

    let input = fs::read_to_string(format!("src/bin/{YEAR}day{DAY}.txt")).unwrap();

    // part 1
    //let result1 = part1(&input);
    //println!(">> part 1: {result1}");

    // part 2
    let result2 = part2(&input);
    println!(">> part 2: {result2}");
}

fn part1(input: &str) -> usize {
    let lines: Vec<&str> = input.lines().collect();
    let mut lines_of_digits: Vec<Vec<&str>> = Vec::new();
    for line in lines {
        let vec_digits: Vec<&str> = line.split_whitespace().collect();
        lines_of_digits.push(vec_digits);
    }
    let mut sum: usize = 0;
    for index in 0..lines_of_digits.first().expect("first digit line").len() {
        let digits1: usize = lines_of_digits
            .first()
            .expect("line of digits")
            .get(index)
            .expect("char")
            .to_string()
            .parse()
            .unwrap();
        let digits2: usize = lines_of_digits
            .get(1)
            .expect("line of digits")
            .get(index)
            .expect("char")
            .to_string()
            .parse()
            .unwrap();
        let digits3: usize = lines_of_digits
            .get(2)
            .expect("line of digits")
            .get(index)
            .expect("char")
            .to_string()
            .parse()
            .unwrap();
        let digits4: usize = lines_of_digits
            .get(3)
            .expect("line of digits")
            .get(index)
            .expect("char")
            .to_string()
            .parse()
            .unwrap();
        let operator = lines_of_digits
            .get(4)
            .expect("line of digits")
            .get(index)
            .expect("operator");
        match *operator {
            "+" => sum += digits1 + digits2 + digits3 + digits4,
            "*" => sum += digits1 * digits2 * digits3 * digits4,
            _ => sum += 0,
        };
    }
    sum
}

fn part2(input: &str) -> usize {
    let lines: Vec<&str> = input.lines().collect();
    let mut lines_of_digits: Vec<Vec<&str>> = Vec::new();
    for line in lines {
        let vec_digits: Vec<&str> = line.split_whitespace().collect();
        lines_of_digits.push(vec_digits);
    }
    let mut sum: usize = 0;
    for index in 0..lines_of_digits.first().expect("first digit line").len() {
        // 63x
        // 61x
        // 632
        // 276
        let digits1 = lines_of_digits
            .first()
            .expect("line of digits")
            .get(index)
            .expect("char")
            .to_string();
        let digits2 = lines_of_digits
            .get(1)
            .expect("line of digits")
            .get(index)
            .expect("char")
            .to_string();
        let digits3 = lines_of_digits
            .get(2)
            .expect("line of digits")
            .get(index)
            .expect("char")
            .to_string();
        let digits4 = lines_of_digits
            .get(3)
            .expect("line of digits")
            .get(index)
            .expect("char")
            .to_string();
        let operator = lines_of_digits
            .get(4)
            .expect("line of digits")
            .get(index)
            .expect("operator");
        dbg!(&digits1);
        dbg!(&digits2);
        dbg!(&digits3);
        dbg!(&digits4);
        dbg!(&operator);
    }
    sum
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    const TEST_INPUT: &str = r#"
123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  
"#;
    const TEST_SOLUTION_P1: usize = 4277556;
    const TEST_SOLUTION_P2: usize = 3263827;

    //#[test]
    //fn test_solution_p1() {
    //    let input = TEST_INPUT;
    //
    //    assert_eq!(TEST_SOLUTION_P1, part1(input));
    //}

    //#[test]
    //fn test_solution_p2() {
    //    let input = TEST_INPUT;
    //    assert_eq!(TEST_SOLUTION_P2, sum);
    //}
}

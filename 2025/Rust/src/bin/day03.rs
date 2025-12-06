use std::{fs, ops::Index};

static DAY: &str = "03";

fn main() {
    println!("day{DAY}");

    let input = fs::read_to_string(format!("../input/day{DAY}.txt")).unwrap();
    let split = input.split_whitespace();

    // part 1
    let mut sum = 0;
    for s in split.clone() {
        sum += highest_two_digt_int(s);
    }
    let result1 = sum;
    println!(">> part 1: {result1}");

    // part 2
    let mut sum = 0;
    for s in split {
        sum += highest_twelve_digit(s);
    }
    let result2 = sum;
    println!(">> part 2: {}", result2);
}

fn highest_two_digt_int(input: &str) -> usize {
    //
    let digits: Vec<usize> = input
        .chars()
        .map(|c| c.to_string().parse::<usize>().expect("int string"))
        .collect();
    let mut highest = 0;
    for index in 0..digits.len() - 1 {
        let first = digits.get(index).unwrap();
        for index2 in index + 1..digits.len() {
            let second = digits.get(index2).unwrap();
            if (first * 10 + second) > highest {
                highest = first * 10 + second;
            }
        }
    }
    highest
}

// my version after seeing a solution
fn highest_twelve_digit(input: &str) -> usize {
    let capacity = 12;
    let mut window = capacity;

    let mut result = String::new();

    let mut index = 0;
    while result.len() < capacity {
        'inner: for n in (0..=9).rev() {
            if let Some(idx) = input[index..].find(&n.to_string()) {
                if input[index + idx..].len() >= window {
                    result.push_str(&n.to_string());
                    index += idx;
                    window -= 1;
                    if result.len() == 12 {
                        return result.parse::<usize>().expect("integer");
                    }
                    index += 1;
                    break 'inner;
                }
            }
        }
    }

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r#"987654321111111
811111111111119
234234234234278
818181911112111"#;
    const TEST_SOLUTION: usize = 357;
    const TEST_SOLUTION_PART2: u128 = 3121910778619;

    #[test]
    fn test_test_solution_p1() {
        let vec_str = TEST_INPUT.split_whitespace();

        let mut sum = 0;
        for s in vec_str {
            sum += highest_two_digt_int(s);
        }

        assert_eq!(TEST_SOLUTION, sum);
    }
    #[test]
    fn test_test_solution_p2() {
        let input = TEST_INPUT.split_whitespace();

        let mut sum = 0;

        for s in input {
            sum += highest_twelve_digit(s);
        }

        assert_eq!(TEST_SOLUTION_PART2, sum.try_into().unwrap());
    }

    #[test]
    fn test_highest_int() {
        let input = "818181911112111";

        assert_eq!(92, highest_two_digt_int(input));
    }
    #[test]
    fn test_highest_twelve_digit() {
        let input = "234234234234278";

        assert_eq!(434234234278, highest_twelve_digit(input));
    }
}

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

// this is an LLM(Gemini/fast) solution
// - as I didn't have enough time to solve it
fn highest_twelve_digit(input: &str) -> usize {
    let k: usize = 12; // The desired length of the resulting number

    if input.len() < k {
        // Not enough digits to form a 12-digit number
        return 0;
    }

    let digits: Vec<u8> = input
        .chars()
        .map(|c| c.to_digit(10).expect("Input must contain only digits") as u8)
        .collect();

    let mut result: Vec<u8> = Vec::with_capacity(k);
    let mut current_index: usize = 0;

    // Loop until we have selected 12 digits for the result
    while result.len() < k {
        // The number of digits we still need to select
        let digits_needed: usize = k - result.len();

        // The search window ends at the position where the number of remaining
        // input digits equals the number of digits we still need.
        // i.e., input.len() - (digits_needed - 1)
        let search_end_index = digits.len() - digits_needed + 1;

        // Search for the largest digit within the valid window
        let mut max_digit: u8 = 0;
        let mut max_index: usize = current_index;

        // Iterate through the valid search window
        for i in current_index..search_end_index {
            if digits[i] > max_digit {
                max_digit = digits[i];
                max_index = i;
            }
            // Optimization: If we find a '9', we can't do better, so we stop searching
            // the current window immediately.
            if max_digit == 9 {
                break;
            }
        }

        // Append the largest digit found in the window to the result
        result.push(max_digit);

        // Move the starting point of the next search to the position *after* the selected digit
        current_index = max_index + 1;
    }

    // Convert the vector of digits back into a single usize number
    let result_string: String = result.into_iter().map(|d| d.to_string()).collect();

    // The parse will succeed because we only collected valid digits (u8)
    result_string
        .parse::<usize>()
        .expect("Should be a valid number")
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

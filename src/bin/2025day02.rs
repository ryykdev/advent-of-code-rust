use std::fs;

static DAY: &str = "02";

fn main() {
    println!("day{}", DAY);

    let input = fs::read_to_string(format!("../input/day{}.txt", DAY))
        .unwrap()
        .remove_whitespaces();

    // part 1
    let ranges = ranges_usize(&input);
    let mut vec_invalid_ids = Vec::new();
    for (start, end) in ranges.clone() {
        for id in start..=end {
            vec_invalid_ids.push(return_invalid_id(id));
        }
    }
    let result1: usize = vec_invalid_ids.iter().sum();
    println!(">> part 1: {}", result1);

    // part 2
    let mut vec_invalid_ids = Vec::new();
    for (start, end) in ranges {
        for id in start..=end {
            vec_invalid_ids.push(return_invalid_ids_part2(id));
        }
    }
    let result2: usize = vec_invalid_ids.iter().sum();
    println!(">> part 2: {}", result2);
}

// helper for removing whitespaces
trait RemoveWhitespaces {
    fn remove_whitespaces(&self) -> String;
}

impl RemoveWhitespaces for String {
    fn remove_whitespaces(&self) -> String {
        self.chars().filter(|c| !c.is_whitespace()).collect()
    }
}

/// returns ids with repeating digits, like 995995
fn return_invalid_id(id: usize) -> usize {
    let s = id.to_string();

    let (x, y) = s.split_at(s.len() / 2);

    if x == y {
        return id;
    }
    0
}

// find repeating patterns like: 1111, 121212, 123123123, 95559555
fn return_invalid_ids_part2(id: usize) -> usize {
    let s = id.to_string();

    for i in 1..=(s.len() / 2) {
        //
        if s == s[0..i].repeat(s.len() / i) {
            return id;
        }
    }
    0
}

fn ranges_usize(input: &str) -> Vec<(usize, usize)> {
    let mut vec_ranges: Vec<(usize, usize)> = Vec::new();
    for range in input.split(",") {
        let (x, y) = range.split_at(range.find("-").expect("- delimiter"));
        vec_ranges.push((
            x.parse().map_err(|e| println!("x={x} error: {e}")).unwrap(),
            y.replace("-", "")
                .parse()
                .map_err(|e| println!("y={y} error: {e}"))
                .unwrap(),
        ));
    }
    vec_ranges
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
    const TEST_SOLUTION: usize = 1227775554;
    const TEST_SOLUTION_PART2: usize = 4174379265;

    #[test]
    fn test_test_input() {
        let input = TEST_INPUT;
        let ranges = ranges_usize(input);
        let mut vec_invalid_ids = Vec::new();
        for (start, end) in ranges {
            for id in start..=end {
                vec_invalid_ids.push(return_invalid_id(id));
            }
        }
        let sum_ids: usize = vec_invalid_ids.iter().sum();
        assert_eq!(TEST_SOLUTION, sum_ids);
    }

    #[test]
    fn test_return_invalid_id() {
        let id: usize = 95559555;
        assert_eq!(id, return_invalid_id(id));
    }
    #[test]
    fn test_return_invalid_id_part2() {
        let id: usize = 95559555;
        assert_eq!(id, return_invalid_ids_part2(id));
    }
    #[test]
    fn test_ranges_usize() {
        let input = "824824821-824824827";
        let ranges = ranges_usize(input);
        assert_eq!(vec![(11, 22), (42, 66)], ranges);
    }
}

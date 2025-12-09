use std::fs;

const YEAR: &str = "2025";
const DAY: &str = "05";

fn main() {
    println!("day{DAY}");

    let string = fs::read_to_string(format!("src/bin/{YEAR}day{DAY}.txt")).unwrap();
    let _input = string.split_whitespace();

    // part 1
    let result1 = part1(&string);
    println!(">> part 1: {result1}");

    // part 2
    let result2 = part2(&string);
    println!(">> part 2: {result2}");
}

fn part1(input: &str) -> usize {
    let (ranges, ids) = input.split_once("\n\n").unwrap();
    let ranges: Vec<&str> = ranges.split_whitespace().collect();
    let ranges: Vec<(usize, usize)> = ranges
        .into_iter()
        .map(|range| {
            let (a_str, b_str) = range.split_once("-").expect("range like '1-5'");
            let a = a_str.parse::<usize>().expect("integer");
            let b = b_str.parse::<usize>().expect("integer");
            (a, b)
        })
        .collect();
    let ids: Vec<usize> = ids.split_whitespace().map(|s| s.parse().unwrap()).collect();

    let mut fresh_ingredients = 0;
    for id in ids {
        for (start, end) in &ranges {
            if id >= *start && id <= *end {
                fresh_ingredients += 1;
                break;
            }
        }
    }
    fresh_ingredients
}

fn part2(input: &str) -> usize {
    let (ranges, _) = input.split_once("\n\n").unwrap();
    let ranges: Vec<&str> = ranges.split_whitespace().collect();
    let mut ranges: Vec<(usize, usize)> = ranges
        .into_iter()
        .map(|range| {
            let (a_str, b_str) = range.split_once("-").expect("range like '1-5'");
            let a = a_str.parse::<usize>().expect("integer");
            let b = b_str.parse::<usize>().expect("integer");
            (a, b)
        })
        .collect();

    ranges.sort();

    // since the ranges are sorted you only
    // need to check if the next range starts
    // before the selected range ends if yes
    // - extend that range if no - its a new
    // range, merge these ranges
    let mut merged_ranges = Vec::new();

    let mut current_merged_range = ranges[0];

    for i in 1..ranges.len() {
        let next_range = ranges[i];
        if next_range.0 <= current_merged_range.1 + 1 {
            current_merged_range.1 = current_merged_range.1.max(next_range.1);
        } else {
            println!(
                "merged ranges {}-{}",
                current_merged_range.0, current_merged_range.1
            );
            merged_ranges.push(current_merged_range);
            current_merged_range = next_range;
        }
    }

    merged_ranges.push(current_merged_range);

    let mut count = 0;
    for (start, end) in merged_ranges {
        count += end - start + 1;
    }
    count
}

#[cfg(test)]
mod tests {

    use crate::part2;

    const TEST_INPUT: &str = r#"
3-5
10-14
16-20
12-18

1
5
8
11
17
32
"#;
    const TEST_SOLUTION_P1: usize = 3;
    const TEST_SOLUTION_P2: usize = 14;

    #[test]
    fn test_solution_p1() {
        let (ranges, ids) = TEST_INPUT.split_once("\n\n").unwrap();
        let ranges: Vec<&str> = ranges.split_whitespace().collect();
        let ranges: Vec<(usize, usize)> = ranges
            .into_iter()
            .map(|range| {
                let (a_str, b_str) = range.split_once("-").expect("range like '1-5'");
                let a = a_str.parse::<usize>().expect("integer");
                let b = b_str.parse::<usize>().expect("integer");
                (a, b)
            })
            .collect();
        let ids: Vec<usize> = ids.split_whitespace().map(|s| s.parse().unwrap()).collect();

        let mut fresh_ingredients = 0;
        for id in ids {
            'outer: for (start, end) in &ranges {
                for i in *start..=*end {
                    if i == id {
                        fresh_ingredients += 1;
                        break 'outer;
                    }
                }
            }
        }

        assert_eq!(TEST_SOLUTION_P1, fresh_ingredients);
    }

    #[test]
    fn test_solution_p2() {
        let input = TEST_INPUT;
        let solution = part2(input);
        assert_eq!(TEST_SOLUTION_P2, solution);
    }
}

use std::{cmp::max, fs};

const YEAR: &str = "2025";
const DAY: &str = "09";

fn main() {
    println!("day{DAY}");

    let input = fs::read_to_string(format!("src/bin/{YEAR}day{DAY}.txt")).unwrap();

    // part 1
    let result1 = part1(&input);
    println!(">> part 1: {result1}");

    // part 2
    let result2 = part2(&input);
    println!(">> part 2: {result2}");
}

fn part1(input: &str) -> usize {
    // parse list of "digit, digit" -> (usize, usize)
    // matrix (row, column)
    let mut matrix: Vec<(usize, usize)> = input
        .lines()
        .map(|line| {
            let (x_str, y_str) = line.split_once(',').expect("digit, digit");
            let x: usize = x_str.parse().expect("digits");
            let y: usize = y_str.parse().expect("digits");
            (x, y)
        })
        .collect();

    matrix.sort();

    // for each point - find the biggest square
    let mut biggest = 0;
    for a in &matrix {
        for b in matrix.iter().skip(1) {
            let candidate = square_poits(*a, *b);
            biggest = max(biggest, candidate);
        }
    }
    biggest
}

fn part2(input: &str) -> usize {
    todo!()
}

/// p1 (row, column), p2 (row, column)
fn square_poits(a: (usize, usize), b: (usize, usize)) -> usize {
    // 1-7 and 1-11
    // 5-2 and 7-9
    let rows = a.0.abs_diff(b.0) + 1;
    let columns = a.1.abs_diff(b.1) + 1;

    rows * columns
}

#[cfg(test)]
mod tests {
    use std::cmp::max;

    use crate::square_poits;

    const TEST_INPUT: &str = r#"7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3"#;
    const TEST_SOLUTION_P1: usize = 50;
    const TEST_SOLUTION_P2: usize = 0;

    #[test]
    fn test_square_points() {
        let a = (2, 5);
        let b = (9, 7);

        assert_eq!(24, square_poits(a, b));

        let a = (7, 1);
        let b = (11, 7);

        assert_eq!(35, square_poits(a, b));
    }

    #[test]
    fn test_solution_p1() {
        // parse list of "digit, digit" -> (usize, usize)
        // matrix (row, column)
        let mut matrix: Vec<(usize, usize)> = TEST_INPUT
            .lines()
            .map(|line| {
                let (x_str, y_str) = line.split_once(',').expect("digit, digit");
                let x: usize = x_str.parse().expect("digits");
                let y: usize = y_str.parse().expect("digits");
                (x, y)
            })
            .collect();

        matrix.sort();

        //for p in matrix {
        //    println!("{}, {}", p.0, p.1);
        //}
        // for each point - find the biggest square
        // then result is biggest square
        let mut biggest = 0;
        for a in &matrix {
            for b in matrix.iter().skip(1) {
                let candidate = square_poits(*a, *b);
                biggest = max(biggest, candidate);
            }
        }

        assert_eq!(TEST_SOLUTION_P1, biggest);
    }

    #[test]
    fn test_solution_p2() {
        let _vec_str = TEST_INPUT.split_whitespace();

        assert_eq!(TEST_SOLUTION_P2, 123);
    }
}

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
    // parse list of "digit, digit" -> (usize, usize)
    // matrix (column, row)
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

    // plot # and X in a matrix
    let mut x_max = 0;
    let mut y_max = 0;
    for point in &matrix {
        x_max = point.0.max(x_max);
        y_max = point.1.max(y_max);
    }
    let mut grid = vec![vec!['.'; x_max + 1]; y_max + 1];

    for point in &matrix {
        grid[point.1][point.0] = '#';
    }

    // (column, row)
    // ( p.0, p.1 )
    //7,1
    //11,1
    //11,7
    //9,7
    //9,5
    //2,5
    //2,3
    //7,3
    for a in &matrix {
        for b in matrix.iter().skip(1) {
            // horizontal Xes if same row
            if a.1 == b.1 && a.0 < b.0 {
                for index in a.0 + 1..b.0 {
                    grid[a.1][index] = 'X';
                }
            }
            // vertical X's if in same column
            if a.0 == b.0 && a.1 < b.1 {
                for index in a.1 + 1..b.1 {
                    grid[index][a.0] = 'X';
                }
            }
        }
    }
    // make all marks X
    for row in &mut grid {
        for column_index in 0..row.len() {
            let c = row[column_index];
            if c == '#' {
                row[column_index] = 'X';
            }
        }
    }
    // fill in Xs between the Xs
    for row in &mut grid {
        // find first X
        let left = row.iter().position(|&c| c == 'X');
        let right = row.iter().rposition(|&c| c == 'X');
        if let (Some(start), Some(end)) = (left, right) {
            if start < end {
                row[start..=end].fill('X');
            }
        }
    }
    // print grid
    //for row in &grid {
    //    let line: String = row.iter().collect();
    //    println!("{}", line);
    //}

    // find biggest square which doenst
    // contain '.'
    let mut biggest = 0;
    for a in &matrix {
        for b in matrix.iter().skip(1) {
            // (column, row)
            //
            let candidate = square_poits(*a, *b);
            // 2,5 11,1
            let row_range = a.1.min(b.1)..=a.1.max(b.1);
            let col_range = a.0.min(b.0)..=a.0.max(b.0);
            let candidate_is_filled = 'outer: {
                for row_index in row_range.clone() {
                    //
                    for char_index in col_range.clone() {
                        //
                        let c = grid[row_index][char_index];
                        //println!("{}", c);
                        if c == '.' {
                            //println!("break 'outer");
                            break 'outer false;
                        }
                    }
                }
                true
            };
            if candidate_is_filled {
                //println!("is filled: {candidate_is_filled}");
                biggest = max(biggest, candidate);
                //println!("{} points a({},{}), b({},{})", biggest, a.0, a.1, b.0, b.1);
            }
        }
    }
    biggest
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
    const TEST_SOLUTION_P2: usize = 24;

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
        // matrix (column, row)
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
        // parse list of "digit, digit" -> (usize, usize)
        // matrix (column, row)
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

        // plot # and X in a matrix
        let mut x_max = 0;
        let mut y_max = 0;
        for point in &matrix {
            x_max = point.0.max(x_max);
            y_max = point.1.max(y_max);
        }
        let mut grid = vec![vec!['.'; x_max + 1]; y_max + 1];

        for point in &matrix {
            grid[point.1][point.0] = '#';
        }

        // (column, row)
        // ( p.0, p.1 )
        //7,1
        //11,1
        //11,7
        //9,7
        //9,5
        //2,5
        //2,3
        //7,3
        for a in &matrix {
            for b in matrix.iter().skip(1) {
                // horizontal Xes if same row
                if a.1 == b.1 && a.0 < b.0 {
                    for index in a.0 + 1..b.0 {
                        grid[a.1][index] = 'X';
                    }
                }
                // vertical X's if in same column
                if a.0 == b.0 && a.1 < b.1 {
                    for index in a.1 + 1..b.1 {
                        grid[index][a.0] = 'X';
                    }
                }
            }
        }
        // make all marks X
        for row in &mut grid {
            for column_index in 0..row.len() {
                let c = row[column_index];
                if c == '#' {
                    row[column_index] = 'X';
                }
            }
        }
        // fill in Xs between the Xs
        for row in &mut grid {
            // find first X
            let left = row.iter().position(|&c| c == 'X');
            let right = row.iter().rposition(|&c| c == 'X');
            if let (Some(start), Some(end)) = (left, right) {
                if start < end {
                    row[start..=end].fill('X');
                }
            }
        }
        // print grid
        //for row in &grid {
        //    let line: String = row.iter().collect();
        //    println!("{}", line);
        //}

        // find biggest square which doenst
        // contain '.'
        let mut biggest = 0;
        for a in &matrix {
            for b in matrix.iter().skip(1) {
                // (column, row)
                //
                let candidate = square_poits(*a, *b);
                // 2,5 11,1
                let row_range = a.1.min(b.1)..=a.1.max(b.1);
                let col_range = a.0.min(b.0)..=a.0.max(b.0);
                let candidate_is_filled = 'outer: {
                    for row_index in row_range.clone() {
                        //
                        for char_index in col_range.clone() {
                            //
                            let c = grid[row_index][char_index];
                            //println!("{}", c);
                            if c == '.' {
                                println!("break 'outer");
                                break 'outer false;
                            }
                        }
                    }
                    true
                };
                if candidate_is_filled {
                    //println!("is filled: {candidate_is_filled}");
                    biggest = max(biggest, candidate);
                    //println!("{} points a({},{}), b({},{})", biggest, a.0, a.1, b.0, b.1);
                }
            }
        }

        assert_eq!(TEST_SOLUTION_P2, biggest);
    }
}

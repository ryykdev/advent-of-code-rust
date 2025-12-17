use std::fs;

const YEAR: &str = "2025";
const DAY: &str = "07";

fn main() {
    println!("day{DAY}");

    let input = fs::read_to_string(format!("src/bin/{YEAR}day{DAY}.txt")).unwrap();

    // part 1
    println!(">> part 1: {}", part1(&input));

    // part 2
    println!(">> part 2: {}", part2(&input));
}

fn part1(input: &str) -> usize {
    let mut lines_as_chars: Vec<Vec<char>> = input
        .lines() // Use .lines() for rows, or .split_whitespace() for words
        .map(|line| line.chars().collect())
        .collect();

    // find S and and replace char under it with "|"
    let s_index = lines_as_chars
        .iter()
        .enumerate()
        .find_map(|(y, row)| row.iter().position(|&c| c == 'S').map(|x| (y, x)));
    let second_line = lines_as_chars.get_mut(1).unwrap();
    let c = second_line.get_mut(s_index.unwrap().1).unwrap();
    *c = '|';

    // if beam above beam splitter - split beam
    // if no beam splitter extend beam
    for line_index in 0..lines_as_chars.len() {
        let top_line_index = line_index;
        let bot_line_index = line_index + 1;
        if bot_line_index >= lines_as_chars.len() {
            break;
        }
        for char_index in 0..lines_as_chars.first().unwrap().len() {
            //
            //dbg!(line_index);
            //dbg!(&char_index);
            if lines_as_chars[top_line_index][char_index] == '|' {
                if lines_as_chars[bot_line_index][char_index] == '.' {
                    lines_as_chars[bot_line_index][char_index] = '|';
                } else if lines_as_chars[bot_line_index][char_index] == '^' {
                    lines_as_chars[bot_line_index][char_index - 1] = '|';
                    lines_as_chars[bot_line_index][char_index + 1] = '|';
                }
            }
            //
        }
    }
    //let strings: Vec<String> = lines_as_chars
    //    .into_iter()
    //    .map(|char_vec| char_vec.into_iter().collect())
    //    .collect();
    //dbg!(strings);

    // count beam-splits: |
    //                    ^
    let mut count = 0;
    for line_index in 0..lines_as_chars.len() {
        let top_line_index = line_index;
        let bot_line_index = line_index + 1;
        if bot_line_index >= lines_as_chars.len() {
            break;
        }
        for char_index in 0..lines_as_chars.first().unwrap().len() {
            //
            //dbg!(line_index);
            //dbg!(&char_index);
            if lines_as_chars[top_line_index][char_index] == '|'
                && lines_as_chars[bot_line_index][char_index] == '^'
            {
                count += 1;
            }
            //
        }
    }
    count
}

fn part2(input: &str) -> usize {
    123
}

#[cfg(test)]
mod tests {
    const TEST_INPUT: &str = r#".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
..............."#;
    const TEST_SOLUTION_P1: usize = 21;
    const TEST_SOLUTION_P2: usize = 0;

    #[test]
    fn test_solution_p1() {
        let mut lines_as_chars: Vec<Vec<char>> = TEST_INPUT
            .lines() // Use .lines() for rows, or .split_whitespace() for words
            .map(|line| line.chars().collect())
            .collect();

        // find S and and replace char under it with "|"
        let s_index = lines_as_chars
            .iter()
            .enumerate()
            .find_map(|(y, row)| row.iter().position(|&c| c == 'S').map(|x| (y, x)));
        let second_line = lines_as_chars.get_mut(1).unwrap();
        let c = second_line.get_mut(s_index.unwrap().1).unwrap();
        *c = '|';

        // if beam above beam splitter - split beam
        // if no beam splitter extend beam
        for line_index in 0..lines_as_chars.len() {
            let top_line_index = line_index;
            let bot_line_index = line_index + 1;
            if bot_line_index >= lines_as_chars.len() {
                break;
            }
            for char_index in 0..lines_as_chars.first().unwrap().len() {
                //
                //dbg!(line_index);
                //dbg!(&char_index);
                if lines_as_chars[top_line_index][char_index] == '|' {
                    if lines_as_chars[bot_line_index][char_index] == '.' {
                        lines_as_chars[bot_line_index][char_index] = '|';
                    } else if lines_as_chars[bot_line_index][char_index] == '^' {
                        lines_as_chars[bot_line_index][char_index - 1] = '|';
                        lines_as_chars[bot_line_index][char_index + 1] = '|';
                    }
                }
                //
            }
        }
        //let strings: Vec<String> = lines_as_chars
        //    .into_iter()
        //    .map(|char_vec| char_vec.into_iter().collect())
        //    .collect();
        //dbg!(strings);

        // count beam-splits: |
        let mut count = 0;
        for line_index in 0..lines_as_chars.len() {
            let top_line_index = line_index;
            let bot_line_index = line_index + 1;
            if bot_line_index >= lines_as_chars.len() {
                break;
            }
            for char_index in 0..lines_as_chars.first().unwrap().len() {
                //
                //dbg!(line_index);
                //dbg!(&char_index);
                if lines_as_chars[top_line_index][char_index] == '|'
                    && lines_as_chars[bot_line_index][char_index] == '^'
                {
                    count += 1;
                }
                //
            }
        }

        assert_eq!(TEST_SOLUTION_P1, count);
    }

    #[test]
    fn test_solution_p2() {
        let _vec_str = TEST_INPUT.split_whitespace();

        assert_eq!(TEST_SOLUTION_P2, 1);
    }
}

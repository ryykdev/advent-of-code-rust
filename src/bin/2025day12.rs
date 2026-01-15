use std::fs;

const YEAR: &str = "2025";
const DAY: &str = "12";

fn main() {
    println!(">> Advent of Code {YEAR} Day {DAY}");

    let input = fs::read_to_string(format!("src/bin/{YEAR}day{DAY}.txt")).unwrap();

    // part 1
    let result1 = part1(&input);
    println!(">> part 1: {result1}");

    // part 2
    let result2 = part2(&input);
    println!(">> part 2: {result2}");
}

fn part1(input: &str) -> usize {
    let mut blocks: Vec<&str> = input.split("\n\n").collect();
    let places = blocks.pop().unwrap();
    let mut shapes: Vec<&str> = Vec::new();
    for block in blocks {
        let (_, shape) = block.split_once(':').unwrap();
        shapes.push(shape);
    }

    let places_lines: Vec<&str> = places.lines().collect();
    let mut spaces_with_instructions: Vec<(&str, Vec<&str>)> = Vec::new();
    let mut instr_vec: Vec<&str> = Vec::new();
    for line in places_lines {
        let split: Vec<&str> = line.split_whitespace().collect();
        let (s, instructions) = split.split_first().unwrap();
        let space = s;
        instr_vec = instructions
            .iter()
            .flat_map(|s| s.split_whitespace())
            .collect();
        spaces_with_instructions.push((space, instr_vec));
    }

    // remove spaces that overflow with shapes
    let mut potential_spaces: Vec<(&str, Vec<&str>)> = Vec::new();
    for x in spaces_with_instructions {
        let mut string = String::new();
        let (space, instructions) = x;
        for i in 0..shapes.len() {
            let shape = shapes.get(i).unwrap();
            let times: usize = instructions.get(i).unwrap().parse().unwrap();
            for i in 0..times {
                string.push_str(shape);
            }
        }
        // count # in string and drop space where no_hashes > board
        let hash_count = string.chars().filter(|&c| c == '#').count();
        let (a, _) = space.split_once(":").unwrap();
        let (a, b) = a.split_once("x").unwrap();
        let a: usize = a.parse().unwrap();
        let b: usize = b.parse().unwrap();
        if hash_count < (a * b) {
            potential_spaces.push((space, instructions));
        }
    }

    println!("potential spaces: {}", potential_spaces.len());

    // this solution works, I just checked if the presents
    // could potentially fit if they are aranged perfectly
    // counting the '#'s and comparing that to the grid.
    potential_spaces.len()
}

fn part2(input: &str) -> usize {
    todo!()
}

struct Shapes {
    variants: Vec<u16>,
}

struct Board {
    cells: Vec<bool>,
    height: usize,
    width: usize,
}

fn fit_shapes(shapes: Vec<&str>, instructions: (&str, Vec<&str>)) -> bool {
    //
    //

    false
}

#[cfg(test)]
mod tests {
    const TEST_INPUT: &str = r#"0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
12x5: 1 0 1 0 3 2"#;
    const TEST_SOLUTION_P1: usize = 0;
    const TEST_SOLUTION_P2: usize = 0;

    #[test]
    fn test_solution_p1() {
        let mut blocks: Vec<&str> = TEST_INPUT.split("\n\n").collect();
        let places = blocks.pop().unwrap();
        let mut shapes: Vec<&str> = Vec::new();
        for block in blocks {
            let (_, shape) = block.split_once(':').unwrap();
            shapes.push(shape);
        }
        dbg!(&shapes);

        let places_lines: Vec<&str> = places.lines().collect();
        let mut spaces_with_instructions: Vec<(&str, Vec<&str>)> = Vec::new();
        let mut instr_vec: Vec<&str> = Vec::new();
        for line in places_lines {
            let split: Vec<&str> = line.split_whitespace().collect();
            let (s, instructions) = split.split_first().unwrap();
            let space = s;
            instr_vec = instructions
                .iter()
                .flat_map(|s| s.split_whitespace())
                .collect();
            spaces_with_instructions.push((space, instr_vec));
        }

        dbg!(&spaces_with_instructions);

        // remove spaces that overflow with shapes
        for x in spaces_with_instructions {
            let mut string = String::new();
            let (space, instructions) = x;
            for i in 0..shapes.len() {
                let shape = shapes.get(i).unwrap();
                let times: usize = instructions.get(i).unwrap().parse().unwrap();
                for i in 0..times {
                    string.push_str(shape);
                }
            }
            // count # in string and drop space where no_hashes > board
            let hash_count = string.chars().filter(|&c| c == '#').count();
            let (a, b) = space.split_once("x").unwrap();
            let a: usize = a.parse().unwrap();
            let b: usize = b.parse().unwrap();
            if hash_count > (a * b) {
                panic!("hash_count > space");
            }
        }

        // tetris-like algorithm goes here

        assert_eq!(TEST_SOLUTION_P1, 123);
    }

    #[test]
    fn test_solution_p2() {
        let _vec_str = TEST_INPUT.split_whitespace();

        assert_eq!(TEST_SOLUTION_P2, 123);
    }
}

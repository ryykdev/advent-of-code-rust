use std::{collections::HashMap, fs};

const YEAR: &str = "2025";
const DAY: &str = "11";

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
    let lines = input.lines();
    let mut nodes: HashMap<&str, Vec<&str>> = HashMap::new();
    for line in lines {
        //
        let (origin, edges) = line.split_once(':').unwrap();
        let vec_edges: Vec<&str> = edges.split_whitespace().collect();
        nodes.insert(origin, vec_edges);
    }

    let current: String = "you".to_string();
    let target: &str = "out";
    let map: &HashMap<&str, Vec<&str>> = &nodes;
    let current_path: &mut Vec<String> = &mut Vec::new();
    let all_paths: &mut Vec<Vec<String>> = &mut Vec::new();
    solve(current, target, map, current_path, all_paths);

    all_paths.len()
}

fn part2(input: &str) -> usize {
    todo!()
}

// recursive solution
fn solve(
    current: String,
    target: &str,
    map: &HashMap<&str, Vec<&str>>,
    current_path: &mut Vec<String>,
    all_paths: &mut Vec<Vec<String>>,
) {
    current_path.push(current.clone());

    if current == target {
        all_paths.push(current_path.clone());
    } else {
        let edges = map.get(&*current).unwrap();
        for edge in edges {
            solve(edge.to_string(), target, map, current_path, all_paths);
        }
    }

    current_path.pop();
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::solve;

    const TEST_INPUT: &str = r#"aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out"#;
    const TEST_SOLUTION_P1: usize = 5;
    const TEST_SOLUTION_P2: usize = 0;

    #[test]
    fn test_solution_p1() {
        let lines = TEST_INPUT.lines();
        let mut nodes: HashMap<&str, Vec<&str>> = HashMap::new();
        for line in lines {
            //
            let (origin, edges) = line.split_once(':').unwrap();
            let vec_edges: Vec<&str> = edges.split_whitespace().collect();
            nodes.insert(origin, vec_edges);
        }

        let current: String = "you".to_string();
        let target: &str = "out";
        let map: &HashMap<&str, Vec<&str>> = &nodes;
        let current_path: &mut Vec<String> = &mut Vec::new();
        let all_paths: &mut Vec<Vec<String>> = &mut Vec::new();
        solve(current, target, map, current_path, all_paths);

        let number_of_paths = all_paths.len();

        assert_eq!(TEST_SOLUTION_P1, number_of_paths);
    }

    #[test]
    fn test_solution_p2() {
        let _vec_str = TEST_INPUT.split_whitespace();

        assert_eq!(TEST_SOLUTION_P2, 123);
    }
}

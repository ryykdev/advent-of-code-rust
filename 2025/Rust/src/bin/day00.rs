use std::fs;

static DAY: &str = "00";

fn main() {
    println!("day{}", DAY);
    let (result1, result2) = (0, 0);

    let input = fs::read_to_string(format!("../input/day{}.txt", DAY)).unwrap();
    let _split = input.split_whitespace();

    // part 1
    println!(">> part 1: {}", result1);

    // part 2
    println!(">> part 2: {}", result2);
}

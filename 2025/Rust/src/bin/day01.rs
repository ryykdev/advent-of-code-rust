use std::{fs, str::FromStr};

// day01
fn main() {
    println!("day01");
    let input = fs::read_to_string("../input/day01.txt").unwrap();
    let split = input.split_whitespace();

    // a Vec<Dails> for further processing
    let mut dials = Vec::new();
    for s in split {
        if let Ok(dial) = Dial::from_str(s) {
            dials.push(dial);
        }
    }

    // part 1
    // we count the number of times the dail (values 0..99)
    // stops at zero
    println!(">> part 1: {}", count_zeros(&dials));

    // part 2
    // we count every time the lock passes or stops at zero
    let mut lock = Lock::default();
    let many_zeros = lock.dial_counting_all_zeros(&dials);
    println!(">> part 2: {many_zeros}");
}

#[derive(PartialEq, Debug, Clone)]
enum Direction {
    L,
    R,
}

#[derive(PartialEq, Debug)]
struct Dial {
    direction: Direction,
    ticks: i32,
}

#[derive(Debug)]
pub struct ParseDialError;

impl FromStr for Dial {
    type Err = ParseDialError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() < 2 {
            return Err(ParseDialError);
        }

        let (dir_char, ticks_str) = s.split_at(1);

        let direction = match dir_char {
            "L" => Direction::L,
            "R" => Direction::R,
            _ => return Err(ParseDialError),
        };

        let ticks: i32 = ticks_str.parse().map_err(|_| ParseDialError)?;

        Ok(Dial { direction, ticks })
    }
}

struct Lock {
    capacity: i32,
    position: i32,
}

impl Default for Lock {
    fn default() -> Self {
        Lock {
            capacity: 100,
            position: 50,
        }
    }
}

impl Lock {
    fn dial(&mut self, dial: &Dial) -> i32 {
        let mut ticks = dial.ticks % self.capacity;
        if dial.direction == Direction::L {
            ticks *= -1;
        }

        let new_position = ((self.position + self.capacity) + ticks) % self.capacity;
        self.position = new_position;
        self.position
    }

    fn dial_counting_all_zeros(&mut self, dials: &Vec<Dial>) -> i32 {
        let mut count = 0;
        let mut lock = Lock::default();
        for dial in dials {
            for _d in 0..dial.ticks {
                lock.dial(&Dial {
                    direction: dial.direction.clone(),
                    ticks: 1,
                });
                if lock.position == 0 {
                    count += 1;
                }
            }
        }

        count
    }
}

fn count_zeros(dials: &Vec<Dial>) -> u32 {
    let mut count = 0;
    let mut lock = Lock::default();
    for dial in dials {
        if lock.dial(dial) == 0 {
            count += 1;
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    const TEST_INPUT: &str = r#"
L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
        "#;

    // test if the lock behaves well
    #[test]
    fn test_lock() {
        let mut lock = Lock::default();
        let dial_l50 = Dial {
            direction: Direction::L,
            ticks: 50,
        };
        let dial_l150 = Dial {
            direction: Direction::L,
            ticks: 150,
        };
        let dial_l55 = Dial {
            direction: Direction::L,
            ticks: 55,
        };
        let dial_r50 = Dial {
            direction: Direction::R,
            ticks: 50,
        };
        let dial_r150 = Dial {
            direction: Direction::R,
            ticks: 150,
        };

        assert_eq!(0, lock.dial(&dial_l50));
        assert_eq!(50, lock.dial(&dial_l150));
        assert_eq!(95, lock.dial(&dial_l55));
        assert_eq!(45, lock.dial(&dial_r50));
        assert_eq!(95, lock.dial(&dial_r150));

        let dials = vec![dial_l50, dial_l150, dial_l55, dial_r50, dial_r150];

        assert_eq!(1, count_zeros(&dials));
    }

    // test against the days test input
    // if this works it should also work for
    // the full input str. 
    #[test]
    fn test_short_probe() {
        let vec_str = TEST_INPUT.split_whitespace();

        let mut dials = Vec::new();
        for s in vec_str {
            if let Ok(dial) = Dial::from_str(s) {
                dials.push(dial);
            }
        }
        let count = count_zeros(&dials);
        assert_eq!(3, count);
    }
}

use std::fs;

const YEAR: &str = "2025";
const DAY: &str = "10";

fn main() {
    println!(">> running day{DAY}");

    let input = fs::read_to_string(format!("src/bin/{YEAR}day{DAY}.txt")).unwrap();

    // part 1
    let result1 = part1(&input);
    println!(">> part 1: {result1}");

    // part 2
    let result2 = part2(&input);
    println!(">> part 2: {result2}");
}

fn part1(input: &str) -> usize {
    let lines: Vec<&str> = input.lines().collect();
    // Vec<(lights, buttons, joltage)>
    let mut vec_tup: Vec<(&str, Vec<&str>, &str)> = Vec::new();
    for line in lines {
        //
        let mut vec_str: Vec<&str> = line.split_whitespace().collect();
        let lights = vec_str.remove(0);
        let joltage = vec_str.pop().unwrap();
        let buttons = vec_str;
        vec_tup.push((lights, buttons, joltage));
    }

    // find the button presses necessary for each line (tuple)
    let mut vec_but_presses: Vec<usize> = Vec::new();
    'outer: for tup in vec_tup {
        let lights = tup.0.trim_matches(|c| c == '[' || c == ']');
        let lights_len = lights.len();
        let buttons = tup.1;

        // layer 1 - 1 button presses
        for button in &buttons {
            let mut light_box = LightBox::with_size(lights_len);
            let light_config = light_box.push_button(button);
            if light_config == lights {
                vec_but_presses.push(1);
                continue 'outer;
            }
        }

        // layer 2 - 2 button presses
        for first in &buttons {
            for second in &buttons {
                let mut light_box = LightBox::with_size(lights_len);
                let light_config = light_box.push_button(first);
                let light_config = light_box.push_button(second);
                if light_config == lights {
                    vec_but_presses.push(2);
                    continue 'outer;
                }
            }
        }
        // layer 3 - 3 button presses
        for first in &buttons {
            for second in &buttons {
                for third in &buttons {
                    let mut light_box = LightBox::with_size(lights_len);
                    let light_config = light_box.push_button(first);
                    let light_config = light_box.push_button(second);
                    let light_config = light_box.push_button(third);
                    if light_config == lights {
                        vec_but_presses.push(3);
                        continue 'outer;
                    }
                }
            }
        }
        // layer 4
        for first in &buttons {
            for second in &buttons {
                for third in &buttons {
                    for fourth in &buttons {
                        let mut light_box = LightBox::with_size(lights_len);
                        let light_config = light_box.push_button(first);
                        let light_config = light_box.push_button(second);
                        let light_config = light_box.push_button(third);
                        let light_config = light_box.push_button(fourth);
                        if light_config == lights {
                            vec_but_presses.push(4);
                            continue 'outer;
                        }
                    }
                }
            }
        }
        // layer 5
        for first in &buttons {
            for second in &buttons {
                for third in &buttons {
                    for fourth in &buttons {
                        for fifth in &buttons {
                            let mut light_box = LightBox::with_size(lights_len);
                            let light_config = light_box.push_button(first);
                            let light_config = light_box.push_button(second);
                            let light_config = light_box.push_button(third);
                            let light_config = light_box.push_button(fourth);
                            let light_config = light_box.push_button(fifth);
                            if light_config == lights {
                                vec_but_presses.push(5);
                                continue 'outer;
                            }
                        }
                    }
                }
            }
        }
        // layer 6
        for first in &buttons {
            for second in &buttons {
                for third in &buttons {
                    for fourth in &buttons {
                        for fifth in &buttons {
                            for sixth in &buttons {
                                let mut light_box = LightBox::with_size(lights_len);
                                let light_config = light_box.push_button(first);
                                let light_config = light_box.push_button(second);
                                let light_config = light_box.push_button(third);
                                let light_config = light_box.push_button(fourth);
                                let light_config = light_box.push_button(fifth);
                                let light_config = light_box.push_button(sixth);
                                if light_config == lights {
                                    vec_but_presses.push(6);
                                    continue 'outer;
                                }
                            }
                        }
                    }
                }
            }
        }
        // layer 7
        for first in &buttons {
            for second in &buttons {
                for third in &buttons {
                    for fourth in &buttons {
                        for fifth in &buttons {
                            for sixth in &buttons {
                                for seventh in &buttons {
                                    let mut light_box = LightBox::with_size(lights_len);
                                    let light_config = light_box.push_button(first);
                                    let light_config = light_box.push_button(second);
                                    let light_config = light_box.push_button(third);
                                    let light_config = light_box.push_button(fourth);
                                    let light_config = light_box.push_button(fifth);
                                    let light_config = light_box.push_button(sixth);
                                    let light_config = light_box.push_button(seventh);
                                    if light_config == lights {
                                        vec_but_presses.push(7);
                                        continue 'outer;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        // layer 8
        for first in &buttons {
            for second in &buttons {
                for third in &buttons {
                    for fourth in &buttons {
                        for fifth in &buttons {
                            for sixth in &buttons {
                                for seventh in &buttons {
                                    for eigth in &buttons {
                                        let mut light_box = LightBox::with_size(lights_len);
                                        let light_config = light_box.push_button(first);
                                        let light_config = light_box.push_button(second);
                                        let light_config = light_box.push_button(third);
                                        let light_config = light_box.push_button(fourth);
                                        let light_config = light_box.push_button(fifth);
                                        let light_config = light_box.push_button(sixth);
                                        let light_config = light_box.push_button(seventh);
                                        let light_config = light_box.push_button(eigth);
                                        if light_config == lights {
                                            vec_but_presses.push(8);
                                            continue 'outer;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        // panic if more layers are needed
        panic!(">> ran past layers without result!");
    }
    vec_but_presses.iter().sum()
}

fn part2(input: &str) -> usize {
    todo!()
}

#[derive(Debug)]
struct LightBox {
    state: Vec<bool>,
}

impl LightBox {
    fn with_size(size: usize) -> Self {
        Self {
            state: vec![false; size],
        }
    }
    fn push_button(&mut self, button: &str) -> String {
        //
        let cleaned = button.replace("(", "").replace(")", "");
        let button: Vec<&str> = cleaned.split(',').collect();
        let button: Vec<u8> = button
            .iter()
            .map(|i| i.parse::<u8>().expect("expect sigle digit"))
            .collect();
        for index in button {
            //
            let light = self
                .state
                .get_mut(index as usize)
                .expect("mut_ref at index of state");
            *light = !*light;
        }
        // return pattern
        let mut s = String::new();
        for i in &self.state {
            if *i {
                s.push('#');
            } else {
                s.push('.');
            }
        }

        s
    }
}

#[cfg(test)]
mod tests {
    use std::panic;

    use crate::LightBox;

    const TEST_INPUT: &str = r#"[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}"#;
    const TEST_SOLUTION_P1: usize = 7;
    const TEST_SOLUTION_P2: usize = 0;

    #[test]
    fn test_solution_p1() {
        let lines: Vec<&str> = TEST_INPUT.lines().collect();
        // Vec<(lights, buttons, joltage)>
        let mut vec_tup: Vec<(&str, Vec<&str>, &str)> = Vec::new();
        for line in lines {
            //
            let mut vec_str: Vec<&str> = line.split_whitespace().collect();
            let lights = vec_str.remove(0);
            let joltage = vec_str.pop().unwrap();
            let buttons = vec_str;
            vec_tup.push((lights, buttons, joltage));
        }

        // find the button presses necessary for each line (tuple)
        let mut vec_but_presses: Vec<usize> = Vec::new();
        'outer: for tup in vec_tup {
            let lights = tup.0.trim_matches(|c| c == '[' || c == ']');
            let lights_len = lights.len();
            let buttons = tup.1;

            // layer 1 - 1 button presses
            for button in &buttons {
                let mut light_box = LightBox::with_size(lights_len);
                let light_config = light_box.push_button(button);
                if light_config == lights {
                    vec_but_presses.push(1);
                    continue 'outer;
                }
            }

            // layer 2 - 2 button presses
            for first in &buttons {
                for second in &buttons {
                    let mut light_box = LightBox::with_size(lights_len);
                    let light_config = light_box.push_button(first);
                    let light_config = light_box.push_button(second);
                    if light_config == lights {
                        vec_but_presses.push(2);
                        continue 'outer;
                    }
                }
            }
            // layer 3 - 3 button presses
            for first in &buttons {
                for second in &buttons {
                    for third in &buttons {
                        let mut light_box = LightBox::with_size(lights_len);
                        let light_config = light_box.push_button(first);
                        let light_config = light_box.push_button(second);
                        let light_config = light_box.push_button(third);
                        if light_config == lights {
                            vec_but_presses.push(3);
                            continue 'outer;
                        }
                    }
                }
            }
            // panic if more layers are needed
            panic!(">> ran past layer 3 without result!");
        }
        let solution = vec_but_presses.iter().sum();

        assert_eq!(TEST_SOLUTION_P1, solution);
    }

    #[test]
    fn test_solution_p2() {
        let _vec_str = TEST_INPUT.split_whitespace();

        assert_eq!(TEST_SOLUTION_P2, 123);
    }
}

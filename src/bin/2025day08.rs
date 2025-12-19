use std::fs;

const YEAR: &str = "2025";
const DAY: &str = "08";

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
    123
}

fn part2(input: &str) -> usize {
    123
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use glam::Vec3;

    const TEST_INPUT: &str = r#"162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689"#;
    const TEST_SOLUTION_P1: usize = 40;
    const TEST_SOLUTION_P2: usize = 0;

    #[test]
    fn test_solution_p1() {
        let lines = TEST_INPUT.lines();
        let mut vec_coord: Vec<Vec3> = Vec::new();
        for line in lines {
            let split: Vec<&str> = line.split(",").collect();
            let vec3 = Vec3 {
                x: split[0].trim().parse().unwrap(),
                y: split[1].trim().parse().unwrap(),
                z: split[2].trim().parse().unwrap(),
            };
            vec_coord.push(vec3);
        }

        let mut distances: Vec<(f32, Vec3, Vec3)> = Vec::new();
        for index in 0..vec_coord.len() - 1 {
            let p1 = vec_coord.get(index).unwrap();
            for index2 in index + 1..vec_coord.len() {
                let p2 = vec_coord.get(index2).unwrap();
                let dist = p1.distance(*p2);
                distances.push((dist, *p1, *p2));
            }
        }
        distances.sort_by(|a, b| a.0.total_cmp(&b.0));
        distances.truncate(20);
        // count number of novel circuits
        // discard if two points are already part of the same circuit
        //

        // count circuits
        let mut circuits: Vec<Vec<(f32, Vec3, Vec3)>> = Vec::new();
        let mut count_circuits = 0;
        while count_circuits < 10 {
            //
            let candidate = distances.pop().unwrap();
            for circuit in circuits.iter_mut() {
                for p in &mut *circuit {
                    if (candidate.1 == p.1 || candidate.1 == p.2)
                        && (candidate.2 == p.1 || candidate.2 == p.2)
                    {
                        break;
                    } else if candidate.1 == p.1
                        || candidate.1 == p.2
                        || candidate.2 == p.1
                        || candidate.2 == p.2
                    {
                        circuit.push(candidate);
                        count_circuits += 1;
                        println!("count {count_circuits}");
                        break;
                    }
                }
            }
            circuits.push(vec![candidate]);
            count_circuits += 1;
        }
        // look at circuits
        for circuit in &circuits {
            println!("circuit:");
            for p in circuit {
                println!(">> {} {} {}", p.0, p.1, p.2);
            }
        }

        let mut vec_len = Vec::new();
        for circuit in circuits {
            let mut points = HashSet::new();
            for p in circuit {
                points.insert(p.1.round().as_ivec3());
                points.insert(p.2.round().as_ivec3());
            }
            println!(">> {}", points.len());
            vec_len.push(points.len());
        }

        vec_len.sort();
        //vec_len.truncate(3);
        dbg!(vec_len);

        assert_eq!(TEST_SOLUTION_P1, 123);
    }

    #[test]
    fn test_solution_p2() {
        let _vec_str = TEST_INPUT.split_whitespace();

        assert_eq!(TEST_SOLUTION_P2, 123);
    }
}

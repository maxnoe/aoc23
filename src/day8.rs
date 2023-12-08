use crate::input::get_input;
use std::collections::HashMap;

enum Direction {
    L,
    R
}


struct Input<'a> {
    instructions: Vec<Direction>,
    network: HashMap<&'a str, (&'a str, &'a str)>,
}




fn parse_input(input: &str) -> Input {
    let (instructions, network) = input.split_once("\n\n").unwrap();
    let instructions = instructions.chars().map(|c| if c == 'L' {Direction::L} else {Direction::R}).collect();
    let mut res = Input{instructions, network: HashMap::new()};
    for line in network.lines() {
        let (from, to) = line.split_once(" = ").unwrap();
        let leftright = to.strip_prefix("(").unwrap().strip_suffix(")").unwrap().split_once(", ").unwrap();
        res.network.insert(from, leftright);

    }
    res
}


fn part1(input: &Input) -> i64 {
    let mut pos = "AAA";
    let mut instruction = 0;
    let mut steps = 0;
    while pos != "ZZZ" {
        pos = match input.instructions[instruction] {
            Direction::R => input.network[pos].1,
            Direction::L => input.network[pos].0,
                
        };
        steps += 1;
        instruction = (instruction + 1) % input.instructions.len();
    }
    steps
}

fn part2(input: &Input) -> i64 {
    let mut positions: Vec<&str> = input.network.keys().filter(|k| k.ends_with("A")).cloned().collect();
    let mut steps = 0;
    let mut instruction = 0;

    while !positions.iter().all(|p| p.ends_with("Z")) {
        for i in 0..positions.len() {
            positions[i] = match input.instructions[instruction] {
                Direction::R => input.network[positions[i]].1,
                Direction::L => input.network[positions[i]].0,
                    
            };
        }
        steps += 1;
        instruction = (instruction + 1) % input.instructions.len();
    }

    steps
}

pub fn day8() {
    let input = get_input(8, 2023).expect("Error getting input");

    let now = std::time::Instant::now();
    let input = parse_input(&input);
    let elapsed = now.elapsed();
    println!("Parsing input took {} µs",  elapsed.as_micros());

    let now = std::time::Instant::now();
    let answer1 = part1(&input);
    let elapsed = now.elapsed();
    println!("Part1: {} (in {} µs)", answer1, elapsed.as_micros());

    let now = std::time::Instant::now();
    let answer2 = part2(&input);
    let elapsed = now.elapsed();
    println!("Part2: {} (in {} µs)", answer2, elapsed.as_micros());
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day6_part1() {
        let input = parse_input("RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)");

        assert_eq!(part1(&input), 2);

        let input = parse_input("LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)");
        assert_eq!(part1(&input), 6);
    }

    #[test]
    fn test_day8_part2() {
        let input = parse_input("LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)
");

        assert_eq!(part2(&input), 6);
    }

}

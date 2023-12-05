use std::collections::HashMap;

use crate::input::get_input;

#[derive(Debug)]
struct Mapping {
    destination_start: i64,
    source_start: i64,
    length: i64,
}

#[derive(Debug)]
struct Map {
    mappings: Vec<Mapping>,
}

impl Map {
    fn get(&self, number: i64) -> i64 {
        for mapping in &self.mappings {
            if number >= mapping.source_start && number < mapping.source_start + mapping.length {
                let offset = number - mapping.source_start;
                return mapping.destination_start + offset;
            }
        }
        return number;
    }
}

#[derive(Debug)]
struct Input<'a> {
    seeds: Vec<i64>,
    mappings: HashMap<&'a str, (&'a str, Map)>,
}


fn parse_input(input: &str) -> Input {
    let mut iter = input.split("\n\n");

    let chunk = iter.next().unwrap();
    
    let seeds = chunk
        .split_once(": ").unwrap().1
        .split_whitespace().map(|n| n.parse().unwrap())
        .collect();
    let mut ret = Input{seeds, mappings: HashMap::new()};

    for chunk in iter {
        let mut lines = chunk.lines();
        let first = lines.next().unwrap();
        let (from, to) = first.split_once(" ").unwrap().0.split_once("-to-").unwrap();
        let mut mappings: Vec<Mapping> = Vec::new();
        for line in lines {
            let nums: Vec<i64> = line.split_whitespace().map(|n| n.parse().unwrap()).collect();
            mappings.push(Mapping{destination_start: nums[0], source_start: nums[1], length: nums[2]});
        }
        ret.mappings.insert(from, (to, Map{mappings}));

    }
    ret
}

fn location_number(seed: i64, input: &Input) -> i64 {
    let mut from = "seed";
    let mut number = seed;
    while from != "location" {
        let (to, mapping) = input.mappings.get(from).unwrap();
        number = mapping.get(number);
        from = to;
    }
    number
}


fn part1(input: &Input) -> i64 {
    input.seeds.iter().map(|s| location_number(*s, input)).min().unwrap()
}

fn part2(input: &Input) -> i64 {
    let mut min = i64::max_value();
    let pairs = input.seeds.len() / 2;
    for i in 0..pairs {
        println!("{} of {}", i + 1, pairs);
        let start = input.seeds[2 * i];
        let end = start + input.seeds[2 * i + 1];
        min = i64::min(min, (start..end).map(|s| location_number(s, input)).min().unwrap());
    }
    min
}

pub fn day5() {
    let input = get_input(5, 2023).expect("Error getting input");

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

    const TEST_INPUT: &'static str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
";


    #[test]
    fn test_day5_part1() {
        let input = parse_input(&TEST_INPUT);
        assert_eq!(part1(&input), 35);
    }

    #[test]
    fn test_day5_part2() {
        let input = parse_input(&TEST_INPUT);
        assert_eq!(part2(&input), 46);
    }

}


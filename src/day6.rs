use crate::input::get_input;


struct Race {
    time: i64,
    record: i64,
}

type Input = Vec<Race>;



fn parse_input1(input: &str) -> Input {
    let mut lines = input.lines();
    let parse_nums = |l: &str| l.split_once(": ").unwrap().1.split_whitespace().map(|n| n.parse().unwrap()).collect();
    let times: Vec<i64> = parse_nums(lines.next().unwrap());
    let records: Vec<i64> = parse_nums(lines.next().unwrap());

    times.iter().zip(records).map(|(time, record)| Race{time: *time, record}).collect()
}

fn parse_input2(input: &str) -> Race {
    let mut lines = input.lines();
    let parse_num = |l: &str| l.split_once(": ").unwrap().1.replace(" ", "").parse().unwrap();
    let time: i64 = parse_num(lines.next().unwrap());
    let record: i64 = parse_num(lines.next().unwrap());
    Race{time, record}
}


fn distance(total_time: i64, button_time: &i64) -> i64 {
    button_time * (total_time - button_time)
}

fn possible_wins(race: &Race) -> i64 {

    let first_larger = (0..race.time / 2 + 1).filter(|t| distance(race.time, t) > race.record).next().unwrap();
    race.time + 1 - 2 * first_larger
}


fn part1(input: &Input) -> i64 {
    input.iter().map(possible_wins).product()
}

fn part2(input: &Race) -> i64 {
    possible_wins(input)
}

pub fn day6() {
    let input = get_input(6, 2023).expect("Error getting input");

    let now = std::time::Instant::now();
    let input1 = parse_input1(&input);
    let input2 = parse_input2(&input);
    let elapsed = now.elapsed();
    println!("Parsing input took {} µs",  elapsed.as_micros());

    let now = std::time::Instant::now();
    let answer1 = part1(&input1);
    let elapsed = now.elapsed();
    println!("Part1: {} (in {} µs)", answer1, elapsed.as_micros());

    let now = std::time::Instant::now();
    let answer2 = part2(&input2);
    let elapsed = now.elapsed();
    println!("Part2: {} (in {} µs)", answer2, elapsed.as_micros());
}


#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &'static str = "Time:      7  15   30
Distance:  9  40  200
";

    #[test]
    fn test_day6_possible_wins() {
        assert_eq!(possible_wins(&Race{time: 7, record: 9}), 4);
        assert_eq!(possible_wins(&Race{time: 15, record: 40}), 8);
        assert_eq!(possible_wins(&Race{time: 30, record: 200}), 9);
    }

    #[test]
    fn test_day6_part1() {
        let input = parse_input1(&TEST_INPUT);
        assert_eq!(part1(&input), 288);
    }

    #[test]
    fn test_day6_part2() {
        let input = parse_input2(&TEST_INPUT);
        assert_eq!(part2(&input), 71503);
    }

}



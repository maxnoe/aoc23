use crate::input::get_input;
use std::collections::HashSet;

struct ScratchCard {
    id: i32,
    winning: i32,
}


fn parse_input(input: &str) -> Vec<ScratchCard> {
    input
        .lines()
        .map(|l| {
            let (card, numbers) = l.split_once(":").unwrap();
            let id: i32 = card.strip_prefix("Card").unwrap().trim().parse().unwrap();
            let (winning, own) = numbers.split_once("|").unwrap();
            let winning: HashSet<i32> = winning.trim().split_whitespace().map(|n| n.parse::<i32>().unwrap()).collect();
            let own: HashSet<i32> = own.trim().split_whitespace().map(|n| n.parse::<i32>().unwrap()).collect();

            let winning = winning.intersection(&own).count() as i32;
            ScratchCard{id, winning}
        })
        .collect()
     
}


fn part1(input: &Vec<ScratchCard>) -> i64 {
    input.iter()
        .map(|c| c.winning)
        .filter(|n| *n > 0)
        .map(|n| 2i64.pow(n as u32 - 1))
        .sum()
}

fn part2(input: &Vec<ScratchCard>) -> i64 {
    let mut total: i64 = 0;
    let mut stack: Vec<&ScratchCard> = input.iter().collect();

    while stack.len() > 0 {
        let card = stack.pop().unwrap();
        total += 1;
        for next in card.id as usize..(card.id + card.winning) as usize {
            if next < input.len().try_into().unwrap() {
                stack.push(&input[next]);
            }
        }
    }

    total
}

pub fn day4() {
    let input = get_input(4, 2023).expect("Error getting input");

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

    const TEST_INPUT: &'static str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
";


    #[test]
    fn test_day4_part1() {
        let input = parse_input(&TEST_INPUT);
        assert_eq!(part1(&input), 13);
    }

    #[test]
    fn test_day4_part2() {
        let input = parse_input(&TEST_INPUT);
        assert_eq!(part2(&input), 30);
    }
}

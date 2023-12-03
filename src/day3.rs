use crate::input::get_input;
use std::collections::HashMap;


fn is_part_number(chars: &Vec<Vec<char>> , row: usize, col: usize, length: usize) -> (bool, Option<(usize, usize)>) {
    // left
    let n_rows = chars.len();
    let n_cols = chars[0].len();

    if col > 0 && chars[row][col - 1] != '.' {
        let gear = if chars[row][col - 1] == '*' {
            Some((row, col - 1))
        } else {
            None
        };
        return (true, gear);
    }

    // right
    if col + length < n_cols && chars[row][col + length] != '.' {
        let gear = if chars[row][col + length] == '*' {
            Some((row, col + length))
        } else {
            None
        };
        return (true, gear);
    }

    let left = if col == 0 {0} else {col - 1};
    let right = std::cmp::min(col + length + 1, n_cols);
    // above
    if row > 0 {
        if let Some(col) = (left..right).filter(|col| chars[row - 1][*col] != '.').next() {
            let gear = if chars[row - 1][col] == '*' {
                Some((row - 1, col))
            } else {
                None
            };
            return (true, gear);
        }
    }
    // below
    if (row + 1) < n_rows {
        if let Some(col) = (left..right).filter(|col| chars[row + 1][*col] != '.').next() {
            let gear = if chars[row + 1][col] == '*' {
                Some((row + 1, col))
            } else {
                None
            };
            return (true, gear);
        }
    }
    (false, None)
}

#[derive(Debug)]
struct Plan {
    part_numbers: Vec<i64>,
    gears: HashMap<(usize, usize), Vec<i64>>,
}


fn parse_input(input: &str) -> Plan {

    let chars: Vec<Vec<char>> = input.lines()
        .map(|l| l.chars().collect())
        .collect();

    let n_rows = chars.len();
    let n_cols = chars[0].len();

    let mut plan = Plan{
        part_numbers: Vec::new(),
        gears: HashMap::new(),
    };

    for row in 0..n_rows {
        let mut parsing = false;
        let mut first = 0;

        for col in 0..n_cols {
            let is_digit = chars[row][col].is_digit(10);

            // if we reach a non-number or end-of-line, we have a whole number
            if parsing && (!is_digit || col + 1 == n_cols) {
                parsing = false;
                let length = if is_digit {col - first + 1} else {col - first};
                let (is_part, gear) = is_part_number(&chars, row, first, length);
                let number: i64 = chars[row][first..first+length].iter()
                    .collect::<String>()
                    .parse().unwrap();
                if !is_part {
                    continue;
                }
                plan.part_numbers.push(number);
                if let Some(gear) = gear {
                    if !plan.gears.contains_key(&gear) {
                        plan.gears.insert(gear, Vec::new());
                    }
                    plan.gears.get_mut(&gear).unwrap().push(number);
                }
            } else if !parsing && is_digit {
                parsing = true;
                first = col;
            }
        }
    }
    plan
}


fn part1(plan: &Plan) -> i64 {
    plan.part_numbers.iter().sum()
}

fn part2(input: &Plan) -> i64 {
    input.gears.values().filter(|v| v.len() == 2).map(|v| v[0] * v[1]).sum()
}

pub fn day3() {
    let input = get_input(3, 2023).expect("Error getting input");
    let input = parse_input(&input);
    println!("{:?}", input);

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

    const TEST_INPUT: &'static str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
";


    #[test]
    fn test_day3_part1() {
        let input = parse_input(TEST_INPUT);
        assert_eq!(part1(&input), 4361);
    }

    #[test]
    fn test_day3_part2() {
        let input = parse_input(TEST_INPUT);
        assert_eq!(part2(&input), 467835);
    }
}

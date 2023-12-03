use crate::input::get_input;


type Plan = Vec<Vec<char>>;

fn is_part_number(plan: &Plan , row: usize, col: usize, length: usize) -> bool{
    // left
    let n_rows = plan.len();
    let n_cols = plan[0].len();

    if col > 0 && plan[row][col - 1] != '.' {
        return true;
    }

    // right
    if col + length < n_cols && plan[row][col + length] != '.' {
        return true;
    }

    let left = if col == 0 {0} else {col - 1};
    let right = std::cmp::min(col + length + 1, n_cols);
    // above
    if row > 0 && plan[row - 1][left..right].iter().any(|c| !c.is_digit(10) && *c != '.') {
        return true;
    }
    // below
    if (row + 1) < n_rows && plan[row + 1][left..right].iter().any(|c| !c.is_digit(10) && *c != '.') {
        return true;
    }
    false
}


fn part1(input: &str) -> i64 {
    let plan: Plan = input.lines()
        .map(|l| l.chars().collect())
        .collect();

    let mut sum = 0;
    let n_rows = plan.len();
    let n_cols = plan[0].len();

    for row in 0..n_rows {
        let mut parsing = false;
        let mut first = 0;

        for col in 0..n_cols {
            let is_digit = plan[row][col].is_digit(10);

            // if we reach a non-number or end-of-line, we have a whole number
            if parsing && (!is_digit || col + 1 == n_cols) {
                parsing = false;
                let length = if is_digit {col - first + 1} else {col - first};
                if !is_part_number(&plan, row, first, length) {
                    continue;
                }
                let number: String = plan[row][first..first+length].iter().collect();
                sum += number.parse::<i64>().unwrap();
            } else if !parsing && is_digit {
                parsing = true;
                first = col;
            }
        }
    }

    return sum;
}

fn part2(input: &str) -> i64 {
    0
}

pub fn day3() {
    let input = get_input(3, 2023).expect("Error getting input");

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
        assert_eq!(part1(&TEST_INPUT), 4361);
    }
}

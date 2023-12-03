use crate::input::get_input;


#[derive(Debug,Eq,PartialEq)]
struct Draw {
    red: i64,
    green: i64,
    blue: i64,
}


#[derive(Debug,Eq,PartialEq)]
struct Game {
    id: i64,
    draws: Vec<Draw>,
}


fn parse_input(input: &str) -> Vec<Game> {
    let mut games : Vec<Game> = Vec::new();

    for line in input.lines() {
        let (id, rest) = line.split_once(":").expect("Invalid line");
        let id: i64 = id.replace("Game ", "").parse().unwrap();

        let mut draws: Vec<Draw> = Vec::new();
        for draw_str in rest.split(";") {
            let mut draw = Draw{red: 0, green: 0, blue: 0};
            for dice_str in draw_str.split(",") {
                let (number, color) = dice_str.trim().split_once(" ").unwrap();
                let number: i64 = number.parse().unwrap();
                match color {
                    "red" => draw.red = number,
                    "green" => draw.green = number,
                    "blue" => draw.blue = number,
                    _ => panic!("Unknown color")
                }
            }
            draws.push(draw);
        }
        games.push(Game{id, draws});
    }

    games
}
fn is_possible(game: &&Game) -> bool {
    for draw in &game.draws {
        if draw.red > 12 || draw.green > 13 || draw.blue > 14 {
            return false;
        }
    }
    true
}

fn part1(games: &Vec<Game>) -> i64 {
    games.iter().filter(is_possible).map(|g| {g.id}).sum()
}


fn power(game: &Game) -> i64 {
    let red = game.draws.iter().map(|d| {d.red}).max().unwrap();
    let green = game.draws.iter().map(|d| {d.green}).max().unwrap();
    let blue = game.draws.iter().map(|d| {d.blue}).max().unwrap();
    return red * green * blue;
}

fn part2(games: &Vec<Game>) -> i64 {
    games.iter().map(power).sum()
}

pub fn day2() {
    let input = get_input(2, 2023).expect("Error getting input");

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

    const TEST_INPUT: &'static str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
";


    #[test]
    fn test_day2_parse_input() {
        let input = parse_input(TEST_INPUT);
        for (i, game) in input.iter().enumerate() {
            assert_eq!(game.id, i as  i64 + 1);
        }
        assert_eq!(input[0].draws[0], Draw{red: 4, blue:3, green: 0});
        assert_eq!(input[1].draws[1], Draw{red: 1, blue:4, green: 3});
    }

    #[test]
    fn test_day2_part1() {
        let input = parse_input(TEST_INPUT);
        assert_eq!(part1(&input), 8);
    }

    #[test]
    fn test_day2_part2() {
        let input = parse_input(TEST_INPUT);
        assert_eq!(part2(&input), 2286);
    }
}

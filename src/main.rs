mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod input;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: aocmaxnoe2021 <day>");
        std::process::exit(1);
    }

    let day: u8 = args[1].parse().expect("Day must be a number");

    match day {
        1 => day1::day1(),
        2 => day2::day2(),
        3 => day3::day3(),
        4 => day4::day4(),
        5 => day5::day5(),
        6 => day6::day6(),
        _ => {
            println!("Day {} not yet implemented", day);
            std::process::exit(1);
        }
    }
}

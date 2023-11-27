use std::env;
use reqwest::blocking::Client;


pub fn get_input(day: i32, year: i32) -> Result<String, reqwest::Error> {
    let url = format!("https://adventofcode.com/{}/day/{}/input", year, day);

    let cookie: String = env::var("AOC_SESSION").expect("You need to set the AOC_SESSION env variable with your cookie");

    Client::new()
        .get(url)
        .header("Cookie", format!("session={}", cookie))
        .send()?
        .text()
}


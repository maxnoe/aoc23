use std::env;
use reqwest::blocking::Client;



pub fn get_input(day: i32, year: i32) -> Result<String, Box<dyn std::error::Error>> {
    let cookie: String = env::var("AOC_SESSION").expect("You need to set the AOC_SESSION env variable with your cookie");

    let url = format!("https://adventofcode.com/{}/day/{}/input", year, day);
    let xdg_dirs = xdg::BaseDirectories::with_prefix("aoc").unwrap();
    let cache_path = xdg_dirs.place_cache_file(format!("{}/input/{}/{}", year, day, &cookie[..16])).unwrap();

    if cache_path.exists() {
        return Ok(std::fs::read_to_string(cache_path)?);
    }



    let result = Client::new()
        .get(url)
        .header("Cookie", format!("session={}", cookie))
        .send()?
        .text()?;

    std::fs::write(cache_path, &result).expect("Error caching input");

    return Ok(result);

}


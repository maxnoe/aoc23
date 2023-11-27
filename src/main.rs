mod input;
use crate::input::get_input;

fn main() {
    let input = get_input(1, 2022).expect("Failed getting input");
    println!("{}", input);
}

mod days;
mod utils;

use days::day01 as d01;
use utils as u;

fn main() {
    let input = u::get_file_content("inputs/day01.txt");
    let result = d01::s2(input);
    println!("Result: {}", result);
}

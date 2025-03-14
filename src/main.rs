mod days;
mod utils;

use days::{day01 as d01, day02 as d02, day04 as d04};
use utils as u;

fn main() {
    let input = u::get_file_content("inputs/day04.txt");
    let result = d04::s2(input);
    println!("Result: {}", result);
}

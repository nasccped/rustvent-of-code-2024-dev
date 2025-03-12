mod days;
mod utils;

use days::{day01 as d01, day02 as d02};
use utils as u;

fn main() {
    let input = u::get_file_content("inputs/day02.txt");
    let result = d02::s1(input);
    println!("Result: {}", result);
}

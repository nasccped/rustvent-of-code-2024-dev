mod days;
mod utils;

use days::{
    day01 as d01, day02 as d02, day03 as d03, day04 as d04, day05 as d05, day06 as d06,
    day07 as d07,
};
use std::path::PathBuf;
use utils::InputFile;

/// Exit the program with an error code (`1`).
fn error_exit() -> ! {
    println!("Aborting...");
    std::process::exit(1)
}

type DayPair = (Option<fn(String) -> usize>, Option<fn(String) -> usize>);

fn main() {
    let solves: [DayPair; 1] = [
        (Some(d01::s1), Some(d01::s1)),
        //(Some(d02::s1), Some(d02::s2)),
        //(Some(d03::s1), Some(d03::s2)),
        //(Some(d04::s1), Some(d04::s2)),
        //(Some(d05::s1), Some(d05::s2)),
        //(Some(d06::s1), Some(d06::s2)),
        //(Some(d07::s1), Some(d07::s2)),
    ];
    let target_day: u8;
    let mut args = std::env::args().skip(1);
    match args.next() {
        None => {
            println!("\x1b[91merror:\x1b[0m no day number provided.");
            error_exit();
        }
        Some(x) if args.len() == 0 => match x.parse::<u8>() {
            Err(_) => {
                println!(
                    "\x1b[91merror:\x1b[0m `\x1b[96m{}\x1b[0m` isn't a valid day number.",
                    x
                );
                error_exit();
            }
            Ok(0) => {
                println!("\x1b[91merror:\x1b[0m day `0` isn't allowed.");
                error_exit();
            }
            Ok(d) => target_day = d,
        },
        Some(_) => {
            println!("\x1b[91merror:\x1b[0m passing more than 1 arg isn't valid.",);
            error_exit();
        }
    }
    let ifl = match InputFile::try_from((PathBuf::from("inputs"), target_day)) {
        Ok(x) => x,
        Err(e) => {
            println!(
                "\x1b[91merror:\x1b[0m couldn't open the file ({})",
                e.0.display()
            );
            error_exit();
        }
    };
    match solves.get(target_day as usize - 1) {
        Some(pair) => {
            println!("Solves for the \x1b[96mday {:02}\x1b[0m:", target_day);
        }
        None => {
            println!(
                "Solves for the \x1b[96mday {:02}\x1b[0m wasn't implemented yet.",
                target_day
            );
        }
    }
}

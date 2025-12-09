mod days;
mod utils;

use days::SOLVES;
use utils::InputFile;

/// Exit the program with an error code (`1`).
fn error_exit() -> ! {
    println!("Aborting...");
    std::process::exit(1)
}

fn main() {
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
    let ifl = match InputFile::try_from(("inputs", target_day)) {
        Ok(x) => x,
        Err(e) => {
            println!(
                "\x1b[91merror:\x1b[0m couldn't open the file ({})",
                e.0.display()
            );
            error_exit();
        }
    };
    match SOLVES.get(target_day as usize - 1) {
        Some(pair) => {
            println!("Solves for the \x1b[96mday {:02}\x1b[0m:", target_day);
            println!(
                "  \x1b[92mpart 1\x1b[0m: {}",
                pair.0
                    .map(|f| f(ifl.content.clone()).to_string())
                    .unwrap_or("not implemented yet".into())
            );
            println!(
                "  \x1b[92mpart 2\x1b[0m: {}",
                pair.1
                    .map(|f| f(ifl.content).to_string())
                    .unwrap_or("not implemented yet".into())
            );
        }
        None => {
            println!(
                "Solves for the \x1b[96mday {:02}\x1b[0m \x1b[91mwasn't implemented yet\x1b[0m.",
                target_day
            );
        }
    }
}

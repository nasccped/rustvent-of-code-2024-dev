mod days;
mod err_report;
mod inputs;
mod utils;

use days::SOLVES;
use err_report::ErrReport;
use inputs::INPUTS;

fn print_err_and_exit(message: impl Into<ErrReport>) -> ! {
    let e = message.into();
    e.print_and_exit()
}

const USAGE_TIP: &str = "Usage tip: `\x1b[92mcargo run -- \x1b[96m<DAY_NUMBER>\x1b[0m`";

fn main() {
    let target_day: usize;
    let mut args = std::env::args().skip(1);
    match args.next() {
        None => print_err_and_exit(("no day number provided", USAGE_TIP)),
        Some(x) if args.len() == 0 => match x.parse::<usize>() {
            Err(_) => print_err_and_exit((format!("`{}` isn't a valid day number", x), USAGE_TIP)),
            Ok(0) => print_err_and_exit("day `0` isn't allowed"),
            Ok(d) => target_day = d,
        },
        Some(_) => {
            print_err_and_exit((
                format!("invalid amount of args (`{}`)", args.len() + 1),
                USAGE_TIP,
            ));
        }
    }
    match (SOLVES.get(target_day - 1), INPUTS.get(target_day - 1)) {
        (None, _) => {
            print_err_and_exit((
                format!("the day `{}` isn't available", target_day),
                "This occurs when passing a target day greater\n\
                 than 25 (or when the solve wasn't done yet).",
            ));
        }
        (_, None) => {
            print_err_and_exit((
                format!("the input for day `{}` isn't available", target_day),
                "This occurs when the input file wasn't added yet.",
            ));
        }
        (Some(pair), Some(input)) => {
            println!("Solves for the \x1b[96mday {:02}\x1b[0m:", target_day);
            println!(
                "  - part 1: \x1b[92m{}\x1b[0m",
                pair.0
                    .map(|func| func(input).to_string())
                    .unwrap_or("\x1b[91mnot implemented yet\x1b[0m".into())
            );
            println!(
                "  - part 2: \x1b[92m{}\x1b[0m",
                pair.1
                    .map(|func| func(input).to_string())
                    .unwrap_or("\x1b[91mnot implemented yet\x1b[0m".into())
            );
        }
    }
}

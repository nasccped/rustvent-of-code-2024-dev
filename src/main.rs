mod days;
mod temp;
mod utils;

use days::{day01 as d01, day02 as d02, day03 as d03, day04 as d04};
use temp::{DayMap, DayMapTrait, DaySolve};

fn main() {
    let solves: &[(DaySolve, DaySolve)] = &[
        (Some(d01::s1), Some(d01::s2)),
        (Some(d02::s1), Some(d02::s2)),
        (Some(d03::s1), Some(d03::s2)),
        (Some(d04::s1), Some(d04::s2)),
    ];

    let advent_map = DayMap::new_advent(solves);
    let (targ_day, targ_solve) = (3, 2);
    println!("Result for day {}, solve {}", targ_day, targ_solve);
    let result = advent_map.run(targ_day, targ_solve);
    println!(
        "Returned {}",
        if let Some(val) = result {
            val.to_string()
        } else {
            "None".to_string()
        }
    );
}

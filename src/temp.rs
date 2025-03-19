use std::collections::HashMap;
use std::path::Path;

use crate::utils as u;

fn file_exists(path: &str) -> bool {
    Path::new(path).exists()
}

type DayNumber = i32;
type DayInput = Vec<String>;
pub type DaySolve = Option<fn(Vec<String>) -> i64>;
pub type DayMap = HashMap<DayNumber, (DayInput, (DaySolve, DaySolve))>;
pub trait DayMapTrait {
    fn new_advent(solves: &[(DaySolve, DaySolve)]) -> DayMap;
    fn run(&self, target_day: DayNumber, target_solve: i32) -> Option<i64>;
}
impl DayMapTrait for DayMap {
    fn new_advent(solves: &[(DaySolve, DaySolve)]) -> Self {
        let mut store: DayMap = HashMap::new();

        for i in 1..=25 {
            if i > solves.len() {
                break;
            }

            let search_for = format!(
                "inputs/day{}.txt",
                match i {
                    v if v < 10 => format!("0{}", i),
                    v => v.to_string(),
                }
            );

            if !file_exists(&search_for) {
                continue;
            }

            store.insert(i as i32, (u::get_file_content(&search_for), solves[i - 1]));
        }
        store
    }

    fn run(&self, target_day: DayNumber, target_solve: i32) -> Option<i64> {
        if let Some(node) = self.get(&target_day) {
            let inpu: Vec<String> = node.0.clone();
            match (target_solve, node.1 .0, node.1 .1) {
                (x, ..) if !(1..=2).contains(&x) => {
                    println!("\x1b[1;31mInvalid target solve: {}\x1b[0m", x);
                }
                (1, Some(func), _) => {
                    return Some(func(inpu));
                }
                (2, _, Some(func)) => {
                    return Some(func(inpu));
                }
                (x, ..) => {
                    println!("\x1b[1;31mThe target solve ({}) is None!\x1b[0m", x);
                    println!("\x1b[1;31mProbably not implemented...\x1b[0m");
                }
            }
        } else {
            println!(
                "\x1b[1;31mTarget day search ({}) returned none!\x1b[0m",
                target_day
            );
            println!("\x1b[1;31mThe input file/given target probably doesn't exists\x1b[0m");
        }
        None
    }
}

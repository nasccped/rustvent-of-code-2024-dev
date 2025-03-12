fn report_is_safe(rep: &Vec<i32>) -> bool {
    let start_diff = rep[1] - rep[0];
    let mut cur_diff: i32;
    for i in 1..(rep.len()) {
        cur_diff = rep[i] - rep[i - 1];
        if !(1..=3).contains(&cur_diff.abs()) || (cur_diff * start_diff) < 0 {
            return false;
        }
    }
    true
}

// Day 02: Solve 1
pub fn s1(input: Vec<String>) -> i32 {
    input
        .iter()
        .map(|row| {
            row.split(" ")
                .map(|lvl| lvl.trim().parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .filter(|report| report_is_safe(report))
        .count() as i32
}

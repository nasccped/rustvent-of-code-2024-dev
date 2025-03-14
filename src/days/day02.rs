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

pub fn s1(input: Vec<String>) -> i32 {
    input
        .iter()
        .map(|row| {
            row.split_whitespace()
                .map(|lvl| lvl.trim().parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .filter(|report| report_is_safe(report))
        .count() as i32
}

pub fn s2(input: Vec<String>) -> i32 {
    input
        .iter()
        .map(|row| {
            row.split_whitespace()
                .map(|lvl| lvl.trim().parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .filter(|report| {
            report_is_safe(report) || {
                for i in 0..report.len() {
                    let left = &mut report[0..i].to_vec();
                    let right = &report[(i + 1)..];
                    left.extend(right);
                    if report_is_safe(left) {
                        return true;
                    }
                }
                return false;
            }
        })
        .count() as i32
}

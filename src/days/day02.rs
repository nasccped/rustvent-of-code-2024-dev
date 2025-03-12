// external func for this solve
fn report_is_safe(rep: &Vec<i32>) -> bool {
    // TODO: explanation >
    // it will be safe only if:
    //  1. all levels are decreasing / increasing
    //  2. the levels diff is between 1 - 3
    //  so...

    // take the start diff (important to detect decrease / increase)
    let start_diff = rep[1] - rep[0];
    // start an iter diff holder
    let mut cur_diff: i32;

    // iter through index 1 - last
    for i in 1..(rep.len()) {
        // take the current diff by subtracting the current - the previous
        cur_diff = rep[i] - rep[i - 1];
        // if diff outsides the limit (negat. 1 - 3 || posit. 1 - 3) or
        // if cur_diff * start_diff less than 0 (means them both haven't the same signal)
        if !(1..=3).contains(&cur_diff.abs()) || (cur_diff * start_diff) < 0 {
            return false;
        }
    }
    // if loop ends
    true
}

// Day 02: Solve 1
pub fn s1(input: Vec<String>) -> i32 {
    // TODO:
    //  1. split each row by whitespaces
    //  2. parse each lvl string to integer
    //  3. collect the row lvlv as report
    //  4. pass the report to the report test
    //  5. filter only safes
    //  6. get the lenght
    input
        .iter()
        .map(|row| {
            // NOTE: (1)
            row.split_whitespace()
                // NOTE: (2)
                .map(|lvl| lvl.trim().parse::<i32>().unwrap())
                // NOTE: (3)
                .collect::<Vec<i32>>()
        })
        // NOTE: (4, 5)
        .filter(|report| report_is_safe(report))
        // NOTE: (6)
        .count() as i32
}

// Day 02: Solve 2
pub fn s2(input: Vec<String>) -> i32 {
    // TODO:
    //  1. split each row by whitespaces
    //  2. parse each lvl string to integer
    //  3. collect the row lvl as report
    //  4. pass the report to the report test
    //  5. filter if safe else do test for each lvl removal
    //  6. get the lenght
    input
        .iter()
        .map(|row| {
            // NOTE: (1)
            row.split_whitespace()
                // NOTE: (2)
                .map(|lvl| lvl.trim().parse::<i32>().unwrap())
                // NOTE: (3)
                .collect::<Vec<i32>>()
        })
        // NOTE: (4)
        .filter(|report| {
            // NOTE: (5)
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
        // NOTE: (6)
        .count() as i32
}

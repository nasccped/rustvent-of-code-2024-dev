use std::ops::RangeInclusive;

const SAFE_RANGE: RangeInclusive<i8> = 1..=3;
const REPORT_SEPARATOR: &str = " ";

/// Test if a i8 slice (reference) is a saffe report
fn is_safe(arr: &[i8]) -> bool {
    // WARN:
    // this function was clipboarded from reddit:
    //    https://www.reddit.com/r/adventofcode/comments/1h4ncyr/2024_day_2_solutions/
    // I'm sorry about this, but it have a good solve approach :^D
    let (mut increasing, mut decreasing) = (true, true); // init with both inc. & dec. as true
    let (mut prev, mut curr): (&i8, &i8); // holds previous value and current value in our slice

    for i in 1..arr.len() {
        (prev, curr) = (&arr[i - 1], &arr[i]);

        if prev < curr {
            // if previous is smaller, the values are increasing, so
            decreasing = false;
        } else if prev > curr {
            // but if the previous is bigger...
            increasing = false;
        } else {
            // else (are the same value -> invalid)
            return false;
        }
        // test if abs diff is in safe range
        if !SAFE_RANGE.contains(&(prev - curr).abs()) {
            return false;
        }
    }

    increasing || decreasing
}

/// solve 1
pub fn s1(input: Vec<String>) -> i64 {
    input
        .into_iter() // for each row in our input
        .map(|row| {
            row.split(REPORT_SEPARATOR) // split it by the separator
                .map(|val| val.parse::<i8>().unwrap()) // parse each item as i8
                .collect::<Vec<_>>() // collect as vector
        })
        .filter(|nums| is_safe(nums)) // filter only safe reports
        .count() as i64 // return the lenght
}

/// solve 2
pub fn s2(input: Vec<String>) -> i64 {
    // used variables along the way:
    let mut accum = 0; // safe report count
    let mut nums: Vec<i8>; // nums (row)
    let mut len: usize; // nums len (preserve length when removing items)
    let mut removed: i8; // store removed item (maybe a safe report)

    // for each row in our input
    for row in input {
        // get nums (Vector of i8)
        nums = row
            .split(REPORT_SEPARATOR)
            .map(|val| val.parse::<i8>().unwrap())
            .collect();
        // if is safe, +1 to count and continue to next row
        if is_safe(&nums) {
            accum += 1;
            continue;
        }
        // else, take vec length
        len = nums.len();
        // for each index in lenght range
        for i in 0..len {
            // remove the value
            removed = nums.remove(i);
            // test if is safe
            if is_safe(&nums) {
                accum += 1;
                break;
            }
            // if isn't, insert value again and continue the loop
            nums.insert(i, removed);
        }
    }
    accum
}

#[cfg(test)]
mod tests {
    use super::{s1, s2};
    use crate::utils::get_file_content;
    use crate::utils::StrArrVecString;

    #[test]
    #[ignore]
    fn test1() {
        let input = [
            "7 6 4 2 1",
            "1 2 7 8 9",
            "9 7 6 2 1",
            "1 3 2 4 5",
            "8 6 4 4 1",
            "1 3 6 7 9",
        ]
        .into_vecstring();
        let result = s1(input);
        assert_eq!(result, 2);
    }

    #[test]
    fn test2() {
        let input = [
            "7 6 4 2 1",
            "1 2 7 8 9",
            "9 7 6 2 1",
            "1 3 2 4 5",
            "8 6 4 4 1",
            "1 3 6 7 9",
        ]
        .into_vecstring();
        let result = s2(input);
        assert_eq!(result, 4);
    }

    #[test]
    #[ignore]
    fn solve1() {
        let input = get_file_content("inputs/day02.txt");
        let result = s1(input);
        assert_eq!(result, 369);
    }

    #[test]
    fn solve2() {
        let input = get_file_content("inputs/day02.txt");
        let result = s2(input);
        assert_eq!(result, 428);
    }
}

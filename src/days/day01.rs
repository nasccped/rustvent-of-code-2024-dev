use crate::utils::VecStringTrim;
use std::collections::HashMap;

pub fn s1(input: Vec<String>) -> i64 {
    let input = input.foreach_trim();
    let (mut left, mut right): (Vec<i64>, Vec<i64>) = (vec![], vec![]);
    for row in input {
        let mut items = row
            .split("   ")
            .map(|item| item.parse::<i64>().unwrap())
            .into_iter();
        left.push(items.next().unwrap());
        right.push(items.next().unwrap());
    }
    left.sort();
    right.sort();
    left.iter()
        .zip(right.iter())
        .map(|(l, r)| (l - r).abs())
        .sum()
}

pub fn s2(input: Vec<String>) -> i64 {
    let input = input.foreach_trim();
    let mut left_map: HashMap<i64, (i64, i64)> = HashMap::new();
    let right_items: Vec<i64> = input
        .into_iter()
        .map(|row| {
            let mut pair = row
                .split("   ")
                .map(|item| item.parse::<i64>().unwrap())
                .into_iter();
            let first = pair.next().unwrap();
            if let Some((left_times, _)) = left_map.get_mut(&first) {
                *left_times += 1;
            } else {
                left_map.insert(first, (1, 0));
            }
            pair.next().unwrap()
        })
        .collect();

    for item in right_items.iter() {
        if let Some((_, r_times)) = left_map.get_mut(item) {
            *r_times += 1;
        }
    }
    left_map.iter().map(|(k, v)| k * v.0 * v.1).sum()
}

#[cfg(test)]
mod day01_tests {
    use super::{s1, s2};
    use crate::utils::get_file_content;
    use crate::utils::StrArrVecString;

    #[test]
    fn solve1_test() {
        let input = ["3   4", "4   3", "2   5", "1   3", "3   9", "3   3"].into_vecstring();
        let result = s1(input);
        assert_eq!(result, 11);
    }

    #[test]
    fn solve2_test() {
        let input = ["3   4", "4   3", "2   5", "1   3", "3   9", "3   3"].into_vecstring();
        let result = s2(input);
        assert_eq!(result, 31);
    }

    #[test]
    fn solve1_run() {
        let input = get_file_content("inputs/day01.txt");
        let result = s1(input);
        assert_eq!(result, 1579939);
    }

    #[test]
    fn solve2_run() {
        let input = get_file_content("inputs/day01.txt");
        let result = s2(input);
        assert_eq!(result, 20351745);
    }
}

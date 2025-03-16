use crate::utils::VecStringTrim;

fn report_is_safe(rep: &Vec<i32>) -> bool {
    let previous: Vec<&i32> = rep.iter().take(rep.len() - 1).collect();
    let next: Vec<&i32> = rep.iter().skip(1).collect();
    if !(previous.iter().zip(next.iter()).all(|(p, n)| p < n)
        || previous
            .into_iter()
            .zip(next.into_iter())
            .all(|(p, n)| p > n))
    {
        return false;
    }
    let mut rep = rep.clone();
    rep.sort();
    for i in 1..(rep.len()) {
        if !(1..=3).contains(&(rep[i] - rep[i - 1])) {
            return false;
        }
    }
    true
}

pub fn s1(input: Vec<String>) -> i32 {
    let input = input.foreach_trim();
    input
        .iter()
        .map(|row| {
            row.split_whitespace()
                .map(|lvl| lvl.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .filter(|report| report_is_safe(report))
        .count() as i32
}

pub fn s2(input: Vec<String>) -> i32 {
    let input = input.foreach_trim();
    input
        .iter()
        .map(|row| {
            row.split_whitespace()
                .map(|lvl| lvl.parse::<i32>().unwrap())
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

#[cfg(test)]
mod day02_tests {
    use super::{s1, s2};
    use crate::utils::get_file_content;
    use crate::utils::StrArrVecString;

    #[test]
    fn solve1_test() {
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
    fn solve2_test() {
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
    fn solve1_run() {
        let input = get_file_content("inputs/day02.txt");
        let result = s1(input);
        assert_eq!(result, 369);
    }

    #[test]
    fn solve2_run() {
        let input = get_file_content("inputs/day02.txt");
        let result = s2(input);
        assert_eq!(result, 428);
    }
}

use crate::utils::VecStringTrim;
use itertools::Itertools;

#[derive(Copy, Clone, Debug)]
enum OperationType {
    Add,
    Mul,
    Con,
}

pub fn s1(input: Vec<String>) -> i64 {
    let input = input.foreach_trim();
    let oper_types = vec![OperationType::Add, OperationType::Mul];
    let mut accum: i64 = 0;

    for row in input {
        let mut row_vals = row.split(":");
        let total: i64 = row_vals.next().unwrap().parse().unwrap();
        let samples: Vec<i64> = row_vals
            .next()
            .unwrap()
            .split_whitespace()
            .map(|v| v.parse().unwrap())
            .collect();
        let possibilities = (0..samples.len())
            .map(|_| &oper_types)
            .multi_cartesian_product();
        for pos in possibilities {
            let mut sampliter = samples.iter();
            let start = sampliter.next().unwrap();
            let res = pos.iter().zip(sampliter).fold(*start, |a, (o, v)| match o {
                OperationType::Add => a + v,
                _ => a * v,
            });

            if res == total {
                accum += total;
                break;
            }
        }
    }
    accum as i64
}

pub fn s2(input: Vec<String>) -> i64 {
    let input = input.foreach_trim();
    let oper_types = vec![OperationType::Add, OperationType::Mul, OperationType::Con];
    let mut accum: i64 = 0;

    for row in input.iter() {
        let mut row_vals = row.split(":");
        let total: i64 = row_vals.next().unwrap().parse().unwrap();
        let samples: Vec<i64> = row_vals
            .next()
            .unwrap()
            .split_whitespace()
            .map(|v| v.parse().unwrap())
            .collect();
        let possibilities = (0..samples.len())
            .map(|_| &oper_types)
            .multi_cartesian_product();
        for pos in possibilities.into_iter() {
            let mut sampliter = samples.iter();
            let start = sampliter.next().unwrap();
            let res = pos.iter().zip(sampliter).fold(*start, |a, (o, v)| match o {
                OperationType::Add => a + v,
                OperationType::Mul => a * v,
                OperationType::Con => {
                    let mut as_str = a.to_string();
                    as_str.push_str(&(v.to_string()));
                    as_str.parse::<i64>().unwrap()
                }
            });

            if res == total {
                accum += total;
                break;
            }
        }
    }
    accum as i64
}

#[cfg(test)]
mod day07_tests {
    use super::{s1, s2};
    use crate::utils::get_file_content;
    use crate::utils::StrArrVecString;

    #[test]
    fn solve1_test() {
        let input = [
            "190: 10 19",
            "3267: 81 40 27",
            "83: 17 5",
            "156: 15 6",
            "7290: 6 8 6 15",
            "161011: 16 10 13",
            "192: 17 8 14",
            "21037: 9 7 18 13",
            "292: 11 6 16 20",
        ]
        .into_vecstring();
        let result = s1(input);
        assert_eq!(result, 3749);
    }

    #[test]
    fn solve2_test() {
        let input = [
            "190: 10 19",
            "3267: 81 40 27",
            "83: 17 5",
            "156: 15 6",
            "7290: 6 8 6 15",
            "161011: 16 10 13",
            "192: 17 8 14",
            "21037: 9 7 18 13",
            "292: 11 6 16 20",
        ]
        .into_vecstring();
        let result = s2(input);
        assert_eq!(result, 11387);
    }

    #[test]
    fn solve1_run() {
        let input = get_file_content("inputs/day07.txt");
        let result = s1(input);
        assert_eq!(result as i64, 663613490587);
    }

    #[test]
    fn solve2_run() {
        let input = get_file_content("inputs/day07.txt");
        let result = s2(input);
        assert_eq!(result as i64, 110365987435001);
    }
}

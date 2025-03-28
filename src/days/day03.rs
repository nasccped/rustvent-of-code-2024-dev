use regex::Regex;

const ONLY_MULS_REGEX: &str = r"mul\([0-9]+,[0-9]+\)";
const MULS_DO_DONTS_REGEX: &str = r"do\(\)|mul\([0-9]+,[0-9]+\)|don't\(\)";
const NUM_REGEX: &str = r"[0-9]+";

struct PatternStruct {
    self_value: String,
}

impl PatternStruct {
    fn from_str<'a>(input: &'a str) -> Self {
        Self {
            self_value: input.to_string(),
        }
    }

    fn is_do(&self) -> bool {
        self.self_value == "do()"
    }

    fn is_dont(&self) -> bool {
        self.self_value == "don't()"
    }

    fn get_mul_value(&self) -> i64 {
        if self.is_do() || self.is_dont() {
            0
        } else {
            self.find_self_values().into_iter().product()
        }
    }

    fn find_self_values(&self) -> Vec<i64> {
        let num_finder = Regex::new(NUM_REGEX).unwrap();

        num_finder
            .find_iter(&self.self_value)
            .map(|val| val.as_str().parse().unwrap())
            .collect()
    }
}

trait PatternStructEval {
    fn evaluate_from_vec(&self, dont_flag: &mut bool) -> i64;
}

impl PatternStructEval for Vec<PatternStruct> {
    fn evaluate_from_vec(&self, dont_flag: &mut bool) -> i64 {
        let mut accum = 0;

        for pattern in self {
            if pattern.is_dont() {
                *dont_flag = true;
                continue;
            }
            if pattern.is_do() {
                *dont_flag = false;
                continue;
            }
            if *dont_flag {
                continue;
            }
            accum += pattern.get_mul_value();
        }
        accum
    }
}

pub fn s1(input: Vec<String>) -> i64 {
    let mul_pattern = Regex::new(ONLY_MULS_REGEX).unwrap();
    let mut accum: i64 = 0;

    for row in input.iter() {
        let vec_muls: Vec<PatternStruct> = mul_pattern
            .find_iter(row)
            .map(|m| PatternStruct::from_str(m.as_str()))
            .collect();

        accum += vec_muls.evaluate_from_vec(&mut false);
    }
    accum
}

pub fn s2(input: Vec<String>) -> i64 {
    let pattern = Regex::new(MULS_DO_DONTS_REGEX).unwrap();
    let mut dont_flag = false;
    let mut accum = 0;

    for row in input.iter() {
        let pattern_vec: Vec<PatternStruct> = pattern
            .find_iter(row)
            .map(|p| PatternStruct::from_str(p.as_str()))
            .collect();
        accum += pattern_vec.evaluate_from_vec(&mut dont_flag)
    }

    accum
}

#[cfg(test)]
mod day03_tests {
    use super::{s1, s2};
    use crate::utils::get_file_content;
    use crate::utils::StrArrVecString;

    #[test]
    fn solve1_test() {
        let input = ["xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"]
            .into_vecstring();
        let result = s1(input);
        assert_eq!(result, 161);
    }

    #[test]
    fn solve2_test() {
        let input = ["xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"]
            .into_vecstring();
        let result = s2(input);
        assert_eq!(result, 48);
    }

    #[test]
    fn solve1_run() {
        let input = get_file_content("inputs/day03.txt");
        let result = s1(input);
        assert_eq!(result, 184511516);
    }

    #[test]
    fn solve2_run() {
        let input = get_file_content("inputs/day03.txt");
        let result = s2(input);
        assert_eq!(result, 90044227);
    }
}

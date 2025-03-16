use regex::Regex;

pub fn s1(input: Vec<String>) -> i32 {
    let mulre = Regex::new(r"mul\([0-9]+,[0-9]+\)").unwrap();
    let numre = Regex::new(r"[0-9]+").unwrap();
    let mut accum = 0;

    for row in input.iter() {
        let muls: Vec<&str> = mulre.find_iter(row).map(|m| m.as_str()).collect();
        accum += muls
            .iter()
            .map(|m| {
                numre
                    .find_iter(m)
                    .map(|val| val.as_str().parse::<i32>().unwrap())
                    .product::<i32>()
            })
            .sum::<i32>()
    }
    accum
}

pub fn s2(input: Vec<String>) -> i32 {
    let mulre = Regex::new(r"do\(\)|mul\([0-9]+,[0-9]+\)|don't\(\)").unwrap();
    let numre = Regex::new(r"[0-9]+").unwrap();
    let mut dont_called = false;
    let mut accum = 0;

    for row in input.iter() {
        let muls: Vec<&str> = mulre.find_iter(row).map(|m| m.as_str()).collect();

        for m in muls {
            if m == "don't()" {
                dont_called = true;
                continue;
            }
            if m == "do()" {
                dont_called = false;
                continue;
            }
            if dont_called {
                continue;
            }
            accum += numre
                .find_iter(m)
                .map(|val| val.as_str().parse::<i32>().unwrap())
                .product::<i32>();
        }
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

use regex::Regex;

pub fn s1(input: &str) -> usize {
    let pattern = Regex::new(r"mul\([0-9]+,[0-9]+\)").unwrap();
    let num_pattern = Regex::new(r"[0-9]+").unwrap();
    let mut accum = 0;
    for line in input.lines() {
        accum += pattern
            .find_iter(line)
            .map(|matching| {
                num_pattern
                    .find_iter(matching.as_str())
                    .map(|val| val.as_str().parse::<usize>().unwrap())
                    .product::<usize>()
            })
            .sum::<usize>();
    }
    accum
}

pub fn s2(input: &str) -> usize {
    let pattern = Regex::new(r"do\(\)|mul\([0-9]+,[0-9]+\)|don't\(\)").unwrap();
    let num_pattern = Regex::new(r"[0-9]+").unwrap();
    let mut accum = 0;
    let mut enabled = true;
    for line in input.lines() {
        for matching in pattern.find_iter(line) {
            match matching.as_str() {
                "do()" => enabled = true,
                "don't()" => enabled = false,
                x if enabled => {
                    accum += num_pattern
                        .find_iter(x)
                        .map(|val| val.as_str().parse::<usize>().unwrap())
                        .product::<usize>()
                }
                _ => {}
            }
        }
    }
    accum
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::inputs::INPUTS;

    const INPUT1: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    const INPUT2: &str =
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn part1() {
        assert_eq!(s1(INPUT1), 161);
        assert_eq!(s1(INPUTS[2]), 184511516)
    }

    #[test]
    fn part2() {
        assert_eq!(s2(INPUT2), 48);
        assert_eq!(s2(INPUTS[2]), 90044227);
    }
}

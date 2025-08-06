use regex::Regex;

/// Accumulator helper.
///
/// It will take a `mul(<integer>,<integer>)` sentence and evaluate the result.
/// Then, add it to an accumulator field.
///
/// It also have turn off / turn on features (required to part 2 solve).
struct MulAccumulator {
    accumulator: i64,
    add_mul: bool,
    num_regex: Regex,
}

impl MulAccumulator {
    fn new() -> Self {
        Self {
            accumulator: 0,
            add_mul: true,
            num_regex: Regex::new(r"[0-9]+").unwrap(),
        }
    }

    /// Turn off / on the counter increment.
    fn set_add_mul(&mut self, new_value: bool) {
        self.add_mul = new_value;
    }

    /// Evaluate and increment the evaluated `mul` sentence (ONLY IF increment
    /// is turned on).
    fn increment_with_mul(&mut self, mul: &str) {
        if !self.add_mul {
            return;
        }
        self.accumulator += self
            .num_regex
            .find_iter(mul)
            .map(|val| val.as_str().parse::<i64>().unwrap())
            .product::<i64>();
    }
}

/// # Solve for challenge 1 at day 3:
///
/// We should iter through a bunch of rows and found all
/// `mul(<integer>,<integer>)` occurences. To do so, we can use regex matching
/// with the Regex crate.
///
/// Then, we need to:
///
/// - create an accumulator
/// - for all `row`s:
///     - for all `mul`s in this row:
///         - multiple the inner integers
///         - add the result to the accumulator
/// - return the accumulator
pub fn s1(input: &str) -> i64 {
    let mul_pattern = Regex::new(r"mul\([0-9]+,[0-9]+\)").unwrap();
    input
        .lines()
        .map(|row| mul_pattern.find_iter(row).map(|mtc| mtc.as_str()))
        .fold(MulAccumulator::new(), |mut mulacc, muls| {
            muls.for_each(|m| mulacc.increment_with_mul(m));
            mulacc
        })
        .accumulator
}

/// # Solve for challenge 1 at day 3:
///
/// We should iter through a bunch of rows and found all `do()`,
/// `mul(<integer>,<integer>)` and `don't()` occurences. To do so, we can use
/// regex matching with the Regex crate.
///
/// Then, we need to:
///
/// - create an accumulator (with turn on/turn off features)
/// - for all `row`s:
///     - for all <TARGET OCCURENCE>s in this row:
///         - if it's `don't()`, turn off the increment accumulator
///         - if it's `do()`, turn on the increment accumulator
///         - if it's `mul(<integer>,<integer>)` and increment is enabled
///             - multiple the inner integers
///             - add the result to the accumulator
/// - return the accumulator
pub fn s2(input: &str) -> i64 {
    let mul_pattern = Regex::new(r"do\(\)|mul\([0-9]+,[0-9]+\)|don't\(\)").unwrap();
    input
        .lines()
        .map(|row| mul_pattern.find_iter(row).map(|mtc| mtc.as_str()))
        .fold(MulAccumulator::new(), |mut mulacc, muls| {
            muls.for_each(|m| match m {
                "don't()" => mulacc.set_add_mul(false),
                "do()" => mulacc.set_add_mul(true),
                x => mulacc.increment_with_mul(x),
            });
            mulacc
        })
        .accumulator
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../../inputs/day03.txt");
    const SAMPLE_1: &str =
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    const SAMPLE_2: &str =
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn solve1_test() {
        assert_eq!(s1(SAMPLE_1), 161);
        assert_eq!(s1(INPUT), 184511516)
    }

    #[test]
    fn solve2_test() {
        assert_eq!(s2(SAMPLE_2), 48);
        assert_eq!(s2(INPUT), 90044227);
    }
}

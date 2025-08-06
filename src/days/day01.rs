use std::collections::HashMap;

/// # Solve for challenge 1 at day 1:
///
/// Our input is a list of integer pairs. We should separate our input into
/// elements at left and elements at right. Then, sort both and sum the
/// distance between each element.
pub fn s1(input: &str) -> i64 {
    let lines = input.lines();
    let (mut left, mut right): (Vec<i64>, Vec<i64>) = lines.map(|row| row.split("   ")).fold(
        (Vec::new(), Vec::new()),
        |(mut l, mut r), mut pair| {
            let mut next = || pair.next().unwrap().parse().unwrap();
            l.push(next());
            r.push(next());
            (l, r)
        },
    );
    left.sort();
    right.sort();
    left.iter()
        .zip(right.iter())
        .map(|(l, r)| (l - r).abs())
        .sum()
}

/// # Solve for challenge 2 at day 1:
///
/// Our input is a list of integer pairs. We should separate our input into
/// elements at left and elements at right. Then, for each element in left
/// list, we should count how many times it appears in right list, multiply the
/// left value by the times, and finally sum all the thing to get the result.
pub fn s2(input: &str) -> i64 {
    let lines = input.lines();
    // NOTE: we'll store it in a HashMap. It's a little complex but pretty
    // faster than doing nested for loops. The Key-Value pair will be `&str`
    // and (i64, i64) since the number shouldn't be parsed if it doesn't appear
    // in left or right list + left list can hold duplicates.
    let mapping = lines.map(|row| row.split("   ")).fold(
        HashMap::<&str, (i64, i64)>::new(),
        |mut map, mut pair| {
            let mut next = || pair.next().unwrap();
            map.entry(next())
                .and_modify(|v| v.0 += 1)
                .or_insert_with(|| (1, 0));
            map.entry(next())
                .and_modify(|v| v.1 += 1)
                .or_insert_with(|| (0, 1));
            map
        },
    );
    mapping.into_iter().fold(0, |acc, (k, v)| {
        if v.0 == 0 || v.1 == 0 {
            return acc;
        }
        acc + (k.parse::<i64>().unwrap() * v.0 * v.1)
    })
}

#[cfg(test)]
mod tests {

    use super::*;

    const INPUT: &str = include_str!("../../inputs/day01.txt");
    const SAMPLE_1: &str = r#"3   4
4   3
2   5
1   3
3   9
3   3"#;
    const SAMPLE_2: &str = r#"3   4
4   3
2   5
1   3
3   9
3   3"#;

    #[test]
    fn solve1_test() {
        assert_eq!(s1(SAMPLE_1), 11);
        assert_eq!(s1(INPUT), 1579939);
    }

    #[test]
    fn solve2_test() {
        assert_eq!(s2(SAMPLE_2), 31);
        assert_eq!(s2(INPUT), 20351745);
    }
}

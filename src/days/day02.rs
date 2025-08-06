/// Function to test report levels safeness.
fn is_safe(arr: &[i8]) -> bool {
    let (mut prev, mut curr, mut next): (i8, i8, i8);
    for i in 1..(arr.len() - 1) {
        prev = arr[i - 1];
        curr = arr[i];
        next = arr[i + 1];
        if (curr <= prev && curr <= next)
            || (curr >= prev && curr >= next)
            || (prev - curr).abs() > 3
            || (next - curr).abs() > 3
        {
            return false;
        }
    }
    true
}

/// # Solve for challenge 1 at day 2:
///
/// We should turn each row of our input into a collection of ints (Vector),
/// and then, count how many are safe.
pub fn s1(input: &str) -> i64 {
    input
        .lines()
        .map(|row| {
            row.split(" ")
                .map(|item| item.parse::<i8>().unwrap())
                .collect::<Vec<i8>>()
        })
        .filter(|report| is_safe(report))
        .count() as i64
}

/// # Solve for challenge 2 at day 2:
///
/// We should turn each row of our input ointo a collection of ints (Vector),
/// and then, count how many are safe + how many can be safe by removing 1
/// item.
pub fn s2(input: &str) -> i64 {
    input
        .lines()
        .map(|row| {
            row.split(" ")
                .map(|item| item.parse::<i8>().unwrap())
                .collect::<Vec<i8>>()
        })
        .filter_map(|mut report| {
            if is_safe(&report) {
                return Some(report);
            }
            let mut popped: i8;
            for i in 0..report.len() {
                popped = report.remove(i);
                if is_safe(&report) {
                    return Some(report);
                }
                report.insert(i, popped);
            }
            None
        })
        .count() as i64
}

#[cfg(test)]
mod tests {

    use super::*;

    const INPUT: &str = include_str!("../../inputs/day02.txt");
    const SAMPLE_1: &str = r#"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"#;
    const SAMPLE_2: &str = r#"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"#;

    #[test]
    fn solve1_test() {
        assert_eq!(s1(SAMPLE_1), 2);
        assert_eq!(s1(INPUT), 369);
    }

    #[test]
    fn solve2_test() {
        assert_eq!(s2(SAMPLE_2), 4);
        assert_eq!(s2(INPUT), 428);
    }
}

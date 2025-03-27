#[derive(Clone)]
struct Report {
    levels: Vec<i64>,
}

impl Report {
    fn from_string(levels: String) -> Self {
        let levels: Vec<i64> = levels
            .split_whitespace()
            .map(|val| val.parse().unwrap())
            .collect();
        Report { levels }
    }

    fn is_safe(&self) -> bool {
        let levels_ref = &self.levels;
        let length = levels_ref.len();
        let previous: &[i64] = &levels_ref[..(length - 1)];
        let next: &[i64] = &levels_ref[1..];

        match (
            previous.iter().zip(next.iter()).all(|(p, n)| p < n),
            previous.iter().zip(next.iter()).all(|(p, n)| p > n),
        ) {
            (false, false) => return false,
            _ => {}
        }

        let mut cloned = self.levels.clone();
        cloned.sort();

        let previous = &cloned[..(length - 1)];
        let next = &cloned[1..];
        let scope = 1..=3;

        for (p, n) in previous.into_iter().zip(next.into_iter()) {
            let diff = &(n - p);
            if !scope.contains(diff) {
                return false;
            }
        }

        true
    }

    fn remove_from_ind(&mut self, index: usize) {
        self.levels.remove(index);
    }

    fn lvl_length(&self) -> usize {
        self.levels.len()
    }
}

pub fn s1(input: Vec<String>) -> i64 {
    let reports: Vec<Report> = input
        .into_iter()
        .map(|row| Report::from_string(row))
        .collect();

    reports
        .into_iter()
        .filter(|report| report.is_safe())
        .count() as i64
}

pub fn s2(input: Vec<String>) -> i64 {
    let reports: Vec<Report> = input
        .into_iter()
        .map(|row| Report::from_string(row))
        .collect();

    reports
        .into_iter()
        .filter(|report| {
            report.is_safe() || {
                for i in 0..report.lvl_length() {
                    let mut repclone = report.clone();
                    repclone.remove_from_ind(i);
                    if repclone.is_safe() {
                        return true;
                    }
                }
                false
            }
        })
        .count() as i64
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

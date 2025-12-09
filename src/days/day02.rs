trait ReportTrait {
    fn report_from_str(s: &str) -> Self;
    fn is_safe(&self) -> bool;
}

impl ReportTrait for Vec<i8> {
    fn report_from_str(s: &str) -> Self {
        s.split_whitespace()
            .map(|level| level.parse::<i8>().unwrap())
            .collect()
    }

    fn is_safe(&self) -> bool {
        let (mut ascending, mut descending) = (false, false);
        for i in 1..self.len() {
            let (prev, curr) = (self[i - 1], self[i]);
            let diff = (prev - curr).abs();
            if diff <= 0 || diff > 3 {
                return false;
            }
            if prev < curr {
                ascending = true
            } else {
                descending = true
            }
        }
        ascending ^ descending
    }
}

pub fn s1(input: &str) -> usize {
    let reports = input.lines().fold(Vec::new(), |mut v: Vec<Vec<i8>>, row| {
        v.push(Vec::report_from_str(row));
        v
    });
    reports.into_iter().filter(|lvls| lvls.is_safe()).count()
}

pub fn s2(input: &str) -> usize {
    let mut reports = input.lines().fold(Vec::new(), |mut v: Vec<Vec<i8>>, row| {
        v.push(Vec::report_from_str(row));
        v
    });
    let mut safes = reports.len();
    reports.retain(|r| !r.is_safe());
    safes -= reports.len();
    let fixes = reports
        .into_iter()
        .filter_map(|mut r| {
            for i in 0..r.len() {
                let curr = r.remove(i);
                if r.is_safe() {
                    return Some(r);
                }
                r.insert(i, curr);
            }
            None
        })
        .count();
    safes + fixes
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::inputs::INPUTS;

    const INPUT: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

    #[test]
    fn test1() {
        let result = s1(INPUT);
        assert_eq!(result, 2);
    }

    #[test]
    fn test2() {
        let result = s2(INPUT);
        assert_eq!(result, 4);
    }

    #[test]
    fn run1() {
        let result = s1(INPUTS[1]);
        assert_eq!(result, 369);
    }

    #[test]
    fn run2() {
        let result = s2(INPUTS[1]);
        assert_eq!(result, 428);
    }
}

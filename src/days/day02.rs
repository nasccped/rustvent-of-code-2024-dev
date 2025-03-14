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
    input
        .iter()
        .map(|row| {
            row.split_whitespace()
                .map(|lvl| lvl.trim().parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .filter(|report| report_is_safe(report))
        .count() as i32
}

pub fn s2(input: Vec<String>) -> i32 {
    input
        .iter()
        .map(|row| {
            row.split_whitespace()
                .map(|lvl| lvl.trim().parse::<i32>().unwrap())
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

use std::collections::HashMap;

fn get_vecs(input: &str) -> (Vec<isize>, Vec<isize>) {
    input
        .lines()
        .fold((Vec::new(), Vec::new()), |(mut l, mut r), row| {
            let mut pair = row.split("   ");
            let mut next = || pair.next().unwrap().parse::<isize>().unwrap();
            l.push(next());
            r.push(next());
            (l, r)
        })
}

pub fn s1(input: &str) -> usize {
    let (mut left, mut right) = get_vecs(input);
    left.sort();
    right.sort();
    left.iter()
        .zip(right.iter())
        .map(|(l, r)| (l - r).unsigned_abs())
        .sum()
}

pub fn s2(input: &str) -> usize {
    let hm: HashMap<&str, (usize, usize)> = input.lines().fold(HashMap::new(), |mut hm, row| {
        let mut curr = row.split("   ");
        let mut next = || curr.next().unwrap();
        hm.entry(next())
            .and_modify(|(l, _)| *l += 1)
            .or_insert((1, 0));
        hm.entry(next())
            .and_modify(|(_, r)| *r += 1)
            .or_insert((0, 1));
        hm
    });
    hm.iter().fold(0, |accum, (num, (l, r))| {
        accum + (num.parse::<usize>().unwrap() * l * r)
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::inputs::INPUTS;
    const INPUT: &str = "3   4
4   3
2   5
1   3
3   9
3   3";

    #[test]
    fn part1() {
        assert_eq!(s1(INPUT), 11);
        assert_eq!(s1(INPUTS[0]), 1579939);
    }

    #[test]
    fn part2() {
        assert_eq!(s2(INPUT), 31);
        assert_eq!(s2(INPUTS[0]), 20351745);
    }
}

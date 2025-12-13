use std::collections::HashMap;

trait Day05 {
    fn is_right_order<'a>(&mut self, order_rules: &HashMap<&'a str, OrderRule<'a>>) -> bool;
    fn get_middle(&self) -> usize;
    fn fix_ordering<'a>(&mut self, order_rules: &HashMap<&'a str, OrderRule<'a>>);
}

fn common_iter<'a>(
    v: &mut Vec<&str>,
    order_rules: &HashMap<&'a str, OrderRule<'a>>,
    fix: bool,
) -> bool {
    let mut fixed = true;
    for i in 0..v.len() {
        for j in 0..v.len() {
            if (j > i
                && order_rules
                    .get(v[i])
                    .is_some_and(|or| or.left_contains(v[j])))
                || (j < i
                    && order_rules
                        .get(v[i])
                        .is_some_and(|or| or.right_contains(v[j])))
            {
                if fix {
                    fixed = false;
                    v.swap(i, j);
                } else {
                    return false;
                }
            }
        }
    }
    if fix && !fixed {
        common_iter(v, order_rules, fix)
    } else {
        fixed
    }
}

impl Day05 for Vec<&str> {
    fn is_right_order<'a>(&mut self, order_rules: &HashMap<&'a str, OrderRule<'a>>) -> bool {
        common_iter(self, order_rules, false)
    }

    fn fix_ordering<'a>(&mut self, order_rules: &HashMap<&'a str, OrderRule<'a>>) {
        common_iter(self, order_rules, true);
    }

    fn get_middle(&self) -> usize {
        self[self.len() / 2].parse::<usize>().expect("surely usize")
    }
}

fn get_order_rules<'a>(s: &'a str) -> HashMap<&'a str, OrderRule<'a>> {
    s.lines()
        .fold(HashMap::<&str, OrderRule>::new(), |mut ors, row| {
            let mut pages = row.split("|");
            let mut next = || pages.next().expect("2 items were expect");
            let left = next();
            let right = next();
            ors.entry(left)
                .and_modify(|or| or.push_on_right(right))
                .or_insert(OrderRule::new_with_right(right));
            ors.entry(right)
                .and_modify(|or| or.push_on_left(left))
                .or_insert(OrderRule::new_with_left(left));
            ors
        })
}

fn separate_parts(s: &str) -> (&str, &str) {
    let mut all = s.split("\n\n");
    let mut next = || all.next().unwrap();
    (next(), next())
}

#[derive(Default, Debug)]
struct OrderRule<'a> {
    left: Vec<&'a str>,
    right: Vec<&'a str>,
}

impl<'a> OrderRule<'a> {
    fn new_with_left(item: &'a str) -> Self {
        let mut or = Self::default();
        or.push_on_left(item);
        or
    }

    fn new_with_right(item: &'a str) -> Self {
        let mut or = Self::default();
        or.push_on_right(item);
        or
    }

    fn push_on_left(&mut self, item: &'a str) {
        self.left.push(item);
    }

    fn push_on_right(&mut self, item: &'a str) {
        self.right.push(item);
    }

    fn left_contains(&self, item: &'a str) -> bool {
        self.left.contains(&item)
    }

    fn right_contains(&self, item: &'a str) -> bool {
        self.right.contains(&item)
    }
}

pub fn s1(input: &str) -> usize {
    let (rules, pages) = separate_parts(input);
    let order_rules = get_order_rules(rules);
    let mut accum = 0;
    let mut vals: Vec<&str>;
    for row in pages.lines() {
        vals = row.split(",").collect();
        if vals.is_right_order(&order_rules) {
            accum += vals.get_middle();
        }
    }
    accum
}

pub fn s2(input: &str) -> usize {
    let (rules, pages) = separate_parts(input);
    let order_rules = get_order_rules(rules);
    let mut accum = 0;
    let mut vals: Vec<&str>;
    for row in pages.lines() {
        vals = row.split(",").collect();
        if !vals.is_right_order(&order_rules) {
            vals.fix_ordering(&order_rules);
            accum += vals.get_middle();
        }
    }
    accum
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::inputs::INPUTS;

    const INPUT: &str = r#"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"#;

    #[test]
    fn part1() {
        assert_eq!(s1(INPUT), 143);
        assert_eq!(s1(INPUTS[4]), 5329);
    }

    #[test]
    fn part2() {
        assert_eq!(s2(INPUT), 123);
        assert_eq!(s2(INPUTS[4]), 5833);
    }
}

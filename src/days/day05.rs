use crate::utils::VecStringTrim;
use std::collections::HashMap;

struct PagesHelper {
    pages_at_left: Vec<i32>,
    pages_at_right: Vec<i32>,
}

impl PagesHelper {
    fn new() -> Self {
        PagesHelper {
            pages_at_left: Vec::new(),
            pages_at_right: Vec::new(),
        }
    }

    fn add_to_left(&mut self, value: i32) {
        self.pages_at_left.push(value);
    }

    fn add_to_right(&mut self, value: i32) {
        self.pages_at_right.push(value);
    }

    fn get_pages_at_left(&self) -> Vec<i32> {
        self.pages_at_left.clone()
    }

    fn get_pages_at_right(&self) -> Vec<i32> {
        self.pages_at_right.clone()
    }
}

fn get_page_ordergin_rules(input: Vec<String>) -> Vec<String> {
    let hash_at = input
        .iter()
        .enumerate()
        .find(|(_, row)| row.starts_with("#"))
        .unwrap()
        .0;
    input.into_iter().take(hash_at).collect()
}

fn get_page_number_updates(input: Vec<String>) -> Vec<String> {
    let hash_at = input
        .iter()
        .enumerate()
        .find(|(_, row)| row.starts_with("#"))
        .unwrap()
        .0;
    input.into_iter().skip(hash_at + 1).collect()
}

pub fn s1(input: Vec<String>) -> i32 {
    let input = input.foreach_trim();
    let page_order_rules = get_page_ordergin_rules(input.clone());
    let page_numbers = get_page_number_updates(input);

    let mut page_order_map: HashMap<i32, PagesHelper> = HashMap::new();

    for pg in page_order_rules {
        let mut pgs = pg.split("|");
        let at_left: i32 = pgs.next().unwrap().parse().unwrap();
        let at_right: i32 = pgs.next().unwrap().parse().unwrap();
        if let Some(page) = page_order_map.get_mut(&at_left) {
            page.add_to_right(at_right);
        } else {
            let mut new_pg_helper = PagesHelper::new();
            new_pg_helper.add_to_right(at_right);
            page_order_map.insert(at_left, new_pg_helper);
        }

        if let Some(page) = page_order_map.get_mut(&at_right) {
            page.add_to_left(at_left);
        } else {
            let mut new_pg_helper = PagesHelper::new();
            new_pg_helper.add_to_left(at_left);
            page_order_map.insert(at_right, new_pg_helper);
        }
    }

    let correct_pages: Vec<String> = page_numbers
        .into_iter()
        .filter(|row| {
            let pgs: Vec<i32> = row.split(",").map(|v| v.parse().unwrap()).collect();
            for i in 0..pgs.len() {
                let cur_page = pgs[i];
                let previous = &pgs[..i];
                let next = &pgs[i..];
                let left_list = page_order_map.get(&cur_page).unwrap().get_pages_at_left();
                let right_list = page_order_map.get(&cur_page).unwrap().get_pages_at_right();
                for p in previous {
                    if right_list.contains(p) {
                        return false;
                    }
                }
                for n in next {
                    if left_list.contains(n) {
                        return false;
                    }
                }
            }
            true
        })
        .collect();

    let middles: Vec<i32> = correct_pages
        .into_iter()
        .map(|row| {
            let parsed: Vec<i32> = row.split(",").map(|v| v.parse().unwrap()).collect();
            let targ = (parsed.len() as f64 / 2.0).round() as usize - 1;
            parsed[targ]
        })
        .collect();

    middles.iter().sum()
}

#[allow(dead_code)]
pub fn s2(_input: Vec<String>) -> i32 {
    // TODO:
    todo!()
}

#[cfg(test)]
mod day05_tests {
    use super::{s1, s2};
    use crate::utils::StrArrVecString;

    #[test]
    fn solve1_test() {
        let input: Vec<String> = [
            "47|53",
            "97|13",
            "97|61",
            "97|47",
            "75|29",
            "61|13",
            "75|53",
            "29|13",
            "97|29",
            "53|29",
            "61|53",
            "97|53",
            "61|29",
            "47|13",
            "75|47",
            "97|75",
            "47|61",
            "75|61",
            "47|29",
            "75|13",
            "53|13",
            "#",
            "75,47,61,53,29",
            "97,61,53,29,13",
            "75,29,13",
            "75,97,47,61,53",
            "61,13,29",
            "97,13,75,29,47",
        ]
        .into_vecstring();
        let result = s1(input);
        assert_eq!(result, 143);
    }

    #[test]
    fn solve2_test() {
        let input: Vec<String> = [].into_vecstring();
        let result = s2(input);
        assert_eq!(result, -1);
    }
}

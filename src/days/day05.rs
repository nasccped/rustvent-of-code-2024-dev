use std::collections::HashMap;

struct PagesHelper {
    pages_at_left: Vec<i64>,
    pages_at_right: Vec<i64>,
}

impl PagesHelper {
    fn new() -> Self {
        PagesHelper {
            pages_at_left: Vec::new(),
            pages_at_right: Vec::new(),
        }
    }

    fn add_to_left(&mut self, value: i64) {
        self.pages_at_left.push(value);
    }

    fn add_to_right(&mut self, value: i64) {
        self.pages_at_right.push(value);
    }

    fn get_pages_at_left(&self) -> Vec<i64> {
        self.pages_at_left.clone()
    }

    fn get_pages_at_right(&self) -> Vec<i64> {
        self.pages_at_right.clone()
    }
}

fn get_page_ordering_rules(input: Vec<String>) -> Vec<String> {
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

pub fn s1(input: Vec<String>) -> i64 {
    let page_order_rules = get_page_ordering_rules(input.clone());
    let page_numbers = get_page_number_updates(input);

    let mut page_order_map: HashMap<i64, PagesHelper> = HashMap::new();

    for pg in page_order_rules {
        let mut pgs = pg.split("|");
        let at_left: i64 = pgs.next().unwrap().parse().unwrap();
        let at_right: i64 = pgs.next().unwrap().parse().unwrap();
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
            let pgs: Vec<i64> = row.split(",").map(|v| v.parse().unwrap()).collect();
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

    let middles: Vec<i64> = correct_pages
        .into_iter()
        .map(|row| {
            let parsed: Vec<i64> = row.split(",").map(|v| v.parse().unwrap()).collect();
            let targ = (parsed.len() as f64 / 2.0).round() as usize - 1;
            parsed[targ]
        })
        .collect();

    middles.iter().sum()
}

pub fn s2(input: Vec<String>) -> i64 {
    let page_order_rules = get_page_ordering_rules(input.clone());
    let page_numbers = get_page_number_updates(input);

    let mut page_order_map: HashMap<i64, PagesHelper> = HashMap::new();

    for pg in page_order_rules {
        let mut pgs = pg.split("|");
        let at_left: i64 = pgs.next().unwrap().parse().unwrap();
        let at_right: i64 = pgs.next().unwrap().parse().unwrap();
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

    let incorrect_pages: Vec<String> = page_numbers
        .into_iter()
        .filter(|row| {
            let pgs: Vec<i64> = row.split(",").map(|v| v.parse().unwrap()).collect();
            for i in 0..pgs.len() {
                let cur_page = pgs[i];
                let previous = &pgs[..i];
                let next = &pgs[i..];
                let left_list = page_order_map.get(&cur_page).unwrap().get_pages_at_left();
                let right_list = page_order_map.get(&cur_page).unwrap().get_pages_at_right();
                for p in previous {
                    if right_list.contains(p) {
                        return true;
                    }
                }
                for n in next {
                    if left_list.contains(n) {
                        return true;
                    }
                }
            }
            false
        })
        .collect();

    let incorrects_as_vec: Vec<Vec<i64>> = incorrect_pages
        .into_iter()
        .map(|row| row.split(",").map(|v| v.parse().unwrap()).collect())
        .collect();

    let mut correcting: Vec<Vec<i64>> = Vec::new();

    for vals in incorrects_as_vec.into_iter() {
        let mut this_row: Vec<i64> = Vec::new();

        for v in vals {
            let target_ind = this_row.iter().enumerate().fold(0, |ini, (i, elem)| {
                if page_order_map
                    .get(&v)
                    .unwrap()
                    .get_pages_at_left()
                    .contains(elem)
                {
                    i + 1
                } else if page_order_map
                    .get(&v)
                    .unwrap()
                    .get_pages_at_right()
                    .contains(elem)
                {
                    ini
                } else {
                    0
                }
            });
            this_row.insert(target_ind, v);
        }
        correcting.push(this_row);
    }

    correcting
        .into_iter()
        .map(|vector| {
            let mid = ((vector.len() as f64 / 2.0).ceil() - 1.0) as usize;
            vector[mid]
        })
        .sum()
}

#[cfg(test)]
mod day05_tests {
    use super::{s1, s2};
    use crate::utils::get_file_content;
    use crate::utils::StrArrVecString;

    #[test]
    fn solve1_test() {
        let input = [
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
        let input = [
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
        let result = s2(input);
        assert_eq!(result, 123);
    }

    #[test]
    fn solve1_run() {
        let input = get_file_content("inputs/day05.txt");
        let result = s1(input);
        assert_eq!(result, 5329);
    }

    #[test]
    fn solve2_run() {
        let input = get_file_content("inputs/day05.txt");
        let result = s2(input);
        assert_eq!(result, 5833);
    }
}

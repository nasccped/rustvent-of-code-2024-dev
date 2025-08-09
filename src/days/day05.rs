use std::collections::HashMap;
use std::collections::HashSet;

/// A Helper struct to handle page ordering. It should be ran with HashMap.
struct PageOrdering<'a> {
    lefts: HashSet<&'a str>,
    rights: HashSet<&'a str>,
}

impl<'a> PageOrdering<'a> {
    /// Initialize page ordering with an element at left.
    fn with_left(page: &'a str) -> Self {
        Self {
            lefts: HashSet::from([page]),
            rights: HashSet::new(),
        }
    }

    /// Initialize page ordering with an element at right.
    fn with_right(page: &'a str) -> Self {
        Self {
            lefts: HashSet::new(),
            rights: HashSet::from([page]),
        }
    }

    fn insert_left(&mut self, page: &'a str) {
        self.lefts.insert(page);
    }

    fn insert_right(&mut self, page: &'a str) {
        self.rights.insert(page);
    }

    /// Check if a given value is contained at left set.
    fn contains_at_left(&self, value: &'a str) -> bool {
        self.lefts.contains(value)
    }

    /// Check if a given value is contained at right set.
    fn contains_at_right(&self, value: &'a str) -> bool {
        self.rights.contains(value)
    }
}

/// Usefull traits to handle PageOrdering within a HashMap.
trait PageAnalysis<'a> {
    /// Check if a given vector reference (array slice) is correctly ordered.
    fn is_correctly_ordered(&self, pages: &[&'a str]) -> bool;
    /// Convert an incorrectly ordered Vector into a correct one (based on the
    /// `&self` object).
    fn fix_order(&self, pages: Vec<&'a str>) -> Vec<&'a str>;
}

impl<'a> PageAnalysis<'a> for HashMap<&'a str, PageOrdering<'a>> {
    fn is_correctly_ordered(&self, pages: &[&'a str]) -> bool {
        (0..pages.len()).all(|i| {
            let lefts = &pages[0..i];
            let rights = &pages[(i + 1)..];
            let mid = &pages[i];
            !(lefts.iter().any(|l| {
                self.get(mid)
                    .map(|ord| ord.contains_at_right(l))
                    .unwrap_or(false)
            }) || rights.iter().any(|r| {
                self.get(mid)
                    .map(|ord| ord.contains_at_left(r))
                    .unwrap_or(false)
            }))
        })
    }
    fn fix_order(&self, pages: Vec<&'a str>) -> Vec<&'a str> {
        let mut result = Vec::with_capacity(pages.len());
        pages.into_iter().for_each(|page| {
            let mut ind = result.len();
            while ind > 0 {
                if self
                    .get(&page)
                    .map(|ord| {
                        ord.contains_at_left(result[ind - 1])
                            || !ord.contains_at_right(result[ind - 1])
                    })
                    .unwrap_or(false)
                {
                    break;
                }
                ind -= 1;
            }
            result.insert(ind, page);
        });
        result
    }
}

/// # Solve for challenge 1 at day 5:
///
/// We should separate our input in two parts (1: the page ordering rules, 2:
/// the update sections).
///
/// Then, we'll create a HashMap of `<A: &str, B: PageOrdering<&str>>` which `A`
/// means a single page, and `B` stores it's pages at left and right. Split
/// each first section row into small data pieces and place it in our HashMap
/// accordingly to the previous explanation. Finally, we'll split the second
/// section into rows, and each row into page data, then, filter only the valid
/// page updates and sum the mid page of it.
pub fn s1(input: &str) -> i64 {
    let empty_idx = input
        .split("\n")
        .position(|row| row.is_empty())
        .expect("Missing empty line");
    let page_ord = input.split("\n").take(empty_idx).fold(
        HashMap::<&str, PageOrdering>::new(),
        |mut hmap, row| {
            let (left, right) = {
                let mut pages = row.split("|");
                let mut next = || pages.next().unwrap();
                (next(), next())
            };
            hmap.entry(left)
                .and_modify(|po| po.insert_right(right))
                .or_insert(PageOrdering::with_right(right));
            hmap.entry(right)
                .and_modify(|po| po.insert_left(left))
                .or_insert(PageOrdering::with_left(left));
            hmap
        },
    );
    input
        .split("\n")
        .skip(empty_idx + 1)
        .filter(|row| !row.is_empty())
        .filter_map(|row| {
            let pages: Vec<&str> = row.split(",").collect();
            if page_ord.is_correctly_ordered(&pages) {
                Some(pages)
            } else {
                None
            }
        })
        .fold(0, |accum, pages| {
            accum + pages[pages.len() / 2].parse::<i64>().unwrap()
        })
}

/// # Solve for challenge 2 at day 5:
///
/// Works the same as solve 1, but we'll filter only the **INVALID** page
/// updates, fix them and sum the mid page to get our final result.
pub fn s2(input: &str) -> i64 {
    let empty_idx = input
        .split("\n")
        .position(|row| row.is_empty())
        .expect("Missing empty line");
    let page_ord = input.split("\n").take(empty_idx).fold(
        HashMap::<&str, PageOrdering>::new(),
        |mut hmap, row| {
            let (left, right) = {
                let mut pages = row.split("|");
                let mut next = || pages.next().unwrap();
                (next(), next())
            };
            hmap.entry(left)
                .and_modify(|po| po.insert_right(right))
                .or_insert(PageOrdering::with_right(right));
            hmap.entry(right)
                .and_modify(|po| po.insert_left(left))
                .or_insert(PageOrdering::with_left(left));
            hmap
        },
    );
    input
        .split("\n")
        .skip(empty_idx + 1)
        .filter(|row| !row.is_empty())
        .filter_map(|row| {
            let pages: Vec<_> = row.split(",").collect();
            if !page_ord.is_correctly_ordered(&pages) {
                Some(pages)
            } else {
                None
            }
        })
        .map(|pages| page_ord.fix_order(pages))
        .fold(0, |accum, pages| {
            accum + pages[pages.len() / 2].parse::<i64>().unwrap()
        })
}

#[cfg(test)]
mod tests {

    use super::*;

    const INPUT: &str = include_str!("../../inputs/day05.txt");
    const SAMPLE_1: &str = r#"47|53
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
    fn solve1_test() {
        assert_eq!(s1(SAMPLE_1), 143);
        assert_eq!(s1(INPUT), 5329);
    }

    #[test]
    fn solve2_test() {
        assert_eq!(s2(SAMPLE_1), 123);
        assert_eq!(s2(INPUT), 5833);
    }
}

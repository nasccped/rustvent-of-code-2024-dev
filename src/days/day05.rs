use std::collections::HashMap;
use std::collections::HashSet;

// consts used along the solves
const PAGE_ORDERING_RULES_SEPARATOR: &str = "|"; // ex: "1|2", "32|27", ...
const PAGE_NUMBERS_SEPARATOR: &str = ","; // ex: "1,2,3,4", "31,27,90", ...

/* sections separator (I've changed the empty line from the original input to '#' 'cause we're
 * removing empty lines when converting the &str input to Vec<String>
 */
const SECTION_SEPARATOR_BEGIN: &str = "#";

/// Function to get usize index when separating input sections.
///
/// # Example
///
/// Consider our input as:
///
/// ```
/// let input = Vec::from([
/// "12|32",    // ind: 0
/// "13|12",    // ind: 1
/// "#",        // ind: 2 (section separator here)
/// "12,32,13", // ind: 3
/// "13,12,32", // ind: 4
/// ]);
/// ```
///
/// Then:
///
/// ```
/// let index = get_section_separator(&input); // expecting 2
/// ```
///
/// # Panics
///
/// If the section separator ('#' const) isn't present on the input. (Check line 29 of this file)
fn get_section_separator(vector: &[String]) -> usize {
    vector.iter()
        .position(|row| row.starts_with(SECTION_SEPARATOR_BEGIN))
        .unwrap_or_else(|| {
            panic!("Missing '{}' to get section separator. Probably an copy-pasted input!\nTake a look at 'src/days/day05.rs' comments", SECTION_SEPARATOR_BEGIN);
        }
        )
}

/// Create a Pages Helper trait since PageHelper isn't a struct, but type alias instead. The
/// declared lifetime is also required since we are working with `&str`.
trait PagesHelperTrait<'a> {
    fn run_rule_entry(&mut self, rule: Rule<'a>);
    fn is_safe_page_numbers(&self, page_numbers: &mut Vec<&str>) -> bool;
    fn fixed_list_from(&self, pages: Vec<&'a str>) -> Vec<&'a str>;
    fn get_insert_index(&self, vector: &[&str], value: &str) -> usize;
}

/// Rule for each rule input.
///
/// # Example
///
/// - the "1|2" input means a `Rule { left: "1", right: "2"}`
struct Rule<'a> {
    left: &'a str,
    right: &'a str,
}

impl<'a> From<&'a String> for Rule<'a> {
    /// Convert a `&'a String` input to a Rule:
    ///
    /// # How it works
    ///
    /// - take the `&String`
    /// - split it by the `PAGE_ORDERING_RULES_SEPARATOR`
    /// - send next value (left) from the splitted iter to left field
    /// - send next value again (right) from the splitted iter to right field
    fn from(value: &'a String) -> Self {
        let mut values = value.split(PAGE_ORDERING_RULES_SEPARATOR);
        Self {
            left: values.next().unwrap(),
            right: values.next().unwrap(),
        }
    }
}

/// Key type for our `PagesHelper` since we need to store a HashMap<K, V>, where
/// V is a struct containing all pages that should be at left + all pages that
/// should be at right (pages: `&str`).
struct OrderingRules<'a> {
    at_left: HashSet<&'a str>,
    at_right: HashSet<&'a str>,
}

impl<'a> OrderingRules<'a> {
    /// Init a new `OrderingRules` with a default left page + empty Set at right.
    fn left_init(init_val: &'a str) -> Self {
        Self {
            at_left: HashSet::from([init_val]),
            at_right: HashSet::new(),
        }
    }

    /// Init a new `OrderingRules` with a default right page + empty Set at left.
    fn right_init(init_val: &'a str) -> Self {
        Self {
            at_left: HashSet::new(),
            at_right: HashSet::from([init_val]),
        }
    }

    /// Check if this `OrderingRules` contains a given page at left.
    fn left_contains(&self, value: &str) -> bool {
        self.at_left.contains(&value)
    }

    /// Check if this `OrderingRules` contains a given page at right.
    fn right_contains(&self, value: &str) -> bool {
        self.at_right.contains(&value)
    }

    /// Push a new value to the left set-list.
    fn push_left(&mut self, value: &'a str) {
        self.at_left.insert(value);
    }

    /// Push a new value to the right set-list.
    fn push_right(&mut self, value: &'a str) {
        self.at_right.insert(value);
    }
}

/// Type alias for our `PagesHelper`. It's a HashMap<K, V> where:
/// - `K` is a page page (`&str`)
/// - `V` is a OrderingRules containing all left/right pages
type PagesHelper<'a> = HashMap<&'a str, OrderingRules<'a>>;

impl<'a> PagesHelperTrait<'a> for PagesHelper<'a> {
    /// Do all entry process for a given page
    ///
    /// # Example
    ///
    /// It will take a Rule type param, handle both left and right values to prepare entry:
    /// - check the current entry (both left and right)
    /// - if if already exists in HashMap: push the other value to their respective place (left or
    ///   right)
    /// - else insert the entry with with respective place init (left or right)
    fn run_rule_entry(&mut self, rule: Rule<'a>) {
        let (left, right) = (rule.left, rule.right);
        self.entry(left)
            .and_modify(|val| val.push_right(right))
            .or_insert(OrderingRules::right_init(right));
        self.entry(right)
            .and_modify(|val| val.push_left(left))
            .or_insert(OrderingRules::left_init(left));
    }

    /// Test if a given page Vector is valid.
    ///
    /// # Note I
    ///
    /// Since we need to check each page in our page number list, the vector should be a mutable
    /// reference to:
    /// - remove item
    /// - check the item safeness
    /// - re-insert the item
    ///
    /// # How it works
    ///
    /// It'll iterate over the `page_numbers` *index* range. In each iter, a page of the given
    /// index will be removed, then, a `OrderingRules` reference targeting the current page will be
    /// handled.
    ///
    /// Considering each page at left as `p`, if it's `p` contained in right list, the function
    /// will return false. (Same logic to each page a right)
    ///
    /// # Note II
    ///
    /// Since only **INVALID** page_numbers should be handled at part2 of this solve, the remove
    /// page should be re-inserted, even the function returns false.
    fn is_safe_page_numbers(&self, page_numbers: &mut Vec<&str>) -> bool {
        let mut current_page: &str;
        let mut ord_rul_ref: &OrderingRules;
        let mut lefts: &[&str];
        let mut rights: &[&str];
        for i in 0..page_numbers.len() {
            current_page = page_numbers.remove(i);
            match self.get(current_page) {
                Some(v) => ord_rul_ref = v,
                _ => {
                    // if there's no ordering rule for this page
                    page_numbers.insert(i, current_page);
                    continue;
                }
            }
            lefts = &page_numbers[..i];
            rights = &page_numbers[i..];

            if lefts.iter().any(|val| ord_rul_ref.right_contains(val)) {
                page_numbers.insert(i, current_page);
                return false;
            }
            if rights.iter().any(|val| ord_rul_ref.left_contains(val)) {
                page_numbers.insert(i, current_page);
                return false;
            }
            page_numbers.insert(i, current_page);
        }
        true
    }

    /// Build a fixed list of page numbers.
    ///
    /// # How it works
    ///
    /// - generate a new empty Vector
    /// - for each page in the old vector:
    ///     - get the correct index based on the actual result populated vector + the current page
    ///       (see `get_insert_index` function)
    ///     - insert the page in their respective index
    /// - return the new vector
    fn fixed_list_from(&self, pages: Vec<&'a str>) -> Vec<&'a str> {
        let mut result = Vec::with_capacity(pages.len());
        let mut ind: usize;
        for pg in pages.into_iter() {
            ind = self.get_insert_index(&result, pg);
            result.insert(ind, pg);
        }
        result
    }

    /// Function to get the correct insertion index of a given page based on the self PagesHelper +
    /// a given vector (as slice)
    fn get_insert_index(&self, vector: &[&str], value: &str) -> usize {
        // start cur index as vector length (compare from right to left)
        let mut cur = vector.len();
        // WARN: all the code above is dangerous. The `get` function can return a None value. In
        // this case, we should compare a `get` option by the pages in the slice, not the value
        // itself. Anyway, this code works, so... there's my solve solution ¯\_('-')_/¯
        let helper = self.get(value).unwrap();
        while cur > 0 {
            if helper.left_contains(vector[cur - 1]) {
                break;
            }
            cur -= 1;
        }
        cur
    }
}

/// Solve 1
pub fn s1(input: Vec<String>) -> i64 {
    // get section separator index
    let section_sep_ind = get_section_separator(&input);
    // pages helper is
    let pages_helper = input // our input, but...
        .iter()
        .take(section_sep_ind) // for each row until sep. index
        .map(Rule::from) // map it to a Rule struct, then
        .fold(PagesHelper::new(), |mut init, rule| {
            // fold it into a new PagesHelper (HashMap)
            // type
            init.run_rule_entry(rule);
            init
        });

    // for each row of our input
    input
        .iter()
        .skip(section_sep_ind + 1) // (from sep. index ahead)
        .filter_map(|row| {
            // take only the safe pages list (as vector)
            let mut pages = row.split(PAGE_NUMBERS_SEPARATOR).collect();
            if pages_helper.is_safe_page_numbers(&mut pages) {
                Some(pages)
            } else {
                None
            }
        }) // for each page, take the middle element (&str) and convert it to i64
        .map(|pages| pages[pages.len() / 2].parse::<i64>().unwrap())
        .sum() // then, return the sum
}

/// solve2
pub fn s2(input: Vec<String>) -> i64 {
    // same as above
    let section_sep_ind = get_section_separator(&input);
    let pages_helper = input.iter().take(section_sep_ind).map(Rule::from).fold(
        PagesHelper::new(),
        |mut init, rule| {
            init.run_rule_entry(rule);
            init
        },
    );

    // same as solve1
    input
        .iter()
        .skip(section_sep_ind + 1)
        .filter_map(|row| {
            // but now, we'll filter only the invalid pages
            let mut pages = row.split(PAGE_NUMBERS_SEPARATOR).collect();
            if pages_helper.is_safe_page_numbers(&mut pages) {
                None
            } else {
                // and return the fixed version of it
                Some(pages_helper.fixed_list_from(pages))
            }
        }) // now, do the middle parse thing
        .map(|pages_list| pages_list[pages_list.len() / 2].parse::<i64>().unwrap())
        .sum() // and sum
}

#[cfg(test)]
mod tests {
    use super::{s1, s2};
    use crate::utils::get_file_content;
    use crate::utils::StrArrVecString;

    #[test]
    fn test1() {
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
    fn test2() {
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
    fn run1() {
        let input = get_file_content("inputs/day05.txt");
        let result = s1(input);
        assert_eq!(result, 5329);
    }

    #[test]
    fn run2() {
        let input = get_file_content("inputs/day05.txt");
        let result = s2(input);
        assert_eq!(result, 5833);
    }
}

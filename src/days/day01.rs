use std::array::IntoIter;
use std::collections::BTreeMap;
use std::collections::HashMap;

const LEFT_RIGHT_SEPARATOR: &str = "   ";

/// All data structures for day 01 solves aren't structs or enums, but type alias instead, so, we
/// need to create a new trait block for the foreign data structures where:
/// - EntryType: type passed when calling `add_value_entry` function
/// - OutputType: type of the returned value when alling `unwrap_value` function
trait Day01Trait<EntryType, OutputType> {
    fn add_value_entry(&mut self, value: EntryType);
    fn unwrap_value(&mut self) -> OutputType;
}

/// ItemCounter: as it says, it counts how many times an i64 item appears in a collection.
///
/// # Note
///
/// A `BTreeMap` is self sorted. So, we don't need to take time by using `.sort()` in a Vector
/// variable. The problem is: 'Map' data structure can hold only 1 key from a given value (no
/// duplicates), and that's why we store a value for the given key: how many times it repeats
/// in our collection.
type ItemCounter = BTreeMap<i64, u8>;

impl Day01Trait<i64, Option<i64>> for ItemCounter {
    /// Function to insert/modify a value in a BTreeMap
    fn add_value_entry(&mut self, value: i64) {
        self.entry(value) // given value entry
            .and_modify(|v| *v += 1) // if it already exists, add 1
            .or_insert(1); // else insert default (1)
    }

    /// Function to get next positive value and modify it
    fn unwrap_value(&mut self) -> Option<i64> {
        self.iter_mut() // iterate in BTreeMap as mutable items
            .find(|(_, v)| *v > &mut 0) // find the first pair where the value is positive
            .map(|(k, v)| {
                *v -= 1; // decrease the value by 1
                *k // return the key
            })
    }
}

/// solve 1
pub fn s1(input: Vec<String>) -> i64 {
    // prepare our items tree (left + right)
    let (mut left_items, mut right_items) = input.into_iter().fold(
        (ItemCounter::new(), ItemCounter::new()), // init an empty tree for both
        |(mut left, mut right), row| {
            // for each row, split the values and parse them
            let mut vals = row
                .split(LEFT_RIGHT_SEPARATOR)
                .map(|val| val.parse::<i64>().unwrap());
            let mut next = || vals.next().unwrap();
            // add entry to both left and right
            left.add_value_entry(next());
            right.add_value_entry(next());
            // return both trees
            (left, right)
        },
    );

    // while positives in left and right list
    let mut accum = 0;
    while let (Some(l), Some(r)) = (left_items.unwrap_value(), right_items.unwrap_value()) {
        // add the diff to accumulator
        accum += (l - r).abs();
    }
    accum
}

/// This type doesn't have a meaningful name. It was taken from official AoC 2024 website.
///
/// To this solve, we need to count how many times a page from left list appears in our right list,
/// multiply the page number by it, and then, sum the obtained values
///
/// # Warning
///
/// If we consider only the prompt above, we'll success the sample test but fail at real input
/// because of this:
///
/// Our input is:
///
/// 2   3
/// 3   2
/// 3   2
///
/// When running the solve, we get:
/// 2 -> 4 because the number '2' appear 2 times in right list
/// 3 -> 6 ??? shouldn't it be 3? the number '3' appear just one time in right list!
///
/// Yeah, but the number 3 appear 2 times in left list, so we need to count it two times too:
///
/// | left list item | right list count | mul. value |
/// | -------------- | ---------------- | ---------- |
/// | 2              | 2, so (2*2)      | 4          |
/// | 3              | 1, so (3*1)      | 3          |
/// | 3              | 1, so (3*1)      | 3          |
///
/// Final result: 10, not 7.
///
/// With this in mind, we should store two values per left page:
/// - How many times it appear in left list
/// - How many times it appear in right list
///
/// # Why not a BTreeMap again?
///
/// In this case, the elements ordering isn't required, so, let's use a HashMap for a better
/// insertion/lookup :^D
type SimilarityScore = HashMap<i64, (i64, i64)>;

impl<T> Day01Trait<T, i64> for SimilarityScore
where
    T: Iterator<Item = i64>,
{
    /// Here, we need to insert or change an entry at our HashMap, but the input type is an i64
    /// Iterator, so we should:
    /// - take input as mutable
    /// - do the entry thing:
    ///     - if it already exists, add 1 to left (.0 at tuple index)
    ///     - else, create one with default val (1)
    /// - do the same with the next item (right)
    fn add_value_entry(&mut self, mut value: T) {
        self.entry(value.next().unwrap())
            .and_modify(|(left, _)| *left += 1)
            .or_insert((1, 0));
        self.entry(value.next().unwrap())
            .and_modify(|(_, right)| *right += 1)
            .or_insert((0, 1));
    }

    /// We should iter over our HashMap taking as final val the page number mul. by how many times
    /// it appear in left FOR EACH PAGE AT LEFT, EVEN DUPLICATES, so, mul. by how many times it
    /// appear in right too.
    fn unwrap_value(&mut self) -> i64 {
        self.iter().map(|(k, (l, r))| (k * r) * l).sum()
    }
}

/// solve 2
pub fn s2(input: Vec<String>) -> i64 {
    let mut similarity_score = input
        .into_iter() // iter over each input row
        .fold(SimilarityScore::new(), |mut h_map, row| {
            // take this row as splited + parsed integers
            let iter = row
                .split(LEFT_RIGHT_SEPARATOR)
                .map(|val| val.parse::<i64>().unwrap());
            // do the entry thing
            h_map.add_value_entry(iter.into_iter());
            h_map
        });
    Day01Trait::<IntoIter<i64, 2>, i64>::unwrap_value(&mut similarity_score)
}

#[cfg(test)]
mod tests {
    use super::{s1, s2};
    use crate::utils::get_file_content;
    use crate::utils::StrArrVecString;

    #[test]
    fn test1() {
        let input = ["3   4", "4   3", "2   5", "1   3", "3   9", "3   3"].into_vecstring();
        let result = s1(input);
        assert_eq!(result, 11);
    }

    #[test]
    fn test2() {
        let input = ["3   4", "4   3", "2   5", "1   3", "3   9", "3   3"].into_vecstring();
        let result = s2(input);
        assert_eq!(result, 31);
    }

    #[test]
    fn solve1() {
        let input = get_file_content("inputs/day01.txt");
        let result = s1(input);
        assert_eq!(result, 1579939);
    }

    #[test]
    fn solve2() {
        let input = get_file_content("inputs/day01.txt");
        let result = s2(input);
        assert_eq!(result, 20351745);
    }
}

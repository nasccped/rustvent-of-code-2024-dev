use std::collections::HashMap;

const LEFT_RIGHT_SEPARATOR: &str = "   ";

/// solve 1
pub fn s1(input: Vec<String>) -> i64 {
    // prepare vector pairs (left / right)
    let mut pairs = (vec![], vec![]);

    // for each row in input
    for row in input {
        // split and get values as int 64
        let mut vals = row
            .split(LEFT_RIGHT_SEPARATOR)
            .map(|val| val.parse::<i64>().unwrap());
        // use closure function to get next element
        let mut next = || vals.next().unwrap();
        pairs.0.push(next()); // push first to left
        pairs.1.push(next()); // push second to right
    }

    // sort both vectors
    pairs.0.sort();
    pairs.1.sort();

    // iter on left
    // zip with right
    // get absolute diff
    // and sum them
    pairs
        .0
        .into_iter()
        .zip(pairs.1)
        .map(|(l, r)| (l - r).abs())
        .sum()
}

/// solve 2
pub fn s2(input: Vec<String>) -> i64 {
    // prepare the similarity score with a HashMap<A, (B, C)>, where:
    //      - A: base value (from left or right, doesn't matter)
    //      - B: how many times the A value appear in the left list
    //      - C: how many times the A value appear in right list
    let mut similarity_score: HashMap<i64, (i64, i64)> = HashMap::new();

    for row in input {
        // same as function above
        let mut vals = row
            .split(LEFT_RIGHT_SEPARATOR)
            .map(|val| val.parse::<i64>().unwrap());
        let mut next = || vals.next().unwrap();

        // in hashmap
        similarity_score
            .entry(next()) // do the left entry
            .and_modify(|tup| tup.0 += 1) // if it already exists, change B to +1
            .or_insert((1, 0)); // if it doesn't, set B to 1 (first time in left list)
        similarity_score
            .entry(next()) // do the right entry
            .and_modify(|tup| tup.1 += 1) // if it already exists, change C to +1
            .or_insert((0, 1)); // if it doesn't, set C to 1 (first time in right list)
    }
    // iter over hashmap elements
    // take the base value, multiply it by how many times it
    // appear on right list, and then, how many time it appear on right list
    // finally, sum them
    similarity_score
        .into_iter()
        .map(|(k, v)| (k * v.1) * v.0)
        .sum()

    // NOTE: I know, I know.
    // I can just use two vectors and extend this function +30 lines away but this is the most fast
    // way to solve the challenge (+ considering left list duplicates)
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

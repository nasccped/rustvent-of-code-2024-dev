// Day 01: Solve 1
pub fn s1(input: Vec<String>) -> i32 {
    // TODO:
    //  1. create two item collections (left, right)
    //  2. take each row of our input
    //  3. split it + parse
    //  4. send first item to left / second to right collection
    //  5. sort them
    //  6. sum all differences

    // NOTE: (1)
    let (mut left, mut right): (Vec<i32>, Vec<i32>) = (vec![], vec![]);
    // NOTE: (2)
    for row in input {
        // NOTE: (3)
        let mut items = row
            .split("   ")
            .map(|item| item.trim().parse::<i32>().unwrap())
            .into_iter();
        // NOTE: (4)
        left.push(items.next().unwrap());
        right.push(items.next().unwrap());
    }
    // NOTE: (5)
    left.sort();
    right.sort();
    // NOTE: (6)
    left.iter()
        .zip(right.iter())
        .map(|(l, r)| (l - r).abs())
        .sum()
}

// Day 01: Solve 2
pub fn s2(input: Vec<String>) -> i32 {
    // TODO:
    //  1. create a HashMap (store left items)
    //  2. create a vector (store right items)
    //  3. search for each item at right in HashMap by the item key
    //  4. increase the Hash node value +1 if successly founded
    //  5. multiply each key by his value
    //  6. sum all the values
    use std::collections::HashMap;
    // NOTE: (1)
    let mut left_map: HashMap<i32, (i32, i32)> = HashMap::new();
    // NOTE: (2)
    let right_items: Vec<i32> = input
        .into_iter()
        .map(|row| {
            let mut pair = row
                .split("   ")
                .map(|item| item.trim().parse::<i32>().unwrap())
                .into_iter();
            let first = pair.next().unwrap();
            if let Some((left_times, _)) = left_map.get_mut(&first) {
                *left_times += 1;
            } else {
                left_map.insert(first, (1, 0));
            }
            pair.next().unwrap()
        })
        .collect();

    // NOTE: (3)
    for item in right_items.iter() {
        // NOTE: (4)
        if let Some((_, r_times)) = left_map.get_mut(item) {
            *r_times += 1;
        }
    }
    // NOTE: (5, 6)
    left_map.iter().map(|(k, v)| k * v.0 * v.1).sum()
}

#[cfg(test)]
mod day01_tests {
    use super::{s1, s2};

    #[test]
    fn solve1_test() {
        let input: Vec<String> = ["3   4", "4   3", "2   5", "1   3", "3   9", "3   3"]
            .iter()
            .map(|row| row.to_string())
            .collect();
        let result = s1(input);
        assert_eq!(result, 11);
    }

    #[test]
    fn solve2_test() {
        let input: Vec<String> = ["3   4", "4   3", "2   5", "1   3", "3   9", "3   3"]
            .iter()
            .map(|row| row.to_string())
            .collect();
        let result = s2(input);
        assert_eq!(result, 31);
    }
}

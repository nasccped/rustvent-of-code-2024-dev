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

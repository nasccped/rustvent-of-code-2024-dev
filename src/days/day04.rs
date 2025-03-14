pub fn s2(input: Vec<String>) -> i32 {
    let input: Vec<String> = input
        .into_iter()
        .map(|row| row.trim().to_string())
        .collect();

    let mut accum = 0;

    for y in 1..(input.len() - 1) {
        for x in 1..(input.len() - 1) {
            if input[y].chars().nth(x).unwrap() != 'A' {
                continue;
            }

            match (
                input[y - 1].chars().nth(x - 1).unwrap(),
                input[y + 1].chars().nth(x + 1).unwrap(),
            ) {
                ('M', 'S') => {}
                ('S', 'M') => {}
                _ => continue,
            }

            match (
                input[y - 1].chars().nth(x + 1).unwrap(),
                input[y + 1].chars().nth(x - 1).unwrap(),
            ) {
                ('M', 'S') => {}
                ('S', 'M') => {}
                _ => continue,
            }

            accum += 1;
        }
    }

    accum
}

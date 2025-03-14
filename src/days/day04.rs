pub fn s1(input: Vec<String>) -> i32 {
    let input: Vec<String> = input
        .into_iter()
        .map(|row| row.trim().to_string())
        .collect();

    let mut accum = 0;

    for i in 0..(input.len()) {
        for j in 0..(input[i].len()) {
            let hor: String = input[i].chars().skip(j).take(4).collect();
            let ver: String = input
                .iter()
                .skip(i)
                .take(4)
                .map(|row| row.chars().nth(j).unwrap())
                .collect();

            let _ = [hor, ver]
                .iter()
                .filter(|&word| word == "XMAS" || word == "SAMX")
                .for_each(|_| accum += 1);
        }
    }

    for y in 0..(input.len()) {
        if y > input.len() - 4 {
            break;
        }
        for x in 0..(input[y].len()) {
            if x > input[y].len() - 4 {
                break;
            }
            let mut diagoes = String::new();
            let mut diaback = String::new();
            for i in 0..4 {
                diagoes.push(input[y + i].chars().nth(x + i).unwrap());
                diaback.push(
                    input[y + i]
                        .chars()
                        .nth(input[y].len() - x - i - 1)
                        .unwrap(),
                );
            }

            let _ = [diagoes, diaback]
                .iter()
                .filter(|&word| word == "XMAS" || word == "SAMX")
                .for_each(|_| accum += 1);
        }
    }
    accum
}

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

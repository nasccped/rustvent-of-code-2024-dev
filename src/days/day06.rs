use crate::utils::VecStringTrim;
use std::collections::HashSet;

pub fn s1(input: Vec<String>) -> i64 {
    let input = input.foreach_trim();
    let mut iy = input
        .iter()
        .enumerate()
        .find(|(_, row)| row.contains("^"))
        .unwrap()
        .0 as i64;
    let mut ix = input[iy as usize].find("^").unwrap() as i64;

    let mut coordinates: HashSet<(i64, i64)> = HashSet::new();
    let moves: [(i64, i64); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];
    let mut target_move = 0;

    while (0..input.len()).contains(&(iy as usize)) && (0..input[0].len()).contains(&(ix as usize))
    {
        if input[iy as usize].chars().nth(ix as usize).unwrap() == '#' {
            iy -= moves[target_move].0;
            ix -= moves[target_move].1;
            target_move = (target_move + 1) % 4;
            continue;
        }
        coordinates.insert((iy, ix));
        iy += moves[target_move].0;
        ix += moves[target_move].1;
    }
    coordinates.len() as i64
}

pub fn s2(input: Vec<String>) -> i64 {
    let input = input.foreach_trim();
    let char_input: Vec<Vec<char>> = input.iter().map(|row| row.chars().collect()).collect();
    let my = char_input.len();
    let mx = char_input[0].len();
    let iy = char_input
        .iter()
        .enumerate()
        .find(|(_, row)| row.contains(&'^'))
        .unwrap()
        .0;
    let ix = char_input
        .iter()
        .nth(iy)
        .unwrap()
        .iter()
        .enumerate()
        .find(|(_, c)| *c == &'^')
        .unwrap()
        .0;

    let mut accum = 0;

    println!("This will take a long time (+3 min, probably)!");

    for y in 0..input.len() {
        for x in 0..input.iter().nth(y).unwrap().len() {
            let mut inp_copy = char_input.clone();
            if inp_copy[y][x] == '#' || inp_copy[y][x] == '^' {
                continue;
            }
            inp_copy[y][x] = '#';
            let (mut cy, mut cx) = (iy as i64, ix as i64);
            let moves = [(-1, 0), (0, 1), (1, 0), (0, -1)];
            let mut c_move = 0;
            let mut coords: HashSet<(i64, i64, usize)> = HashSet::new();

            while (0..my).contains(&(cy as usize)) && (0..mx).contains(&(cx as usize)) {
                if inp_copy[cy as usize][cx as usize] == '#' {
                    cy -= moves[c_move].0;
                    cx -= moves[c_move].1;
                    c_move = (c_move + 1) % 4;
                    continue;
                }
                if coords.contains(&(cy, cx, c_move)) {
                    accum += 1;
                    break;
                }
                coords.insert((cy, cx, c_move));
                cy += moves[c_move].0;
                cx += moves[c_move].1;
            }
        }
    }

    accum
}

#[cfg(test)]
mod day06_tests {
    use super::{s1, s2};
    use crate::utils::get_file_content;
    use crate::utils::StrArrVecString;

    #[test]
    fn solve1_test() {
        let input = [
            "....#.....",
            ".........#",
            "..........",
            "..#.......",
            ".......#..",
            "..........",
            ".#..^.....",
            "........#.",
            "#.........",
            "......#...",
        ]
        .into_vecstring();
        let result = s1(input);
        assert_eq!(result, 41);
    }

    #[test]
    fn solve2_test() {
        let input = [
            "....#.....",
            ".........#",
            "..........",
            "..#.......",
            ".......#..",
            "..........",
            ".#..^.....",
            "........#.",
            "#.........",
            "......#...",
        ]
        .into_vecstring();
        let result = s2(input);
        assert_eq!(result, 6);
    }

    #[test]
    fn solve1_run() {
        let input = get_file_content("inputs/day06.txt");
        let result = s1(input);
        assert_eq!(result, 5269);
    }

    #[test]
    #[ignore]
    fn solve2_run() {
        let input = get_file_content("inputs/day06.txt");
        let result = s2(input);
        assert_eq!(result, 1957);
    }
}

use std::collections::HashSet;

const MOVES: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

fn is_infinite_traversal(grid: Vec<String>, mut y: i32, mut x: i32) -> bool {
    let mut step_path = HashSet::new();
    let mut cur_move: usize = 0;

    while (0..grid.len() as i32).contains(&y) && (0..grid[0].len() as i32).contains(&x) {
        let cur_char = grid[y as usize].chars().nth(x as usize).unwrap();
        if cur_char == '#' {
            y -= MOVES[cur_move].0;
            x -= MOVES[cur_move].1;
            cur_move = (cur_move + 1) % 4;
            continue;
        }
        let cur_step = (y, x, cur_move);
        if step_path.contains(&cur_step) {
            return true;
        }
        step_path.insert(cur_step);
        y += MOVES[cur_move].0;
        x += MOVES[cur_move].1;
    }

    false
}

fn get_locations(grid: Vec<String>, ini_y: usize, ini_x: usize) -> HashSet<(i32, i32)> {
    let mut result = HashSet::new();
    let mut y: i32 = ini_y as i32;
    let mut x: i32 = ini_x as i32;
    let mut cur_move_ind: usize = 0;

    while (0..grid.len() as i32).contains(&y) && (0..grid[0].len() as i32).contains(&x) {
        let cur_char = grid
            .iter()
            .nth(y as usize)
            .unwrap()
            .chars()
            .nth(x as usize)
            .unwrap();

        let cur_move_pair = MOVES[cur_move_ind];

        if cur_char == '#' {
            y -= cur_move_pair.0;
            x -= cur_move_pair.1;
            cur_move_ind = (cur_move_ind + 1) % 4;
            continue;
        }
        result.insert((y, x));
        y += cur_move_pair.0;
        x += cur_move_pair.1;
    }

    result
}

pub fn s1(input: Vec<String>) -> i64 {
    let iy = input
        .iter()
        .enumerate()
        .find(|(_, row)| row.contains("^"))
        .unwrap()
        .0;
    let ix = input[iy as usize].find("^").unwrap();
    get_locations(input, iy, ix).len() as i64
}

pub fn s2(input: Vec<String>) -> i64 {
    let ini_y = input.iter().position(|row| row.contains("^")).unwrap();
    let ini_x = input
        .iter()
        .nth(ini_y)
        .unwrap()
        .chars()
        .position(|c| c == '^')
        .unwrap();
    let mut pos_moves = get_locations(input.clone(), ini_y, ini_x);
    pos_moves.remove(&(ini_y as i32, ini_x as i32));

    let mut accum = 0;

    for (y, x) in pos_moves.into_iter() {
        let input_clone: Vec<String> = input
            .clone()
            .iter()
            .enumerate()
            .map(|(i, row)| {
                if i == y as usize {
                    let mut charray: Vec<char> = row.chars().collect();
                    charray[x as usize] = '#';
                    charray.iter().collect::<String>()
                } else {
                    row.to_owned()
                }
            })
            .collect();

        if is_infinite_traversal(input_clone, ini_y as i32, ini_x as i32) {
            accum += 1;
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
    fn solve2_run() {
        let input = get_file_content("inputs/day06.txt");
        let result = s2(input);
        assert_eq!(result, 1957);
    }
}

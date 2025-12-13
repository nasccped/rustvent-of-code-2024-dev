use crate::utils::GetElementAt;

const XMAS: &str = "XMAS";
const MAS: &str = "MAS";
const SAM: &str = "SAM";

struct Next {
    y: isize,
    x: isize,
}

impl Default for Next {
    fn default() -> Self {
        Self { y: -1, x: -1 }
    }
}

impl Next {
    fn new(y: isize, x: isize) -> Self {
        Self { y, x }
    }
}

struct Coordinate<'a> {
    y: isize,
    x: isize,
    next: &'a Next,
}

impl<'a> Coordinate<'a> {
    fn new(y: isize, x: isize, next: &'a Next) -> Self {
        Self { y, x, next }
    }

    fn go_next(&mut self) -> &mut Self {
        self.y += self.next.y;
        self.x += self.next.x;
        self
    }

    fn go_previous(&mut self) -> &mut Self {
        self.y -= self.next.y;
        self.x -= self.next.x;
        self
    }
}

trait XMasTrait {
    fn y_out_of_range(&self, y: isize) -> bool;
    fn x_out_of_range(&self, x: isize) -> bool;
    fn explore_linear(&self, coord: &mut Coordinate, target: &str) -> bool;
    fn is_x_mas(&self, y: isize, x: isize) -> bool;
}

impl XMasTrait for Vec<&str> {
    fn y_out_of_range(&self, y: isize) -> bool {
        y < 0 || y >= self.len() as isize
    }

    fn x_out_of_range(&self, x: isize) -> bool {
        x < 0 || x >= self.len() as isize
    }

    fn explore_linear(&self, coord: &mut Coordinate, target: &str) -> bool {
        match (self.get_element_at(coord), target) {
            (_, "") => true,
            (Some(a), b) if b.chars().next().is_some_and(|c| c == a) => {
                let r = self.explore_linear(coord.go_next(), &target[1..]);
                coord.go_previous();
                r
            }
            _ => false,
        }
    }

    fn is_x_mas(&self, y: isize, x: isize) -> bool {
        let go_down_right = Next::new(1, 1);
        let go_down_left = Next::new(1, -1);
        let mut coord = Coordinate::new(y, x, &go_down_right);
        if !self.explore_linear(coord.go_previous(), MAS) && !self.explore_linear(&mut coord, SAM) {
            return false;
        }
        coord.go_next();
        coord.next = &go_down_left;
        self.explore_linear(coord.go_previous(), MAS) || self.explore_linear(&mut coord, SAM)
    }
}

impl GetElementAt<char, &Coordinate<'_>> for Vec<&str> {
    fn get_element_at(&self, coord: &Coordinate) -> Option<char> {
        let Coordinate { y, x, .. } = coord;
        match (*y, *x) {
            (y, x) if y < 0 || x < 0 => None,
            (y, x) => self
                .get(y as usize)
                .map(|row| row.chars().nth(x as usize))?,
        }
    }
}

pub fn s1(input: &str) -> usize {
    let grid: Vec<&str> = input.split("\n").collect();
    let mut accum = 0;
    let directions = [
        Next::new(-1, -1),
        Next::new(-1, 1),
        Next::new(1, -1),
        Next::new(1, 1),
        Next::new(0, -1),
        Next::new(0, 1),
        Next::new(1, 0),
        Next::new(-1, 0),
    ];
    let (mut y, mut x) = (0, 0);
    loop {
        if grid.y_out_of_range(y) {
            break;
        } else if grid.x_out_of_range(x) {
            y += 1;
            x = 0;
            continue;
        }
        accum += directions
            .iter()
            .map(|d| {
                let mut coord = Coordinate::new(y, x, d);
                grid.explore_linear(&mut coord, XMAS)
            })
            .filter(|b| *b)
            .count();
        x += 1;
    }
    accum
}

pub fn s2(input: &str) -> usize {
    let grid: Vec<&str> = input.split("\n").collect();
    let mut accum = 0;
    let (mut y, mut x) = (0, 0);
    loop {
        if grid.y_out_of_range(y) {
            break;
        } else if grid.x_out_of_range(x) {
            y += 1;
            x = 0;
            continue;
        }
        if grid.is_x_mas(y, x) {
            accum += 1;
        }
        x += 1;
    }
    accum
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::inputs::INPUTS;
    const INPUT: &str = r#"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"#;

    #[test]
    fn part1() {
        assert_eq!(s1(INPUT), 18);
        assert_eq!(s1(INPUTS[3]), 2378);
    }

    #[test]
    fn part2() {
        assert_eq!(s2(INPUT), 9);
        assert_eq!(s2(INPUTS[3]), 1796);
    }
}

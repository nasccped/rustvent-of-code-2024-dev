/// Direction enum to define grid (bidimensional array) traversal.
enum GoDirection {
    Up,
    Down,
    Left,
    Right,
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
}

/// Shortcut struct to handle grid coordinates (as isize, since negative
/// positions should be also handled).
#[derive(Clone)]
struct Coordinates {
    x: isize,
    y: isize,
}

impl Coordinates {
    fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    /// Tests if the self `Coordinates` have valid positions (within grid
    /// scope)
    fn have_valid_position(&self, grid: &[Vec<char>]) -> bool {
        match (self.x, self.y) {
            (x, y) if x < 0 || y < 0 => false,
            (x, y) if x >= grid[0].len() as isize || y >= grid.len() as isize => false,
            _ => true,
        }
    }

    /// Update the coordinate values based on a Direction enum variant.
    fn update(&mut self, direction: &GoDirection) {
        let (x, y) = (self.x, self.y);
        let (x, y) = match direction {
            GoDirection::Up => (x, y - 1),
            GoDirection::Down => (x, y + 1),
            GoDirection::Left => (x - 1, y),
            GoDirection::Right => (x + 1, y),
            GoDirection::UpLeft => (x - 1, y - 1),
            GoDirection::UpRight => (x + 1, y - 1),
            GoDirection::DownLeft => (x - 1, y + 1),
            GoDirection::DownRight => (x + 1, y + 1),
        };
        self.x = x;
        self.y = y;
    }

    /// Get inner values as usize `(x, y)`
    ///
    /// # Panic
    ///
    /// This function can panic if any of returned values are less than 0.
    fn unwrap_usizes(&self) -> (usize, usize) {
        (self.x as usize, self.y as usize)
    }
}

/// Finds "XMAS" occurrences withing a grid by using recursion.
fn find_xmas_occurrence(
    grid: &[Vec<char>],
    mut grid_pos: Coordinates,
    word: &[char],
    word_ind: usize,
    direction: &GoDirection,
) -> bool {
    // good case
    if word_ind >= word.len() {
        return true;
    }
    // outside grid
    if !grid_pos.have_valid_position(grid) {
        return false;
    }
    // not the expected char
    let (x, y) = grid_pos.unwrap_usizes();
    if grid[y][x] != word[word_ind] {
        return false;
    }
    // update coordinates + use recursion
    grid_pos.update(direction);
    find_xmas_occurrence(grid, grid_pos, word, word_ind + 1, direction)
}

/// Struct for the X-"MAS" thing (part 2).
struct XMas {
    center: char,
    left_up: char,
    right_up: char,
    left_down: char,
    right_down: char,
}

impl XMas {
    fn new(grid: &[Vec<char>], y: usize, x: usize) -> Self {
        let (center, left_up, right_up, left_down, right_down) = (
            grid[y][x],
            grid[y - 1][x - 1],
            grid[y - 1][x + 1],
            grid[y + 1][x - 1],
            grid[y + 1][x + 1],
        );
        Self {
            center,
            left_up,
            left_down,
            right_up,
            right_down,
        }
    }

    fn is_a_perfect_x(&self) -> bool {
        let ms = ['M', 'S'];
        match (
            self.center,
            self.left_up,
            self.right_up,
            self.left_down,
            self.right_down,
        ) {
            (c, ..) if c != 'A' => false,
            (_, lu, .., rd) if lu == rd || !ms.contains(&lu) || !ms.contains(&rd) => false,
            (.., ru, ld, _) if ru == ld || !ms.contains(&ru) || !ms.contains(&ld) => false,
            _ => true,
        }
    }
}

/// # Solve for challenge 1 at day 4:
///
/// We should iter over all grid positions (bidimensional array traversal) and
/// call a separated `find_xmas_occurrence` function for each possible
/// direction. Then, add how many calls returned positive (true - means "XMAS"
/// occurrence found) into our accumulator.
pub fn s1(input: &str) -> i64 {
    // define target, grid and all possible directions
    let target = ['X', 'M', 'A', 'S'];
    let grid: Vec<Vec<char>> = input.lines().map(|row| row.chars().collect()).collect();
    let directions = [
        GoDirection::Up,
        GoDirection::Down,
        GoDirection::Left,
        GoDirection::Right,
        GoDirection::UpLeft,
        GoDirection::UpRight,
        GoDirection::DownLeft,
        GoDirection::DownRight,
    ];
    let (y_range, x_range) = (0..grid.len(), 0..grid[0].len());
    let mut accum = 0;
    let mut coord: Coordinates;
    for y in y_range.clone() {
        for x in x_range.clone() {
            coord = Coordinates::new(x as isize, y as isize);
            accum += directions
                .iter()
                .filter(|d| find_xmas_occurrence(&grid, coord.clone(), &target, 0, d))
                .count() as i64;
        }
    }
    accum
}

/// # Solve for challenge 2 at day 4:
///
/// We should iter over all indexes with 1 index gap from the grid wall, then,
/// check if any of the positions can build a 'X' defined in the AoC question.
pub fn s2(input: &str) -> i64 {
    let grid: Vec<Vec<char>> = input.lines().map(|row| row.chars().collect()).collect();
    let mut accum = 0;
    let (y_range, x_range) = (1..(grid.len() - 1), 1..(grid[0].len() - 1));
    for y in y_range.clone() {
        for x in x_range.clone() {
            if XMas::new(&grid, y, x).is_a_perfect_x() {
                accum += 1
            }
        }
    }
    accum
}

#[cfg(test)]
mod tests {

    use super::*;

    const INPUT: &str = include_str!("../../inputs/day04.txt");
    const SAMPLE_1: &str = r#"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"#;
    const SAMPLE_2: &str = r#".M.S......
..A..MSMS.
.M.S.MAA..
..A.ASMSM.
.M.S.M....
..........
S.S.S.S.S.
.A.A.A.A..
M.M.M.M.M.
.........."#;

    #[test]
    fn solve1_test() {
        assert_eq!(s1(SAMPLE_1), 18);
        assert_eq!(s1(INPUT), 2378);
    }

    #[test]
    fn solve2_test() {
        assert_eq!(s2(SAMPLE_2), 9);
        assert_eq!(s2(INPUT), 1796);
    }
}

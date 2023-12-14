use crate::common::Solution;
use crate::vec2d::{RowCol, Vec2d};

pub enum Day14 {}

impl Solution for Day14 {
    fn solve(lines: impl Iterator<Item = impl AsRef<str>>) -> String {
        let grid = Grid::from_lines(lines);
        let tilted = grid.tilted_north();
        tilted.get_weight().to_string()
    }
}

#[derive(Clone)]
struct Grid {
    tiles: Vec2d<Tile>,
}

impl Grid {
    pub fn from_lines(lines: impl Iterator<Item = impl AsRef<str>>) -> Grid {
        let matrix = Vec2d::from_lines(lines);
        Grid {
            tiles: matrix.map(|&c| c.into()),
        }
    }

    pub fn tilted_north(mut self) -> Self {
        for row in 0..self.tiles.num_rows() {
            for col in 0..self.tiles.first_num_cols() {
                let tile = self.tiles.get(row, col).unwrap();
                if matches!(tile, Tile::Round) {
                    let new_spot = self.find_last_empty_above((row, col).into());
                    *self.tiles.get_mut(row, col).unwrap() = Tile::Empty;
                    *self.tiles.get_mut(new_spot.row, new_spot.col).unwrap() = Tile::Round;
                }
            }
        }
        self
    }

    pub fn get_weight(&self) -> usize {
        self.tiles
            .cells()
            .filter(|cell| cell.is_rounded())
            .map(|cell| self.tiles.num_rows() - cell.row)
            .sum()
    }

    fn find_last_empty_above(&self, row_col: RowCol) -> RowCol {
        let col = row_col.col;
        let row = (0..row_col.row)
            .rev()
            .find(|row| !self.tiles.get(*row, col).unwrap().is_empty())
            .map(|row| row + 1)
            .unwrap_or(0);
        RowCol { row, col }
    }
}

#[derive(Eq, PartialEq, Clone)]
enum Tile {
    Empty,
    Round,
    Square,
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '.' => Tile::Empty,
            '#' => Tile::Square,
            'O' => Tile::Round,
            _ => panic!("Invalid value received: {}", value),
        }
    }
}

impl Tile {
    pub fn is_empty(&self) -> bool {
        matches!(self, Tile::Empty)
    }

    pub fn is_rounded(&self) -> bool {
        matches!(self, Tile::Round)
    }
}

#[cfg(test)]
mod test {
    use crate::common::Solution;
    use crate::day14::Day14;

    const EXAMPLE_INPUT: &str = r#"O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#...."#;

    #[test]
    fn test_example() {
        assert_eq!(Day14::solve(EXAMPLE_INPUT.lines()), "136");
    }
}

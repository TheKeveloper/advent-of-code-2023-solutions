use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter, Write};

use crate::common::Solution;
use crate::vec2d::{RowCol, Vec2d};

pub enum Day14 {}

impl Solution for Day14 {
    fn solve(lines: impl Iterator<Item = impl AsRef<str>>) -> String {
        let mut grid = Grid::from_lines(lines);
        grid.tilt_north();
        grid.get_weight().to_string()
    }
}

pub enum Day14P2 {}

impl Solution for Day14P2 {
    fn solve(lines: impl Iterator<Item = impl AsRef<str>>) -> String {
        let mut grid = Grid::from_lines(lines);
        grid.cycle_iterations(1000000000);
        grid.get_weight().to_string()
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

    pub fn tilt_north(&mut self) {
        for row in 0..self.tiles.num_rows() {
            for col in 0..self.tiles.first_num_cols() {
                let tile = self.tiles.get(row, col).unwrap();
                if tile.is_rounded() {
                    let new_spot = self.find_last_empty_north((row, col).into());
                    self.move_tile(&(row, col).into(), &new_spot);
                }
            }
        }
    }

    pub fn tilt_south(&mut self) {
        for row in (0..self.tiles.num_rows()).rev() {
            for col in 0..self.tiles.first_num_cols() {
                let tile = self.tiles.get(row, col).unwrap();
                if tile.is_rounded() {
                    let new_spot = self.find_last_empty_south((row, col).into());
                    self.move_tile(&(row, col).into(), &new_spot);
                }
            }
        }
    }

    pub fn tilt_west(&mut self) {
        for row in 0..self.tiles.num_rows() {
            for col in 0..self.tiles.first_num_cols() {
                let tile = self.tiles.get(row, col).unwrap();
                if tile.is_rounded() {
                    let new_spot = self.find_last_empty_west((row, col).into());
                    self.move_tile(&(row, col).into(), &new_spot);
                }
            }
        }
    }

    pub fn tilt_east(&mut self) {
        for row in 0..self.tiles.num_rows() {
            for col in (0..self.tiles.first_num_cols()).rev() {
                let tile = self.tiles.get(row, col).unwrap();
                if tile.is_rounded() {
                    let new_spot = self.find_last_empty_east((row, col).into());
                    self.move_tile(&(row, col).into(), &new_spot);
                }
            }
        }
    }

    pub fn cycle(&mut self) {
        self.tilt_north();
        self.tilt_west();
        self.tilt_south();
        self.tilt_east();
    }

    pub fn cycle_iterations(&mut self, iterations: usize) {
        let mut last_index: HashMap<Vec<RowCol>, usize> = HashMap::new();
        for i in 0..iterations {
            self.cycle();
            let positions: Vec<_> = self.get_rounded_locations().collect();
            let entry = last_index.entry(positions);
            match entry {
                Entry::Occupied(prev) => {
                    let prev = prev.get();
                    let cycle_length = i - prev;
                    let end_cycle_index = (iterations - i - 1) % cycle_length;

                    // slightly suboptimal but probably easier to just run the cycles manually
                    for _ in 0..end_cycle_index {
                        self.cycle();
                    }
                    return;
                }
                Entry::Vacant(entry) => {
                    entry.insert(i);
                }
            }
        }
    }

    fn move_tile(&mut self, start: &RowCol, end: &RowCol) {
        let tile = *self.tiles.get(start.row, start.col).unwrap();
        *self.tiles.get_mut(start.row, start.col).unwrap() = Tile::Empty;
        let dest = self.tiles.get_mut(end.row, end.col).unwrap();
        debug_assert!(dest.is_empty());
        *dest = tile;
    }

    pub fn get_weight(&self) -> usize {
        self.tiles
            .cells()
            .filter(|cell| cell.is_rounded())
            .map(|cell| self.tiles.num_rows() - cell.row)
            .sum()
    }

    fn find_last_empty_north(&self, row_col: RowCol) -> RowCol {
        let col = row_col.col;
        let row = (0..row_col.row)
            .rev()
            .find(|row| !self.tiles.get(*row, col).unwrap().is_empty())
            .map(|row| row + 1)
            .unwrap_or(0);
        RowCol { row, col }
    }
    fn find_last_empty_south(&self, row_col: RowCol) -> RowCol {
        let col = row_col.col;
        let row = ((row_col.row + 1)..self.tiles.num_rows())
            .find(|row| !self.tiles.get(*row, col).unwrap().is_empty())
            .map(|row| row - 1)
            .unwrap_or(self.tiles.num_rows() - 1);
        RowCol { row, col }
    }

    fn find_last_empty_west(&self, row_col: RowCol) -> RowCol {
        let row = row_col.row;
        let col = (0..row_col.col)
            .rev()
            .find(|&col| !self.tiles.get(row, col).unwrap().is_empty())
            .map(|col| col + 1)
            .unwrap_or(0);
        RowCol { row, col }
    }

    fn find_last_empty_east(&self, row_col: RowCol) -> RowCol {
        let row = row_col.row;
        let col = ((row_col.col + 1)..self.tiles.first_num_cols())
            .find(|&col| !self.tiles.get(row, col).unwrap().is_empty())
            .map(|col| col - 1)
            .unwrap_or(self.tiles.first_num_cols() - 1);
        RowCol { row, col }
    }

    pub fn get_rounded_locations(&self) -> impl Iterator<Item = RowCol> + '_ {
        self.tiles
            .cells()
            .filter(|cell| cell.is_rounded())
            .map(|cell| cell.coords())
    }
}

#[derive(Eq, PartialEq, Clone, Copy)]
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

impl Display for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile::Empty => f.write_char('.')?,
            Tile::Round => f.write_char('O')?,
            Tile::Square => f.write_char('#')?,
        };
        Ok(())
    }
}

impl Debug for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

#[cfg(test)]
mod test {
    use crate::common::Solution;
    use crate::day14::{Day14, Day14P2};

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

    #[test]
    fn test_example_p2() {
        assert_eq!(Day14P2::solve(EXAMPLE_INPUT.lines()), "64");
    }
}

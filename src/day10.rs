#![allow(dead_code)]

use crate::common::Solution;
use crate::vec2d::{Cell, Vec2d};

pub enum Day10 {}

impl Solution for Day10 {
    fn solve(lines: impl Iterator<Item = impl AsRef<str>>) -> String {
        let matrix = Vec2d::from_lines(lines);
        let matrix = matrix.map(|c| Tile::from(*c));
        (matrix.compute_loop_size() / 2).to_string()
    }
}

enum Tile {
    NorthSouth,
    EastWest,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
    Empty,
    Start,
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '.' => Tile::Empty,
            '|' => Tile::NorthSouth,
            '-' => Tile::EastWest,
            'L' => Tile::NorthEast,
            'J' => Tile::NorthWest,
            '7' => Tile::SouthWest,
            'F' => Tile::SouthEast,
            'S' => Tile::Start,
            value => panic!("Invalid char received: {}", value),
        }
    }
}

impl Tile {
    pub fn is_start(&self) -> bool {
        match self {
            Tile::Start => true,
            _ => false,
        }
    }
}

impl Vec2d<Tile> {
    pub fn find_start(&self) -> Cell<Tile> {
        self.cells().find(|c| c.value().is_start()).unwrap()
    }

    pub fn compute_loop_size(&self) -> usize {
        let mut count = 0;
        let mut cur = self.find_start();
        let mut prev = self.find_start();
        while !cur.value().is_start() || prev.eq(&cur) {
            let next_cells = cur.get_next_cells();
            let next_cell = next_cells.into_iter().filter(|cell| !prev.eq(cell)).next();
            match next_cell {
                None => {
                    return count + 1;
                }
                Some(next) => {
                    count += 1;
                    prev = cur.clone();
                    cur = self.get_cell(next.row(), next.col()).unwrap();
                }
            }
        }
        count
    }
}

impl<'a> Cell<'a, Tile> {
    fn get_next_cells(&'a self) -> Vec<Cell<Tile>> {
        match self.value() {
            Tile::NorthSouth => [self.get_top(), self.get_below()]
                .into_iter()
                .filter_map(|val| val)
                .collect(),
            Tile::EastWest => [self.get_left(), self.get_right()]
                .into_iter()
                .filter_map(|val| val)
                .collect(),
            Tile::NorthEast => [self.get_top(), self.get_right()]
                .into_iter()
                .filter_map(|val| val)
                .collect(),
            Tile::NorthWest => [self.get_top(), self.get_left()]
                .into_iter()
                .filter_map(|val| val)
                .collect(),
            Tile::SouthEast => [self.get_below(), self.get_right()]
                .into_iter()
                .filter_map(|val| val)
                .collect(),
            Tile::SouthWest => [self.get_below(), self.get_left()]
                .into_iter()
                .filter_map(|val| val)
                .collect(),
            Tile::Empty => vec![],
            Tile::Start => [
                self.get_top(),
                self.get_left(),
                self.get_right(),
                self.get_below(),
            ]
            .into_iter()
            .filter_map(|val| val)
            .filter(|val| !matches!(val.value(), Tile::Empty))
            .collect(),
        }
    }
}
#[cfg(test)]
mod test {
    use crate::common::Solution;
    use crate::day10::Day10;

    const SIMPLE_EXAMPLE_INPUT: &str = r#".....
.S-7.
.|.|.
.L-J.
....."#;

    const COMPLEX_EXAMPLE_INPUT: &str = r#"..F7.
.FJ|.
SJ.L7
|F--J
LJ..."#;

    #[test]
    fn test_simple_example() {
        assert_eq!(Day10::solve(SIMPLE_EXAMPLE_INPUT.lines()), "4");
    }
    #[test]
    fn test_complex_example() {
        assert_eq!(Day10::solve(COMPLEX_EXAMPLE_INPUT.lines()), "8");
    }
}

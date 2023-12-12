#![allow(dead_code)]

use crate::common::Solution;
use crate::vec2d::{Cell, Vec2d};

pub enum Day10 {}

impl Solution for Day10 {
    fn solve(lines: impl Iterator<Item = impl AsRef<str>>) -> String {
        let matrix = Vec2d::from_lines(lines);
        let matrix = matrix.map(|c| Tile::from(*c));
        ((matrix.compute_loop().len() + 1) / 2).to_string()
    }
}

pub enum Day10P2 {}

impl Solution for Day10P2 {
    fn solve(lines: impl Iterator<Item = impl AsRef<str>>) -> String {
        let matrix = Vec2d::from_lines(lines);
        let matrix = matrix.map(|c| Tile::from(*c));
        let boundary: Vec<_> = matrix.compute_loop();

        let boundary_size = boundary.len();
        let mut vertices: Vec<_> = boundary
            .into_iter()
            .filter(|v| v.value().is_vertex())
            .collect();

        // add start so that shoelace formula works
        vertices.push(matrix.find_start());

        // https://en.wikipedia.org/wiki/Shoelace_formula#Triangle_formula
        let shoelace_loop_area = vertices
            .windows(2)
            .map(|val| {
                let [a, b] = val else { panic!() };
                ((a.col() * b.row()) as f64) - ((a.row() * b.col()) as f64)
            })
            .sum::<f64>()
            .abs()
            / 2f64;

        // https://en.wikipedia.org/wiki/Pick%27s_theorem
        (shoelace_loop_area - (boundary_size as f64 / 2f64) + 1.0)
            .round()
            .to_string()
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
        matches!(self, Tile::Start)
    }

    pub fn is_vertex(&self) -> bool {
        matches!(
            self,
            Tile::NorthWest | Tile::NorthEast | Tile::SouthWest | Tile::SouthEast | Tile::Start
        )
    }
}

impl Vec2d<Tile> {
    pub fn find_start(&self) -> Cell<Tile> {
        self.cells().find(|c| c.value().is_start()).unwrap()
    }

    pub fn compute_loop(&self) -> Vec<Cell<Tile>> {
        let mut result = Vec::new();
        let mut cur = self.find_start();
        let mut prev = self.find_start();
        while !cur.value().is_start() || prev.eq(&cur) {
            result.push(cur.clone());
            let next_cells = cur.get_next_cells();
            let next_cell = next_cells.into_iter().find(|cell| !prev.eq(cell));
            match next_cell {
                None => {
                    return result;
                }
                Some(next) => {
                    prev = cur.clone();
                    cur = self.get_cell(next.row(), next.col()).unwrap();
                }
            }
        }
        result
    }
}

impl<'a> Cell<'a, Tile> {
    fn get_next_cells(&'a self) -> Vec<Cell<Tile>> {
        match self.value() {
            Tile::NorthSouth => [self.get_top(), self.get_below()]
                .into_iter()
                .flatten()
                .collect(),
            Tile::EastWest => [self.get_left(), self.get_right()]
                .into_iter()
                .flatten()
                .collect(),
            Tile::NorthEast => [self.get_top(), self.get_right()]
                .into_iter()
                .flatten()
                .collect(),
            Tile::NorthWest => [self.get_top(), self.get_left()]
                .into_iter()
                .flatten()
                .collect(),
            Tile::SouthEast => [self.get_below(), self.get_right()]
                .into_iter()
                .flatten()
                .collect(),
            Tile::SouthWest => [self.get_below(), self.get_left()]
                .into_iter()
                .flatten()
                .collect(),
            Tile::Empty => vec![],
            Tile::Start => [
                self.get_top(),
                self.get_left(),
                self.get_right(),
                self.get_below(),
            ]
            .into_iter()
            .flatten()
            .filter(|val| !matches!(val.value(), Tile::Empty))
            .collect(),
        }
    }
}
#[cfg(test)]
mod test {
    use crate::common::Solution;
    use crate::day10::{Day10, Day10P2};

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

    #[test]
    fn test_part2_example_basic() {
        let input = r#"...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
..........."#;

        assert_eq!(Day10P2::solve(input.lines()), "4")
    }

    #[test]
    fn test_part2_example_last() {
        let input = r#"FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L"#;
        assert_eq!(Day10P2::solve(input.lines()), "10")
    }
}

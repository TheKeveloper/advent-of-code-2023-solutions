use crate::common::Solution;
use crate::vec2d::{Cell, Vec2d};

pub enum Day10 {}

impl Solution for Day10 {
    fn solve(lines: impl Iterator<Item = impl AsRef<str>>) -> String {
        let matrix = Vec2d::from_lines(lines);
        let matrix = matrix.map(|c| Tile::from(*c));
        let mut distances: Vec2d<Option<usize>> = matrix.map(|_| None);
        let start = matrix.find_start();
        *distances.get_mut(start.row(), start.col()).unwrap() = Some(0);
        let mut stack: Vec<Cell<Tile>> = Vec::new();
        stack.push(start);

        while let Some(tile) = stack.pop() {
            let tile_distance = *distances
                .flat_get(tile.row(), tile.col())
                .expect("Everything in this loop should have a distance already");
            let directions = tile.get_next_directions();
            for (row_diff, col_diff) in directions {
                let next_cell = tile.get_diff(row_diff, col_diff);
                if let Some(next_cell) = next_cell {
                    let distance = distances.get_mut(next_cell.row(), next_cell.col()).unwrap();
                    if distance.is_none() {
                        *distance = Some(tile_distance + 2);
                        stack.push(matrix.get_cell(next_cell.row(), next_cell.col()).unwrap())
                    }
                }
            }
        }
        distances
            .cells()
            .filter_map(|cell| cell.value().as_ref().cloned())
            .max()
            .unwrap()
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
    /// whether this tile is a pipe that can connect to the tile above
    pub fn connects_above(&self) -> bool {
        match self {
            Tile::NorthSouth | Tile::NorthEast | Tile::NorthWest => true,
            _ => false,
        }
    }

    /// whether this tile is a pipe that can connect to the tile below it
    pub fn connects_below(&self) -> bool {
        match self {
            Tile::SouthEast | Tile::SouthWest | Tile::NorthSouth => true,
            _ => false,
        }
    }

    /// whether this tile is a pipe that can connect to the tile to its left
    pub fn connects_left(&self) -> bool {
        match self {
            Tile::SouthWest | Tile::NorthWest | Tile::EastWest => true,
            _ => false,
        }
    }

    /// whether this tile is a pipe that can connect to the tile to its right
    pub fn connects_right(&self) -> bool {
        match self {
            Tile::NorthEast | Tile::SouthEast | Tile::EastWest => true,
            _ => false,
        }
    }

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
}

impl<'a> Cell<'a, Tile> {
    fn get_next_directions(&'a self) -> Vec<(isize, isize)> {
        let mut directions: Vec<(isize, isize)> = Vec::with_capacity(4);

        if let Some(top) = self.get_top() {
            match top.value() {
                Tile::NorthSouth => directions.push((-2, 0)),
                Tile::SouthEast => directions.push((-1, 1)),
                Tile::SouthWest => directions.push((-1, -1)),
                _ => {}
            };
        }

        if let Some(below) = self.get_below() {
            match below.value() {
                Tile::NorthSouth => directions.push((2, 0)),
                Tile::NorthEast => directions.push((1, 1)),
                Tile::NorthWest => directions.push((1, -1)),
                _ => {}
            };
        }

        if let Some(left) = self.get_left() {
            match left.value() {
                Tile::EastWest => directions.push((0, -2)),
                Tile::NorthEast => directions.push((-1, -1)),
                Tile::SouthEast => directions.push((1, -1)),
                _ => {}
            };
        }

        if let Some(right) = self.get_right() {
            match right.value() {
                Tile::EastWest => directions.push((0, 2)),
                Tile::NorthWest => directions.push((-1, 1)),
                Tile::SouthWest => directions.push((1, 1)),
                _ => {}
            };
        }

        directions
    }
}
#[cfg(test)]
mod test {
    use crate::common::Solution;
    use crate::day10::Day10;

    const SIMPLE_EXAMPLE_INPUT: &'static str = r#".....
.S-7.
.|.|.
.L-J.
....."#;

    const COMPLEX_EXAMPLE_INPUT: &'static str = r#"..F7.
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

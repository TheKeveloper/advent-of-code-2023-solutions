use std::collections::HashSet;

use crate::common::Solution;
use crate::vec2d::{Cell, Vec2d};

pub enum Day21 {}

impl Solution for Day21 {
    fn solve(lines: impl Iterator<Item = impl AsRef<str>>) -> String {
        let grid = Grid::from_lines(lines);
        grid.start_reachable_n_steps(64).to_string()
    }
}

pub enum Day21P2 {}
impl Solution for Day21P2 {
    fn solve(lines: impl Iterator<Item = impl AsRef<str>>) -> String {
        let grid = InfiniteGrid::from_lines(lines);

        let grid_len = grid.tiles.num_rows();

        let mut reachable = HashSet::new();
        reachable.insert(grid.get_start().into());

        // the total steps is 26501365 = grid_len * 202300 + 65
        // we want to get past the first 65 steps and then use a quadratic equation at each grid
        // boundary. The quadratic pattern is from someone on reddit.
        for i in 0..=327 {
            if (i - 65) % grid_len == 0 {
                println!("{}, {}", i, reachable.len());
            }
            reachable = grid.one_step_reachable(reachable.iter());
        }

        // take the printed values, use an online quadratic regression, and with values 1, 2, 3
        // then put in 202300 as the input.

        "".to_string()
    }
}

struct Grid {
    tiles: Vec2d<Tile>,
}

impl Grid {
    pub fn from_lines(lines: impl Iterator<Item = impl AsRef<str>>) -> Self {
        Grid {
            tiles: Vec2d::from_lines(lines).map(|&c| c.into()),
        }
    }

    pub fn get_start(&self) -> Cell<Tile> {
        self.tiles
            .cells()
            .find(|cell| cell.value().is_start())
            .unwrap()
    }

    pub fn one_step_reachable<'a>(
        &'a self,
        tiles: impl Iterator<Item = Cell<'a, Tile>>,
    ) -> HashSet<Cell<'a, Tile>> {
        let mut set = HashSet::new();
        for tile in tiles {
            tile.cardinal_neighbors()
                .filter(|cell| !cell.is_rock())
                .for_each(|cell| {
                    set.insert(self.tiles.get_cell(cell.row(), cell.col()).unwrap());
                });
        }
        set
    }

    pub fn start_reachable_n_steps(&self, steps: usize) -> usize {
        let mut reachable = HashSet::new();
        reachable.insert(self.get_start());

        for _ in 0..steps {
            reachable = self.one_step_reachable(reachable.iter().cloned());
        }

        reachable.len()
    }
}

struct InfiniteGrid {
    tiles: Vec2d<Tile>,
}

impl InfiniteGrid {
    pub fn from_lines(lines: impl Iterator<Item = impl AsRef<str>>) -> Self {
        InfiniteGrid {
            tiles: Vec2d::from_lines(lines).map(|&c| c.into()),
        }
    }

    pub fn get_start(&self) -> Cell<Tile> {
        self.tiles
            .cells()
            .find(|cell| cell.value().is_start())
            .unwrap()
    }

    pub fn one_step_reachable<'a>(
        &self,
        points: impl Iterator<Item = &'a Point>,
    ) -> HashSet<Point> {
        let mut set = HashSet::new();
        for point in points {
            point
                .neighbors()
                .iter()
                .filter(|p| !self.get_value_on_grid(p).is_rock())
                .for_each(|&p| {
                    set.insert(p);
                })
        }
        set
    }

    pub fn start_reachable_n_steps(&self, steps: usize) -> usize {
        let mut reachable = HashSet::new();
        reachable.insert(self.get_start().into());

        for _ in 0..steps {
            reachable = self.one_step_reachable(reachable.iter());
        }

        reachable.len()
    }

    fn get_value_on_grid(&self, point: &Point) -> Tile {
        let row = point.row.rem_euclid(self.tiles.num_rows() as i64);
        let col = point.col.rem_euclid(self.tiles.first_num_cols() as i64);
        self.tiles.get(row as usize, col as usize).unwrap().clone()
    }
}

#[derive(Eq, PartialEq, Hash, Clone, Copy)]
struct Point {
    row: i64,
    col: i64,
}

impl Point {
    pub fn neighbors(&self) -> [Point; 4] {
        [self.above(), self.right(), self.below(), self.left()]
    }

    fn diff(&self, row_diff: i64, col_diff: i64) -> Point {
        Point {
            row: self.row + row_diff,
            col: self.col + col_diff,
        }
    }

    fn above(&self) -> Point {
        self.diff(-1, 0)
    }

    fn below(&self) -> Point {
        self.diff(1, 0)
    }

    fn left(&self) -> Point {
        self.diff(0, -1)
    }

    fn right(&self) -> Point {
        self.diff(0, 1)
    }
}

impl<'a, T> From<Cell<'a, T>> for Point {
    fn from(value: Cell<'a, T>) -> Self {
        Point {
            row: value.row() as i64,
            col: value.col() as i64,
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum Tile {
    Starting,
    Garden,
    Rock,
}

impl Tile {
    pub fn is_start(&self) -> bool {
        matches!(self, Tile::Starting)
    }

    pub fn is_rock(&self) -> bool {
        matches!(self, Tile::Rock)
    }
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            'S' => Tile::Starting,
            '.' => Tile::Garden,
            '#' => Tile::Rock,
            _ => panic!("Could not convert char to tile: {}", value),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::day21::{Grid, InfiniteGrid};

    const EXAMPLE_INPUT: &str = r"...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........";
    #[test]
    fn test_example() {
        let grid = Grid::from_lines(EXAMPLE_INPUT.lines());
        assert_eq!(grid.start_reachable_n_steps(6), 16);
    }

    #[test]
    fn test_example_p2() {
        let grid = InfiniteGrid::from_lines(EXAMPLE_INPUT.lines());
        assert_eq!(grid.start_reachable_n_steps(6), 16);
        assert_eq!(grid.start_reachable_n_steps(10), 50);
        assert_eq!(grid.start_reachable_n_steps(50), 1594);
    }
}

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
        panic!(
            "lines: {:?}",
            lines.map(|s| s.as_ref().to_string()).collect::<Vec<_>>()
        )
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
    use crate::common::Solution;
    use crate::day21::{Day21P2, Grid};

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
    #[should_panic]
    fn test_example_p2() {
        assert_eq!(Day21P2::solve(EXAMPLE_INPUT.lines()), "")
    }
}

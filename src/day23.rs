use std::collections::HashSet;

use crate::common::Solution;
use crate::vec2d::{Cell, Direction, RowCol, Vec2d};

pub enum Day23 {}

impl Solution for Day23 {
    fn solve(lines: impl Iterator<Item = impl AsRef<str>>) -> String {
        let trail = Trail::from_lines(lines);
        trail.get_max_path().to_string()
    }
}

pub enum Day23P2 {}
impl Solution for Day23P2 {
    fn solve(lines: impl Iterator<Item = impl AsRef<str>>) -> String {
        panic!(
            "lines: {:?}",
            lines.map(|s| s.as_ref().to_string()).collect::<Vec<_>>()
        )
    }
}

struct Trail {
    tiles: Vec2d<Tile>,
}

impl Trail {
    pub fn from_lines(lines: impl Iterator<Item = impl AsRef<str>>) -> Trail {
        let grid = Vec2d::from_lines(lines);
        Trail {
            tiles: grid.map(|&c| c.into()),
        }
    }

    pub fn get_start(&self) -> RowCol {
        self.tiles
            .get_row(0)
            .unwrap()
            .iter()
            .enumerate()
            .find(|(_, &value)| value.eq(&Tile::Empty))
            .map(|(col, _)| RowCol { row: 0, col })
            .unwrap()
    }

    pub fn get_max_path(&self) -> usize {
        self.get_max_path_inner(&self.get_start(), HashSet::new())
    }

    fn get_max_path_inner(&self, start: &RowCol, mut visited: HashSet<RowCol>) -> usize {
        if !visited.insert(start.clone()) {
            return 0;
        }

        let Some(cell) = self.tiles.get_cell(start.row, start.col) else {
            return 0;
        };

        cell.next_tiles()
            .into_iter()
            .filter(|coords| !&visited.contains(coords))
            .map(|coords| self.tiles.get_cell(coords.row, coords.col))
            .flatten()
            .map(|cell| self.get_max_path_inner(&cell.coords(), visited.clone()))
            .max()
            .map(|val| val + 1)
            .unwrap_or(0)
    }
}

#[derive(Eq, PartialEq, Hash, Clone, Copy)]
enum Tile {
    Empty,
    Forest,
    Slope(Direction),
}

impl Tile {
    pub fn is_empty(&self) -> bool {
        matches!(self, Tile::Empty)
    }

    pub fn is_forest(&self) -> bool {
        matches!(self, Tile::Forest)
    }
}

impl<'a> Cell<'a, Tile> {
    pub fn is_end(&self) -> bool {
        self.row == self.parent.num_rows() - 1 && self.is_empty()
    }

    pub fn next_tiles(&self) -> Vec<RowCol> {
        match self.value() {
            Tile::Forest => vec![],
            Tile::Empty => self
                .cardinal_neighbors()
                .filter(|cell| !cell.is_forest())
                .map(|cell| cell.coords())
                .collect(),
            Tile::Slope(direction) => direction
                .next(self.coords())
                .into_iter()
                .filter(|coords| self.parent.get_row_col(coords).is_some())
                .collect(),
        }
    }
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '.' => Tile::Empty,
            '#' => Tile::Forest,
            '>' => Tile::Slope(Direction::Right),
            '<' => Tile::Slope(Direction::Left),
            'v' => Tile::Slope(Direction::Down),
            '^' => Tile::Slope(Direction::Up),
            _ => panic!("Invalid char to tile: {}", value),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::common::Solution;
    use crate::day23::{Day23, Day23P2};

    const EXAMPLE_INPUT: &str = r"#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#";
    #[test]
    fn test_example() {
        assert_eq!(Day23::solve(EXAMPLE_INPUT.lines()), "94")
    }

    #[test]
    #[should_panic]
    fn test_example_p2() {
        assert_eq!(Day23P2::solve(EXAMPLE_INPUT.lines()), "")
    }
}

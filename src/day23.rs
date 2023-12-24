use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;

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
        let trail = Trail::from_lines(lines);
        trail.get_max_path_p2().to_string()
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

        if cell.is_end() {
            return 0;
        }

        cell.next_tiles()
            .into_iter()
            .filter(|coords| !&visited.contains(coords))
            .flat_map(|coords| self.tiles.get_cell(coords.row, coords.col))
            .map(|cell| self.get_max_path_inner(&cell.coords(), visited.clone()))
            .max()
            .map(|val| val + 1)
            .unwrap_or(0)
    }

    pub fn get_max_path_p2(&self) -> usize {
        self.get_max_path_inner_p2(
            &self.get_start(),
            Rc::new(RefCell::new(self.tiles.map(|_| false))),
        )
        .unwrap()
    }

    fn get_max_path_inner_p2(
        &self,
        start: &RowCol,
        visited: Rc<RefCell<Vec2d<bool>>>,
    ) -> Option<usize> {
        let Some(cell) = self.tiles.get_cell(start.row, start.col) else {
            return None;
        };

        if cell.is_end() {
            return Some(0);
        }

        {
            let mut visited = visited.borrow_mut();
            let cur_visited = visited.get_mut(start.row, start.col).unwrap();
            if *cur_visited {
                return None;
            } else {
                *cur_visited = true;
            }
        }

        let result = cell
            .next_tiles_p2()
            .into_iter()
            .filter(|coords| !visited.borrow().get_row_col(coords).unwrap())
            .flat_map(|coords| self.tiles.get_cell(coords.row, coords.col))
            .map(|cell| self.get_max_path_inner_p2(&cell.coords(), visited.clone()))
            .flatten()
            .max()
            .map(|val| val + 1);
        *visited.borrow_mut().get_mut(start.row, start.col).unwrap() = false;
        result
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

    pub fn next_tiles_p2(&self) -> Vec<RowCol> {
        match self.value() {
            Tile::Forest => unreachable!(),
            Tile::Empty | Tile::Slope(_) => self
                .cardinal_neighbors()
                .filter(|cell| !cell.is_forest())
                .map(|cell| cell.coords())
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
    fn test_example_p2() {
        assert_eq!(Day23P2::solve(EXAMPLE_INPUT.lines()), "154")
    }
}
